use std::collections::BTreeMap;
use serde::{Serialize, Deserialize};

/// Info object.
///
/// <https://github.com/OAI/OpenAPI-Specification/blob/master/versions/2.0.md#infoObject>
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Info {
    pub version: String,
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact: Option<Contact>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license: Option<License>,
    #[serde(rename = "termsOfService", skip_serializing_if = "Option::is_none")]
    pub terms_of_service: Option<url::Url>,
    /// Inline extensions to this object.
    #[serde(
    flatten,
    skip_serializing_if = "BTreeMap::is_empty"
    )]
    pub extensions: BTreeMap<String, serde_json::Value>,
}

/// Contact object.
///
/// <https://github.com/OAI/OpenAPI-Specification/blob/master/versions/2.0.md#contactObject>
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Contact {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<url::Url>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>, //@todo email type to ensure validity ?
}

/// License object.
///
/// <https://github.com/OAI/OpenAPI-Specification/blob/master/versions/2.0.md#licenseObject>
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct License {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", flatten)]
    pub url_or_identifier: Option<LicenseOrIdentifier>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LicenseOrIdentifier {
    Url(url::Url),
    Identifier(String)
}
