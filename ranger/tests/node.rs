mod common;

#[cfg(test)]
mod tests {
    use crate::common::create_mock_vmware_server;
    use actix::Actor;
    use actix_rt::System;
    use anyhow::{Error, Result};
    use ranger::node::{CreateNode, DeleteNode, NodeClient};
    use ranger_grpc::{
        Configuration, DeploymentParameters, Identifier, Node, NodeDeployment, NodeIdentifier,
        NodeType,
    };

    #[test]
    fn node_created_successfully() -> Result<()> {
        let socket_address = create_mock_vmware_server().run_node_server()?;
        let system = System::new();
        let node_id = system.block_on(async {
            let node_deployer_client =
                NodeClient::new(format!("http://{}", socket_address).clone())
                    .await?
                    .start();
            let node_id = node_deployer_client
                .send(CreateNode(NodeDeployment {
                    parameters: Some(DeploymentParameters {
                        name: "some-name".to_string(),
                        exercise_name: "some-exercise".to_string(),
                        template_id: "some-uuid".to_string(),
                    }),
                    node: Some(Node {
                        identifier: Some(NodeIdentifier {
                            identifier: None,
                            node_type: NodeType::Vm.into(),
                        }),
                        configuration: Some(Configuration {
                            cpu: 1,
                            ram: 536870912, //512mib
                        }),
                    }),
                }))
                .await??;
            Ok::<NodeIdentifier, Error>(node_id)
        })?;

        insta::assert_debug_snapshot!(node_id);
        Ok(())
    }

    #[test]
    fn node_creation_failed() -> Result<()> {
        let socket_address = create_mock_vmware_server()
            .successful_create(false)
            .run_node_server()?;
        let system = System::new();
        let result = system.block_on(async {
            let node_deployer_client =
                NodeClient::new(format!("http://{}", socket_address).clone())
                    .await?
                    .start();
            node_deployer_client
                .send(CreateNode(NodeDeployment {
                    parameters: Some(DeploymentParameters {
                        name: "some-name".to_string(),
                        exercise_name: "some-exercise".to_string(),
                        template_id: "some-uuid".to_string(),
                    }),
                    node: Some(Node {
                        identifier: Some(NodeIdentifier {
                            identifier: None,
                            node_type: NodeType::Vm.into(),
                        }),
                        configuration: Some(Configuration {
                            cpu: 1,
                            ram: 536870912, //512mib
                        }),
                    }),
                }))
                .await??;
            Ok::<(), Error>(())
        });

        assert!(result.is_err());
        Ok(())
    }

    #[test]
    fn node_deleted_successfully() -> Result<()> {
        let socket_address = create_mock_vmware_server().run_node_server()?;
        let system = System::new();
        system.block_on(async {
            let node_deployer_client =
                NodeClient::new(format!("http://{}", socket_address).clone())
                    .await?
                    .start();
            node_deployer_client
                .send(DeleteNode(NodeIdentifier {
                    identifier: Some(Identifier {
                        value: "some-identifier".to_string(),
                    }),
                    node_type: NodeType::Vm.into(),
                }))
                .await??;
            Ok::<(), Error>(())
        })?;

        Ok(())
    }

    #[test]
    fn node_deletion_failed() -> Result<()> {
        let socket_address = create_mock_vmware_server()
            .successful_delete(false)
            .run_node_server()?;
        let system = System::new();
        let result = system.block_on(async {
            let node_deployer_client =
                NodeClient::new(format!("http://{}", socket_address).clone())
                    .await?
                    .start();
            node_deployer_client
                .send(DeleteNode(NodeIdentifier {
                    identifier: Some(Identifier {
                        value: "some-identifier".to_string(),
                    }),
                    node_type: NodeType::Vm.into(),
                }))
                .await??;
            Ok::<(), Error>(())
        });

        assert!(result.is_err());
        Ok(())
    }
}
