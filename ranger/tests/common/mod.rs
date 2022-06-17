use actix::Actor;
use actix_web::web::Data;
use anyhow::Result;
use lazy_static::lazy_static;
use rand::Rng;
use ranger::{
    configuration::Configuration,
    database::{AddScenario, Database},
    deployer::DeploymentManager,
    node::NodeClient,
    AppState,
};
use ranger_grpc::{
    node_service_server::{NodeService, NodeServiceServer},
    Identifier, NodeDeployment, NodeIdentifier, NodeType,
};

use std::time::Duration;
use tokio::runtime::Runtime;
use tonic::{transport::Server, Request, Response, Status};

lazy_static! {
    pub static ref CONFIGURATION: Configuration = Configuration {
        node_deployer_addresses: vec![
            "http://localhost:9999".to_string(),
            "http://localhost:9998".to_string(),
            "http://localhost:9997".to_string(),
        ],
        host: "localhost".to_string(),
        port: 3000,
    };
}

pub struct MockNodeService {
    builder: MockNodeBuilder,
}

impl MockNodeService {
    pub(crate) fn new(builder: MockNodeBuilder) -> Self {
        Self { builder }
    }
}

#[tonic::async_trait]
impl NodeService for MockNodeService {
    async fn create(&self, _: Request<NodeDeployment>) -> Result<Response<NodeIdentifier>, Status> {
        if self.builder.successful_create {
            Status::ok("Node created successfully");
            return Ok(Response::new(NodeIdentifier {
                identifier: Some(Identifier {
                    value: String::from("Some UUID"),
                }),
                node_type: NodeType::Vm.into(),
            }));
        }

        return Err(Status::internal("Failed to create node"));
    }

    async fn delete(&self, _: Request<NodeIdentifier>) -> Result<Response<()>, Status> {
        if self.builder.successful_delete {
            return Ok(Response::new(()));
        }
        return Err(Status::internal("Failed to delete node"));
    }
}

pub(crate) fn create_mock_node_server() -> MockNodeBuilder {
    MockNodeBuilder {
        successful_create: true,
        successful_delete: true,
        server_address: None,
        timeout_millis: 100,
    }
}

#[derive(Clone)]
pub(crate) struct MockNodeBuilder {
    pub(crate) successful_create: bool,
    pub(crate) successful_delete: bool,
    pub(crate) timeout_millis: u64,
    pub(crate) server_address: Option<String>,
}

impl MockNodeBuilder {
    fn random_port() -> u16 {
        rand::thread_rng().gen_range(1024..65535)
    }

    pub fn successful_create(mut self, value: bool) -> Self {
        self.successful_create = value;
        self
    }

    pub fn successful_delete(mut self, value: bool) -> Self {
        self.successful_delete = value;
        self
    }

    #[allow(dead_code)]
    pub fn timeout_millis(mut self, value: u64) -> Self {
        self.timeout_millis = value;
        self
    }

    #[allow(dead_code)]
    pub fn server_address(mut self, value: Option<String>) -> Self {
        self.server_address = value;
        self
    }

    pub fn run(self) -> Result<String> {
        let server_address_string = match self.server_address.clone() {
            Some(address) => address,
            None => format!("127.0.0.1:{}", Self::random_port()),
        };
        let server_address = server_address_string.parse()?;
        let mock_server = MockNodeService::new(self.clone());
        let tokio_runtime = Runtime::new()?;

        std::thread::spawn(move || {
            tokio_runtime.block_on(async move {
                Server::builder()
                    .add_service(NodeServiceServer::new(mock_server))
                    .serve(server_address)
                    .await?;
                Ok::<(), anyhow::Error>(())
            })?;
            Ok::<(), anyhow::Error>(())
        });
        std::thread::sleep(Duration::from_millis(self.timeout_millis));

        Ok(server_address_string)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test, App};
    use ranger::routes::exercise::add_exercise;
    use sdl_parser::test::{TEST_SCHEMA, TEST_SCHEMA_STRING};
    


    pub async fn create_test_app_state() -> Data<AppState> {
        let socket_address = crate::common::create_mock_node_server().run().unwrap();
        let node_client_address = NodeClient::new(format!("http://{}", socket_address))
            .await
            .unwrap()
            .start();
        let database_address = Database::new().start();
        database_address
            .send(AddScenario(TEST_SCHEMA.scenario.clone()))
            .await
            .unwrap();
        let deployer_address = DeploymentManager::new(node_client_address).start();
        let app_state = AppState {
            database_address,
            deployer_address,
        };
        Data::new(app_state)
    }

    #[actix_web::test]
    pub async fn node_deployed_successfully() -> Result<()> {
        let app_state = create_test_app_state().await;
        let app = test::init_service(App::new().app_data(app_state).service(add_exercise)).await;
        let payload = Vec::try_from(TEST_SCHEMA_STRING)?;
        let request = test::TestRequest::post()
            .uri("/exercise")
            .set_payload(payload)
            .to_request();
        let response = test::call_service(&app, request).await;
        assert_eq!(response.status(), 200);
        Ok(())
    }
}
