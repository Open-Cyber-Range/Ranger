mod common;

#[cfg(test)]
mod tests {
    use crate::common::create_mock_capability_server;
    use anyhow::Result;
    use ranger::{
        deployers::GetDeployerGroups,
        errors::{RangerError, ServerResponseError},
        AppState,
    };
    use std::collections::HashMap;

    #[actix_web::test]
    async fn add_deployer_groups() -> Result<()> {
        let mut deployers = HashMap::new();
        deployers.insert(
            "my_cool_machiner_one".to_string(),
            format!("http://{}", create_mock_capability_server().run()?),
        );
        deployers.insert(
            "my_cool_machiner_two".to_string(),
            format!("http://{}", create_mock_capability_server().run()?),
        );

        let mut deployer_groups = HashMap::new();
        deployer_groups.insert(
            "my_cool_machiner_group_one".to_string(),
            vec![
                "my_cool_machiner_one".to_string(),
                "my_cool_machiner_two".to_string(),
            ],
        );
        deployer_groups.insert(
            "my_cool_machiner_group_two".to_string(),
            vec!["my_cool_machiner_one".to_string()],
        );

        let app_state = AppState::new();
        app_state
            .add_initial_deployergroups(deployer_groups, deployers)
            .await?;

        let validated_deployer_groups = app_state
            .deployer_actor_address
            .send(GetDeployerGroups)
            .await
            .map_err(|_| ServerResponseError(RangerError::ActixMailBoxError.into()))
            .unwrap();
        insta::with_settings!({sort_maps => true}, {
           insta::assert_yaml_snapshot!(validated_deployer_groups, {
              "[\"my_cool_machiner_group_one\"].machiners[\"my_cool_machiner_one\"]" => "http://some-host:some-port",
              "[\"my_cool_machiner_group_one\"].machiners[\"my_cool_machiner_two\"]" => "http://some-host:some-port",
              "[\"my_cool_machiner_group_two\"].machiners[\"my_cool_machiner_one\"]" => "http://some-host:some-port"
           });
        });
        Ok(())
    }
}
