mod ledger;
mod node;
mod template;

use super::deployer::DeployerDistribution;
use crate::services::deployment::template::DeployableTemplates;
use crate::services::scheduler::Scheduler;
use crate::{models::Deployment, services::deployment::node::DeployableNodes};
use actix::{
    Actor, ActorFutureExt, Addr, Context, Handler, Message, ResponseActFuture, WrapFuture,
};
use anyhow::{anyhow, Ok, Result};
use ledger::Ledger;
use log::{error, info};
use sdl_parser::Scenario;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct DeploymentManager {
    scheduler: Addr<Scheduler>,
    distributor: Addr<DeployerDistribution>,
    deployment_group: HashMap<String, Vec<String>>,
    default_deployment_group: String,
}

impl DeploymentManager {
    pub fn new(
        scheduler: Addr<Scheduler>,
        distributor: Addr<DeployerDistribution>,
        real_deployment_group: HashMap<String, Vec<String>>,
        default_deployment_group: String,
    ) -> Self {
        Self {
            scheduler,
            distributor,
            deployment_group: real_deployment_group,
            default_deployment_group,
        }
    }

    async fn deploy(
        deployers: &[String],
        scenario: &Scenario,
        scheduler_address: &Addr<Scheduler>,
        distributor_address: &Addr<DeployerDistribution>,
        exercise_name: &str,
        deployment: &Deployment,
    ) -> Result<Addr<Ledger>> {
        let ledger = Ledger::new().start();
        scenario
            .deploy_templates(distributor_address, deployers, &ledger)
            .await?;
        scenario
            .deploy_nodes(
                distributor_address,
                scheduler_address,
                &ledger,
                exercise_name,
                deployment,
                deployers,
            )
            .await?;

        info!("Deployment {} successful", deployment.name);
        Ok(ledger)
    }
}

impl Actor for DeploymentManager {
    type Context = Context<Self>;
}

#[derive(Message, Debug)]
#[rtype(result = "Result<()>")]
pub struct StartDeployment(
    pub(crate) Scenario,
    pub(crate) Deployment,
    pub(crate) String,
);

impl Handler<StartDeployment> for DeploymentManager {
    type Result = ResponseActFuture<Self, Result<()>>;

    fn handle(&mut self, msg: StartDeployment, _: &mut Context<Self>) -> Self::Result {
        let StartDeployment(scenario, deployment, exercise_name) = msg;
        let requested_deployer_group_name = deployment
            .deployment_group
            .clone()
            .unwrap_or_else(|| self.default_deployment_group.clone());

        let deployers_result = self
            .deployment_group
            .get(requested_deployer_group_name.as_str())
            .ok_or_else(|| {
                anyhow!(
                    "No deployment group found for {}",
                    requested_deployer_group_name
                )
            })
            .cloned();
        info!("Using deployment group: {}", &requested_deployer_group_name);
        let scheduler_address = self.scheduler.clone();
        let distributor_address = self.distributor.clone();

        Box::pin(
            async move {
                let deployers = deployers_result?;
                DeploymentManager::deploy(
                    &deployers,
                    &scenario,
                    &scheduler_address,
                    &distributor_address,
                    &exercise_name,
                    &deployment,
                )
                .await
            }
            .into_actor(self)
            .map(move |result, _act, _| {
                if result.is_err() {
                    error!("Deployment failed: {:?}", result.as_ref().err());
                    return Err(anyhow!("Deployment failed"));
                }
                Ok(())
            }),
        )
    }
}

#[derive(Message, Debug)]
#[rtype(result = "()")]
pub struct TearDownDeployment(pub(crate) String);
