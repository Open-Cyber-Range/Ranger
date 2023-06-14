use super::condition::ConditionProperties;
use super::Database;
use crate::constants::NAIVEDATETIME_DEFAULT_VALUE;
use crate::models::helpers::uuid::Uuid;
use crate::models::{DeploymentElement, ElementStatus, Exercise};
use crate::services::database::deployment::GetDeploymentElementByEventId;
use crate::services::database::event::{CreateEvent, GetEvent, UpdateEvent};
use crate::services::deployment::inject::DeployableInject;
use crate::utilities::scenario::get_injects_and_roles_by_node_event;
use crate::utilities::{scenario::get_conditions_by_event, try_some};
use crate::Addressor;
use actix::{Actor, Addr, AsyncContext, Context, Handler, Message, ResponseActFuture, WrapFuture};
use anyhow::{Ok, Result};
use async_trait::async_trait;
use chrono::{Duration, Utc};
use futures::future::try_join_all;
use log::{debug, info};
use sdl_parser::inject::Inject;
use sdl_parser::{node::Node, node::Role, Scenario};
use std::collections::HashMap;
use std::ops::Add;
use tokio::time::sleep;

#[async_trait]
pub trait DeployableEvents {
    async fn create_events(
        &self,
        addressor: &Addressor,
        deployed_nodes: &[(Node, DeploymentElement, Uuid)],
    ) -> Result<Vec<(Node, DeploymentElement, Uuid, Vec<ConditionProperties>)>>;

    async fn deploy_event_pollers(
        &self,
        addressor: &Addressor,
        exercise: &Exercise,
        deployers: &[String],
        deployed_nodes: &[(Node, DeploymentElement, Uuid, Vec<ConditionProperties>)],
    ) -> Result<()>;
}
#[async_trait]
impl DeployableEvents for Scenario {
    async fn create_events(
        &self,
        addressor: &Addressor,
        deployed_nodes: &[(Node, DeploymentElement, Uuid)],
    ) -> Result<Vec<(Node, DeploymentElement, Uuid, Vec<ConditionProperties>)>> {
        let events = self.events.clone().unwrap_or_default();
        let conditions = &self.conditions.clone().unwrap_or_default();
        let event_tranche = &events
            .into_iter()
            .map(|(event_name, sdl_event)| {
                let conditions = get_conditions_by_event(self, &sdl_event);
                (Uuid::random(), event_name, sdl_event, conditions)
            })
            .collect::<Vec<_>>();

        let output = try_join_all(deployed_nodes.iter().map(
            |(node, deployment_element, template_id)| async move {
                let mut node_event_conditions: Vec<ConditionProperties> = vec![];

                let node_roles = try_some(node.roles.clone(), "Node missing Roles")?;
                let node_condition_roles = node.conditions.clone().unwrap_or_default();
                let node_conditions = conditions
                    .iter()
                    .filter_map(
                        |(name, condition)| match node_condition_roles.contains_key(name) {
                            true => Some((name.to_owned(), condition.clone())),
                            false => None,
                        },
                    )
                    .collect::<HashMap<_, _>>();

                for (condition_name, condition) in node_conditions.clone().iter() {
                    let condition_role_name = try_some(
                        node_condition_roles.get(condition_name),
                        "Condition RoleName not found under Node Conditions",
                    )?;
                    let condition_role = try_some(
                        node_roles.get(condition_role_name),
                        "Condition Role not found under Node Roles",
                    )?;

                    if !event_tranche.is_empty() {
                        for (event_id, event_name, event, event_conditions) in event_tranche {
                            if node_condition_roles.contains_key(condition_name)
                                && event_conditions.contains_key(condition_name)
                            {
                                let scripts = self.scripts.clone().unwrap_or_default();
                                let potential_event_script = scripts
                                    .iter()
                                    .find(|(_, script)| script.events.contains(event_name));
                                let (_script_name, script) = try_some(
                                    potential_event_script,
                                    "Event not found among Scripts",
                                )?;

                                let mut event_start_seconds = script.start_time;

                                let event_end_in = Duration::seconds(script.end_time.try_into()?);
                                let scenario_start = self.start;
                                let scenario_end = self.end;

                                if let Some(relative_time_multiplier) = event.time {
                                    let script_duration = script.end_time - script.start_time;
                                    event_start_seconds =
                                        (script_duration as f64 * relative_time_multiplier as f64)
                                            .round() as u64;
                                }

                                let event_start_in = Duration::seconds(event_start_seconds as i64);
                                let event_start = scenario_start.add(event_start_in).naive_utc();

                                let event_end =
                                    match scenario_start.add(event_end_in) > scenario_end {
                                        true => scenario_end.naive_utc(),
                                        false => scenario_start.add(event_end_in).naive_utc(),
                                    };

                                debug!(
                                    "Event {:?} starts: {:?} - ends: {:?}",
                                    event_name, event_start, event_end
                                );

                                let injects = get_injects_and_roles_by_node_event(
                                    self,
                                    event,
                                    &deployment_element.scenario_reference,
                                );

                                let new_event = addressor
                                    .database
                                    .send(CreateEvent {
                                        event_id: *event_id,
                                        event_name: event_name.to_owned(),
                                        event: event.clone(),
                                        start: event_start,
                                        end: event_end,
                                        use_shared_connection: true,
                                    })
                                    .await??;

                                node_event_conditions.push(ConditionProperties {
                                    name: condition_name.to_owned(),
                                    condition: condition.clone(),
                                    role: condition_role.clone(),
                                    event_id: Some(new_event.id),
                                    injects: Some(injects),
                                });
                            }
                        }
                    } else {
                        node_event_conditions.push(ConditionProperties {
                            name: condition_name.to_owned(),
                            condition: condition.clone(),
                            role: condition_role.clone(),
                            event_id: None,
                            injects: None,
                        });
                    }
                }

                Ok((
                    node.clone(),
                    deployment_element.clone(),
                    *template_id,
                    node_event_conditions,
                ))
            },
        ))
        .await?;

        Ok(output)
    }

