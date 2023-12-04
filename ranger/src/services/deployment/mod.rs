pub(crate) mod condition;
pub(crate) mod event;
pub mod event_info;
mod feature;
mod inject;
mod node;
mod template;

use self::node::RemoveableNodes;
use super::database::{deployment::GetDeploymentElementByDeploymentId, Database};
use crate::{
    models::{helpers::uuid::Uuid, Deployment, Exercise},
    services::deployment::{
        condition::DeployableConditions, event_info::EventInfoUnpacker,
        feature::DeployableFeatures, node::DeployableNodes,
    },
    services::deployment::{event::DeployableEvents, template::DeployableTemplates},
    Addressor,
};
use actix::{Actor, ActorFutureExt, Context, Handler, Message, ResponseActFuture, WrapFuture};
use anyhow::{anyhow, Ok, Result};
use log::{error, info};
use sdl_parser::Scenario;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct DeploymentManager {
    addressor: Addressor,
    deployment_group: HashMap<String, Vec<String>>,
    default_deployment_group: String,
}

impl DeploymentManager {
    pub fn new(
        addressor: Addressor,
        deployment_group: HashMap<String, Vec<String>>,
        default_deployment_group: String,
    ) -> Self {
        Self {
            addressor,
            deployment_group,
            default_deployment_group,
        }
    }

    async fn deploy(
        deployers: &[String],
        scenario: &Scenario,
        addressor: &Addressor,
        exercise: &Exercise,
        deployment: &Deployment,
    ) -> Result<()> {
        scenario
            .deploy_templates(addressor, deployers, deployment, exercise)
            .await?;
        let deployed_nodes = scenario
            .deploy_nodes(addressor, exercise, deployment, deployers)
            .await?;

        scenario
            .deploy_scenario_features(addressor, exercise, deployers, &deployed_nodes)
            .await?;

        let nodes_with_conditions = scenario
            .create_events(addressor, &deployed_nodes, deployment)
            .await?;

        scenario
            .deploy_scenario_conditions(addressor, exercise, deployers, &nodes_with_conditions)
            .await?;

        scenario
            .create_event_info_pages(addressor, deployers, &nodes_with_conditions)
            .await?;

        scenario
            .deploy_event_pollers(addressor, exercise, deployers, &nodes_with_conditions)
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
        let addressor = self.addressor.clone();

        info!("Starting new Deployment {:?}", deployment.name);
        Box::pin(
            async move {
                let deployers = deployers_result?;
                DeploymentManager::deploy(&deployers, &scenario, &addressor, &exercise, &deployment)
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
        let addressor = self.addressor.clone();
        let deployers_result = self.get_deployers(&deployment);
        Box::pin(
            async move {
                let deployers = deployers_result?;
                let deployment_elements = addressor
                    .database
                    .send(GetDeploymentElementByDeploymentId(deployment.id, false))
                    .await??;
                deployment_elements
                    .undeploy_nodes(&addressor, &deployers, &exercise_id)
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

#[derive(Message, Debug)]
#[rtype(result = "Result<Vec<String>>")]
pub struct GetDefaultDeployers();

impl Handler<GetDefaultDeployers> for DeploymentManager {
    type Result = ResponseActFuture<Self, Result<Vec<String>>>;

    fn handle(&mut self, _msg: GetDefaultDeployers, _: &mut Context<Self>) -> Self::Result {
        let deployment_groups = self
            .deployment_group
            .get(&self.default_deployment_group)
            .cloned();

        Box::pin(
            async move {
                let default_deployment_group = deployment_groups
                    .ok_or_else(|| anyhow!("No default deployment group found"))?;

                Ok(default_deployment_group)
            }
            .into_actor(self),
        )
    }
}
