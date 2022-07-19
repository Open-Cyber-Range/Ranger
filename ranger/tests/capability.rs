mod common;

#[cfg(test)]
mod tests {
    use crate::common::create_mock_capability_server;
    use actix::Actor;
    use actix_rt::System;
    use anyhow::{Error, Result};
    use ranger::{capability::GetCapabilities, node::NodeClient};
    use ranger_grpc::Capabilities;

    #[test]
    fn successful_capability_response() -> Result<()> {
        let socket_address = create_mock_capability_server().run()?;
        let system = System::new();
        let capabilities = system.block_on(async {
            let node_deployer_client = NodeClient::new(format!("http://{}", socket_address))
                .await?
                .start();

            let capabilities = node_deployer_client.send(GetCapabilities).await??;
            Ok::<Capabilities, Error>(capabilities)
        })?;

        insta::assert_debug_snapshot!(capabilities);
        Ok(())
    }
}
