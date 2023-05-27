use super::Database;
use crate::models::helpers::uuid::Uuid;
use crate::models::{DeploymentElement, ElementStatus, Exercise};
use crate::services::client::{ConditionResponse, ConditionStream};
use crate::services::database::account::GetAccount;
use crate::services::database::deployment::{CreateDeploymentElement, UpdateDeploymentElement};
use crate::services::deployer::Deploy;
use crate::utilities::try_some;
use crate::Addressor;
use actix::{Actor, Addr, Context, Handler, Message, ResponseActFuture, WrapFuture};
use anyhow::{Ok, Result};
use async_trait::async_trait;
use futures::future::try_join_all;
use log::{info, warn};
use ranger_grpc::capabilities::DeployerTypes;
use ranger_grpc::{Account as GrpcAccount, Condition as GrpcCondition, Source as GrpcSource};
use sdl_parser::condition::Condition;
use sdl_parser::{node::Node, node::NodeType, node::Role, Scenario};
use std::collections::HashMap;

#[async_trait]
pub trait DeployableConditions {
    async fn deploy_scenario_conditions(
        &self,
        addressor: &Addressor,
        exercise: &Exercise,
        deployers: &[String],
        deployed_nodes: &[(Node, DeploymentElement, Uuid)],
    ) -> Result<()>;
}
#[async_trait]
impl DeployableConditions for Scenario {
    async fn deploy_scenario_conditions(
        &self,
        addressor: &Addressor,
        exercise: &Exercise,
        deployers: &[String],
        deployed_nodes: &[(Node, DeploymentElement, Uuid)],
    ) -> Result<()> {
        if self.conditions.is_some() {
            try_join_all(deployed_nodes.iter().map(
                |(node, deployment_element, template_id)| async move {
                    if matches!(node.type_field, NodeType::VM) {
                        addressor.condition_aggregator.do_send(DeployConditions {
                            addressor: addressor.clone(),
                            deployers: deployers.to_owned(),
                            scenario: self.clone(),
                            node: node.clone(),
                            node_deployment_element: deployment_element.clone(),
                            exercise_id: exercise.id,
                            template_id: *template_id,
                        });
                    }
                    Ok(())
                },
            ))
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

    fn started(&mut self, _ctx: &mut Context<Self>) {
        println!("ConditionAggregator is alive");
    }

    fn stopped(&mut self, _ctx: &mut Context<Self>) {
        println!("ConditionAggregator is stopped");
    }
}

#[derive(Message)]
#[rtype(result = "Result<()>")]
pub struct DeployConditions {
    pub addressor: Addressor,
    pub deployers: Vec<String>,
    pub scenario: Scenario,
    pub node: Node,
    pub node_deployment_element: DeploymentElement,
    pub exercise_id: Uuid,
    pub template_id: Uuid,
}

impl Handler<DeployConditions> for ConditionAggregator {
    type Result = ResponseActFuture<Self, Result<()>>;

    fn handle(&mut self, msg: DeployConditions, _ctx: &mut Self::Context) -> Self::Result {
        Box::pin(
            async move {
                let DeployConditions {
                    addressor,
                    deployers,
                    scenario,
                    node,
                    exercise_id,
                    template_id,
                    node_deployment_element,
                } = &msg;

                let scenario_conditions =
                    try_some(scenario.clone().conditions, "Scenario conditions not found")?;

                try_join_all(
                    scenario_conditions
                        .iter()
                        .map(|(condition_name, condition)| async move {
                            warn!("{:#?}", condition_name);
                            let node_conditions =
                                try_some(node.clone().conditions, "Node conditions not found")?;
                            let roles = try_some(node.clone().roles, "Node Roles not found")?;

                            let virtual_machine_id = try_some(
                                node_deployment_element.clone().handler_reference,
                                "Deployment element handler reference not found",
                            )?;

                            for (node_condition_name, node_role_name) in node_conditions.iter() {
                                if condition_name.eq_ignore_ascii_case(node_condition_name) {
                                    info!(
                                        "Deploying condition '{condition_name}' for VM {node_name}",
                                        node_name = node_deployment_element.scenario_reference
                                    );

                                    let mut condition_deployment_element = addressor
                                        .database
                                        .send(CreateDeploymentElement(
                                            *exercise_id,
                                            DeploymentElement::new_ongoing(
                                                node_deployment_element.deployment_id,
                                                Box::new(condition_name.to_owned()),
                                                DeployerTypes::Condition,
                                            ),
                                            true,
                                        ))
                                        .await??;

                                    let condition_request = create_condition_request(
                                        &addressor.database,
                                        &virtual_machine_id,
                                        template_id,
                                        roles.clone(),
                                        condition,
                                        node_condition_name,
                                        node_role_name,
                                    )
                                    .await?;

                                    match addressor
                                        .distributor
                                        .send(Deploy(
                                            DeployerTypes::Condition,
                                            condition_request,
                                            deployers.to_owned(),
                                        ))
                                        .await?
                                    {
                                        anyhow::Result::Ok(handler_response) => {
                                            let (condition_identifier, condition_stream) =
                                                ConditionResponse::try_from(handler_response)?;

                                            condition_deployment_element
                                                .update(
                                                    &addressor.database,
                                                    *exercise_id,
                                                    ElementStatus::Success,
                                                    Some(condition_identifier.value.clone()),
                                                )
                                                .await?;

                                            addressor
                                                .distributor
                                                .clone()
                                                .send(ConditionStream(
                                                    *exercise_id,
                                                    condition_deployment_element.to_owned(),
                                                    node_deployment_element.clone(),
                                                    addressor.database.clone(),
                                                    condition_stream,
                                                    scenario.clone().metrics,
                                                    node_deployment_element
                                                        .clone()
                                                        .scenario_reference,
                                                ))
                                                .await??;
                                        }

                                        Err(error) => {
                                            condition_deployment_element.status =
                                                ElementStatus::Failed;

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
    roles: HashMap<String, Role>,
    condition: &Condition,
    node_condition_name: &str,
    node_role_name: &str,
) -> Result<Box<GrpcCondition>> {
    let (_, role) = try_some(
        roles.get_key_value(node_role_name),
        "Username in roles list not found",
    )?;

    let template_account = database_address
        .send(GetAccount(*template_id, role.username.to_owned()))
        .await??;

    let source: Option<GrpcSource> = match condition.clone().source {
        Some(condition_source) => Some(GrpcSource {
            name: condition_source.name,
            version: condition_source.version,
        }),
        None => None,
    };

    Ok(Box::new(GrpcCondition {
        name: node_condition_name.to_owned(),
        source,
        command: condition.clone().command.unwrap_or_default(),
        interval: condition.interval.unwrap_or_default() as i32,
        virtual_machine_id: virtual_machine_id.to_owned(),
        account: Some(GrpcAccount {
            username: template_account.username,
            password: template_account.password.unwrap_or_default(),
            private_key: template_account.private_key.unwrap_or_default(),
        }),
    }))
}
