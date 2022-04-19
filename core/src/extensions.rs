use std::cmp::Ordering;
use std::fmt;
use std::sync::Arc;
use once_cell::sync::Lazy;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// Media range for JSON.
pub static JSON_MIME: Lazy<MediaRange> =
    Lazy::new(|| MediaRange("application/json".parse().expect("parsing mime")));
/// Default coder for JSON.
pub static JSON_CODER: Lazy<Arc<Coder>> = Lazy::new(|| {
    Arc::new(Coder {
        encoder_path: "serde_json::to_writer".into(),
        decoder_path: "serde_json::from_reader".into(),
        any_value: "serde_json::Value".into(),
        error_path: "serde_json::Error".into(),
        prefer: false,
        builtin: true,
    })
});
/// Media range for YAML.
pub static YAML_MIME: Lazy<MediaRange> =
    Lazy::new(|| MediaRange("application/yaml".parse().expect("parsing mime")));
/// Default coder for YAML.
pub static YAML_CODER: Lazy<Arc<Coder>> = Lazy::new(|| {
    Arc::new(Coder {
        encoder_path: "serde_yaml::to_writer".into(),
        decoder_path: "serde_yaml::from_reader".into(),
        any_value: "serde_yaml::Value".into(),
        error_path: "serde_yaml::Error".into(),
        prefer: false,
        builtin: true,
    })
});

/// Wrapper for `mime::MediaRange` to support `BTree{Set, Map}`.
#[derive(Debug, Clone)]
pub struct MediaRange(pub mime::Mime);

#[cfg(feature = "codegen")]
impl MediaRange {
    /// Implementation from https://github.com/hyperium/mime/blob/65ea9c3d0cad4cb548b41124050c545120134035/src/range.rs#L155
    pub(crate) fn matches_params(&self, r: &Self) -> bool {
        for (name, value) in self.0.params() {
            if name != "q" && r.0.get_param(name) != Some(value) {
                return false;
            }
        }

        true
    }
}

/// Represents the en/decoder for some MIME media range.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Coder {
    /// Path to the encoding function.
    pub encoder_path: String,
    /// Path to the decoding function.
    pub decoder_path: String,
    /// Path to the error type.
    pub error_path: String,
    /// Path to the struct/enum that represents `Any` (such as `serde_json::Value`).
    pub any_value: String,
    /// Whether this media type should be preferred when multiple
    /// types are available. When multiple types are preferred,
    /// it's unspecified as to which is chosen.
    #[serde(default)]
    pub prefer: bool,
    /// Whether this en/decoder is built-in.
    #[serde(skip)]
    pub builtin: bool,
}

/* Common trait impls */

impl PartialEq for MediaRange {
    fn eq(&self, other: &MediaRange) -> bool {
        self.0.eq(&other.0)
    }
}

impl Eq for MediaRange {}

impl PartialOrd for MediaRange {
    fn partial_cmp(&self, other: &MediaRange) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for MediaRange {
    fn cmp(&self, other: &MediaRange) -> Ordering {
        self.0.as_ref().cmp(other.0.as_ref())
    }
}

impl Serialize for MediaRange {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        serializer.serialize_str(self.0.as_ref())
    }
}

impl<'de> Deserialize<'de> for MediaRange {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = MediaRange;

            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                f.write_str("a valid media range")
            }

            fn visit_str<E>(self, value: &str) -> Result<MediaRange, E>
                where
                    E: serde::de::Error,
            {
                value.parse().map_err(E::custom).map(MediaRange)
            }
        }

        deserializer.deserialize_str(Visitor)
    }
}
