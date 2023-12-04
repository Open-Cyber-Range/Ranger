use crate::errors::RangerError;
use anyhow::Result;
use std::collections::HashMap;

pub mod admin;
pub mod basic;
pub mod deployers;
pub mod deputy_query;
pub mod exercise;
pub mod logger;
pub mod participant;
pub mod upload;

pub fn get_query_param(
    query_params: &HashMap<String, String>,
    param: &str,
) -> Result<String, RangerError> {
    query_params
        .get(param)
        .cloned()
        .ok_or(RangerError::MissingParameter(param.to_string()))
}
