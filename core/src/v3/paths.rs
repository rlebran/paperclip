use crate::{
    common::{Either, HttpMethod},
    v3::server::Server,
};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// Path item object.
///
/// <https://github.com/OAI/OpenAPI-Specification/blob/master/versions/2.0.md#pathItemObject>
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct PathItem<P, B, R> {
    #[serde(rename = "$ref", skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(flatten, default = "BTreeMap::default")]
    pub methods: BTreeMap<HttpMethod, Operation<P, B, R>>,
    #[serde(default = "Vec::default", skip_serializing_if = "Vec::is_empty")]
    pub servers: Vec<Server>,
    #[serde(default = "Vec::default", skip_serializing_if = "Vec::is_empty")]
    pub parameters: Vec<Either<Reference, P>>,
}

/// Operation object.
///
/// <https://github.com/OAI/OpenAPI-Specification/blob/master/versions/2.0.md#operationObject>
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Operation<P, B, R> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_docs: Option<ExternalDoc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_id: Option<String>,
    #[serde(default = "Vec::default", skip_serializing_if = "Vec::is_empty")]
    pub parameters: Vec<Either<Reference, P>>,
    #[serde(
        rename = "requestBody",
        default = "Vec::default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub request_body: Vec<Either<Reference, B>>,
    #[serde(default = "BTreeMap::default")]
    pub responses: BTreeMap<String, Either<Reference, R>>,
    // pub callbacks // @todo support for callbacks: see https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.1.0.md#operationObject
    #[serde(default, skip_serializing_if = "is_false")]
    pub deprecated: bool,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub security: Vec<BTreeMap<String, Vec<String>>>,
    #[serde(default = "Vec::default", skip_serializing_if = "Vec::is_empty")]
    pub servers: Vec<Server>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ExternalDoc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub url: url::Url,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct Reference {
    #[serde(rename = "$ref")]
    pub reference: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
