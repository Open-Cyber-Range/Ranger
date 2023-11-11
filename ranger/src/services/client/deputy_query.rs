use super::{DeploymentClient, DeploymentClientResponse, DeploymentInfo};
use actix::{Actor, Addr, Context};
use anyhow::{Ok, Result};
use async_trait::async_trait;
use ranger_grpc::deputy_query_service_client::DeputyQueryServiceClient;
use tonic::transport::Channel;

pub struct DeputyQueryClient {
    deputy_query_client: DeputyQueryServiceClient<Channel>,
}

impl DeputyQueryClient {
    pub async fn new(server_address: String) -> Result<Self> {
        Ok(Self {
            deputy_query_client: DeputyQueryServiceClient::connect(server_address).await?,
        })
    }
}

impl Actor for DeputyQueryClient {
    type Context = Context<Self>;
}

#[async_trait]
impl DeploymentClient<Box<dyn DeploymentInfo>> for Addr<DeputyQueryClient> {
    async fn deploy(
        &mut self,
        _deployment_struct: Box<dyn DeploymentInfo>,
    ) -> Result<DeploymentClientResponse> {
        // todo
        Ok(DeploymentClientResponse::DeputyQueryResponse(()))
    }

    async fn undeploy(&mut self, _handler_reference: String) -> Result<()> {
        // todo
        Ok(())
    }
}
