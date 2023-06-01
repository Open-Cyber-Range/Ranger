use super::helpers::uuid::Uuid;
use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use crate::models::ConditionMessage;

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Score {
    #[serde(default = "Uuid::random")]
    pub id: Uuid,
    pub exercise_id: Uuid,
    pub deployment_id: Uuid,
    pub metric_name: String,
    pub vm_name: String,
    pub vm_uuid: Uuid,
    pub value: BigDecimal,
    pub timestamp: NaiveDateTime,
}

impl Score {
    pub fn new(
        exercise_id: Uuid,
        deployment_id: Uuid,
        metric_name: String,
        vm_name: String,
        vm_uuid: Uuid,
        value: BigDecimal,
        timestamp: NaiveDateTime,
    ) -> Self {
        Self {
            id: Uuid::random(),
            exercise_id,
            deployment_id,
            metric_name,
            vm_name,
            vm_uuid,
            value,
            timestamp,
        }
    }
}

impl From<ConditionMessage> for Score {
    fn from(condition_message: ConditionMessage) -> Self {
        Score {
            id: condition_message.id,
            exercise_id: condition_message.exercise_id,
            deployment_id: condition_message.deployment_id,
            metric_name: condition_message.condition_name,
            vm_name: condition_message.virtual_machine_id.to_string(),
            vm_uuid: condition_message.virtual_machine_id,
            value: condition_message.value,
            timestamp: condition_message.created_at,
        }
    }
}
