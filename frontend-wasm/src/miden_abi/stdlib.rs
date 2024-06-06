//! Function types and lowered signatures for the Miden stdlib API functions

use std::sync::OnceLock;

use super::ModuleFunctionTypeMap;
use crate::translation_utils::sanitize_name;

pub(crate) mod crypto;
pub(crate) mod mem;

pub(crate) fn signatures() -> &'static ModuleFunctionTypeMap {
    static TYPES: OnceLock<ModuleFunctionTypeMap> = OnceLock::new();
    TYPES.get_or_init(|| {
        let mut m: ModuleFunctionTypeMap = Default::default();
        m.extend(crypto::hashes::signatures());
        m.extend(crypto::dsa::signatures());
        m.extend(mem::signatures());
        let m_sanitized: ModuleFunctionTypeMap = m
            .into_iter()
            .map(|(module, v)| {
                let module_sanitized = sanitize_name(&module);
                (module_sanitized, v)
            })
            .collect();
        m_sanitized
    })
}
