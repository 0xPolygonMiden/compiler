use alloc::{fmt, sync::Arc};

use miden_assembly::Library as CompiledLibrary;
use miden_core::{utils::Deserializable, Program};
use miden_processor::Digest;

use crate::MastArtifact;

pub fn deserialize_digest<'de, D>(deserializer: D) -> Result<Digest, D::Error>
where
    D: serde::Deserializer<'de>,
{
    const DIGEST_BYTES: usize = 32;

    let bytes: [u8; DIGEST_BYTES] = serde_bytes::deserialize(deserializer)?;

    Digest::try_from(bytes).map_err(serde::de::Error::custom)
}

pub fn deserialize_mast<'de, D>(deserializer: D) -> Result<MastArtifact, D::Error>
where
    D: serde::Deserializer<'de>,
{
    struct MastArtifactVisitor;

    impl<'de> serde::de::Visitor<'de> for MastArtifactVisitor {
        type Value = MastArtifact;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("mast artifact")
        }

        fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            if let Some(bytes) = v.strip_prefix(b"PRG\0") {
                Program::read_from_bytes(bytes)
                    .map(Arc::new)
                    .map(MastArtifact::Executable)
                    .map_err(serde::de::Error::custom)
            } else if let Some(bytes) = v.strip_prefix(b"LIB\0") {
                CompiledLibrary::read_from_bytes(bytes)
                    .map(Arc::new)
                    .map(MastArtifact::Library)
                    .map_err(serde::de::Error::custom)
            } else {
                Err(serde::de::Error::invalid_value(
                    serde::de::Unexpected::Bytes(v.get(0..4).unwrap_or(v)),
                    &"expected valid mast artifact type tag",
                ))
            }
        }
    }
    deserializer.deserialize_bytes(MastArtifactVisitor)
}
