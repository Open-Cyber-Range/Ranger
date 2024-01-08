use super::node::DeployedNode;
use super::Database;
use crate::models::helpers::uuid::Uuid;
use crate::models::{DeploymentElement, ElementStatus, Exercise};
use crate::services::client::{ConditionResponse, ConditionStream};
use crate::services::database::account::GetAccount;
use crate::services::database::deployment::{CreateDeploymentElement, UpdateDeploymentElement};
use crate::services::deployer::Deploy;
use crate::utilities::scenario::get_metric_by_condition;
use crate::utilities::try_some;
use crate::Addressor;
use actix::{Actor, Addr, Context, Handler, Message, ResponseActFuture, WrapFuture};
use anyhow::{anyhow, Ok, Result};
use async_trait::async_trait;
use futures::future::try_join_all;
use log::debug;
use ranger_grpc::capabilities::DeployerType as GrpcDeployerType;
use ranger_grpc::{Account as GrpcAccount, Condition as GrpcCondition, Source as GrpcSource};
use sdl_parser::condition::Condition;
use sdl_parser::inject::Inject;
use sdl_parser::metric::Metrics;
use sdl_parser::{node::Role, Scenario};
use serde::{Deserialize, Serialize};

#[async_trait]
pub trait DeployableConditions {
    async fn deploy_scenario_conditions(
        &self,
        addressor: &Addressor,
        exercise: &Exercise,
        deployers: &[String],
        deployed_nodes: &[(DeployedNode, Vec<ConditionProperties>)],
    ) -> Result<()>;
}
#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct ConditionProperties {
    pub name: String,
    pub condition: Condition,
    pub role: Role,
    pub event_id: Option<Uuid>,
    pub event_name: Option<String>,
    pub injects: Option<Vec<(String, Inject, Role)>>,
}

#[async_trait]
impl DeployableConditions for Scenario {
    async fn deploy_scenario_conditions(
        &self,
        addressor: &Addressor,
        exercise: &Exercise,
        deployers: &[String],
        deployed_nodes: &[(DeployedNode, Vec<ConditionProperties>)],
    ) -> Result<()> {
        if self.conditions.is_some() {
            try_join_all(
                deployed_nodes
                    .iter()
                    .map(|(deployed_node, conditions)| async move {
                        let DeployedNode {
                            deployment_element,
                            template_id,
                            ..
                        } = deployed_node;
                        debug!(
                            "Deploying conditions for Node: {:?}",
                            deployment_element.scenario_reference
                        );
                        addressor.condition_aggregator.do_send(DeployConditions {
                            addressor: addressor.clone(),
                            deployers: deployers.to_owned(),
                            condition_properties: conditions.clone(),
                            deployment_element: deployment_element.clone(),
                            exercise_id: exercise.id,
                            template_id: *template_id,
                            metrics: self.metrics.clone(),
                        });

                        Ok(())
                    }),
            )
            .await?;
        }
        Ok(())
    }
}

pub struct ConditionAggregator();

impl ConditionAggregator {
    pub fn new() -> Self {
        Self()
    }
}

impl Default for ConditionAggregator {
    fn default() -> Self {
        Self::new()
    }
}

impl Actor for ConditionAggregator {
    type Context = Context<Self>;
}

#[derive(Message)]
#[rtype(result = "Result<()>")]
pub struct DeployConditions {
    pub addressor: Addressor,
    pub deployers: Vec<String>,
    pub condition_properties: Vec<ConditionProperties>,
    pub deployment_element: DeploymentElement,
    pub exercise_id: Uuid,
    pub template_id: Uuid,
    pub metrics: Option<Metrics>,
}

impl Handler<DeployConditions> for ConditionAggregator {
    type Result = ResponseActFuture<Self, Result<()>>;

