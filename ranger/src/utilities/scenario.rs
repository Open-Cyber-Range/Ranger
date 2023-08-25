use sdl_parser::{
    capability::Capabilities,
    condition::Conditions,
    entity::{Entities, Entity, ExerciseRole, Flatten},
    evaluation::Evaluations,
    event::{Event, Events},
    feature::Features,
    goal::Goals,
    infrastructure::Infrastructure,
    inject::{Inject, Injects},
    metric::{Metric, Metrics},
    node::{Nodes, Role},
    script::Scripts,
    story::Stories,
    training_learning_objective::TrainingLearningObjectives,
    vulnerability::Vulnerabilities,
    Scenario,
};
use std::collections::HashMap;

pub fn set_optionals_to_none(scenario: &Scenario) -> Scenario {
    let mut participant_scenario = scenario.clone();
    participant_scenario.capabilities = None;
    participant_scenario.conditions = None;
    participant_scenario.entities = None;
    participant_scenario.evaluations = None;
    participant_scenario.events = None;
    participant_scenario.features = None;
    participant_scenario.goals = None;
    participant_scenario.infrastructure = None;
    participant_scenario.injects = None;
    participant_scenario.metrics = None;
    participant_scenario.nodes = None;
    participant_scenario.scripts = None;
    participant_scenario.stories = None;
    participant_scenario.tlos = None;
    participant_scenario.vulnerabilities = None;

    participant_scenario
}

pub fn get_flattened_entities_by_role(
    scenario: &Scenario,
    role: ExerciseRole,
) -> HashMap<String, Entity> {
    if let Some(scenario_entities) = scenario.entities.clone() {
        let flattened_entities = scenario_entities.flatten();
        filter_entities_by_role(flattened_entities, role)
    } else {
        HashMap::new()
    }
}

pub fn get_entities_by_role(scenario: &Scenario, role: ExerciseRole) -> HashMap<String, Entity> {
    if let Some(scenario_entities) = scenario.entities.clone() {
        filter_entities_by_role(scenario_entities, role)
    } else {
        HashMap::new()
    }
}

pub fn filter_entities_by_role(entities: Entities, role: ExerciseRole) -> HashMap<String, Entity> {
    entities
        .into_iter()
        .filter_map(|(entity_name, entity)| {
            if let Some(entity_role) = &entity.role {
                if entity_role.eq(&role) {
                    return Some((entity_name, entity));
                };
            }
            None
        })
        .collect::<HashMap<String, Entity>>()
}

pub fn get_tlos_by_entities(
    scenario: &Scenario,
    entities: &Entities,
) -> TrainingLearningObjectives {
    let tlo_names = entities.values().fold(vec![], |mut accumulator, entity| {
        if let Some(tlo_names) = entity.tlos.clone() {
            accumulator.extend(tlo_names);
        }
        accumulator
    });

    tlo_names
        .into_iter()
        .fold(HashMap::new(), |mut tlos, tlo_name| {
            if let Some(scenario_tlos) = scenario.tlos.clone() {
                if scenario_tlos.contains_key(&tlo_name) {
                    tlos.insert(tlo_name.to_owned(), scenario_tlos[&tlo_name].clone());
                }
            }
            tlos
        })
}

pub fn get_entity_vulnerabilities(scenario: &Scenario, entities: &Entities) -> Vulnerabilities {
    let vulnerability_names = entities.values().fold(vec![], |mut accumulator, entity| {
        if let Some(vulnerability_names) = entity.vulnerabilities.clone() {
            accumulator.extend(vulnerability_names);
        }
        accumulator
    });

    vulnerability_names.into_iter().fold(
        HashMap::new(),
        |mut vulnerabilities, vulnerability_name| {
            if let Some(scenario_vulnerabilities) = scenario.vulnerabilities.clone() {
                if scenario_vulnerabilities.contains_key(&vulnerability_name) {
                    vulnerabilities.insert(
                        vulnerability_name.to_owned(),
                        scenario_vulnerabilities[&vulnerability_name].clone(),
                    );
                }
                return vulnerabilities;
            }
            vulnerabilities
        },
    )
}

