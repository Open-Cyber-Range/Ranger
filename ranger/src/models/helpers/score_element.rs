use crate::models::ConditionMessage;

use super::uuid::Uuid;
use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use sdl_parser::{metric::Metrics, Scenario};
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScoreElement {
    #[serde(default = "Uuid::random")]
    pub id: Uuid,
    pub exercise_id: Uuid,
    pub deployment_id: Uuid,
    pub tlo_name: Option<String>,
    pub metric_name: String,
    pub vm_name: String,
    pub value: BigDecimal,
    pub created_at: NaiveDateTime,
}

impl ScoreElement {
    pub fn new(
        exercise_id: Uuid,
        deployment_id: Uuid,
        tlo_name: Option<String>,
        metric_name: String,
        vm_name: String,
        value: BigDecimal,
        created_at: NaiveDateTime,
    ) -> Self {
        Self {
            id: Uuid::random(),
            exercise_id,
            deployment_id,
            tlo_name,
            metric_name,
            vm_name,
            value,
            created_at,
        }
    }

    pub async fn from_condition_messages_by_metric_name(
        exercise_id: Uuid,
        deployment_id: Uuid,
        scenario: Scenario,
        condition_messages: Vec<ConditionMessage>,
        metric_name: String,
    ) -> Option<Vec<ScoreElement>> {
        let metrics = scenario.metrics.unwrap_or_default();

        if !condition_exists_in_metrics(&condition_messages, &metrics, &metric_name) {
            return None;
        }

        let results = condition_messages
            .iter()
            .map(|condition_message| {
                let mut score_multiplier: BigDecimal = Default::default();

                for (scenario_reference, metric) in metrics.iter() {
                    if metric
                        .condition
                        .eq(&Some(condition_message.clone().scenario_reference))
                        && scenario_reference.eq_ignore_ascii_case(&metric_name)
                    {
                        score_multiplier = metric.max_score.into();
                        break;
                    }
                }

                let calculated_score = condition_message.clone().value * score_multiplier;

                ScoreElement::new(
                    exercise_id,
                    deployment_id,
                    None,
                    metric_name.to_owned(),
                    condition_message.virtual_machine_id.to_string(),
                    calculated_score,
                    condition_message.created_at,
                )
            })
            .collect::<Vec<_>>();

        Some(results)
    }
}

fn condition_exists_in_metrics(
    condition_messages: &[ConditionMessage],
    metrics: &Metrics,
    req_metric_name: &str,
) -> bool {
    let mut requested_metric_condition_exist = false;
    for condition_message in condition_messages {
        for (name, metric) in metrics.iter() {
            if name.eq_ignore_ascii_case(req_metric_name)
                && metric
                    .condition
                    .eq(&Some(condition_message.clone().scenario_reference))
            {
                requested_metric_condition_exist = true;
                break;
            }
        }
        if requested_metric_condition_exist {
            break;
        }
    }
    requested_metric_condition_exist
}
