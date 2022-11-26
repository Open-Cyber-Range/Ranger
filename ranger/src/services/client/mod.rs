mod feature;
mod switch;
mod template;
mod virtual_machine;

use anyhow::anyhow;
use anyhow::Result;
use async_trait::async_trait;
pub use feature::*;
use ranger_grpc::FeatureResponse;
use ranger_grpc::Identifier;
use std::any::Any;
pub use switch::*;
pub use template::*;
pub use virtual_machine::*;

pub enum DeploymentClientResponse {
    Identifier(Identifier),
    FeatureResponse(FeatureResponse),
}

#[async_trait]
pub trait DeploymentClient<T>
where
    T: Sized,
{
    async fn deploy(&mut self, deployment_struct: T) -> Result<DeploymentClientResponse>;
    async fn undeploy(&mut self, handler_reference: String) -> Result<()>;
}

pub trait DeploymentInfo
where
    Self: Send,
{
    fn get_deployment(&self) -> Self
    where
        Self: Sized;
    fn as_any(&self) -> &dyn Any;
}

pub trait Deployable {
    fn try_to_deployment_command(&self) -> Result<Box<dyn DeploymentInfo>>;
}

impl TryFrom<DeploymentClientResponse> for Identifier {
    type Error = anyhow::Error;

    fn try_from(client_response: DeploymentClientResponse) -> Result<Self> {
        match client_response {
            DeploymentClientResponse::Identifier(id) => Ok(id),
            _ => Err(anyhow!("Unable to convert ClientResponse into Identifier")),
        }
    }
}

impl TryFrom<DeploymentClientResponse> for FeatureResponse {
    type Error = anyhow::Error;

    fn try_from(client_response: DeploymentClientResponse) -> Result<Self> {
        match client_response {
            DeploymentClientResponse::FeatureResponse(feature_response) => Ok(feature_response),
            _ => Err(anyhow!(
                "Unable to convert ClientResponse into FeatureResponse"
            )),
        }
    }
}