pub fn get_vulnerability_connections(
    scenario: &Scenario,
    vulnerabilities: &Vulnerabilities,
) -> (Capabilities, Features, Nodes) {
    let mut capabilities = HashMap::new();
    let mut features = HashMap::new();
    let mut nodes = HashMap::new();

    vulnerabilities.keys().for_each(|vulnerability_name| {
        if let Some(scenario_capabilities) = scenario.capabilities.clone() {
            scenario_capabilities
                .into_iter()
                .for_each(|(key, capability)| {
                    if let Some(vulnerabilities) = capability.vulnerabilities.clone() {
                        if vulnerabilities.contains(vulnerability_name) {
                            capabilities.insert(key, capability);
                        }
                    }
                })
        }
        if let Some(scenario_features) = scenario.features.clone() {
            scenario_features.into_iter().for_each(|(key, feature)| {
                if let Some(vulnerabilities) = feature.vulnerabilities.clone() {
                    if vulnerabilities.contains(vulnerability_name) {
                        features.insert(key, feature);
                    }
                }
            })
        }
        if let Some(scenario_nodes) = scenario.nodes.clone() {
            scenario_nodes.into_iter().for_each(|(key, node)| {
                if let Some(vulnerabilities) = node.vulnerabilities.clone() {
                    if vulnerabilities.contains(vulnerability_name) {
                        nodes.insert(key, node);
                    }
                }
            })
        }
    });

    (capabilities, features, nodes)
}

pub fn get_goals_by_tlos(scenario: &Scenario, tlos: &TrainingLearningObjectives) -> Goals {
    let mut goals = HashMap::new();

    if let Some(scenario_goals) = &scenario.goals {
        scenario_goals.iter().for_each(|(key, goal)| {
            goal.tlos.iter().for_each(|goal_tlo_name| {
                if tlos.contains_key(goal_tlo_name) {
                    goals.insert(key.to_owned(), goal.clone());
                }
            })
        })
    }

    goals
}

pub fn get_tlo_connections(
    scenario: &Scenario,
    tlos: &TrainingLearningObjectives,
) -> (Injects, Evaluations) {
    let mut injects = HashMap::new();
    let mut evaluations = HashMap::new();

    tlos.iter().for_each(|(tlo_name, tlo)| {
        if let Some(scenario_injects) = scenario.injects.clone() {
            scenario_injects.into_iter().for_each(|(key, inject)| {
                if let Some(tlos) = inject.tlos.clone() {
                    if tlos.contains(tlo_name) {
                        injects.insert(key, inject);
                    }
                }
            })
        }

        let evaluation_name = tlo.evaluation.clone();
        if let Some(scenario_evaluations) = scenario.evaluations.clone() {
            if scenario_evaluations.contains_key(&evaluation_name) {
                evaluations.insert(
                    evaluation_name.to_owned(),
                    scenario_evaluations[&evaluation_name].clone(),
                );
            }
        }
    });

    (injects, evaluations)
}

pub fn get_conditions_by_capabilities(
    scenario: &Scenario,
    capabilities: &Capabilities,
) -> Conditions {
    capabilities
        .iter()
        .fold(HashMap::new(), |mut conditions, capability| {
            let condition_name = capability.1.condition.clone();
            if let Some(scenario_conditions) = scenario.conditions.clone() {
                if scenario_conditions.contains_key(&condition_name) {
                    conditions.insert(
                        condition_name.to_owned(),
                        scenario_conditions[&condition_name].clone(),
                    );
                }
            }
            conditions
        })
}

