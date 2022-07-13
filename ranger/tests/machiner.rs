mod common;

#[cfg(test)]
mod tests {
    use actix_web::test;
    use anyhow::{anyhow, Result};
    use ranger::deployers::DeployerGroup;
    use ranger::machiner::{DeploymentManager, NodeDeploymentTrait};
    use ranger_grpc::NodeDeployment;
    use sdl_parser::test::{RAW_TEST_SCHEMA, TEMPLATED_TEST_SCHEMA};

    #[test]
    async fn verify_vm_node_deployment_struct() -> Result<()> {
        let mut infrastructure = RAW_TEST_SCHEMA
            .scenario
            .infrastructure
            .clone()
            .ok_or_else(|| anyhow!("TEST: Infrastructure is missing"))?;
        let node = infrastructure
            .remove_entry("win10")
            .ok_or_else(|| anyhow!("TEST: Node not found"))?;
        let deployment =
            NodeDeployment::default().initialize_vm(node, "test-exercise".to_string())?;
        insta::assert_debug_snapshot!(deployment);
        Ok(())
    }

    #[actix_web::test]
    pub async fn mock_deploy_vms() -> Result<()> {
        let infrastructure = TEMPLATED_TEST_SCHEMA
            .scenario
            .infrastructure
            .clone()
            .ok_or_else(|| anyhow!("TEST: Infrastructure is missing"))?;

        let mock_node_server = crate::common::create_mock_node_server().run()?;
        let mut deployer_group = DeployerGroup::default();
        deployer_group.machiners.insert(
            "mock-server".to_string(),
            format!("http://{}", mock_node_server),
        );
        let deployment_group = deployer_group.start().await;

        let mut node_identifiers =
            DeploymentManager::deploy_vms(infrastructure, deployment_group, "test-exercise")
                .await?;
        node_identifiers.sort_by_key(|node_name| node_name.1.clone());
        println!("{:?}", node_identifiers);
        insta::assert_debug_snapshot!(node_identifiers);
        Ok(())
    }
}
