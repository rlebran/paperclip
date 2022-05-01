use std::collections::BTreeMap;
use serde::{Deserialize, Serialize};

/// Security Scheme object.
///
/// <https://github.com/OAI/OpenAPI-Specification/blob/master/versions/2.0.md#security-scheme-object>
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct SecurityScheme {
    #[serde(rename = "type")]
    pub type_: SecurityType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(flatten)]
    pub extensions: BTreeMap<String, serde_json::Value>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum SecurityType {
    #[serde(rename = "apiKey")]
    ApiKey(ApiKey),
    #[serde(rename = "http")]
    Http(HttpSecurity),
    #[serde(rename = "mutualTLS")]
    MutualTLS,
    #[serde(rename = "oauth2")]
    Oauth2(Oauth2Security),
    #[serde(rename = "openIdConnect")]
    OpenIdConnect(OpenIdConnectSecurity)
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ApiKeySecurity {
    pub name: String,
    #[serde(rename = "in")]
    pub in_: SecurityIn,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct HttpSecurity {
    pub scheme: String,
    #[serde(rename = "bearerFormat", skip_serializing_if = "Option::is_none")]
    pub bearer_format: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Oauth2Security {
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub flows: BTreeMap<String, Flow>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OpenIdConnectSecurity {
    #[serde(rename = "openIdConnectUrl")]
    pub open_id_connect_url: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum SecurityIn {
    #[serde(rename = "query")]
    Query,
    #[serde(rename = "header")]
    Header,
    #[serde(rename = "cookie")]
    Cookie,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Flow {
    #[serde(rename = "authorizationUrl", skip_serializing_if = "Option::is_none")]
    pub authorization_url: Option<url::Url>,
    #[serde(rename = "tokenUrl", skip_serializing_if = "Option::is_none")]
    pub token_url: Option<url::Url>,
    #[serde(rename = "refreshUrl", skip_serializing_if = "Option::is_none")]
    pub refresh_url: Option<url::Url>,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub scopes: BTreeMap<String, String>,
}