pub fn get_metrics_by_evaluations(scenario: &Scenario, evaluations: &Evaluations) -> Metrics {
    let metrics = evaluations
        .iter()
        .fold(HashMap::new(), |mut accumulator, evaluation| {
            evaluation.1.metrics.iter().for_each(|metric_name| {
                if let Some(scenario_metrics) = scenario.metrics.clone() {
                    if scenario_metrics.contains_key(metric_name) {
                        accumulator.insert(
                            metric_name.to_owned(),
                            scenario_metrics[metric_name].clone(),
                        );
                    }
                }
            });
            accumulator
        });

    metrics
}

pub fn get_conditions_by_metrics(scenario: &Scenario, metrics: &Metrics) -> Conditions {
    let conditions = metrics
        .iter()
        .fold(HashMap::new(), |mut accumulator, metric| {
            if let Some(metric_condition_name) = &metric.1.condition {
                if let Some(scenario_conditions) = scenario.conditions.clone() {
                    if scenario_conditions.contains_key(metric_condition_name) {
                        accumulator.insert(
                            metric_condition_name.to_owned(),
                            scenario_conditions[metric_condition_name].clone(),
                        );
                    }
                }
            }

            accumulator
        });

    conditions
}

pub fn get_inject_capabilities(
    scenario_capabilities: &Capabilities,
    inject: &Inject,
) -> Capabilities {
    let mut required_inject_capabilities = vec![];
    required_inject_capabilities.push(inject.capabilities.executive.clone());
    if let Some(secondary_capabilities) = inject.capabilities.secondary.clone() {
        required_inject_capabilities.extend(secondary_capabilities);
    }

    let output = required_inject_capabilities.iter().fold(
        HashMap::new(),
        |mut capabilities, capability_name| {
            if scenario_capabilities.contains_key(capability_name) {
                capabilities.insert(
                    capability_name.to_owned(),
                    scenario_capabilities[capability_name].clone(),
                );
            }
            capabilities
        },
    );
    output
}

pub fn get_inject_connections(scenario: &Scenario, injects: &Injects) -> (Events, Capabilities) {
    let mut events = HashMap::new();
    let mut capabilities = HashMap::new();

    injects.iter().for_each(|(inject_name, inject)| {
        if let Some(scenario_events) = scenario.events.clone() {
            scenario_events.into_iter().for_each(|(key, event)| {
                if event.injects.contains(inject_name) {
                    events.insert(key, event);
                }
            })
        }

        if let Some(scenario_capabilities) = scenario.capabilities.clone() {
            let inject_capabilities = get_inject_capabilities(&scenario_capabilities, inject);
            capabilities.extend(inject_capabilities);
        }
    });
    (events, capabilities)
}

pub fn get_nodes_by_conditions(scenario: &Scenario, conditions: &Conditions) -> Nodes {
    let nodes = conditions
        .keys()
        .fold(HashMap::new(), |mut accumulator, condition_name| {
            if let Some(scenario_nodes) = &scenario.nodes {
                scenario_nodes.iter().for_each(|(node_name, node)| {
                    if let Some(node_conditions) = &node.conditions {
                        if node_conditions.contains_key(condition_name) {
                            accumulator.insert(node_name.to_owned(), node.clone());
                        }
                    }
                });
            }
            accumulator
        });
    nodes
}

