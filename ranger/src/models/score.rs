use super::helpers::uuid::Uuid;
use crate::models::ConditionMessage;
use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use sdl_parser::metric::Metrics;
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

    pub fn from_conditionmessage_and_metrics(
        condition_message: ConditionMessage,
        metrics: Option<Metrics>,
    ) -> Self {
        let metric_name = if let Some(metrics) = metrics {
            if let Some((metric_key, metric)) = metrics.iter().find(|(_, metric)| {
                metric
                    .condition
                    .eq(&Some(condition_message.clone().condition_name))
            }) {
                match &metric.name {
                    Some(metric_name) => metric_name.to_owned(),
                    None => metric_key.to_owned(),
                }
            } else {
                condition_message.condition_name
            }
        } else {
            condition_message.condition_name
        };

        Self {
            id: condition_message.id,
            exercise_id: condition_message.exercise_id,
            deployment_id: condition_message.deployment_id,
            metric_name,
            vm_name: condition_message.virtual_machine_id.to_string(),
            vm_uuid: condition_message.virtual_machine_id,
            value: condition_message.value.clone(),
            timestamp: condition_message.created_at,
        }
    }
}
