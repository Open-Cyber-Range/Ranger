mod distribution;
mod factory;

use super::client::{SwitchClient, TemplateClient, VirtualMachineClient};
use actix::{Actor, Addr};
use anyhow::Result;
pub use distribution::*;
pub use factory::DeployerFactory;
use ranger_grpc::capabilities::DeployerTypes;

#[derive(Clone)]
pub struct DeployerConnections {
    virtual_machine_client: Option<Addr<VirtualMachineClient>>,
    switch_client: Option<Addr<SwitchClient>>,
    template_client: Option<Addr<TemplateClient>>,
}

impl DeployerConnections {
    pub async fn new(capabilities: Vec<DeployerTypes>, address: &str) -> Result<Self> {
        let mut virtual_machine_client = None;
        let mut template_client = None;
        let mut switch_client = None;

        if capabilities.contains(&DeployerTypes::VirtualMachine) {
            virtual_machine_client = Some(
                VirtualMachineClient::new(address.to_string())
                    .await?
                    .start(),
            );
        }
        if capabilities.contains(&DeployerTypes::Template) {
            template_client = Some(TemplateClient::new(address.to_string()).await?.start());
        }
        if capabilities.contains(&DeployerTypes::Switch) {
            switch_client = Some(SwitchClient::new(address.to_string()).await?.start());
        }
        Ok(Self {
            virtual_machine_client,
            switch_client,
            template_client,
        })
    }
}