pub fn get_node_connections(
    scenario: &Scenario,
    nodes: &Nodes,
) -> (Features, Conditions, Vulnerabilities, Infrastructure) {
    let mut features = HashMap::new();
    let mut conditions = HashMap::new();
    let mut vulnerabilities = HashMap::new();
    let mut infrastructure = HashMap::new();

    nodes.iter().for_each(|(node_name, node)| {
        if let Some(node_features) = node.features.clone() {
            node_features.keys().for_each(|key| {
                if let Some(scenario_features) = scenario.features.clone() {
                    if scenario_features.contains_key(key) {
                        features.insert(key.to_owned(), scenario_features[key].clone());
                    }
                }
            })
        }
        if let Some(node_conditions) = node.conditions.clone() {
            node_conditions.keys().for_each(|key| {
                if let Some(scenario_conditions) = scenario.conditions.clone() {
                    if scenario_conditions.contains_key(key) {
                        conditions.insert(key.to_owned(), scenario_conditions[key].clone());
                    }
                }
            })
        }

        if let Some(node_vulnerabilities) = node.vulnerabilities.clone() {
            node_vulnerabilities.iter().for_each(|key| {
                if let Some(scenario_vulnerabilities) = scenario.vulnerabilities.clone() {
                    if scenario_vulnerabilities.contains_key(key) {
                        vulnerabilities
                            .insert(key.to_owned(), scenario_vulnerabilities[key].clone());
                    }
                }
            })
        }

        if let Some(scenario_infrastructure) = scenario.infrastructure.clone() {
            scenario_infrastructure
                .iter()
                .for_each(|(key, infra_node)| {
                    if key.eq(node_name) {
                        infrastructure.insert(key.to_owned(), infra_node.clone());
                    }
                })
        }
    });

    (features, conditions, vulnerabilities, infrastructure)
}

pub fn get_condition_connections(
    scenario: &Scenario,
    conditions: &Conditions,
) -> (Events, Capabilities) {
    let mut events = HashMap::new();
    let mut capabilities = HashMap::new();

    conditions.keys().for_each(|condition_name| {
        if let Some(scenario_events) = scenario.events.clone() {
            scenario_events.into_iter().for_each(|(key, event)| {
                if let Some(conditions) = event.conditions.clone() {
                    if conditions.contains(condition_name) {
                        events.insert(key, event);
                    }
                }
            })
        }

        if let Some(scenario_capabilities) = scenario.capabilities.clone() {
            scenario_capabilities
                .into_iter()
                .for_each(|(key, capability)| {
                    if capability.condition.eq(condition_name) {
                        capabilities.insert(key, capability);
                    }
                })
        }
    });

    (events, capabilities)
}

pub fn get_event_connections(scenario: &Scenario, events: &Events) -> (Injects, Scripts) {
    let mut injects = HashMap::new();
    let mut scripts = HashMap::new();

    events.iter().for_each(|(event_name, event)| {
        event.injects.iter().for_each(|inject_name| {
            if let Some(scenario_injects) = scenario.injects.clone() {
                if scenario_injects.contains_key(inject_name) {
                    injects.insert(
                        inject_name.to_owned(),
                        scenario_injects[inject_name].clone(),
                    );
                }
            }
        });
        if let Some(scenario_scripts) = scenario.scripts.clone() {
            scenario_scripts.iter().for_each(|(key, script)| {
                if script.events.contains(event_name) {
                    scripts.insert(key.to_owned(), script.clone());
                }
            })
        }
    });

    (injects, scripts)
}

pub fn get_vulnerabilities_by_capabilities(
    scenario: &Scenario,
    capabilities: &Capabilities,
) -> Vulnerabilities {
    capabilities
        .values()
        .fold(HashMap::new(), |mut accumulator, capability| {
            if let Some(capability_vulnerabilities) = capability.vulnerabilities.clone() {
                if let Some(scenario_vulnerabilities) = scenario.vulnerabilities.clone() {
                    capability_vulnerabilities
                        .iter()
                        .for_each(|vulnerability_name| {
                            if scenario_vulnerabilities.contains_key(vulnerability_name) {
                                accumulator.insert(
                                    vulnerability_name.to_owned(),
                                    scenario_vulnerabilities[vulnerability_name].clone(),
                                );
                            }
                        });
                }
            }
            accumulator
        })
}

pub fn get_stories_by_scripts(scenario: &Scenario, scripts: &Scripts) -> Stories {
    scripts
        .keys()
        .fold(HashMap::new(), |mut stories, script_name| {
            if let Some(scenario_stories) = scenario.stories.clone() {
                scenario_stories.iter().for_each(|(key, story)| {
                    if story.scripts.contains(script_name) {
                        stories.insert(key.to_owned(), story.clone());
                    }
                })
            }
            stories
        })
}

