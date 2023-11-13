use super::DeputyQueryDeploymentClient;
use actix::{Actor, Addr, Context, Handler, Message, ResponseFuture};
use anyhow::{Ok, Result};
use async_trait::async_trait;
use ranger_grpc::GetScenarioResponse;
use ranger_grpc::{
    deputy_query_service_client::DeputyQueryServiceClient, GetPackagesQuery, GetPackagesResponse,
    Package, Source,
};
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

#[derive(Message)]
#[rtype(result = "Result<GetPackagesResponse>")]
pub struct GetDeputyPackages(pub GetPackagesQuery);

impl Handler<GetDeputyPackages> for DeputyQueryClient {
    type Result = ResponseFuture<Result<GetPackagesResponse>>;

    fn handle(&mut self, msg: GetDeputyPackages, _ctx: &mut Self::Context) -> Self::Result {
        let get_packages_query = msg.0;
        let mut client = self.deputy_query_client.clone();

        Box::pin(async move {
            let result = client
                .get_packages_by_type(tonic::Request::new(get_packages_query))
                .await?;
            Ok(result.into_inner())
        })
    }
}

#[derive(Message)]
#[rtype(result = "Result<GetScenarioResponse>")]
pub struct GetScenario(pub Source);

impl Handler<GetScenario> for DeputyQueryClient {
    type Result = ResponseFuture<Result<GetScenarioResponse>>;

    fn handle(&mut self, msg: GetScenario, _ctx: &mut Self::Context) -> Self::Result {
        let get_scenario_query = msg.0;
        let mut client = self.deputy_query_client.clone();

        Box::pin(async move {
            let result = client
                .get_scenario(tonic::Request::new(get_scenario_query))
                .await?;
            Ok(result.into_inner())
        })
    }
}

#[async_trait]
impl DeputyQueryDeploymentClient for Addr<DeputyQueryClient> {
    async fn packages_query_by_type(&mut self, package_type: String) -> Result<Vec<Package>> {
        let query_result = self
            .send(GetDeputyPackages(GetPackagesQuery {
                package_type: package_type.clone(),
            }))
            .await??;
        Ok(query_result.packages)
    }

    async fn get_exercise(&mut self, source: Source) -> Result<String> {
        let query_result = self.send(GetScenario(source)).await??;
        Ok(query_result.sdl)
    }
}
