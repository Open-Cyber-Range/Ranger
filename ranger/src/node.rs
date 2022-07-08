use actix::prelude::*;
use actix::{Actor, Handler, Message};
use anyhow::{Ok, Result};
use ranger_grpc::capability_client::CapabilityClient;
use ranger_grpc::{node_service_client::NodeServiceClient, NodeDeployment, NodeIdentifier};
use tonic::transport::Channel;

#[derive(Message)]
#[rtype(result = "Result<NodeIdentifier, anyhow::Error>")]
pub struct CreateNode(pub NodeDeployment);

#[derive(Message)]
#[rtype(result = "Result<(), anyhow::Error>")]
pub struct DeleteNode(pub NodeIdentifier);

pub struct NodeClient {
    node_client: NodeServiceClient<Channel>,
    pub capability_client: CapabilityClient<Channel>,
}

impl NodeClient {
    pub async fn new(server_address: String) -> Result<Self> {
        Ok(Self {
            node_client: NodeServiceClient::connect(server_address.clone()).await?,
            capability_client: CapabilityClient::connect(server_address).await?,
        })
    }
}

impl Actor for NodeClient {
    type Context = Context<Self>;
}

impl Handler<CreateNode> for NodeClient {
    type Result = ResponseFuture<Result<NodeIdentifier>>;

    fn handle(&mut self, msg: CreateNode, _ctx: &mut Self::Context) -> Self::Result {
        let node_deployment = msg.0;
        let mut client = self.node_client.clone();
        Box::pin(async move {
            let result = client.create(tonic::Request::new(node_deployment)).await;
            if let Err(status) = result {
                return Err(anyhow::anyhow!("{:?}", status));
            }
            Ok(result?.into_inner())
        })
    }
}

impl Handler<DeleteNode> for NodeClient {
    type Result = ResponseFuture<Result<()>>;

    fn handle(&mut self, msg: DeleteNode, _ctx: &mut Self::Context) -> Self::Result {
        let node_identifier = msg.0;
        let mut client = self.node_client.clone();
        Box::pin(async move {
            let result = client.delete(tonic::Request::new(node_identifier)).await;
            if let Err(status) = result {
                return Err(anyhow::anyhow!("{:?}", status));
            }
            Ok(())
        })
    }
}
