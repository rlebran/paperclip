use serde::{Deserialize, Deserializer, Serialize, Serializer};

use std::{
    collections::BTreeMap,
    ops::{Deref, DerefMut},
    sync::Arc,
};
use crate::extensions::{Coder, MediaRange};

/// `x-rust-coders` global extension for custom encoders and decoders.
#[derive(Debug, Default, Clone)]
pub struct Coders(BTreeMap<MediaRange, Arc<Coder>>);

#[cfg(feature = "codegen")]
impl Coders {
    /// Returns the matching coder for the given media range (if any).
    ///
    /// Matching algorithm from <https://github.com/hyperium/mime/blob/65ea9c3d0cad4cb548b41124050c545120134035/src/range.rs#L126>
    pub fn matching_coder(&self, ty: &MediaRange) -> Option<Arc<Coder>> {
        self.0
            .get(ty)
            .or_else(|| {
                let (target_t1, target_t2) = (ty.0.type_(), ty.0.subtype());
                for (r, c) in &self.0 {
                    let (source_t1, source_t2) = (r.0.type_(), r.0.subtype());
                    if target_t1 == mime::STAR && r.matches_params(ty) {
                        return Some(c);
                    }

                    if source_t1 != target_t1 {
                        continue;
                    }

                    if target_t2 == mime::STAR && r.matches_params(ty) {
                        return Some(c);
                    }

                    if source_t2 != target_t2 {
                        continue;
                    }

                    return Some(c);
                }

                None
            })
            .map(Clone::clone)
    }
}

/* Common trait impls */

impl Deref for Coders {
    type Target = BTreeMap<MediaRange, Arc<Coder>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Coders {
    fn deref_mut(&mut self) -> &mut <Self as Deref>::Target {
        &mut self.0
    }
}

impl Serialize for Coders {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        self.0.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Coders {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        Ok(Coders(BTreeMap::deserialize(deserializer)?))
    }
}
