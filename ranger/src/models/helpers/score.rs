use super::uuid::Uuid;
use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

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
    pub created_at: NaiveDateTime,
}

impl Score {
    pub fn new(
        exercise_id: Uuid,
        deployment_id: Uuid,
        metric_name: String,
        vm_name: String,
        vm_uuid: Uuid,
        value: BigDecimal,
        created_at: NaiveDateTime,
    ) -> Self {
        Self {
            id: Uuid::random(),
            exercise_id,
            deployment_id,
            metric_name,
            vm_name,
            vm_uuid,
            value,
            created_at,
        }
    }
}
