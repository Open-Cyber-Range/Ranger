use crate::models::helpers::uuid::Uuid;
use crate::models::{DeploymentElement, ElementStatus};
use crate::services::database::account::GetAccount;
use crate::services::database::deployment::{CreateDeploymentElement, UpdateDeploymentElement};
use crate::services::deployer::Deploy;
use crate::utilities::try_some;
use crate::Addressor;
use anyhow::{Ok, Result};
use async_trait::async_trait;
use log::{debug, info};
use ranger_grpc::capabilities::DeployerTypes;
use ranger_grpc::{
    Account as GrpcAccount, ExecutorResponse, Inject as GrpcInject, Source as GrpcSource,
};
use sdl_parser::inject::Inject;

#[async_trait]
pub trait DeployableInject {
    async fn deploy_inject(&self) -> Result<()>;
}
#[async_trait]
impl DeployableInject
    for (
        &Addressor,
        Vec<String>,
        DeploymentElement,
        Uuid,
        String,
        Uuid,
        (String, Inject),
    )
{
    async fn deploy_inject(&self) -> Result<()> {
        let (
            addressor,
            deployers,
            deployment_element,
            exercise_id,
            username,
            template_id,
            (inject_name, inject),
        ) = self;

        debug!(
            "Deploying '{inject_name}' for '{node_name}",
            node_name = deployment_element.scenario_reference
        );
        let parent_node_id_string = try_some(
            deployment_element.handler_reference.clone(),
            "Deployment element handler reference not found",
        )?;

        let virtual_machine_id = Uuid::try_from(parent_node_id_string.as_str())?;
        let inject_source = try_some(inject.source.clone(), "Injects source not found")?;

        let template_account = addressor
            .database
            .send(GetAccount(*template_id, username.to_owned()))
            .await??;

        let mut inject_deployment_element = addressor
            .database
            .send(CreateDeploymentElement(
                *exercise_id,
                DeploymentElement::new_ongoing(
                    deployment_element.deployment_id,
                    Box::new(inject_name.to_owned()),
                    DeployerTypes::Inject,
                    None,
                    Some(virtual_machine_id),
                ),
                false,
            ))
            .await??;

        let inject_deployment = Box::new(GrpcInject {
            name: inject_name.to_owned(),
            virtual_machine_id: parent_node_id_string,
            source: Some(GrpcSource {
                name: inject_source.name.to_owned(),
                version: inject_source.version.to_owned(),
            }),
            account: Some(GrpcAccount {
                username: username.to_owned(),
                password: template_account.password.unwrap_or_default(),
                private_key: template_account.private_key.unwrap_or_default(),
            }),
            to_entities: inject.to_entities.clone().unwrap_or_default(),
        });

        {
            match addressor
                .distributor
                .send(Deploy(
                    DeployerTypes::Inject,
                    inject_deployment,
                    deployers.to_owned(),
                ))
                .await?
            {
                anyhow::Result::Ok(result) => {
                    let response = ExecutorResponse::try_from(result)?;
                    let identifier = try_some(
                        response.identifier,
                        "Successful Inject response did not supply Identifier",
                    )?;

                    inject_deployment_element.status = ElementStatus::Success;
                    inject_deployment_element.handler_reference = Some(identifier.value);
                    inject_deployment_element.executor_log = match response.vm_log.is_empty() {
                        true => None,
                        false => Some(response.vm_log),
                    };

                    addressor
                        .database
                        .send(UpdateDeploymentElement(
                            *exercise_id,
                            inject_deployment_element,
                            false,
                        ))
                        .await??;

                    info!(
                        "Deployed '{inject_name}' on '{node_name}'",
                        node_name = deployment_element.scenario_reference
                    );
                    return Ok(());
                }

                Err(error) => {
                    inject_deployment_element.status = ElementStatus::Failed;
                    addressor
                        .database
                        .send(UpdateDeploymentElement(
                            *exercise_id,
                            inject_deployment_element,
                            false,
                        ))
                        .await??;
                    return Err(error);
                }
            }
        }
    }
}
