//! Function types and lowering for tx kernel API functions

pub(crate) mod account;
pub(crate) mod note;
pub(crate) mod tx;

use std::sync::OnceLock;

use super::ModuleFunctionTypeMap;
use crate::translation_utils::sanitize_name;

pub(crate) fn signatures() -> &'static ModuleFunctionTypeMap {
    static TYPES: OnceLock<ModuleFunctionTypeMap> = OnceLock::new();
    TYPES.get_or_init(|| {
        let mut m: ModuleFunctionTypeMap = Default::default();
        m.extend(account::signatures());
        m.extend(note::signatures());
        m.extend(tx::signatures());

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
