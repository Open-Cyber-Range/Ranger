use actix::prelude::*;
use actix::{Actor, Handler, Message};
use anyhow::{Ok, Result};
use ranger_grpc::{template_service_client::TemplateServiceClient, Identifier, Source};
use tonic::transport::Channel;

#[derive(Message)]
#[rtype(result = "Result<Identifier, anyhow::Error>")]
pub struct CreateTemplate(pub Source);

#[derive(Message)]
#[rtype(result = "Result<(), anyhow::Error>")]
pub struct DeleteTemplate(pub Identifier);

pub struct TemplateClient {
    template_client: TemplateServiceClient<Channel>,
}

impl TemplateClient {
    pub async fn new(server_address: String) -> Result<Self> {
        Ok(Self {
            template_client: TemplateServiceClient::connect(server_address.clone()).await?,
        })
    }
}

impl Actor for TemplateClient {
    type Context = Context<Self>;
}

impl Handler<CreateTemplate> for TemplateClient {
    type Result = ResponseFuture<Result<Identifier>>;

    fn handle(&mut self, msg: CreateTemplate, _ctx: &mut Self::Context) -> Self::Result {
        let template_deployment = msg.0;
        let mut client = self.template_client.clone();
        Box::pin(async move {
            let result = client
                .create(tonic::Request::new(template_deployment))
                .await?;
            Ok(result.into_inner())
        })
    }
}

impl Handler<DeleteTemplate> for TemplateClient {
    type Result = ResponseFuture<Result<()>>;

    fn handle(&mut self, msg: DeleteTemplate, _ctx: &mut Self::Context) -> Self::Result {
        let identifier = msg.0;
        let mut client = self.template_client.clone();
        Box::pin(async move {
            let result = client.delete(tonic::Request::new(identifier)).await;
            if let Err(status) = result {
                return Err(anyhow::anyhow!("{:?}", status));
            }
            Ok(())
        })
    }
}
