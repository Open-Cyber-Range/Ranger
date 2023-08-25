use crate::constants::{EVENT_POLLER_RETRY_DURATION, EVENT_POLLER_TIMEOUT_TRIES};
use crate::models::Event;
use crate::services::database::deployment::GetDeploymentElementByEventIdByParentNodeId;
use crate::services::database::Database;
use crate::{models::helpers::uuid::Uuid, services::database::event::GetEvent};
use actix::Addr;
use anyhow::{Ok, Result};
use chrono::Utc;
use log::info;
use sdl_parser::node::Role;
use std::time::Duration;
use tokio::time::sleep;

pub async fn await_conditions_to_be_deployed(
    database_address: &Addr<Database>,
    event_id: Uuid,
    event_conditions: &Vec<(String, Role)>,
    parent_node_id: &str,
) -> Result<()> {
    let parent_node_id = Uuid::try_from(parent_node_id)?;
    let mut deployment_elements = database_address
        .send(GetDeploymentElementByEventIdByParentNodeId(
            event_id,
            parent_node_id,
            true,
        ))
        .await??;
    let event = database_address.send(GetEvent(event_id)).await??;
    let mut timeout_counter = 0;

    while !deployment_elements.len().eq(&event_conditions.len()) {
        deployment_elements = database_address
            .send(GetDeploymentElementByEventIdByParentNodeId(
                event_id,
                parent_node_id,
                true,
            ))
            .await??;

        timeout_counter += 1;
        if timeout_counter >= EVENT_POLLER_TIMEOUT_TRIES {
            return Err(anyhow::anyhow!(
                "EventPoller Timeout while waiting for conditions to be deployed for event {:?}",
                event.name
            ));
        }
        sleep(EVENT_POLLER_RETRY_DURATION).await;
    }
    Ok(())
}

fn get_event_start_timedelta(event: &Event, current_time: chrono::NaiveDateTime) -> Duration {
    let timedelta = (event.start - current_time).num_seconds();
    if timedelta <= 0 {
        Duration::from_secs(0)
    } else {
        Duration::from_secs(timedelta as u64)
    }
}

pub async fn await_event_start(
    database_address: &Addr<Database>,
    event_id: Uuid,
    node_name: &str,
) -> Result<Event> {
    let event = database_address.send(GetEvent(event_id)).await??;
    let mut current_time = Utc::now().naive_utc();

    while current_time < event.start {
        let event_start_timedelta = get_event_start_timedelta(&event, current_time);
        if event_start_timedelta > Duration::from_secs(0) {
            info!(
                "Starting Polling for Event '{}' in {:?} on node '{}'",
                event.name, event_start_timedelta, node_name
            );
            sleep(event_start_timedelta).await;
            current_time = Utc::now().naive_utc();
        } else {
            break;
        }
    }
    Ok(event)
}