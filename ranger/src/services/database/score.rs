use super::Database;
use crate::constants::RECORD_NOT_FOUND;
use crate::models::helpers::uuid::Uuid;
use crate::models::{NewScore, Score};
use actix::{Handler, Message, ResponseActFuture, WrapFuture};
use actix_web::web::block;
use anyhow::{anyhow, Ok, Result};
use diesel::RunQueryDsl;

#[derive(Message)]
#[rtype(result = "Result<Score>")]
pub struct CreateScore(pub NewScore);

impl Handler<CreateScore> for Database {
    type Result = ResponseActFuture<Self, Result<Score>>;

    fn handle(&mut self, msg: CreateScore, _ctx: &mut Self::Context) -> Self::Result {
        let new_score = msg.0;
        let connection_result = self.get_connection();

        Box::pin(
            async move {
                let mut connection = connection_result?;
                let score = block(move || {
                    new_score.create_insert().execute(&mut connection)?;
                    let score = Score::by_id(new_score.id).first(&mut connection)?;

                    Ok(score)
                })
                .await??;

                Ok(score)
            }
            .into_actor(self),
        )
    }
}

#[derive(Message)]
#[rtype(result = "Result<Vec<Score>>")]
pub struct GetScores(pub Uuid, pub String, pub String);

impl Handler<GetScores> for Database {
    type Result = ResponseActFuture<Self, Result<Vec<Score>>>;

    fn handle(&mut self, msg: GetScores, _ctx: &mut Self::Context) -> Self::Result {
        let connection_result = self.get_connection();
        let deployment_id = msg.0;
        let tlo_name = msg.1;
        let metric_name = msg.2;

        Box::pin(
            async move {
                let mut connection = connection_result?;
                let score = block(move || {
                    let score = Score::by_deployment_id_by_tlo_name_by_metric_name(
                        deployment_id,
                        tlo_name,
                        metric_name,
                    )
                    .load(&mut connection)?;

                    Ok(score)
                })
                .await??;

                Ok(score)
            }
            .into_actor(self),
        )
    }
}

#[derive(Message)]
#[rtype(result = "Result<Score>")]
pub struct UpdateScore(pub Uuid, pub crate::models::UpdateScore);

impl Handler<UpdateScore> for Database {
    type Result = ResponseActFuture<Self, Result<Score>>;

    fn handle(&mut self, msg: UpdateScore, _ctx: &mut Self::Context) -> Self::Result {
        let uuid = msg.0;
        let update_score = msg.1;
        let connection_result = self.get_connection();

        Box::pin(
            async move {
                let mut connection = connection_result?;
                let score = block(move || {
                    let updated_rows = update_score.create_update(uuid).execute(&mut connection)?;
                    if updated_rows != 1 {
                        return Err(anyhow!(RECORD_NOT_FOUND));
                    }
                    let score = Score::by_id(uuid).first(&mut connection)?;

                    Ok(score)
                })
                .await??;

                Ok(score)
            }
            .into_actor(self),
        )
    }
}

#[derive(Message)]
#[rtype(result = "Result<Uuid>")]
pub struct DeleteScore(pub Uuid);

impl Handler<DeleteScore> for Database {
    type Result = ResponseActFuture<Self, Result<Uuid>>;

    fn handle(&mut self, msg: DeleteScore, _ctx: &mut Self::Context) -> Self::Result {
        let id = msg.0;
        let connection_result = self.get_connection();

        Box::pin(
            async move {
                let mut connection = connection_result?;
                let id = block(move || {
                    let score = Score::by_id(id).first(&mut connection)?;
                    score.soft_delete().execute(&mut connection)?;

                    Ok(id)
                })
                .await??;

                Ok(id)
            }
            .into_actor(self),
        )
    }
}
