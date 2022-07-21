mod common;

#[cfg(test)]
mod tests {
    use crate::common::create_mock_vmware_server;
    use actix::Actor;
    use actix_rt::System;
    use anyhow::{Error, Result};
    use ranger::templater::{CreateTemplate, DeleteTemplate, TemplateClient};
    use ranger_grpc::{Identifier, Source};

    #[test]
    fn template_created_successfully() -> Result<()> {
        let socket_address = create_mock_vmware_server().run_template_server()?;
        let system = System::new();
        let template_id = system.block_on(async {
            let template_deployer_client =
                TemplateClient::new(format!("http://{}", socket_address).clone())
                    .await?
                    .start();
            let template_id = template_deployer_client
                .send(CreateTemplate(Source {
                    name: String::from("some-name"),
                    version: String::from("0.1.0"),
                }))
                .await??;
            Ok::<Identifier, Error>(template_id)
        })?;

        insta::assert_debug_snapshot!(template_id);
        Ok(())
    }

    #[test]
    fn template_creation_failed() -> Result<()> {
        let socket_address = create_mock_vmware_server()
            .successful_create(false)
            .run_template_server()?;
        let system = System::new();
        let result = system.block_on(async {
            let template_deployer_client =
                TemplateClient::new(format!("http://{}", socket_address).clone())
                    .await?
                    .start();
            template_deployer_client
                .send(CreateTemplate(Source {
                    name: String::from("some-name"),
                    version: String::from("0.1.0"),
                }))
                .await??;
            Ok::<(), Error>(())
        });

        assert!(result.is_err());
        Ok(())
    }

    #[test]
    fn template_deleted_successfully() -> Result<()> {
        let socket_address = crate::common::create_mock_vmware_server().run_template_server()?;
        let system = System::new();
        system.block_on(async {
            let template_deployer_client =
                TemplateClient::new(format!("http://{}", socket_address).clone())
                    .await?
                    .start();
            template_deployer_client
                .send(DeleteTemplate(Identifier {
                    value: String::from("some-identifier"),
                }))
                .await??;
            Ok::<(), Error>(())
        })?;

        Ok(())
    }

    #[test]
    fn template_deletion_failed() -> Result<()> {
        let socket_address = create_mock_vmware_server()
            .successful_delete(false)
            .run_template_server()?;
        let system = System::new();
        let result = system.block_on(async {
            let template_deployer_client =
                TemplateClient::new(format!("http://{}", socket_address).clone())
                    .await?
                    .start();
            template_deployer_client
                .send(DeleteTemplate(Identifier {
                    value: String::from("some-identifier"),
                }))
                .await??;
            Ok::<(), Error>(())
        });
        assert!(result.is_err());
        Ok(())
    }
}
