use crate::{
    models::event_info::NewEventInfo,
    services::{
        client::EventInfoResponse,
        database::event_info::{CheckEventInfo, CreateEventInfo},
        deployer::Deploy,
    },
    utilities::scenario::get_event_sources,
    Addressor,
};
use anyhow::{anyhow, Ok, Result};
use async_trait::async_trait;
use log::{debug, error};
use ranger_grpc::capabilities::DeployerType as GrpcDeployerType;
use ranger_grpc::Source as GrpcSource;
use sdl_parser::Scenario;
use sha3::{Digest, Sha3_256};

use super::{condition::ConditionProperties, node::DeployedNode};

#[async_trait]
pub trait EventInfoUnpacker {
    async fn create_event_info_pages(
        &self,
        addressor: &Addressor,
        deployers: &[String],
        deployed_nodes: &[(DeployedNode, Vec<ConditionProperties>)],
    ) -> Result<()>;
}

#[async_trait]
impl EventInfoUnpacker for Scenario {
    async fn create_event_info_pages(
        &self,
        addressor: &Addressor,
        deployers: &[String],
        deployed_nodes: &[(DeployedNode, Vec<ConditionProperties>)],
    ) -> Result<()> {
        for (_, event_source) in get_event_sources(&self.events) {
            let grpc_source = Box::new(GrpcSource {
                name: event_source.name.to_owned(),
                version: event_source.version.to_owned(),
            });

            match addressor
                .distributor
                .send(Deploy(
                    GrpcDeployerType::EventInfo,
                    grpc_source,
                    deployers.to_owned(),
                ))
                .await?
            {
                anyhow::Result::Ok(handler_response) => {
                    let (event_create_response, mut event_file_stream) =
                        EventInfoResponse::try_from(handler_response)?;

                    DeployedNode::update_node_events(
                        &addressor.database,
                        deployed_nodes,
                        event_create_response.checksum.clone(),
                    )
                    .await?;

                    let event_info_file_already_exists = addressor
                        .database
                        .send(CheckEventInfo(event_create_response.checksum.clone(), true))
                        .await??;
                    if event_info_file_already_exists {
                        debug!("Event already exists in the database, skipping download step");
                        continue;
                    }

                    let mut event_file_buffer = Vec::new();
                    while let Some(stream_response) = event_file_stream.message().await? {
                        event_file_buffer.extend_from_slice(&stream_response.chunk);
                    }

                    let mut hasher = Sha3_256::new();
                    hasher.update(&event_file_buffer);
                    let computed_checksum = format!("{:x}", hasher.finalize());
                    if computed_checksum == event_create_response.checksum {
                        debug!(
                            "Event {event_name} info file Checksum verification passed",
                            event_name = event_source.name
                        );
                        addressor
                            .database
                            .send(CreateEventInfo(
                                NewEventInfo {
                                    checksum: event_create_response.checksum,
                                    name: event_source.name,
                                    file_name: event_create_response.filename,
                                    file_size: event_create_response.size as u64,
                                    content: event_file_buffer,
                                },
                                true,
                            ))
                            .await??;
                    } else {
                        error!(
                            "Checksum verification for EventInfo file failed for event {event_name}, version {event_version}",
                            event_name = event_source.name, event_version = event_source.version
                        );
                        return Err(anyhow!("Checksum verification for EventInfo file failed"));
                    }
                    Ok(())
                }
                anyhow::Result::Err(error) => return Err(anyhow!("Event info error: {error}")),
            }?;
        }

        Ok(())
    }
}
