use std::sync::Arc;
use serde::{Deserialize, Serialize};
use crate::extensions::{Coder, JSON_CODER, JSON_MIME, MediaRange, YAML_CODER, YAML_MIME};

/// The HTTP method used for an operation.
#[derive(Clone, Copy, Debug, Serialize, Deserialize, Eq, PartialEq, Ord, PartialOrd)]
#[serde(rename_all = "lowercase")]
pub enum HttpMethod {
    Get,
    Put,
    Post,
    Delete,
    Options,
    Head,
    Patch,
    #[cfg(feature = "v3")]
    Trace
}

impl HttpMethod {
    /// Whether this method allows body in requests.
    pub fn allows_body(self) -> bool {
        std::matches!(self, HttpMethod::Post | HttpMethod::Put | HttpMethod::Patch)
    }
}

/// `Either` from "either" crate. We can't use that crate because
/// we don't want the enum to be tagged during de/serialization.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Either<L, R> {
    Left(L),
    Right(R),
}

impl<L, R> Either<L, R> {
    /// Get a readable reference to the right variant (if it exists).
    pub fn right(&self) -> Option<&R> {
        match self {
            Either::Left(_) => None,
            Either::Right(r) => Some(r),
        }
    }

    /// Get a mutable reference to the right variant (if it exists).
    pub fn right_mut(&mut self) -> Option<&mut R> {
        match self {
            Either::Left(_) => None,
            Either::Right(r) => Some(r),
        }
    }

    /// Get a readable reference to the left variant (if it exists).
    pub fn left(&self) -> Option<&L> {
        match self {
            Either::Left(l) => Some(l),
            Either::Right(_) => None,
        }
    }

    /// Get a mutable reference to the left variant (if it exists).
    pub fn left_mut(&mut self) -> Option<&mut L> {
        match self {
            Either::Left(l) => Some(l),
            Either::Right(_) => None,
        }
    }
}



/// The format used by spec (JSON/YAML).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpecFormat {
    Json,
    Yaml,
}

impl SpecFormat {
    /// The en/decoder used for this format.
    pub fn coder(self) -> Arc<Coder> {
        match self {
            SpecFormat::Json => JSON_CODER.clone(),
            SpecFormat::Yaml => YAML_CODER.clone(),
        }
    }

    /// The mime for this format.
    pub fn mime(self) -> &'static MediaRange {
        match self {
            SpecFormat::Json => &*JSON_MIME,
            SpecFormat::Yaml => &*YAML_MIME,
        }
    }
}