    async fn deploy_event_pollers(
        &self,
        addressor: &Addressor,
        exercise: &Exercise,
        deployers: &[String],
        deployed_nodes: &[(Node, DeploymentElement, Uuid, Vec<ConditionProperties>)],
    ) -> Result<()> {
        if self.scripts.is_some() && self.events.is_some() {
            try_join_all(deployed_nodes.iter().map(
                |(_, deployment_element, template_id, node_conditions)| async move {
                    let event_conditions: HashMap<Uuid, Vec<(String, Role)>> = node_conditions
                        .iter()
                        .fold(HashMap::new(), |mut event_conditions, properties| {
                            if let Some(event_id) = properties.event_id {
                                event_conditions
                                    .entry(event_id)
                                    .or_insert_with(Vec::new)
                                    .push((properties.name.to_owned(), properties.role.to_owned()));
                            }
                            event_conditions
                        });
                    let event_injects: HashMap<Uuid, Vec<(String, Inject, Role)>> = node_conditions
                        .iter()
                        .fold(HashMap::new(), |mut event_injects, properties| {
                            if let (Some(event_id), Some(injects)) =
                                (properties.event_id, properties.injects.clone())
                            {
                                event_injects.insert(event_id, injects.to_vec());
                            }
                            event_injects
                        });

                    let event_combo = event_conditions.into_iter().fold(
                        HashMap::new(),
                        |mut accumulator, (uuid, condition)| {
                            if let Some(injects) = event_injects.get(&uuid) {
                                accumulator.insert(uuid, (condition, injects.clone()));
                            }
                            accumulator
                        },
                    );

                    try_join_all(event_combo.iter().map(
                        |(event_id, (conditions, injects))| async move {
                            let event_succeeded = addressor
                                .event_poller
                                .send(StartPolling(
                                    addressor.database.clone(),
                                    *event_id,
                                    conditions.to_vec(),
                                ))
                                .await??;

                            info!(
                                "Event {:?} ended. Result: {:?}",
                                event_id,
                                match event_succeeded {
                                    true => "Success",
                                    false => "Failure",
                                }
                            );

                            if event_succeeded {
                                for (inject_name, inject, inject_role) in injects {
                                    (
                                        addressor,
                                        deployers.to_vec(),
                                        deployment_element.clone(),
                                        exercise.id,
                                        inject_role.username.clone(),
                                        *template_id,
                                        (inject_name.to_owned(), inject.clone()),
                                    )
                                        .deploy_inject()
                                        .await?;
                                }
                            }
                            Ok(())
                        },
                    ))
                    .await?;

                    Ok(())
                },
            ))
            .await?;
        }
        Ok(())
    }
}

#[derive(Message, Clone)]
#[rtype(result = "Result<bool>")]
pub struct StartPolling(Addr<Database>, Uuid, Vec<(String, Role)>);

pub struct EventPoller();

impl Actor for EventPoller {
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Context<Self>) {
        println!("EventPoller is alive");
    }

    fn stopped(&mut self, _ctx: &mut Context<Self>) {
        println!("EventPoller is stopped");
    }
}

impl EventPoller {
    pub fn new() -> Self {
        Self()
    }
}

impl Default for EventPoller {
    fn default() -> Self {
        Self::new()
    }
}

impl Handler<StartPolling> for EventPoller {
    type Result = ResponseActFuture<Self, Result<bool>>;

    fn handle(&mut self, msg: StartPolling, ctx: &mut Context<Self>) -> Self::Result {
        let _address = ctx.address();
        let StartPolling(database_address, event_id, event_conditions) = msg;

        Box::pin(
            async move {
                let mut updated_event = crate::models::UpdateEvent {
                    has_triggered: false,
                    triggered_at: *NAIVEDATETIME_DEFAULT_VALUE,
                };
                let has_succeeded: bool;

                loop {
                    let mut deployment_elements = database_address
                        .send(GetDeploymentElementByEventId(event_id, true))
                        .await??;

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

                    while !deployment_elements.len().eq(&event_conditions.len()) {
                        deployment_elements = database_address
                            .send(GetDeploymentElementByEventId(event_id, true))
                            .await??;
                    }

                    let successful_conditions = deployment_elements
                        .iter()
                        .filter(|condition| matches!(condition.status, ElementStatus::Success))
                        .count();

                    if deployment_elements.len().eq(&successful_conditions) {
                        info!(
                            "Event {:?} has been triggered - deploying injects",
                            event.name
                        );
                        updated_event.has_triggered = true;
                        updated_event.triggered_at = Utc::now().naive_utc();
                        has_succeeded = true;
                        break;
                    } else if current_time > event.end {
                        debug!("Event: {:?} deployment window has ended", event.name);
                        has_succeeded = false;

                        break;
                    }

                    sleep(core::time::Duration::from_secs(3)).await;
                }

                database_address
                    .send(UpdateEvent(event_id, updated_event))
                    .await??;

                Ok(has_succeeded)
            }
            .into_actor(self),
        )
    }
}
