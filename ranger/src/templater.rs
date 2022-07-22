use std::collections::HashMap;

use actix::{Actor, Addr, Context, Handler, Message, ResponseFuture};
use anyhow::{anyhow, Ok, Result};
use futures::Future;
use log::error;
use ranger_grpc::{
    template_service_client::TemplateServiceClient, Identifier, Source as GrpcSource,
};
use sdl_parser::node::Source;
use sdl_parser::Scenario;
use tonic::transport::Channel;

#[derive(Message)]
#[rtype(result = "Result<Identifier, anyhow::Error>")]
pub struct CreateTemplate(pub GrpcSource);

#[derive(Message)]
#[rtype(result = "Result<(), anyhow::Error>")]
pub struct DeleteTemplate(pub Identifier);

pub struct TemplateClient {
    template_client: TemplateServiceClient<Channel>,
}

pub fn initiate_template_clients(
    deployers: HashMap<String, String>,
) -> Vec<impl Future<Output = Result<Addr<TemplateClient>, anyhow::Error>>> {
    deployers
        .into_iter()
        .map(|(_, ip)| async { Ok::<Addr<TemplateClient>>(TemplateClient::new(ip).await?.start()) })
        .collect()
}

pub fn filter_template_clients(
    nodeclient_results: Vec<Result<Addr<TemplateClient>, anyhow::Error>>,
) -> Vec<Addr<TemplateClient>> {
    nodeclient_results
        .into_iter()
        .filter_map(|node_client| {
            node_client
                .map_err(|error| error!("Error setting up TemplateClient: {}", error))
                .ok()
        })
        .collect::<Vec<Addr<TemplateClient>>>()
}

async fn update_source_for_deployment(
    source: Source,
    templater_address: &Addr<TemplateClient>,
) -> Result<Source> {
    let template_id = templater_address
        .send(CreateTemplate(GrpcSource {
            name: source.name.clone(),
            version: source.version.clone(),
        }))
        .await??;
    let source_full_name = format!("{}-{}-{:?}", source.name, &source.version, template_id);
    let source = Source {
        name: source_full_name,
        version: source.version,
    };
    Ok(source)
}

pub async fn template_scenario(
    mut scenario: Scenario,
    templaters: &[Addr<TemplateClient>],
) -> Result<Scenario> {
    let nodes = scenario
        .nodes
        .as_mut()
        .ok_or_else(|| anyhow!("Nodes not found"))?;

    let templater_address = templaters
        .iter()
        .next()
        .ok_or_else(|| anyhow!("No templater available"))?;

    for (_, node) in nodes.iter_mut() {
        let source = node
            .source
            .as_ref()
            .ok_or_else(|| anyhow!("Source not found"))?;
        let templated_source =
            update_source_for_deployment(source.clone(), templater_address).await?;
        node.source = Some(templated_source);
    }
    Ok(scenario)
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
