mod node;
mod template;

use self::node::RemoveableNodes;

use super::{
    database::{deployment::GetDeploymentElementByDeploymentId, Database},
    deployer::DeployerDistribution,
};
use crate::{
    models::{helpers::uuid::Uuid, Deployment, Exercise},
    services::deployment::node::DeployableNodes,
    services::{deployment::template::DeployableTemplates, scheduler::Scheduler},
};
use actix::{
    Actor, ActorFutureExt, Addr, Context, Handler, Message, ResponseActFuture, WrapFuture,
};
use anyhow::{anyhow, Ok, Result};
use log::{error, info};
use sdl_parser::Scenario;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct DeploymentManager {
    scheduler: Addr<Scheduler>,
    distributor: Addr<DeployerDistribution>,
    database: Addr<Database>,
    deployment_group: HashMap<String, Vec<String>>,
    default_deployment_group: String,
}

impl DeploymentManager {
    pub fn new(
        scheduler: Addr<Scheduler>,
        distributor: Addr<DeployerDistribution>,
        database: Addr<Database>,
        deployment_group: HashMap<String, Vec<String>>,
        default_deployment_group: String,
    ) -> Self {
        Self {
            scheduler,
            distributor,
            database,
            deployment_group,
            default_deployment_group,
        }
    }

    async fn deploy(
        deployers: &[String],
        scenario: &Scenario,
        scheduler_address: &Addr<Scheduler>,
        distributor_address: &Addr<DeployerDistribution>,
        database_address: &Addr<Database>,
        exercise: &Exercise,
        deployment: &Deployment,
    ) -> Result<()> {
        scenario
            .deploy_templates(
                distributor_address,
                deployers,
                database_address,
                deployment,
                exercise,
            )
            .await?;
        scenario
            .deploy_nodes(
                distributor_address,
                scheduler_address,
                database_address,
                exercise,
                deployment,
                deployers,
            )
            .await?;

        info!("Deployment {} successful", deployment.name);
        Ok(())
    }

    fn get_deployment_group(&self, deployment: &Deployment) -> String {
        let name = deployment
            .deployment_group
            .clone()
            .unwrap_or_else(|| self.default_deployment_group.clone());
        log::debug!("Using deployment group: {}", &name);
        name
    }

    fn get_deployers(&self, deployment: &Deployment) -> Result<Vec<String>> {
        let group = self.get_deployment_group(deployment);
        self.deployment_group
            .get(group.as_str())
            .ok_or_else(|| anyhow!("No deployment group found for {}", group))
            .cloned()
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
    pub(crate) Exercise,
);

impl Handler<StartDeployment> for DeploymentManager {
    type Result = ResponseActFuture<Self, Result<()>>;

    fn handle(&mut self, msg: StartDeployment, _: &mut Context<Self>) -> Self::Result {
        let StartDeployment(scenario, deployment, exercise) = msg;

        let deployers_result = self.get_deployers(&deployment);
        let scheduler_address = self.scheduler.clone();
        let distributor_address = self.distributor.clone();
        let database_address = self.database.clone();

        Box::pin(
            async move {
                let deployers = deployers_result?;
                DeploymentManager::deploy(
                    &deployers,
                    &scenario,
                    &scheduler_address,
                    &distributor_address,
                    &database_address,
                    &exercise,
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
#[rtype(result = "Result<()>")]
pub struct RemoveDeployment(pub Uuid, pub(crate) Deployment);

impl Handler<RemoveDeployment> for DeploymentManager {
    type Result = ResponseActFuture<Self, Result<()>>;

    fn handle(&mut self, msg: RemoveDeployment, _: &mut Context<Self>) -> Self::Result {
        let RemoveDeployment(exercise_id, deployment) = msg;
        let database_address = self.database.clone();
        let distributor_address = self.distributor.clone();
        let deployers_result = self.get_deployers(&deployment);
        Box::pin(
            async move {
                let deployers = deployers_result?;
                let deployment_elements = database_address
                    .send(GetDeploymentElementByDeploymentId(deployment.id))
                    .await??;
                deployment_elements
                    .undeploy_nodes(
                        &distributor_address,
                        &database_address,
                        &deployers,
                        &exercise_id,
                    )
                    .await?;
                Ok(())
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
