mod common;

#[cfg(test)]
mod tests {
    use actix::Actor;
    use actix_rt::System;
    use anyhow::{Error, Result};
    use ranger::node::{CreateNode, DeleteNode, NodeClient};
    use ranger_grpc::Identifier;

    #[test]
    fn node_created_successfully() -> Result<()> {
        let socket_address = crate::common::create_mock_node_server().run()?;
        let system = System::new();
        let node_id = system.block_on(async {
            let node_deployer_client =
                NodeClient::new(format!("http://{}", socket_address).clone())
                    .await?
                    .start();
            let node_id = node_deployer_client
                .send(CreateNode(ranger_grpc::Node {
                    name: "some-name".to_string(),
                    exercise_name: "some-exercise-name".to_string(),
                    template_name: "debian10".to_string(),
                }))
                .await??;
            Ok::<Identifier, Error>(node_id)
        })?;

        insta::assert_debug_snapshot!(node_id);
        Ok(())
    }

    #[test]
    fn node_creation_failed() -> Result<()> {
        let socket_address = crate::common::create_mock_node_server()
            .successful_create(false)
            .run()?;
        let system = System::new();
        let result = system.block_on(async {
            let node_deployer_client =
                NodeClient::new(format!("http://{}", socket_address).clone())
                    .await?
                    .start();
            node_deployer_client
                .send(CreateNode(ranger_grpc::Node {
                    name: "some-name".to_string(),
                    exercise_name: "some-exercise-name".to_string(),
                    template_name: "debian10".to_string(),
                }))
                .await??;
            Ok::<(), Error>(())
        });

        assert!(result.is_err());
        Ok(())
    }

    #[test]
    fn node_deleted_successfully() -> Result<()> {
        let socket_address = crate::common::create_mock_node_server().run()?;
        let system = System::new();
        system.block_on(async {
            let node_deployer_client =
                NodeClient::new(format!("http://{}", socket_address).clone())
                    .await?
                    .start();
            node_deployer_client
                .send(DeleteNode("some-node-id".to_string()))
                .await??;
            Ok::<(), Error>(())
        })?;

        Ok(())
    }

    #[test]
    fn node_deletion_failed() -> Result<()> {
        let socket_address = crate::common::create_mock_node_server()
            .successful_delete(false)
            .run()?;
        let system = System::new();
        let result = system.block_on(async {
            let node_deployer_client =
                NodeClient::new(format!("http://{}", socket_address).clone())
                    .await?
                    .start();
            node_deployer_client
                .send(DeleteNode("some-node-id".to_string()))
                .await??;
            Ok::<(), Error>(())
        });

        assert!(result.is_err());
        Ok(())
    }
}
