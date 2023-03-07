use crate::{
    models::{DeploymentElement, NewConditionMessage},
    services::{
        database::{condition::CreateConditionMessage, Database},
        deployer::DeployerDistribution,
    },
};

use super::{DeploymentClient, DeploymentClientResponse, DeploymentInfo};
use crate::models::helpers::uuid::Uuid;
use actix::{
    Actor, Addr, Context, Handler, Message, ResponseActFuture, ResponseFuture, WrapFuture,
};
use anyhow::{anyhow, Ok, Result};
use async_trait::async_trait;
use bigdecimal::{BigDecimal, FromPrimitive};
use ranger_grpc::{
    condition_service_client::ConditionServiceClient, Condition as GrpcCondition,
    ConditionStreamResponse, Identifier,
};
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
    pub String,
    pub Addr<Database>,
    pub Streaming<ConditionStreamResponse>,
);
impl Handler<ConditionStream> for DeployerDistribution {
    type Result = ResponseActFuture<Self, Result<()>>;

    fn handle(&mut self, msg: ConditionStream, _ctx: &mut Self::Context) -> Self::Result {
        let exercise_id = msg.0;
        let deployment_element = msg.1;
        let virtual_machine_id = msg.2;
        let database_address = msg.3;
        let mut stream = msg.4;

        Box::pin(
            async move {
                while let Some(stream_item) = stream.message().await? {
                    let condition_id: Uuid = deployment_element
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
                        .send(CreateConditionMessage(NewConditionMessage::new(
                            exercise_id,
                            deployment_element.deployment_id,
                            Uuid::try_from(virtual_machine_id.as_str())?,
                            deployment_element.scenario_reference.to_owned(),
                            condition_id,
                            value,
                        )))
                        .await??;
                }
                Ok(())
            }
            .into_actor(self),
        )
    }
}
