use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Tag {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "externalDocs")]
    pub external_docs: Option<ExternalDocs>,
}

/// External Documentation object.
///
/// <https://github.com/OAI/OpenAPI-Specification/blob/master/versions/2.0.md#external-documentation-object>
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ExternalDocs {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub url: String,
}