pub fn get_injects_and_roles_by_node_event(
    scenario: &Scenario,
    event: &Event,
    node_name: &str,
) -> Vec<(String, Inject, Role)> {
    let event_injects = event
        .injects
        .iter()
        .fold(HashMap::new(), |mut injects, inject_name| {
            if let Some(scenario_injects) = scenario.injects.clone() {
                if scenario_injects.contains_key(inject_name) {
                    injects.insert(
                        inject_name.to_owned(),
                        scenario_injects[inject_name].clone(),
                    );
                }
            }
            injects
        });

    let injects_with_roles: Vec<(String, Inject, Role)> =
        event_injects
            .iter()
            .fold(vec![], |mut accumulator, (inject_name, inject)| {
                let inject_executive_capability = inject.capabilities.executive.clone();
                let scenario_capabilities = scenario.capabilities.clone().unwrap_or_default();
                if let Some(executive_capability) =
                    scenario_capabilities.get(&inject_executive_capability)
                {
                    if let Some(node) = scenario.nodes.clone().unwrap_or_default().get(node_name) {
                        if let Some(condition_role) = node
                            .conditions
                            .clone()
                            .unwrap_or_default()
                            .get(&executive_capability.condition)
                        {
                            if let Some(role) =
                                node.roles.clone().unwrap_or_default().get(condition_role)
                            {
                                accumulator.push((
                                    inject_name.to_owned(),
                                    inject.clone(),
                                    role.clone(),
                                ));
                            }
                        };
                    };
                }
                accumulator
            });
    injects_with_roles
}

pub fn get_conditions_by_event(scenario: &Scenario, event: &Event) -> Conditions {
    let event_conditions =
        event
            .conditions
            .iter()
            .fold(HashMap::new(), |mut conditions, condition_names| {
                if let Some(scenario_conditions) = scenario.conditions.clone() {
                    condition_names.iter().for_each(|condition_name| {
                        if scenario_conditions.contains_key(condition_name) {
                            conditions.insert(
                                condition_name.to_owned(),
                                scenario_conditions[condition_name].clone(),
                            );
                        }
                    })
                }
                conditions
            });
    let event_inject_capabilities =
        event
            .injects
            .iter()
            .fold(HashMap::new(), |mut capabilities, inject_name| {
                if let (Some(scenario_injects), Some(scenario_capabilities)) =
                    (scenario.injects.clone(), scenario.capabilities.clone())
                {
                    capabilities.extend(get_inject_capabilities(
                        &scenario_capabilities,
                        &scenario_injects[inject_name],
                    ));
                }
                capabilities
            });
    let capability_conditions =
        event_inject_capabilities
            .values()
            .fold(HashMap::new(), |mut conditions, capability| {
                if let Some(scenario_conditions) = scenario.conditions.clone() {
                    if scenario_conditions.contains_key(&capability.condition) {
                        conditions.insert(
                            capability.condition.to_owned(),
                            scenario_conditions[&capability.condition].clone(),
                        );
                    }
                }
                conditions
            });

    event_conditions
        .into_iter()
        .chain(capability_conditions)
        .collect()
}

