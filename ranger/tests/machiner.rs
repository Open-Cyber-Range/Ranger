mod common;

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::common::create_mock_vmware_server;
    use actix::Addr;
    use actix_web::test;
    use anyhow::{anyhow, Result};
    use futures::future::join_all;
    use ranger::deployers::DeployerGroup;
    use ranger::services::deployment::NodeDeploymentTrait;
    use ranger::templater::{
        filter_templation_results, initiate_template_clients, TemplateClient, Templation,
    };
    use ranger_grpc::NodeDeployment;
    use sdl_parser::test::TEST_SCHEMA;

    #[test]
    async fn verify_vm_node_deployment_struct() -> Result<()> {
        let mut nodes = TEST_SCHEMA
            .scenario
            .nodes
            .clone()
            .ok_or_else(|| anyhow!("TEST: Nodes list is missing"))?;
        let node = nodes
            .remove_entry("win10")
            .ok_or_else(|| anyhow!("TEST: Node not found"))?;
        let deployment = NodeDeployment::default().initialize_vm(
            node.1,
            node.0,
            "template-id".to_string(),
            "test-exercise",
        )?;
        insta::assert_debug_snapshot!(deployment);
        Ok(())
    }

    #[actix_web::test]
    pub async fn mock_deploy_vms() -> Result<()> {
        let scenario = TEST_SCHEMA.scenario.clone();
        let nodes = scenario
            .clone()
            .nodes
            .ok_or_else(|| anyhow!("TEST: Nodes list is missing"))?;
        let exercise_name = "test-exercise";
        let templater_address = format!(
            "http://{}",
            create_mock_vmware_server().run_template_server()?
        );
        let templater = HashMap::from_iter(vec![(
            "test-templater".to_string(),
            templater_address.clone(),
        )]);
        let templater_actor_results = join_all(initiate_template_clients(templater)).await;
        let templater_actor: HashMap<String, Addr<TemplateClient>> =
            [templater_actor_results.into_iter().next().unwrap().unwrap()]
                .iter()
                .cloned()
                .collect();

        let templation_results = scenario.template_nodes(&templater_actor).await?;
        let template_ids = filter_templation_results(templation_results);
        let mock_node_server = create_mock_vmware_server().run_node_server()?;
        let mut deployer_group = DeployerGroup::default();
        deployer_group.machiners.insert(
            "mock-server".to_string(),
            format!("http://{}", mock_node_server),
        );
        deployer_group.templaters.insert(
            "mock-templater".to_string(),
            format!("http://{}", templater_address),
        );
        let deployment_group = deployer_group.start().await;
        let node_deployments =
            NodeDeployment::default().create_from_nodes(nodes, template_ids, exercise_name)?;
        let mut node_identifiers = deployment_group.deploy_vms(node_deployments).await?;
        node_identifiers.sort_by_key(|node_name| node_name.1.clone());
        insta::assert_debug_snapshot!(node_identifiers);
        Ok(())
    }
}
