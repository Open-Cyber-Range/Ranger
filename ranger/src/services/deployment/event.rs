use super::condition::ConditionProperties;
use super::Database;
use crate::constants::{EVENT_POLLER_RETRY_DURATION, NAIVEDATETIME_DEFAULT_VALUE};
use crate::models::helpers::uuid::Uuid;
use crate::models::{DeploymentElement, ElementStatus, Exercise};
use crate::services::database::deployment::GetDeploymentElementByEventIdByParentNodeId;
use crate::services::database::event::{CreateEvent, UpdateEvent};
use crate::services::deployment::inject::DeployableInject;
use crate::utilities::event::{
    await_conditions_to_be_deployed, await_event_start, calculate_event_start_end_times,
};
use crate::utilities::scenario::get_injects_and_roles_by_node_event;
use crate::utilities::{scenario::get_conditions_by_event, try_some};
use crate::Addressor;
use actix::{Actor, Addr, AsyncContext, Context, Handler, Message, ResponseActFuture, WrapFuture};
use anyhow::{Ok, Result};
use async_trait::async_trait;
use chrono::Utc;
use futures::future::try_join_all;
use log::info;
use sdl_parser::inject::Inject;
use sdl_parser::{node::Node, node::Role, Scenario};
use std::collections::HashMap;
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
        let sdl_events = self.events.clone().unwrap_or_default();
        let conditions = &self.conditions.clone().unwrap_or_default();
        let referenced_event_keys = self
            .scripts
            .clone()
            .unwrap_or_default()
            .iter()
            .flat_map(|(_, script)| script.events.clone())
            .collect::<Vec<_>>();

        let event_conditions_tuple = &sdl_events
            .into_iter()
            .filter_map(
                |(event_key, sdl_event)| match referenced_event_keys.contains(&event_key) {
                    true => {
                        let conditions = get_conditions_by_event(self, &sdl_event);
                        Some((event_key, sdl_event, conditions))
                    }
                    false => None,
                },
            )
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

                    if !event_conditions_tuple.is_empty() {
                        for (event_key, event, event_conditions) in event_conditions_tuple {
                            if node_condition_roles.contains_key(condition_name)
                                && event_conditions.contains_key(condition_name)
                            {
                                let (event_start, event_end) =
                                    calculate_event_start_end_times(self, event_key, event)?;
                                let injects = get_injects_and_roles_by_node_event(
                                    self,
                                    event,
                                    &deployment_element.scenario_reference,
                                );
                                let parent_node_id_string = try_some(
                                    deployment_element.handler_reference.clone(),
                                    "DeploymentElement missing HandlerReference",
                                )?;
                                let event_id = match node_event_conditions.iter().find(
                                    |condition_properties| {
                                        condition_properties.event_name == Some(event_key.clone())
                                    },
                                ) {
                                    Some(event) => try_some(event.event_id, "Event missing Id")?,
                                    None => Uuid::random(),
                                };

                                let new_event = addressor
                                    .database
                                    .send(CreateEvent {
                                        event_id,
                                        event_name: event_key.to_owned(),
                                        event: event.clone(),
                                        deployment_id: deployment_element.deployment_id,
                                        description: event.description.clone(),
                                        parent_node_id: Uuid::try_from(
                                            parent_node_id_string.as_str(),
                                        )?,
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
                                    event_name: Some(event_key.to_owned()),
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
                            event_name: None,
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
                                    .or_default()
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
                                    deployment_element.clone(),
                                ))
                                .await??;

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
pub struct StartPolling(Addr<Database>, Uuid, Vec<(String, Role)>, DeploymentElement);

pub struct EventPoller();

impl Actor for EventPoller {
    type Context = Context<Self>;
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
        let StartPolling(database_address, event_id, event_conditions, node_deployment_element) =
            msg;

        Box::pin(
            async move {
                let mut updated_event = crate::models::UpdateEvent {
                    has_triggered: false,
                    triggered_at: *NAIVEDATETIME_DEFAULT_VALUE,
                };
                let has_succeeded: bool;
                let node_name = node_deployment_element.scenario_reference.clone();
                let vm_handler_reference = try_some(
                    node_deployment_element.handler_reference,
                    "VM Node missing handler reference",
                )?;
                let event = await_event_start(&database_address, event_id, &node_name).await?;

                await_conditions_to_be_deployed(
                    &database_address,
                    event_id,
                    &event_conditions,
                    &vm_handler_reference,
                )
                .await?;

                info!(
                    "Starting Polling for Event '{}' for node '{}'",
                    event.name, &node_deployment_element.scenario_reference
                );
                let parent_node_id = Uuid::try_from(vm_handler_reference.as_str())?;
                loop {
                    let current_time = Utc::now().naive_utc();
                    let condition_deployment_elements = database_address
                        .send(GetDeploymentElementByEventIdByParentNodeId(
                            event_id,
                            parent_node_id,
                            true,
                        ))
                        .await??;

                    let successful_condition_count = condition_deployment_elements
                        .iter()
                        .filter(|condition| matches!(condition.status, ElementStatus::Success))
                        .count();

                    if condition_deployment_elements
                        .len()
                        .eq(&successful_condition_count)
                    {
                        info!(
                            "Event '{}' has been triggered successfully for node '{}'",
                            event.name, node_name
                        );
                        updated_event.has_triggered = true;
                        updated_event.triggered_at = Utc::now().naive_utc();
                        has_succeeded = true;
                        break;
                    } else if current_time > event.end {
                        info!(
                            "Event '{}' deployment window has ended for node '{}'",
                            event.name, node_name
                        );
                        has_succeeded = false;
                        break;
                    }

                    sleep(EVENT_POLLER_RETRY_DURATION).await;
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
