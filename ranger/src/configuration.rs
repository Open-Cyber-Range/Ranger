use crate::constants::default_deployment_group_name;
use anyhow::{Error, Result};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs::read_to_string};

pub type AddressBook = HashMap<String, String>;
pub type DeploymentGroupMap = HashMap<String, Vec<String>>;

fn deployment_group_name() -> String {
    default_deployment_group_name().to_string()
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct Configuration {
    pub host: String,
    pub port: u16,
    pub deployers: AddressBook,
    #[serde(default = "deployment_group_name")]
    pub default_deployment_group: String,
    pub deployment_groups: DeploymentGroupMap,
}

pub fn read_configuration(arguments: Vec<String>) -> Result<Configuration> {
    let file_path = arguments
        .get(1)
        .ok_or_else(|| Error::msg("Configuration path argument missing"))?;

    let configuration_string = read_to_string(file_path)?;
    Ok(serde_yaml::from_str(&configuration_string)?)
}

#[cfg(test)]
mod tests {
    use super::read_configuration;
    use anyhow::Result;
    use std::fs::File;
    use std::io::Write;
    use tempfile::tempdir;

    #[test]
    fn can_parse_the_configuration() -> Result<()> {
        let temporary_directory = tempdir()?;
        let file_path = temporary_directory.path().join("test-config.yml");
        let path_string = file_path.clone().into_os_string().into_string().unwrap();
        let mut file = File::create(file_path)?;
        writeln!(
            file,
            r#"
                host: localhost
                port: 8080
                deployers:
                    my-machiner-deployer: http://ranger-vmware-machiner:9999
                    my-switch-deployer: http://ranger-vmware-switcher:9999
                    ungrouped-deployer: http://some-vmware-deployer:9999

                default_deployment_group: my-cool-group
                deployment_groups:
                    my-cool-group:
                        - my-machiner-deployer
                        - my-switch-deployer
                "#
        )?;
        let arguments = vec![String::from("program-name"), path_string];
        let configuration = read_configuration(arguments)?;

        insta::with_settings!({sort_maps => true}, {
        insta::assert_yaml_snapshot!(configuration);
        });
        Ok(())
    }
}
