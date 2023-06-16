use crate::models::Event;
use crate::services::database::deployment::GetDeploymentElementByEventId;
use crate::services::database::Database;
use crate::{models::helpers::uuid::Uuid, services::database::event::GetEvent};
use actix::Addr;
use anyhow::{Ok, Result};

use chrono::Utc;
use log::info;
use sdl_parser::node::Role;
use tokio::time::sleep;

pub async fn await_conditions_to_be_deployed(
    database_address: &Addr<Database>,
    event_id: Uuid,
    event_conditions: &Vec<(String, Role)>,
) -> Result<()> {
    let mut deployment_elements = database_address
        .send(GetDeploymentElementByEventId(event_id, true))
        .await??;

    while !deployment_elements.len().eq(&event_conditions.len()) {
        deployment_elements = database_address
            .send(GetDeploymentElementByEventId(event_id, true))
            .await??;
        sleep(core::time::Duration::from_secs(3)).await;
    }
    Ok(())
}

pub async fn await_event_start(database_address: &Addr<Database>, event_id: Uuid) -> Result<Event> {
    let event = database_address.send(GetEvent(event_id)).await??;
    let mut current_time = Utc::now().naive_utc();

    while current_time < event.start {
        let relative_event_start_time = {
            let timeout = (event.start - current_time).num_seconds();
            if timeout <= 0 {
                std::time::Duration::from_secs(0)
            } else {
                std::time::Duration::from_secs(timeout as u64)
            }
        };
        if relative_event_start_time > std::time::Duration::from_secs(0) {
            info!(
                "Starting Polling for Event {:?} in {:?}",
                event.name, relative_event_start_time
            );
            sleep(relative_event_start_time).await;
            current_time = Utc::now().naive_utc();
        } else {
            break;
        }
    }
    Ok(event)
}
