mod common;

#[cfg(test)]
mod tests {
    use crate::common::create_mock_vmware_server;
    use actix_web::test;
    use anyhow::{anyhow, Result};
    use ranger::deployers::DeployerGroup;
    use ranger::machiner::NodeDeploymentTrait;
    use ranger::templater::{create_node_deployments, filter_node_deployments};
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
            "test-exercise".to_string(),
        )?;
        insta::assert_debug_snapshot!(deployment);
        Ok(())
    }

    #[actix_web::test]
    pub async fn mock_deploy_vms() -> Result<()> {
        let scenario = TEST_SCHEMA.scenario.clone();
        let exercise_name = "test-exercise";
        let templater_address = create_mock_vmware_server().run_template_server()?;
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

        let node_deployment_results =
            create_node_deployments(scenario, &deployment_group.templaters, exercise_name).await?;
        let node_deployments = filter_node_deployments(node_deployment_results)?;

        let mut node_identifiers = deployment_group.deploy_vms(node_deployments).await?;
        node_identifiers.sort_by_key(|node_name| node_name.1.clone());
        insta::assert_debug_snapshot!(node_identifiers);
        Ok(())
    }
}
