use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Server {
    pub url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default = "BTreeMap::new", skip_serializing_if = "BTreeMap::is_empty")]
    pub variables: BTreeMap<String, ServerVariable>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ServerVariable {
    #[serde(rename = "enum", skip_serializing_if = "Vec::is_empty")]
    pub enum_: Vec<String>,
    pub default: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
