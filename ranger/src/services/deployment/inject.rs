use crate::models::helpers::uuid::Uuid;
use crate::models::{DeploymentElement, ElementStatus};
use crate::services::database::account::GetAccount;
use crate::services::database::deployment::{CreateDeploymentElement, UpdateDeploymentElement};
use crate::services::database::Database;
use crate::services::deployer::{Deploy, DeployerDistribution};
use actix::Addr;
use anyhow::{anyhow, Ok, Result};
use async_trait::async_trait;
use log::info;
use ranger_grpc::capabilities::DeployerTypes;
use ranger_grpc::{Account as GrpcAccount, Identifier, Inject as GrpcInject, Source as GrpcSource};
use sdl_parser::{inject::Inject, node::Node};

#[async_trait]
pub trait DeployableInject {
    async fn deploy_inject(&self) -> Result<()>;
}

#[async_trait]
impl DeployableInject
    for (
        Addr<DeployerDistribution>,
        Addr<Database>,
        Vec<String>,
        Node,
        DeploymentElement,
        Uuid,
        String,
        Option<String>,
        (String, Inject),
    )
{
    async fn deploy_inject(&self) -> Result<()> {
        let (
            distributor_address,
            database_address,
            deployers,
            node,
            deployment_element,
            exercise_id,
            role_name,
            template_id,
            (inject_name, inject),
        ) = self;

        let virtual_machine_id = deployment_element
            .handler_reference
            .clone()
            .ok_or_else(|| anyhow!("Deployment element handler reference not found"))?;

        let roles = node
            .roles
            .clone()
            .ok_or_else(|| anyhow!("Node roles not found"))?;

        let (_, username) = roles
            .get_key_value(&role_name.clone())
            .ok_or_else(|| anyhow!("Username in roles list not found"))?;

        let inject_source = inject
            .source
            .clone()
            .ok_or_else(|| anyhow!("Injects source not found"))?;

        let template_id = template_id
            .clone()
            .ok_or_else(|| anyhow!("Template id not found"))?;

        let template_account = database_address
            .send(GetAccount(
                template_id.as_str().try_into()?,
                username.to_owned(),
            ))
            .await??;

        let mut inject_deployment_element = database_address
            .send(CreateDeploymentElement(
                *exercise_id,
                DeploymentElement::new_ongoing(
                    deployment_element.deployment_id,
                    Box::new(inject_name.to_owned()),
                    DeployerTypes::Inject,
                ),
                false,
            ))
            .await??;

        let inject_deployment = Box::new(GrpcInject {
            name: inject_name.to_owned(),
            virtual_machine_id: virtual_machine_id.clone(),
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
            match distributor_address
                .send(Deploy(
                    DeployerTypes::Inject,
                    inject_deployment,
                    deployers.to_owned(),
                ))
                .await?
            {
                anyhow::Result::Ok(result) => {
                    let id = Identifier::try_from(result)?.value;

                    inject_deployment_element.status = ElementStatus::Success;
                    inject_deployment_element.handler_reference = Some(id);
                    database_address
                        .send(UpdateDeploymentElement(
                            *exercise_id,
                            inject_deployment_element,
                            false,
                        ))
                        .await??;

                    info!(
                        "Finished deploying {inject_name} on {}",
                        deployment_element.scenario_reference
                    );
                    return Ok(());
                }

                Err(error) => {
                    inject_deployment_element.status = ElementStatus::Failed;
                    database_address
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
