use anyhow::Result;
use rand::Rng;
use ranger_grpc::{
    node_service_server::{NodeService, NodeServiceServer},
    Error, Identifier, IdentifierResult, Node, SimpleResult,
};
use std::time::Duration;
use tokio::runtime::Runtime;
use tonic::{transport::Server, Request, Response, Status};

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
    async fn create(&self, _: Request<Node>) -> Result<Response<IdentifierResult>, Status> {
        if self.builder.successful_create {
            return Ok(Response::new(IdentifierResult {
                identifier: Some(Identifier {
                    value: String::from("Some_UUID"),
                }),
                error: None,
            }));
        }
        Ok(Response::new(IdentifierResult {
            identifier: None,
            error: Some(Error {
                message: String::from("Failed to create node"),
            }),
        }))
    }

    async fn delete(&self, _: Request<Identifier>) -> Result<Response<SimpleResult>, Status> {
        if self.builder.successful_delete {
            return Ok(Response::new(SimpleResult { error: None }));
        }
        Ok(Response::new(SimpleResult {
            error: Some(Error {
                message: String::from("Failed to delete node"),
            }),
        }))
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
