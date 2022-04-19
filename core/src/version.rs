use serde::{Serialize, Deserialize};

/// OpenAPI version.
#[derive(Clone, Copy, Debug, Serialize, Deserialize, Eq, PartialEq)]
pub enum Version {
    #[serde(rename = "2.0")]
    V2,
    #[serde(rename = "3.1")]
    V3_1,
}
