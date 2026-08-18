// @generated from upstream/packages/types/src/ParsedAccelerator.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{ShortcutKeyName, ShortcutModifier};

// Source: upstream/packages/types/src/ParsedAccelerator.ts:13 (sha256:9922be3517531ac1f62eed3d778ca5c3e6e7c096c9ff910ca3ca3257fc02d223)
#[derive(Clone)]
pub struct ParsedAccelerator {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub key: crate::FlightUnion2<ShortcutKeyName, String>,
    pub modifiers: Vec<ShortcutModifier>,
}
impl PartialEq for ParsedAccelerator {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