    fn handle(&mut self, msg: DeployConditions, _ctx: &mut Self::Context) -> Self::Result {
        Box::pin(
            async move {
                let DeployConditions {
                    addressor,
                    deployers,
                    condition_properties: conditions,
                    deployment_element,
                    exercise_id,
                    template_id,
                    metrics,
                } = &msg;

                try_join_all(
                    conditions
                        .iter()
                        .map(|condition_properties| async move {
                            let ConditionProperties {
                                name: condition_name,
                                condition,
                                role: condition_role,
                                event_id,
                                event_name: _,
                                injects: _,
                            } = condition_properties;
                            let virtual_machine_id_string = try_some(
                                deployment_element.clone().handler_reference,
                                "Deployment element handler reference not found",
                            )?;
                            let virtual_machine_id =
                                Uuid::try_from(virtual_machine_id_string.as_str())?;

                            if condition_name.eq_ignore_ascii_case(condition_name) {
                                debug!(
                                    "Deploying condition '{condition_name}' for VM '{node_name}'",
                                    node_name = deployment_element.scenario_reference
                                );

                                let mut condition_deployment_element = addressor
                                    .database
                                    .send(CreateDeploymentElement(
                                        *exercise_id,
                                        DeploymentElement::new_ongoing(
                                            deployment_element.deployment_id,
                                            Box::new(condition_name.to_owned()),
                                            GrpcDeployerType::Condition,
                                            *event_id,
                                            Some(virtual_machine_id),
                                        ),
                                        true,
                                    ))
                                    .await??;
                                let condition_request = create_condition_request(
                                    &addressor.database,
                                    &virtual_machine_id_string,
                                    template_id,
                                    condition,
                                    condition_name,
                                    condition_role,
                                )
                                .await?;
                                match addressor
                                    .distributor
                                    .send(Deploy(
                                        GrpcDeployerType::Condition,
                                        condition_request,
                                        deployers.to_owned(),
                                    ))
                                    .await?
                                {
                                    Result::Ok(handler_response) => {
                                        let (condition_identifier, condition_stream) =
                                            ConditionResponse::try_from(handler_response)?;

                                        let condition_status = match event_id {
                                            Some(_) => ElementStatus::ConditionPolling,
                                            None => ElementStatus::Success,
                                        };

                                        condition_deployment_element
                                            .update(
                                                &addressor.database,
                                                *exercise_id,
                                                condition_status,
                                                Some(condition_identifier.value.clone()),
                                            )
                                            .await?;

                                        let condition_metric = get_metric_by_condition(
                                            metrics,
                                            &condition_deployment_element.scenario_reference,
                                        );

                                        addressor
                                            .distributor
                                            .clone()
                                            .send(ConditionStream {
                                                exercise_id: *exercise_id,
                                                condition_deployment_element:
                                                    condition_deployment_element.to_owned(),
                                                node_deployment_element: deployment_element.clone(),
                                                database_address: addressor.database.clone(),
                                                condition_stream,
                                                condition_metric,
                                            })
                                            .await??;
                                    }

                                    Err(error) => {
                                        log::error!("DeployCondition: {:#?}", error);
                                        condition_deployment_element.status = ElementStatus::Failed;

                                        addressor
                                            .database
                                            .send(UpdateDeploymentElement(
                                                *exercise_id,
                                                condition_deployment_element,
                                                true,
                                            ))
                                            .await??;
                                        return Err(error);
                                    }
                                }
                            }
                            Ok(())
                        })
                        .collect::<Vec<_>>(),
                )
                .await?;

                Ok(())
            }
            .into_actor(self),
        )
    }
}

pub async fn create_condition_request(
    database_address: &Addr<Database>,
    virtual_machine_id: &str,
    template_id: &Uuid,
    condition: &Condition,
    condition_name: &str,
    role: &Role,
) -> Result<Box<GrpcCondition>> {
    let template_account = database_address
        .send(GetAccount(*template_id, role.username.to_owned()))
        .await?
        .map_err(|err| anyhow!("ConditionRequest GetAccount: {err}"))?;
    let source: Option<GrpcSource> = match condition.clone().source {
        Some(condition_source) => Some(GrpcSource {
            name: condition_source.name,
            version: condition_source.version,
        }),
        None => None,
    };

    Ok(Box::new(GrpcCondition {
        name: condition_name.to_owned(),
        source,
        command: condition.clone().command.unwrap_or_default(),
        interval: condition.interval.unwrap_or_default() as i32,
        virtual_machine_id: virtual_machine_id.to_owned(),
        account: Some(GrpcAccount {
            username: template_account.username,
            password: template_account.password.unwrap_or_default(),
            private_key: template_account.private_key.unwrap_or_default(),
        }),
        environment: condition.environment.clone().unwrap_or_default(),
    }))
}
