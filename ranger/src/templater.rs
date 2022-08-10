use std::collections::HashMap;

use actix::{Actor, Addr, Context, Handler, Message, ResponseFuture};
use anyhow::{anyhow, Ok, Result};
use async_trait::async_trait;
use futures::future::join_all;
use futures::Future;
use log::error;
use ranger_grpc::NodeDeployment;
use ranger_grpc::{
    template_service_client::TemplateServiceClient, Identifier, Source as GrpcSource,
};
use sdl_parser::node::Source;
use sdl_parser::Scenario;
use tonic::transport::Channel;

use crate::machiner::NodeDeploymentTrait;

#[derive(Message)]
#[rtype(result = "Result<Identifier, anyhow::Error>")]
pub struct CreateTemplate(pub GrpcSource);

#[derive(Message)]
#[rtype(result = "Result<(), anyhow::Error>")]
pub struct DeleteTemplate(pub Identifier);

pub struct TemplateClient {
    template_client: TemplateServiceClient<Channel>,
}

#[async_trait]
trait DeployerActions<T>
where
    Self: Actor,
{
    async fn create_template(templater_address: &Addr<Self>, create_info: T) -> Result<String>;
}
#[async_trait]
impl DeployerActions<Source> for TemplateClient {
    async fn create_template(
        templater_address: &Addr<TemplateClient>,
        source: Source,
    ) -> Result<String> {
        Ok(templater_address
            .send(CreateTemplate(GrpcSource {
                name: source.name.clone(),
                version: source.version.clone(),
            }))
            .await??
            .value)
    }
}

pub fn initiate_template_clients(
    deployers: HashMap<String, String>,
) -> Vec<impl Future<Output = Result<(String, Addr<TemplateClient>), anyhow::Error>>> {
    deployers
        .into_iter()
        .map(|(name, ip)| async {
            Ok::<(String, Addr<TemplateClient>)>((name, TemplateClient::new(ip).await?.start()))
        })
        .collect()
}

pub type TemplateClientResults = Vec<Result<(String, Addr<TemplateClient>), anyhow::Error>>;
pub trait TemplateClientFilter {
    fn filter_template_clients(self) -> HashMap<String, Addr<TemplateClient>>;
}
impl TemplateClientFilter for TemplateClientResults {
    fn filter_template_clients(self) -> HashMap<String, Addr<TemplateClient>> {
        self.into_iter()
            .filter_map(|node_client| {
                node_client
                    .map_err(|error| error!("Error setting up TemplateClient: {}", error))
                    .ok()
            })
            .collect::<HashMap<String, Addr<TemplateClient>>>()
    }
}

pub fn separate_node_deployments_by_type(
    node_deployments: Vec<NodeDeployment>,
) -> Result<(Vec<NodeDeployment>, Vec<NodeDeployment>)> {
    let mut machiner_deployments = vec![];
    let mut switcher_deployments = vec![];
    for node_deployment in node_deployments {
        match node_deployment
            .clone()
            .node
            .ok_or_else(|| anyhow!("Error getting node"))?
            .identifier
            .ok_or_else(|| anyhow!("Error getting identifier"))?
            .node_type()
        {
            ranger_grpc::NodeType::Vm => machiner_deployments.push(node_deployment),
            ranger_grpc::NodeType::Switch => switcher_deployments.push(node_deployment),
        }
    }
    Ok((machiner_deployments, switcher_deployments))
}
pub fn filter_node_deployments(
    node_deployment_results: Vec<Result<NodeDeployment, anyhow::Error>>,
) -> Result<Vec<NodeDeployment>> {
    let node_deployments: Vec<NodeDeployment> = node_deployment_results
        .into_iter()
        .filter_map(|node_deployment| {
            node_deployment
                .map_err(|error| error!("Error creating node deployment: {}", error))
                .ok()
        })
        .collect();
    if node_deployments.is_empty() {
        Err(anyhow!("No nodes to deploy"))
    } else {
        Ok(node_deployments)
    }
}

pub async fn create_node_deployments(
    scenario: Scenario,
    templaters: &HashMap<String, Addr<TemplateClient>>,
    exercise_name: &str,
) -> Result<Vec<Result<NodeDeployment>>> {
    let nodes = scenario.nodes.ok_or_else(|| anyhow!("Nodes not found"))?;
    let node_deployments = join_all(nodes.iter().zip(templaters.iter().cycle()).map(
        |((node_name, node), (_, templater_address))| async move {
            let source = node
                .source
                .as_ref()
                .ok_or_else(|| anyhow!("Source not found"))?;
            let template_id =
                TemplateClient::create_template(templater_address, source.clone()).await?;
            let node_deployment = match node.type_field {
                sdl_parser::node::NodeType::VM => NodeDeployment::default().initialize_vm(
                    node.clone(),
                    node_name.to_string(),
                    template_id,
                    exercise_name.to_string(),
                )?,
                sdl_parser::node::NodeType::Network => NodeDeployment::default()
                    .initialize_switch(
                        node_name.to_string(),
                        template_id,
                        exercise_name.to_string(),
                    )?,
            };
            Ok::<NodeDeployment>(node_deployment)
        },
    ))
    .await;
    Ok(node_deployments)
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
