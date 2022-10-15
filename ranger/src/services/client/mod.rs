mod switch;
mod template;
mod virtual_machine;

use anyhow::Result;
use async_trait::async_trait;
use std::any::Any;
pub use switch::*;
pub use template::*;
pub use virtual_machine::*;

#[async_trait]
pub trait DeploymentClient<T>
where
    T: Sized,
{
    async fn deploy(&mut self, deployment_struct: T) -> Result<String>;
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