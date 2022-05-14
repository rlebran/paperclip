use crate::{
    v3::{components::Example, paths::Reference},
    Either,
};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Header {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default)]
    pub style: HeaderStyle,
    #[serde(default, skip_serializing_if = "is_false")]
    pub required: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deprecated: Option<bool>,
    #[serde(flatten)]
    pub format: ParameterSchemaOrContent,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub example: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "IndexMap::is_empty")]
    pub examples: BTreeMap<String, Either<Reference, Example>>,
    #[serde(flatten, skip_serializing_if = "IndexMap::is_empty")]
    pub extensions: BTreeMap<String, serde_json::Value>,
}
