use super::Database;
use crate::models::helpers::uuid::Uuid;
use crate::models::{Deployment, DeploymentElement, NewDeployment, ScenarioReference};
use crate::services::websocket::{SocketDeployment, SocketDeploymentElement};
use actix::{Handler, Message, ResponseActFuture, WrapFuture};
use actix_web::web::block;
use anyhow::{Ok, Result};
use diesel::RunQueryDsl;

#[derive(Message)]
#[rtype(result = "Result<Deployment>")]
pub struct CreateDeployment(pub NewDeployment);

impl Handler<CreateDeployment> for Database {
    type Result = ResponseActFuture<Self, Result<Deployment>>;

    fn handle(&mut self, msg: CreateDeployment, _ctx: &mut Self::Context) -> Self::Result {
        let new_deployment = msg.0;
        let connection_result = self.get_connection();
        let websocket_manager = self.websocket_manager_address.clone();

        Box::pin(
            async move {
                let mut connection = connection_result?;
                let deployment = block(move || {
                    new_deployment.create_insert().execute(&mut connection)?;
                    let deployment = Deployment::by_id(new_deployment.id).first(&mut connection)?;
                    websocket_manager.do_send(SocketDeployment(
                        deployment.exercise_id,
                        (deployment.exercise_id, deployment.id, deployment.clone()).into(),
                    ));
                    Ok(deployment)
                })
                .await??;

                Ok(deployment)
            }
            .into_actor(self),
        )
    }
}

#[derive(Message)]
#[rtype(result = "Result<Vec<Deployment>>")]
pub struct GetDeployments(pub Uuid);

impl Handler<GetDeployments> for Database {
    type Result = ResponseActFuture<Self, Result<Vec<Deployment>>>;

    fn handle(&mut self, msg: GetDeployments, _ctx: &mut Self::Context) -> Self::Result {
        let id = msg.0;
        let connection_result = self.get_connection();

        Box::pin(
            async move {
                let mut connection = connection_result?;
                let deployments = block(move || {
                    let deployments = Deployment::by_exercise_id(id).load(&mut connection)?;
                    Ok(deployments)
                })
                .await??;

                Ok(deployments)
            }
            .into_actor(self),
        )
    }
}

#[derive(Message)]
#[rtype(result = "Result<Deployment>")]
pub struct GetDeployment(pub Uuid);

impl Handler<GetDeployment> for Database {
    type Result = ResponseActFuture<Self, Result<Deployment>>;

    fn handle(&mut self, msg: GetDeployment, _ctx: &mut Self::Context) -> Self::Result {
        let id = msg.0;
        let connection_result = self.get_connection();

        Box::pin(
            async move {
                let mut connection = connection_result?;
                let deployment = block(move || {
                    let deployment = Deployment::by_id(id).first(&mut connection)?;
                    Ok(deployment)
                })
                .await??;

                Ok(deployment)
            }
            .into_actor(self),
        )
    }
}

#[derive(Message)]
#[rtype(result = "Result<Uuid>")]
pub struct DeleteDeployment(pub Uuid);

impl Handler<DeleteDeployment> for Database {
    type Result = ResponseActFuture<Self, Result<Uuid>>;

    fn handle(&mut self, msg: DeleteDeployment, _ctx: &mut Self::Context) -> Self::Result {
        let id = msg.0;
        let connection_result = self.get_connection();

        Box::pin(
            async move {
                let mut connection = connection_result?;
                let id = block(move || {
                    let deployment = Deployment::by_id(id).first(&mut connection)?;
                    deployment.soft_delete_elements().execute(&mut connection)?;
                    deployment.soft_delete().execute(&mut connection)?;

                    Ok(id)
                })
                .await??;

                Ok(id)
            }
            .into_actor(self),
        )
    }
}

#[derive(Message)]
#[rtype(result = "Result<DeploymentElement>")]
pub struct CreateDeploymentElement(pub Uuid, pub DeploymentElement);

impl Handler<CreateDeploymentElement> for Database {
    type Result = ResponseActFuture<Self, Result<DeploymentElement>>;

