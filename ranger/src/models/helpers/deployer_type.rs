use diesel::{
    backend::RawValue,
    deserialize,
    deserialize::FromSql,
    mysql::Mysql,
    serialize,
    serialize::ToSql,
    serialize::{IsNull, Output},
    sql_types::Text,
    AsExpression, FromSqlRow,
};
use serde::{Deserialize, Serialize};
use std::{
    fmt,
    fmt::{Display, Formatter},
    io::Write,
};

#[derive(Debug, Clone, Copy, FromSqlRow, AsExpression, Hash, Eq, PartialEq)]
#[diesel(sql_type = Text)]
pub struct DeployerType(pub ranger_grpc::capabilities::DeployerTypes);

impl Serialize for DeployerType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let value = match self.0 {
            ranger_grpc::capabilities::DeployerTypes::Switch => "switch",
            ranger_grpc::capabilities::DeployerTypes::Template => "template",
            ranger_grpc::capabilities::DeployerTypes::VirtualMachine => "virtual_machine",
        };
        serializer.serialize_str(value)
    }
}

impl<'de> Deserialize<'de> for DeployerType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let type_string = String::deserialize(deserializer)?;
        match type_string.as_str() {
            "switch" => Ok(DeployerType(
                ranger_grpc::capabilities::DeployerTypes::Switch,
            )),
            "template" => Ok(DeployerType(
                ranger_grpc::capabilities::DeployerTypes::Template,
            )),
            "virtual_machine" => Ok(DeployerType(
                ranger_grpc::capabilities::DeployerTypes::VirtualMachine,
            )),
            _ => Err(serde::de::Error::custom(format!(
                "Invalid deployer type: {}",
                type_string
            ))),
        }
    }
}

impl FromSql<Text, Mysql> for DeployerType {
    fn from_sql(bytes: RawValue<Mysql>) -> deserialize::Result<Self> {
        if let Ok(value) = <String>::from_sql(bytes) {
            return match value.as_str() {
                "switch" => Ok(DeployerType(
                    ranger_grpc::capabilities::DeployerTypes::Switch,
                )),
                "template" => Ok(DeployerType(
                    ranger_grpc::capabilities::DeployerTypes::Template,
                )),
                "virtual_machine" => Ok(DeployerType(
                    ranger_grpc::capabilities::DeployerTypes::VirtualMachine,
                )),
                _ => Err("Invalid deployer type".into()),
            };
        }
        Err("Failed to parse deployer type into string".into())
    }
}

impl Display for DeployerType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl ToSql<Text, Mysql> for DeployerType {
    fn to_sql(&self, out: &mut Output<Mysql>) -> serialize::Result {
        let value = String::from(match self {
            DeployerType(ranger_grpc::capabilities::DeployerTypes::Switch) => "switch",
            DeployerType(ranger_grpc::capabilities::DeployerTypes::Template) => "template",
            DeployerType(ranger_grpc::capabilities::DeployerTypes::VirtualMachine) => {
                "virtual_machine"
            }
        });
        out.write_all(value.as_bytes())?;
        Ok(IsNull::No)
    }
}
