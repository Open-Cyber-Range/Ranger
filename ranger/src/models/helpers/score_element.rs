use crate::{
    models::ConditionMessage,
    services::database::{
        deployment::GetDeploymentElementByDeploymentIdByHandlerReference, Database,
    },
    utilities::{create_database_error_handler, create_mailbox_error_handler},
};

use super::uuid::Uuid;
use actix::Addr;
use anyhow::Result;
use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use sdl_parser::{metric::Metrics, Scenario};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScoreElement {
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

impl ScoreElement {
    pub fn new(
        exercise_id: Uuid,
        deployment_id: Uuid,
        metric_name: String,
        vm_uuid: Uuid,
        value: BigDecimal,
        created_at: NaiveDateTime,
    ) -> Self {
        Self {
            id: Uuid::random(),
            exercise_id,
            deployment_id,
            metric_name,
            vm_name: Default::default(),
            vm_uuid,
            value,
            created_at,
        }
    }

    pub async fn from_condition_messages_by_metric_name(
        exercise_id: Uuid,
        deployment_id: Uuid,
        scenario: Scenario,
        condition_messages: Vec<ConditionMessage>,
        requested_metric_name: String,
    ) -> Option<Vec<ScoreElement>> {
        if let Some(metrics) = scenario.metrics {
            if condition_exists_in_metrics(&condition_messages, &metrics, &requested_metric_name) {
                if let Some(requested_metric) = metrics.get(&requested_metric_name) {
                    if let Some(metric_conditon) = requested_metric.clone().condition {
                        let results = condition_messages
                            .iter()
                            .filter_map(|condition_message| {
                                if !condition_message
                                    .condition_name
                                    .eq_ignore_ascii_case(&metric_conditon)
                                {
                                    return None;
                                }

                                let calculated_score = condition_message.clone().value
                                    * BigDecimal::from(requested_metric.max_score);

                                Some(ScoreElement::new(
                                    exercise_id,
                                    deployment_id,
                                    requested_metric_name.to_owned(),
                                    condition_message.virtual_machine_id,
                                    calculated_score,
                                    condition_message.created_at,
                                ))
                            })
                            .collect::<Vec<_>>();
                        return Some(results);
                    }
                }
            }
        }
        None
    }

    pub async fn populate_vm_names(
        score_elements: Vec<ScoreElement>,
        database_address: Addr<Database>,
        deployment_uuid: Uuid,
    ) -> Result<Vec<ScoreElement>> {
        let unique_vm_uuids: HashSet<Uuid> = score_elements
            .clone()
            .into_iter()
            .map(|element| element.vm_uuid)
            .collect();

        let mut vm_names_by_uuid: HashMap<Uuid, String> = Default::default();

        for vm_uuid in unique_vm_uuids.clone() {
            let deployment_element = database_address
                .send(GetDeploymentElementByDeploymentIdByHandlerReference(
                    deployment_uuid,
                    vm_uuid.to_string(),
                ))
                .await
                .map_err(create_mailbox_error_handler("Database"))?
                .map_err(create_database_error_handler("Get deployment element"))?;
            vm_names_by_uuid.insert(vm_uuid, deployment_element.scenario_reference);
        }

        let score_elements: Vec<ScoreElement> = score_elements
            .iter()
            .map(|element| ScoreElement {
                vm_name: vm_names_by_uuid
                    .get(&element.vm_uuid)
                    .unwrap_or(&element.vm_uuid.to_string())
                    .to_string(),
                ..element.clone()
            })
            .collect();
        Ok(score_elements)
    }
}

fn condition_exists_in_metrics(
    condition_messages: &[ConditionMessage],
    metrics: &Metrics,
    req_metric_name: &str,
) -> bool {
    for condition_message in condition_messages {
        for (name, metric) in metrics.iter() {
            if name.eq_ignore_ascii_case(req_metric_name)
                && metric
                    .condition
                    .eq(&Some(condition_message.clone().condition_name))
            {
                return true;
            }
        }
    }
    false
}
