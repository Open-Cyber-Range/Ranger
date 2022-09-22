use crate::{
    constants::MAX_DEPLOYMENT_NAME_LENGTH,
    errors::RangerError,
    utilities::{default_uuid, Validation},
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Deployment {
    #[serde(default = "default_uuid")]
    pub id: Uuid,
    pub name: String,
    pub deployment_group: Option<String>,
}

impl Validation for Deployment {
    fn validate(&self) -> Result<(), RangerError> {
        if self.name.len() > MAX_DEPLOYMENT_NAME_LENGTH {
            return Err(RangerError::DeploymentNameTooLong);
        }
        Ok(())
    }
}
