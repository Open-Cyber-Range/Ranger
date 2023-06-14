use crate::{
    models::{DeploymentElement, ElementStatus, NewConditionMessage},
    services::{
        database::{condition::CreateConditionMessage, Database},
        deployer::DeployerDistribution,
    },
    utilities::try_some,
};

use super::{DeploymentClient, DeploymentClientResponse, DeploymentInfo};
use crate::models::helpers::uuid::Uuid;
use actix::{
    Actor, Addr, Context, Handler, Message, ResponseActFuture, ResponseFuture, WrapFuture,
};
use anyhow::{anyhow, Ok, Result};
use async_trait::async_trait;
use bigdecimal::{BigDecimal, FromPrimitive};
use log::info;
use ranger_grpc::{
    condition_service_client::ConditionServiceClient, Condition as GrpcCondition,
    ConditionStreamResponse, Identifier,
};
use sdl_parser::metric::Metric;
use std::any::Any;
use tonic::{transport::Channel, Streaming};

impl DeploymentInfo for GrpcCondition {
    fn get_deployment(&self) -> GrpcCondition {
        self.clone()
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}
pub struct ConditionClient {
    condition_client: ConditionServiceClient<Channel>,
}

impl ConditionClient {
    pub async fn new(server_address: String) -> Result<Self> {
        Ok(Self {
            condition_client: ConditionServiceClient::connect(server_address).await?,
        })
    }
}

impl Actor for ConditionClient {
    type Context = Context<Self>;
}

#[async_trait]
impl DeploymentClient<Box<dyn DeploymentInfo>> for Addr<ConditionClient> {
    async fn deploy(
        &mut self,
        deployment_struct: Box<dyn DeploymentInfo>,
    ) -> Result<DeploymentClientResponse> {
        let deployment = CreateCondition(
            deployment_struct
                .as_any()
                .downcast_ref::<GrpcCondition>()
                .unwrap()
                .clone(),
        );
        let identifier = self.send(deployment).await??;

        let stream = self
            .send(CreateConditionStream(Identifier {
                value: identifier.value.to_owned(),
            }))
            .await??;
        Ok(DeploymentClientResponse::ConditionResponse((
            identifier, stream,
        )))
    }

    async fn undeploy(&mut self, handler_reference: String) -> Result<()> {
        let undeploy = DeleteCondition(Identifier {
            value: handler_reference,
        });
        self.send(undeploy).await??;

        Ok(())
    }
}

#[derive(Message)]
#[rtype(result = "Result<Identifier>")]
pub struct CreateCondition(pub GrpcCondition);

impl Handler<CreateCondition> for ConditionClient {
    type Result = ResponseFuture<Result<Identifier>>;

    fn handle(&mut self, msg: CreateCondition, _ctx: &mut Self::Context) -> Self::Result {
        let condition_deployment = msg.0;
        let mut client = self.condition_client.clone();

        Box::pin(async move {
            let result = client
                .create(tonic::Request::new(condition_deployment))
                .await?;
            Ok(result.into_inner())
        })
    }
}

#[derive(Message)]
#[rtype(result = "Result<Streaming<ConditionStreamResponse>>")]
pub struct CreateConditionStream(pub Identifier);

impl Handler<CreateConditionStream> for ConditionClient {
    type Result = ResponseFuture<Result<Streaming<ConditionStreamResponse>>>;

    fn handle(&mut self, msg: CreateConditionStream, _ctx: &mut Self::Context) -> Self::Result {
        let identifier = msg.0;
        let mut client = self.condition_client.clone();

        Box::pin(async move {
            let stream = client
                .stream(tonic::Request::new(identifier))
                .await?
                .into_inner();

            Ok(stream)
        })
    }
}

#[derive(Message)]
#[rtype(result = "Result<()>")]
pub struct DeleteCondition(pub Identifier);

impl Handler<DeleteCondition> for ConditionClient {
    type Result = ResponseFuture<Result<()>>;

    fn handle(&mut self, msg: DeleteCondition, _ctx: &mut Self::Context) -> Self::Result {
        let identifier = msg.0;
        let mut client = self.condition_client.clone();
        Box::pin(async move {
            let result = client.delete(tonic::Request::new(identifier)).await;
            if let Err(status) = result {
                return Err(anyhow::anyhow!("{:?}", status));
            }
            Ok(())
        })
    }
}

#[derive(Message)]
#[rtype(result = "Result<()>")]
pub struct ConditionStream(
    pub Uuid,
    pub DeploymentElement,
    pub DeploymentElement,
    pub Addr<Database>,
    pub Streaming<ConditionStreamResponse>,
    pub Option<(String, Metric)>,
);
impl Handler<ConditionStream> for DeployerDistribution {
    type Result = ResponseActFuture<Self, Result<()>>;

    fn handle(&mut self, msg: ConditionStream, _ctx: &mut Self::Context) -> Self::Result {
        let exercise_id = msg.0;
        let mut condition_deployment_element = msg.1;
        let node_deployment_element = msg.2;
        let database_address = msg.3;
        let mut stream = msg.4;
        let metric = msg.5;

        Box::pin(
            async move {
                let virtual_machine_id = try_some(
                    node_deployment_element.clone().handler_reference,
                    "Deployment element handler reference not found",
                )?;

                info!(
                    "Finished deploying {condition_name} on {node_name}, starting stream",
                    condition_name = condition_deployment_element.scenario_reference,
                    node_name = node_deployment_element.scenario_reference,
                );

                let is_event_condition = condition_deployment_element.event_id.is_some();
                let big_decimal_one =
                    try_some(BigDecimal::from_i8(1), "BigDecimal conversion error")?;

                while let Some(stream_item) = stream.message().await? {
                    let condition_id: Uuid = condition_deployment_element
                        .clone()
                        .handler_reference
                        .ok_or_else(|| anyhow!("Condition id not found"))?
                        .as_str()
                        .try_into()?;

                    let value = BigDecimal::from_f32(stream_item.command_return_value)
                        .ok_or_else(|| anyhow!("Error converting Condition Return value"))?;

                    log::debug!(
                        "Received Condition Id: {:?}, Value: {:?}",
                        stream_item.response,
                        stream_item.command_return_value,
                    );
                    database_address
                        .clone()
                        .send(CreateConditionMessage(
                            NewConditionMessage::new(
                                exercise_id,
                                condition_deployment_element.deployment_id,
                                Uuid::try_from(virtual_machine_id.as_str())?,
                                condition_deployment_element.scenario_reference.to_owned(),
                                condition_id,
                                value,
                            ),
                            metric.clone(),
                            node_deployment_element.scenario_reference.clone(),
                        ))
                        .await??;

                    if is_event_condition {
                        if value == big_decimal_one
                            && condition_deployment_element.status == ElementStatus::Ongoing
                        {
                            condition_deployment_element.status = ElementStatus::Success;
                            condition_deployment_element
                                .update(
                                    &database_address,
                                    exercise_id,
                                    ElementStatus::Success,
                                    condition_deployment_element.handler_reference.clone(),
                                )
                                .await?;
                        } else if value != big_decimal_one
                            && condition_deployment_element.status == ElementStatus::Success
                        {
                            condition_deployment_element.status = ElementStatus::Ongoing;
                            condition_deployment_element
                                .update(
                                    &database_address,
                                    exercise_id,
                                    ElementStatus::Ongoing,
                                    condition_deployment_element.handler_reference.clone(),
                                )
                                .await?;
                        }
                    }
                }
                Ok(())
            }
            .into_actor(self),
        )
    }
}
