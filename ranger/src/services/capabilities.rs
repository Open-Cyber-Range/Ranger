use std::collections::HashMap;

use actix::{Actor, Context, Message};
use anyhow::{Ok, Result};
use futures::future::try_join_all;
use ranger_grpc::capability_client::CapabilityClient;
use tonic::transport::Channel;

pub struct CapabilityManager {
    capabilities: HashMap<String, CapabilityClient<Channel>>,
}

impl CapabilityManager {
    pub async fn new(deployers_map: HashMap<String, String>) -> Result<Self> {
        let capabilities = try_join_all(deployers_map.iter().map(
            |(deployer_name, deployer_address)| async {
                Ok::<(String, CapabilityClient<Channel>)>((
                    deployer_name.clone(),
                    CapabilityClient::connect(deployer_address.to_string()).await?,
                ))
            },
        ))
        .await?
        .into_iter()
        .collect();
        Ok(Self { capabilities })
    }
}

impl Actor for CapabilityManager {
    type Context = Context<Self>;
}

#[derive(Message, Debug)]
#[rtype(result = "Result<Vec<String>, anyhow::Error>")]
pub struct GetWithMatc(String);
