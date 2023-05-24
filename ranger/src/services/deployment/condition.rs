use std::collections::HashMap;

use crate::models::helpers::uuid::Uuid;
use crate::models::{DeploymentElement, ElementStatus};
use crate::services::client::{ConditionResponse, ConditionStream};
use crate::services::database::account::GetAccount;
use crate::services::database::deployment::{CreateDeploymentElement, UpdateDeploymentElement};
use crate::services::database::Database;
use crate::services::deployer::{Deploy, DeployerDistribution};
use crate::utilities::try_some;
use actix::Addr;
use anyhow::{Ok, Result};
use async_trait::async_trait;
use futures::future::try_join_all;
use log::info;
use ranger_grpc::capabilities::DeployerTypes;
use ranger_grpc::{
    Account as GrpcAccount, Condition as GrpcCondition, ConditionStreamResponse,
    Source as GrpcSource,
};
use sdl_parser::condition::Condition;
use sdl_parser::node::Role;
use sdl_parser::{node::Node, Scenario};
use tonic::Streaming;

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
            node_deployment_element,
            exercise_id,
            template_id,
        ) = self;

        let scenario_conditions =
            try_some(scenario.clone().conditions, "Scenario conditions not found")?;

        try_join_all(
            scenario_conditions
                .iter()
                .map(|(condition_name, condition)| async move {
                    let node_conditions =
                        try_some(node.clone().conditions, "Node conditions not found")?;
                    let roles = try_some(node.clone().roles, "Node Roles not found")?;
                    let template_id = try_some(template_id.to_owned(), "Template id not found")?;
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

                            let mut condition_deployment_element = database_address
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
                                database_address,
                                &virtual_machine_id,
                                &template_id,
                                roles.clone(),
                                condition,
                                node_condition_name,
                                node_role_name,
                            )
                            .await?;

                            match distributor_address
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
                                            database_address,
                                            *exercise_id,
                                            ElementStatus::Success,
                                            Some(condition_identifier.value.clone()),
                                        )
                                        .await?;

                                    start_condition_stream(
                                        database_address,
                                        distributor_address,
                                        condition_stream,
                                        &mut condition_deployment_element,
                                        node_deployment_element,
                                        exercise_id,
                                    )
                                    .await?
                                }

                                Err(error) => {
                                    condition_deployment_element.status = ElementStatus::Failed;

                                    database_address
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
}

async fn create_condition_request(
    database_address: &Addr<Database>,
    virtual_machine_id: &str,
    template_id: &str,
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
        .send(GetAccount(
            template_id.try_into()?,
            role.username.to_owned(),
        ))
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

async fn start_condition_stream(
    database_address: &Addr<Database>,
    distributor_address: &Addr<DeployerDistribution>,
    condition_stream: Streaming<ConditionStreamResponse>,
    condition_deployment_element: &mut DeploymentElement,
    node_deployment_element: &DeploymentElement,
    exercise_id: &Uuid,
) -> Result<()> {
    let virtual_machine_id = try_some(
        node_deployment_element.clone().handler_reference,
        "Deployment element handler reference not found",
    )?;

    info!(
        "Finished deploying {condition_name} on {node_name}, starting stream",
        condition_name = condition_deployment_element.scenario_reference,
        node_name = node_deployment_element.scenario_reference,
    );

    distributor_address
        .clone()
        .send(ConditionStream(
            *exercise_id,
            condition_deployment_element.to_owned(),
            virtual_machine_id.clone(),
            database_address.clone(),
            condition_stream,
        ))
        .await??;
    Ok(())
}
