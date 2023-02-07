use crate::models::helpers::uuid::Uuid;
use crate::models::{DeploymentElement, ElementStatus};
use crate::services::client::{ConditionResponse, ConditionStream};
use crate::services::database::account::GetAccount;
use crate::services::database::deployment::{CreateDeploymentElement, UpdateDeploymentElement};
use crate::services::database::Database;
use crate::services::deployer::{Deploy, DeployerDistribution};
use actix::Addr;
use anyhow::{anyhow, Ok, Result};
use async_trait::async_trait;
use futures::future::try_join_all;
use log::info;
use ranger_grpc::capabilities::DeployerTypes;
use ranger_grpc::{Account as GrpcAccount, Condition as GrpcCondition, Source as GrpcSource};
use sdl_parser::{node::Node, Scenario};

#[async_trait]
pub trait DeployableConditions {
    async fn deploy_conditions(&self) -> Result<()>;
}

#[async_trait]
impl DeployableConditions
    for (
        Addr<DeployerDistribution>,
        Addr<Database>,
        Vec<String>,
        Scenario,
        Node,
        DeploymentElement,
        Uuid,
        Option<String>,
    )
{
    async fn deploy_conditions(&self) -> Result<()> {
        let (
            distributor_address,
            database_address,
            deployers,
            scenario,
            node,
            deployment_element,
            exercise_id,
            template_id,
        ) = self;

        let scenario_conditions = scenario
            .clone()
            .conditions
            .ok_or_else(|| anyhow!("Scenario conditions not found"))?;

        try_join_all(
            scenario_conditions
                .iter()
                .map(|(condition_name, condition)| async move {
                    let node_conditions = node
                        .clone()
                        .conditions
                        .ok_or_else(|| anyhow!("Node conditions not found"))?;

                    let roles = node
                        .clone()
                        .roles
                        .ok_or_else(|| anyhow!("Node Roles not found"))?;

                    let template_id = template_id
                        .to_owned()
                        .ok_or_else(|| anyhow!("Template id not found"))?;

                    let virtual_machine_id = deployment_element
                        .clone()
                        .handler_reference
                        .ok_or_else(|| anyhow!("Deployment element handler reference not found"))?;
                    // for (condition_name, condition) in scenario_conditions.iter() {
                    for (node_condition_name, role_name) in node_conditions.iter() {
                        if condition_name.eq_ignore_ascii_case(node_condition_name) {
                            info!(
                                "Deploying condition '{condition_name}' for VM {node_name}",
                                node_name = deployment_element.scenario_reference
                            );

                            let (_, username) = roles
                                .get_key_value(&role_name.clone())
                                .ok_or_else(|| anyhow!("Username in roles list not found"))?;

                            let source: Option<GrpcSource> = match condition.clone().source {
                                Some(condition_source) => Some(GrpcSource {
                                    name: condition_source.name,
                                    version: condition_source.version,
                                }),
                                None => None,
                            };

                            let template_account = database_address
                                .send(GetAccount(
                                    template_id.as_str().try_into()?,
                                    username.to_owned(),
                                ))
                                .await??;

                            let mut condition_deployment_element = database_address
                                .send(CreateDeploymentElement(
                                    *exercise_id,
                                    DeploymentElement::new_ongoing(
                                        deployment_element.deployment_id,
                                        Box::new(condition_name.to_owned()),
                                        DeployerTypes::Condition,
                                    ),
                                ))
                                .await??;

                            let condition_deployment = Box::new(GrpcCondition {
                                name: node_condition_name.to_owned(),
                                source,
                                command: condition.clone().command.unwrap_or_default(),
                                interval: condition.interval.unwrap_or_default() as i32,
                                virtual_machine_id: virtual_machine_id.clone(),
                                account: Some(GrpcAccount {
                                    username: template_account.username,
                                    password: template_account.password.unwrap_or_default(),
                                    private_key: template_account.private_key.unwrap_or_default(),
                                }),
                            });

                            match distributor_address
                                .send(Deploy(
                                    DeployerTypes::Condition,
                                    condition_deployment,
                                    deployers.to_owned(),
                                ))
                                .await?
                            {
                                anyhow::Result::Ok(result) => {
                                    let (condition_identifier, condition_stream) =
                                        ConditionResponse::try_from(result)?;

                                    condition_deployment_element.status = ElementStatus::Success;
                                    condition_deployment_element.handler_reference =
                                        Some(condition_identifier.value.clone());

                                    database_address
                                        .send(UpdateDeploymentElement(
                                            *exercise_id,
                                            condition_deployment_element.clone(),
                                        ))
                                        .await??;

                                    info!(
                                "Finished deploying {condition_name} on {}, starting stream",
                                deployment_element.scenario_reference
                            );

                                    distributor_address
                                        .clone()
                                        .send(ConditionStream(
                                            condition_deployment_element,
                                            database_address.clone(),
                                            condition_stream,
                                        ))
                                        .await??;
                                    return Ok(());
                                }

                                Err(error) => {
                                    condition_deployment_element.status = ElementStatus::Failed;

                                    database_address
                                        .send(UpdateDeploymentElement(
                                            *exercise_id,
                                            condition_deployment_element,
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
}
