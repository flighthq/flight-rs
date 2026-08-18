// @generated from upstream/packages/types/src/ModifierRegistry.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{ModifierDefinition, ModifierKind};

// Source: upstream/packages/types/src/ModifierRegistry.ts:9 (sha256:778ef63f44eba1c663f0dde187afd6c28a7a394c91ab6100af02d72a7819c8fb)
#[derive(Clone, Default)]
pub struct ModifierRegistry {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub definitions: Vec<(ModifierKind, ModifierDefinition)>,
}
impl PartialEq for ModifierRegistry {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
