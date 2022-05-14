#![cfg(feature = "v3")]

use crate::{
    common::SpecFormat,
    v3::{
        components::{schema::Schema, Components},
        info::Info,
        paths::PathItem,
        server::Server,
        tags::{ExternalDocs, Tag},
    },
    version::Version,
};
use std::collections::{BTreeMap, BTreeSet};

mod components;

mod info;
mod paths;
mod security;
mod server;
mod tags;

pub type DefaultResponseRaw = Response<Schema>;

/// OpenAPI v3 spec with defaults.
pub type DefaultApiRaw = Api<DefaultParameterRaw, DefaultResponseRaw, DefaultSchemaRaw>;

/// OpenAPI v3.1 spec generic over parameter and schema.
///
/// <https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.1.0.md#oasObject>
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Api<P, B, R, S> {
    pub openapi: Version,
    pub info: Info,
    #[serde(rename = "jsonSchemaDialect", skip_serializing_if = "Option::is_none")]
    pub json_schema_dialect: Option<url::Url>,
    #[serde(default = "BTreeMap::new")]
    pub servers: BTreeMap<String, Server>,
    #[serde(default = "BTreeMap::new")]
    pub paths: BTreeMap<String, PathItem<P, B, R>>,
    // pub webhooks //@todo support webhooks / callback. See: https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.1.0.md#oasDocument
    pub components: Components<P, B, R, S>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub security: Vec<BTreeMap<String, BTreeSet<String>>>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<Tag>,
    #[serde(rename = "externalDocs", skip_serializing_if = "Option::is_none")]
    pub external_docs: Option<ExternalDocs>,

    /// This field is set manually, because we don't know the format in which
    /// the spec was provided and we need to use this as the fallback encoding.
    #[serde(skip)]
    pub spec_format: SpecFormat,

    #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
    pub extensions: BTreeMap<String, serde_json::Value>,
}
