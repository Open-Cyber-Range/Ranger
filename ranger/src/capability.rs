use actix::prelude::*;
use actix::{Context, Handler, Message};
use anyhow::Result;
use ranger_grpc::Capabilities;

use crate::node::NodeClient;

#[derive(Message, Debug)]
#[rtype(result = "Result<Capabilities, anyhow::Error>")]
pub struct GetCapabilities;

impl Handler<GetCapabilities> for NodeClient {
    type Result = ResponseFuture<Result<Capabilities>>;

    fn handle(&mut self, _: GetCapabilities, _: &mut Context<Self>) -> Self::Result {
        let mut client = self.capability_client.clone();
        Box::pin(async move {
            let result = client.get_capabilities(tonic::Request::new(())).await;
            if let Err(status) = result {
                return Err(anyhow::anyhow!("{:?}", status));
            }
            Ok(result?.into_inner())
        })
    }
}