    fn handle(&mut self, msg: CreateDeploymentElement, _ctx: &mut Self::Context) -> Self::Result {
        let CreateDeploymentElement(exercise_uuid, new_deployment_element) = msg;
        let connection_result = self.get_connection();
        let websocket_manager = self.websocket_manager_address.clone();

        Box::pin(
            async move {
                let mut connection = connection_result?;
                let deployment_element = block(move || {
                    new_deployment_element
                        .create_insert()
                        .execute(&mut connection)?;
                    let deployment_element = DeploymentElement::by_id(new_deployment_element.id)
                        .first(&mut connection)?;
                    websocket_manager.do_send(SocketDeploymentElement(
                        exercise_uuid,
                        (
                            exercise_uuid,
                            deployment_element.id,
                            deployment_element.clone(),
                            false,
                        )
                            .into(),
                    ));

                    Ok(deployment_element)
                })
                .await??;

                Ok(deployment_element)
            }
            .into_actor(self),
        )
    }
}

#[derive(Message)]
#[rtype(result = "Result<DeploymentElement>")]
pub struct UpdateDeploymentElement(pub Uuid, pub DeploymentElement);

impl Handler<UpdateDeploymentElement> for Database {
    type Result = ResponseActFuture<Self, Result<DeploymentElement>>;

    fn handle(&mut self, msg: UpdateDeploymentElement, _ctx: &mut Self::Context) -> Self::Result {
        let UpdateDeploymentElement(exercise_uuid, new_deployment_element) = msg;
        let connection_result = self.get_connection();
        let websocket_manager = self.websocket_manager_address.clone();

        Box::pin(
            async move {
                let mut connection = connection_result?;
                let deployment_element = block(move || {
                    let updated_rows = new_deployment_element
                        .create_update()
                        .execute(&mut connection)?;
                    if updated_rows != 1 {
                        return Err(anyhow::anyhow!("Deployment element not found"));
                    }
                    websocket_manager.do_send(SocketDeploymentElement(
                        exercise_uuid,
                        (
                            exercise_uuid,
                            new_deployment_element.id,
                            new_deployment_element.clone(),
                            true,
                        )
                            .into(),
                    ));

                    Ok(DeploymentElement::by_id(new_deployment_element.id)
                        .first(&mut connection)?)
                })
                .await??;

                Ok(deployment_element)
            }
            .into_actor(self),
        )
    }
}

#[derive(Message)]
#[rtype(result = "Result<DeploymentElement>")]
pub struct GetDeploymentElementByDeploymentIdByScenarioReference(
    pub Uuid,
    pub Box<dyn ScenarioReference>,
);

impl Handler<GetDeploymentElementByDeploymentIdByScenarioReference> for Database {
    type Result = ResponseActFuture<Self, Result<DeploymentElement>>;

    fn handle(
        &mut self,
        msg: GetDeploymentElementByDeploymentIdByScenarioReference,
        _ctx: &mut Self::Context,
    ) -> Self::Result {
        let deployment_id = msg.0;
        let scenario_reference = msg.1;
        let connection_result = self.get_connection();

        Box::pin(
            async move {
                let mut connection = connection_result?;
                let deployment_element = block(move || {
                    DeploymentElement::by_deployer_id_by_scenario_reference(
                        deployment_id,
                        scenario_reference,
                    )
                    .first(&mut connection)
                })
                .await??;

                Ok(deployment_element)
            }
            .into_actor(self),
        )
    }
}

#[derive(Message)]
#[rtype(result = "Result<Vec<DeploymentElement>>")]
pub struct GetDeploymentElementByDeploymentId(pub Uuid);

impl Handler<GetDeploymentElementByDeploymentId> for Database {
    type Result = ResponseActFuture<Self, Result<Vec<DeploymentElement>>>;

    fn handle(
        &mut self,
        msg: GetDeploymentElementByDeploymentId,
        _ctx: &mut Self::Context,
    ) -> Self::Result {
        let deployment_id = msg.0;
        let connection_result = self.get_connection();

        Box::pin(
            async move {
                let mut connection = connection_result?;
                let deployment_elements = block(move || {
                    let deployment_elements =
                        DeploymentElement::by_deployment_id(deployment_id).load(&mut connection)?;

                    Ok(deployment_elements)
                })
                .await??;

                Ok(deployment_elements)
            }
            .into_actor(self),
        )
    }
}
