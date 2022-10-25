const DEFAULT_DEPLOYER_GROUP_NAME: &str = "default";

pub const fn default_deployment_group_name() -> &'static str {
    DEFAULT_DEPLOYER_GROUP_NAME
}

pub const MAX_DEPLOYMENT_NAME_LENGTH: usize = 20;
pub const MAX_EXERCISE_NAME_LENGTH: usize = 20;

pub const RECORD_NOT_FOUND: &str = "Record not found";
pub const FOREIGN_KEY_CONSTRAINT_FAILS: &str = "a foreign key constraint fails";
