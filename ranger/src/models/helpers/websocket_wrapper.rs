use serde::Serialize;

use super::uuid::Uuid;
use crate::models::{Deployment, DeploymentElement, UpdateExercise};

#[derive(Debug, Serialize)]
pub enum MessageType {
    ExerciseUpdate,
    Deployment,
    DeploymentElement,
    DeploymentElementUpdate,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WebsocketWrapper<T>
where
    T: Serialize,
{
    #[serde(rename = "type")]
    pub message_type: MessageType,
    pub exercise_id: Uuid,
    pub own_id: Uuid,
    pub content: T,
}

impl From<(Uuid, Uuid, UpdateExercise)> for WebsocketWrapper<UpdateExercise> {
    fn from(exercise_info: (Uuid, Uuid, UpdateExercise)) -> Self {
        let (exercise_id, own_id, content) = exercise_info;
        Self {
            message_type: MessageType::ExerciseUpdate,
            exercise_id,
            own_id,
            content,
        }
    }
}

impl From<(Uuid, Uuid, Deployment)> for WebsocketWrapper<Deployment> {
    fn from(deployment_info: (Uuid, Uuid, Deployment)) -> Self {
        let (exercise_id, own_id, content) = deployment_info;
        Self {
            message_type: MessageType::Deployment,
            exercise_id,
            own_id,
            content,
        }
    }
}

impl From<(Uuid, Uuid, DeploymentElement, bool)> for WebsocketWrapper<DeploymentElement> {
    fn from(deployment_element_info: (Uuid, Uuid, DeploymentElement, bool)) -> Self {
        let (exercise_id, own_id, content, is_update) = deployment_element_info;
        Self {
            message_type: match is_update {
                true => MessageType::DeploymentElementUpdate,
                false => MessageType::DeploymentElement,
            },
            exercise_id,
            own_id,
            content,
        }
    }
}
