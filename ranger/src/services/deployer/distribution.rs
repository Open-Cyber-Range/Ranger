use super::{
    super::client::{
        DeploymentClient, DeploymentInfo, SwitchClient, TemplateClient, VirtualMachineClient,
    },
    factory::{CreateDeployer, DeployerFactory},
};
use crate::services::{
    client::{DeploymentClientResponse, FeatureClient},
    deployer::DeployerConnections,
};
use actix::{
    Actor, ActorFutureExt, Addr, Context, Handler, Message, ResponseActFuture, WrapFuture,
};
use anyhow::{anyhow, Ok, Result};
use futures::future::try_join_all;
use ranger_grpc::capabilities::DeployerTypes;
use std::collections::HashMap;

#[derive(Clone)]
pub struct DeployerDistribution {
    deployers: HashMap<String, DeployerConnections>,
    usage: HashMap<String, usize>,
}

impl Actor for DeployerDistribution {
    type Context = Context<Self>;
}

type ClientTuple = (Box<dyn DeploymentClient<Box<dyn DeploymentInfo>>>, String);

impl DeployerDistribution {
    fn book_best_deployer(
        &mut self,
        potential_deployers: Vec<String>,
        deployer_type: DeployerTypes,
    ) -> Result<String> {
        let acceptable_deployers = self.deployers.iter().filter_map(|(key, value)| {
            if potential_deployers.contains(key)
                && (deployer_type == DeployerTypes::VirtualMachine
                    && value.virtual_machine_client.is_some()
                    || deployer_type == DeployerTypes::Switch && value.switch_client.is_some()
                    || deployer_type == DeployerTypes::Template && value.template_client.is_some()
                    || deployer_type == DeployerTypes::Feature && value.feature_client.is_some())
            {
                return Some(key.to_string());
            }
            None
        });

        let best_deployer = acceptable_deployers
            .into_iter()
            .min_by_key(|key| self.usage.get(key).unwrap_or(&0))
            .ok_or_else(|| anyhow!("No deployer found"))?;
        self.usage
            .entry(best_deployer.clone())
            .and_modify(|e| *e += 1)
            .or_insert(1);
        Ok(best_deployer)
    }

    fn release_deployer(&mut self, deployer_name: &str) {
        self.usage
            .entry(deployer_name.to_string())
            .and_modify(|e| *e -= 1)
            .or_insert(0);
    }

    fn release_deployer_closure<T>(
        response: Result<(T, String)>,
        actor: &mut DeployerDistribution,
        _ctx: &mut Context<DeployerDistribution>,
    ) -> Result<T> {
        let (value, deployer_name) = response?;
        actor.release_deployer(&deployer_name);
        Ok(value)
    }

    fn get_client(
        &mut self,
        potential_deployers: Vec<String>,
        deployer_type: DeployerTypes,
    ) -> Result<ClientTuple>
    where
        actix::Addr<VirtualMachineClient>: DeploymentClient<Box<dyn DeploymentInfo>>,
        actix::Addr<SwitchClient>: DeploymentClient<Box<dyn DeploymentInfo>>,
        actix::Addr<TemplateClient>: DeploymentClient<Box<dyn DeploymentInfo>>,
        actix::Addr<FeatureClient>: DeploymentClient<Box<dyn DeploymentInfo>>,
    {
        let best_deployer = self.book_best_deployer(potential_deployers, deployer_type)?;
        let connections = self
            .deployers
            .get(&best_deployer)
            .ok_or_else(|| anyhow!("No deployer found"))?;
        Ok((
            match deployer_type {
                DeployerTypes::Template => Box::new(
                    connections
                        .template_client
                        .clone()
                        .ok_or_else(|| anyhow!("No template deployer found"))?,
                ),
                DeployerTypes::Switch => Box::new(
                    connections
                        .switch_client
                        .clone()
                        .ok_or_else(|| anyhow!("No node deployer found"))?,
                ),
                DeployerTypes::VirtualMachine => Box::new(
                    connections
                        .virtual_machine_client
                        .clone()
                        .ok_or_else(|| anyhow!("No node deployer found"))?,
                ),
                DeployerTypes::Feature => Box::new(
                    connections
                        .feature_client
                        .clone()
                        .ok_or_else(|| anyhow!("No feature deployer found"))?,
                ),
                DeployerTypes::Condition => todo!("Add condition client"),
                DeployerTypes::Inject => todo!("Add inject client"),
            },
            best_deployer,
        ))
    }

    pub async fn new(factory: Addr<DeployerFactory>, deployers: Vec<String>) -> Result<Self> {
        let deployers = try_join_all(deployers.iter().map(|deployer_name| async {
            let connections = factory
                .send(CreateDeployer(deployer_name.to_string()))
                .await??;
            Ok((deployer_name.to_string(), connections))
        }))
        .await?;

        Ok(Self {
            deployers: HashMap::from_iter(deployers),
            usage: HashMap::new(),
        })
    }
}

#[derive(Message)]
#[rtype(result = "Result<DeploymentClientResponse>")]
pub struct Deploy(
    pub DeployerTypes,
    pub Box<dyn DeploymentInfo>,
    pub Vec<String>,
);

impl Handler<Deploy> for DeployerDistribution {
    type Result = ResponseActFuture<Self, Result<DeploymentClientResponse>>;

    fn handle(&mut self, msg: Deploy, _ctx: &mut Self::Context) -> Self::Result {
        let deployment_type = msg.0;
        let deployment = msg.1;
        let potential_deployers = msg.2;

        let client_result = self.get_client(potential_deployers, deployment_type);

        Box::pin(
            async move {
                let (mut deployment_client, best_deployer) = client_result?;
                let client_response = deployment_client.deploy(deployment).await?;

                Ok((client_response, best_deployer))
            }
            .into_actor(self)
            .map(Self::release_deployer_closure),
        )
    }
}

#[derive(Message)]
#[rtype(result = "Result<()>")]
pub struct UnDeploy(pub DeployerTypes, pub String, pub Vec<String>);

impl Handler<UnDeploy> for DeployerDistribution {
    type Result = ResponseActFuture<Self, Result<()>>;

    fn handle(&mut self, msg: UnDeploy, _ctx: &mut Self::Context) -> Self::Result {
        let deployment_type = msg.0;
        let handler_reference_id = msg.1;
        let potential_deployers = msg.2;

        let client_result = self.get_client(potential_deployers, deployment_type);

        Box::pin(
            async move {
                let (mut deployment_client, best_deployer) = client_result?;
                deployment_client.undeploy(handler_reference_id).await?;

                Ok(((), best_deployer))
            }
            .into_actor(self)
            .map(Self::release_deployer_closure),
        )
    }
}
