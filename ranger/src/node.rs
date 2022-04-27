use actix::prelude::*;
use actix::{Actor, Handler, Message};
use anyhow::{Ok, Result};
use ranger_grpc::{
    node_service_client::NodeServiceClient, simple_response::Status, Identifier, Node,
    SimpleResponse,
};
use tonic::transport::Channel;

#[derive(Message)]
#[rtype(result = "Result<String, anyhow::Error>")]
pub struct CreateNode(pub Node);

#[derive(Message)]
#[rtype(result = "Result<(), anyhow::Error>")]
pub struct DeleteNode(pub String);

pub struct NodeClient {
    client: NodeServiceClient<Channel>,
}

impl NodeClient {
    pub async fn new(server_address: String) -> Result<Self> {
        Ok(Self {
            client: NodeServiceClient::connect(server_address).await?,
        })
    }
}

impl Actor for NodeClient {
    type Context = actix::Context<Self>;
}

impl Handler<CreateNode> for NodeClient {
    type Result = ResponseFuture<Result<String>>;

    fn handle(&mut self, msg: CreateNode, _ctx: &mut Self::Context) -> Self::Result {
        let node = msg.0;
        //In future this will be id returned by the deployer
        let node_id = format!("{}/{}", node.exercise_name, node.name);
        let mut client = self.client.clone();
        Box::pin(async move {
            let result: SimpleResponse =
                client.create(tonic::Request::new(node)).await?.into_inner();
            if result.status == i32::from(Status::Error) {
                return Err(anyhow::anyhow!(result.message));
            }
            Ok(node_id)
        })
    }
}

impl Handler<DeleteNode> for NodeClient {
    type Result = ResponseFuture<Result<()>>;

    fn handle(&mut self, msg: DeleteNode, _ctx: &mut Self::Context) -> Self::Result {
        let node_id = msg.0;
        //In future this will be id returned by the deployer
        let mut client = self.client.clone();
        Box::pin(async move {
            let result: SimpleResponse = client
                .delete(tonic::Request::new(Identifier { value: node_id }))
                .await?
                .into_inner();
            if result.status == i32::from(Status::Error) {
                return Err(anyhow::anyhow!(result.message));
            }
            Ok(())
        })
    }
}
