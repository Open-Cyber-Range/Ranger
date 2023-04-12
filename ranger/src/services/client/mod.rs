mod condition;
mod feature;
mod inject;
mod switch;
mod template;
mod virtual_machine;

use anyhow::anyhow;
use anyhow::Result;
use async_trait::async_trait;
pub use condition::*;
pub use feature::*;
pub use inject::*;
use ranger_grpc::ConditionStreamResponse;
use ranger_grpc::{ExecutorResponse, Identifier, TemplateResponse};
use std::any::Any;
pub use switch::*;
pub use template::*;
use tonic::Streaming;
pub use virtual_machine::*;

pub type ConditionResponse = (Identifier, Streaming<ConditionStreamResponse>);
pub enum DeploymentClientResponse {
    Identifier(Identifier),
    ExecutorResponse(ExecutorResponse),
    TemplateResponse(TemplateResponse),
    ConditionResponse(ConditionResponse),
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

impl TryFrom<DeploymentClientResponse> for ExecutorResponse {
    type Error = anyhow::Error;

    fn try_from(client_response: DeploymentClientResponse) -> Result<Self> {
        match client_response {
            DeploymentClientResponse::ExecutorResponse(executor_response) => Ok(executor_response),
            _ => Err(anyhow!(
                "Unable to convert ClientResponse into ExecutorResponse"
            )),
        }
    }
}

impl TryFrom<DeploymentClientResponse> for TemplateResponse {
    type Error = anyhow::Error;

    fn try_from(client_response: DeploymentClientResponse) -> Result<Self> {
        match client_response {
            DeploymentClientResponse::TemplateResponse(template_response) => Ok(template_response),
            _ => Err(anyhow!(
                "Unable to convert ClientResponse into TemplateResponse"
            )),
        }
    }
}

impl TryFrom<DeploymentClientResponse> for ConditionResponse {
    type Error = anyhow::Error;

    fn try_from(client_response: DeploymentClientResponse) -> Result<Self> {
        match client_response {
            DeploymentClientResponse::ConditionResponse((id, stream)) => Ok((id, stream)),
            _ => Err(anyhow!(
                "Unable to convert ClientResponse into TemplateResponse"
            )),
        }
    }
}
