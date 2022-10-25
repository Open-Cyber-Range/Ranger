use super::Database;
use crate::constants::RECORD_NOT_FOUND;
use crate::models::helpers::uuid::Uuid;
use crate::models::{Exercise, NewExercise};
use actix::{Handler, Message};
use anyhow::{anyhow, Result};
use diesel::RunQueryDsl;

#[derive(Message)]
#[rtype(result = "Result<Exercise>")]
pub struct CreateExercise(pub NewExercise);

impl Handler<CreateExercise> for Database {
    type Result = Result<Exercise>;

    fn handle(&mut self, msg: CreateExercise, _ctx: &mut Self::Context) -> Self::Result {
        let new_exercise = msg.0;
        let mut connection = self.get_connection()?;

        new_exercise.create_insert().execute(&mut connection)?;
        let exercise = Exercise::by_id(new_exercise.id).first(&mut connection)?;

        Ok(exercise)
    }
}

#[derive(Message)]
#[rtype(result = "Result<Exercise>")]
pub struct GetExercise(pub Uuid);

impl Handler<GetExercise> for Database {
    type Result = Result<Exercise>;

    fn handle(&mut self, msg: GetExercise, _ctx: &mut Self::Context) -> Self::Result {
        let uuid = msg.0;
        let mut connection = self.get_connection()?;

        let exercise = Exercise::by_id(uuid).first(&mut connection)?;

        Ok(exercise)
    }
}

#[derive(Message)]
#[rtype(result = "Result<Exercise>")]
pub struct UpdateExercise(pub Uuid, pub crate::models::UpdateExercise);

impl Handler<UpdateExercise> for Database {
    type Result = Result<Exercise>;

    fn handle(&mut self, msg: UpdateExercise, _ctx: &mut Self::Context) -> Self::Result {
        let uuid = msg.0;
        let update_exercise = msg.1;

        let mut connection = self.get_connection()?;
        let updated_rows = update_exercise
            .create_update(uuid)
            .execute(&mut connection)?;
        if updated_rows == 0 {
            return Err(anyhow!(RECORD_NOT_FOUND));
        }
        let exercise = Exercise::by_id(uuid).first(&mut connection)?;

        Ok(exercise)
    }
}

#[derive(Message)]
#[rtype(result = "Result<Uuid>")]
pub struct DeleteExercise(pub Uuid);

impl Handler<DeleteExercise> for Database {
    type Result = Result<Uuid>;

    fn handle(&mut self, msg: DeleteExercise, _ctx: &mut Self::Context) -> Self::Result {
        let id = msg.0;
        let mut connection = self.get_connection()?;

        let exercise = Exercise::by_id(id).first(&mut connection)?;
        exercise.soft_delete().execute(&mut connection)?;

        Ok(id)
    }
}
