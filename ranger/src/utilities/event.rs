use crate::constants::{EVENT_POLLER_RETRY_DURATION, EVENT_POLLER_TIMEOUT_TRIES};
use crate::models::Event;
use crate::services::database::deployment::GetDeploymentElementByEventIdByParentNodeId;
use crate::services::database::Database;
use crate::{models::helpers::uuid::Uuid, services::database::event::GetEvent};
use actix::Addr;
use anyhow::{anyhow, Ok, Result};
use chrono::{NaiveDateTime, Utc};
use log::debug;
use sdl_parser::{event::Event as SdlEvent, node::Role, Scenario};
use std::ops::Add;
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

fn get_event_start_timedelta(event: &Event, current_time: NaiveDateTime) -> Duration {
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
            debug!(
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

pub fn calculate_event_start_end_times(
    scenario: &Scenario,
    event_key: &str,
    event: &SdlEvent,
    deployment_start: NaiveDateTime,
    deployment_end: NaiveDateTime,
) -> Result<(NaiveDateTime, NaiveDateTime)> {
    let (parent_script_key, parent_script) = scenario
        .scripts
        .as_ref()
        .and_then(|scripts| {
            scripts
                .iter()
                .find(|(_, script)| script.events.contains(&event_key.to_owned()))
        })
        .map(|(script_key, script)| (script_key, script))
        .ok_or_else(|| anyhow!("Failed to find parent script for {event_key}"))?;

    let parent_story = scenario
        .stories
        .as_ref()
        .and_then(|stories| {
            stories
                .iter()
                .find(|(_, story)| story.scripts.contains(&parent_script_key.to_owned()))
        })
        .map(|(_, story)| story)
        .ok_or_else(|| anyhow!("Failed to find parent story for {parent_script_key}"))?;

    let story_and_script_multiplier = parent_story.speed * parent_script.speed as f64;
    let mut adjusted_start_time = parent_script.start_time as f64 / story_and_script_multiplier;
    let adjusted_end_time = parent_script.end_time as f64 / story_and_script_multiplier;

    if let Some(event_relative_multiplier) = event.time {
        adjusted_start_time = (adjusted_start_time
            + (adjusted_end_time - adjusted_start_time) * event_relative_multiplier as f64)
            .round();
    }

    let event_start_duration = chrono::Duration::seconds(adjusted_start_time as i64);
    let event_start_datetime = deployment_start.add(event_start_duration);
    let event_end_duration = chrono::Duration::seconds(adjusted_end_time as i64);
    let event_end_datetime = match deployment_start.add(event_end_duration) > deployment_end {
        true => deployment_end,
        false => deployment_start.add(event_end_duration),
    };

    Ok((event_start_datetime, event_end_datetime))
}