pub fn filter_scenario_by_role(scenario: &Scenario, role: ExerciseRole) -> Scenario {
    let mut participant_scenario = set_optionals_to_none(scenario);
    let flattened_entities = get_flattened_entities_by_role(scenario, role.clone());

    if flattened_entities.is_empty() {
        return participant_scenario;
    }

    let mut vulnerabilities = get_entity_vulnerabilities(scenario, &flattened_entities);
    let (mut capabilities, mut features, mut nodes) =
        get_vulnerability_connections(scenario, &vulnerabilities);
    let mut conditions = get_conditions_by_capabilities(scenario, &capabilities);
    let tlos = get_tlos_by_entities(scenario, &flattened_entities);
    let goals = get_goals_by_tlos(scenario, &tlos);
    let (mut injects, evaluations) = get_tlo_connections(scenario, &tlos);
    let metrics = get_metrics_by_evaluations(scenario, &evaluations);
    let metric_conditions = get_conditions_by_metrics(scenario, &metrics);
    conditions.extend(metric_conditions);

    let (mut events, inject_capabilities) = get_inject_connections(scenario, &injects);
    capabilities.extend(inject_capabilities);

    let condition_nodes = get_nodes_by_conditions(scenario, &conditions);
    nodes.extend(condition_nodes);

    let (node_features, node_conditions, node_vulnerabilities, infrastructure) =
        get_node_connections(scenario, &nodes);
    features.extend(node_features);
    conditions.extend(node_conditions);
    vulnerabilities.extend(node_vulnerabilities);

    let (condition_events, condition_capabilities) =
        get_condition_connections(scenario, &conditions);
    events.extend(condition_events);
    capabilities.extend(condition_capabilities);

    let (event_injects, scripts) = get_event_connections(scenario, &events);
    injects.extend(event_injects);

    let stories = get_stories_by_scripts(scenario, &scripts);
    let capability_vulnerabilities = get_vulnerabilities_by_capabilities(scenario, &capabilities);
    vulnerabilities.extend(capability_vulnerabilities);

    let entities = get_entities_by_role(scenario, role);

    participant_scenario.capabilities = (!capabilities.is_empty()).then_some(capabilities);
    participant_scenario.conditions = (!conditions.is_empty()).then_some(conditions);
    participant_scenario.entities = (!entities.is_empty()).then_some(entities);
    participant_scenario.evaluations = (!evaluations.is_empty()).then_some(evaluations);
    participant_scenario.events = (!events.is_empty()).then_some(events);
    participant_scenario.features = (!features.is_empty()).then_some(features);
    participant_scenario.goals = (!goals.is_empty()).then_some(goals);
    participant_scenario.infrastructure = (!infrastructure.is_empty()).then_some(infrastructure);
    participant_scenario.injects = (!injects.is_empty()).then_some(injects);
    participant_scenario.metrics = (!metrics.is_empty()).then_some(metrics);
    participant_scenario.nodes = (!nodes.is_empty()).then_some(nodes);
    participant_scenario.scripts = (!scripts.is_empty()).then_some(scripts);
    participant_scenario.stories = (!stories.is_empty()).then_some(stories);
    participant_scenario.tlos = (!tlos.is_empty()).then_some(tlos);
    participant_scenario.vulnerabilities = (!vulnerabilities.is_empty()).then_some(vulnerabilities);

    participant_scenario
}

pub fn filter_node_roles_by_entity(
    node_roles: HashMap<String, Role>,
    entity_selector: &str,
) -> HashMap<String, Role> {
    node_roles.into_iter().fold(
        HashMap::new(),
        |mut role_accumulator, (role_name, mut role)| {
            let filtered_role_entities = match role.entities {
                Some(mut entities) => {
                    entities.retain(|entity| entity.eq(&entity_selector));
                    (!entities.is_empty()).then_some(entities)
                }
                None => None,
            };

            if filtered_role_entities.is_some() {
                role.entities = filtered_role_entities;
                role_accumulator.insert(role_name, role);
            }
            role_accumulator
        },
    )
}

pub fn get_metric_by_condition(
    metrics: &Option<Metrics>,
    condition_name: &str,
) -> Option<(String, Metric)> {
    if let Some(metrics) = metrics {
        match metrics
            .iter()
            .find(|(_, metric)| metric.condition.eq(&Some(condition_name.to_string())))
        {
            Some((metric_key, metric)) => return Some((metric_key.to_string(), metric.clone())),
            None => return None,
        }
    }
    None
}
