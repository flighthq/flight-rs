// @generated from upstream/packages/types/src/ModifierDefinition.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{Modifier, ModifierKind, ModifierSlot};

// Source: upstream/packages/types/src/ModifierDefinition.ts:16 (sha256:bc5a0db9693884c4b2159347b87eabaa0ba4e2330a93083c4aaaa4c837b4ddc6)
#[derive(Clone, Default)]
pub struct ModifierDefinition {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub kind: ModifierKind,
    pub slot: ModifierSlot,
    pub get_define_signature: Option<
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(Modifier) -> String + Send + 'static>>>,
    >,
}
impl PartialEq for ModifierDefinition {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
