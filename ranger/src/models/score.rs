use super::helpers::uuid::Uuid;
use crate::models::ConditionMessage;
use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use sdl_parser::metric::Metric;
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
    pub value: BigDecimal,
    pub timestamp: NaiveDateTime,
}

impl Score {
    pub fn new(
        exercise_id: Uuid,
        deployment_id: Uuid,
        metric_name: String,
        vm_name: String,
        value: BigDecimal,
        timestamp: NaiveDateTime,
    ) -> Self {
        Self {
            id: Uuid::random(),
            exercise_id,
            deployment_id,
            metric_name,
            vm_name,
            value,
            timestamp,
        }
    }

    pub fn from_conditionmessage_and_metric(
        condition_message: ConditionMessage,
        metric: Option<(String, Metric)>,
        vm_name: String,
    ) -> Self {
        let mut metric_name = Default::default();
        let mut max_score: BigDecimal = Default::default();

        if let Some((metric_key, metric)) = metric {
            metric_name = match &metric.name {
                Some(metric_name) => metric_name.to_owned(),
                None => metric_key,
            };
            max_score = BigDecimal::from(metric.max_score);
        }

        Self {
            id: condition_message.id,
            exercise_id: condition_message.exercise_id,
            deployment_id: condition_message.deployment_id,
            metric_name,
            vm_name,
            value: condition_message.value * max_score,
            timestamp: condition_message.created_at,
        }
    }
}

impl From<super::Metric> for Score {
    fn from(metric: super::Metric) -> Self {
        let score: BigDecimal = match metric.score {
            Some(score) => BigDecimal::from(score),
            None => BigDecimal::from(0),
        };

        Score::new(
            metric.exercise_id,
            metric.deployment_id,
            metric.name,
            metric.entity_selector,
            score,
            metric.updated_at,
        )
    }
}
