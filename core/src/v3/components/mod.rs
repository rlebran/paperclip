use std::collections::BTreeMap;
use crate::common::Either;
use crate::v3::components::schema::Schema;
use crate::v3::paths::Reference;
use crate::v3::security::SecurityScheme;
use crate::v3::server::Server;

mod schema;

/// Holds a set of reusable objects for different aspects of the OAS.
/// All objects defined within the components object will have no effect
/// on the API unless they are explicitly referenced from properties
/// outside the components object.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Components<P, B, R> {
    /// An object to hold reusable Schema Objects.
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub schemas: BTreeMap<String, Schema>,
    /// An object to hold reusable Response Objects.
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub responses: BTreeMap<String, Either<Reference, R>>,
    /// An object to hold reusable Parameter Objects.
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub parameters: BTreeMap<String, Either<Reference, P>>,
    /// An object to hold reusable Example Objects.
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub examples: BTreeMap<String, Either<Reference, Example>>,
    /// An object to hold reusable Request Body Objects.
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub request_bodies: BTreeMap<String, Either<Reference, B>>,
    /// An object to hold reusable Header Objects.
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub headers: BTreeMap<String, Either<Reference, P>>,
    /// An object to hold reusable Security Scheme Objects.
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub security_schemes: BTreeMap<String, Either<Reference, SecurityScheme>>,
    /// An object to hold reusable Link Objects.
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub links: BTreeMap<String, Either<Reference, Link>>,
    // /// An object to hold reusable Callback Objects.
    // #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    // pub callbacks: BTreeMap<String, Either<Reference, P>>, //@todo support webhooks / callbacks. See: https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.1.0.md#componentsObject
    // /// An object to hold reusable Request Body Objects.
    // #[serde(default, rename = "pathItems", skip_serializing_if = "BTreeMap::is_empty")]
    // pub path_items: BTreeMap<String, Either<Reference, B>>, //@todo WTF ???
    /// Inline extensions to this object.
    #[serde(flatten)]
    pub extensions: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct Example {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<serde_json::Value>,
    #[serde(rename = "externalValue", skip_serializing_if = "Option::is_none")]
    pub external_value: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Link {
    #[serde(flatten)]
    pub operation_identifier: LinkOperationIdentifier,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub parameters: BTreeMap<String, String>,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty", rename = "requestBody")]
    pub request_body: BTreeMap<String, String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "externalValue", skip_serializing_if = "Option::is_none")]
    pub server: Option<Server>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum LinkOperationIdentifier {
    #[serde(rename = "operationId")]
    Id,
    #[serde(rename = "operationRef")]
    Ref
}
