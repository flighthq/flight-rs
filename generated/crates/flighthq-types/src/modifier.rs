// @generated from upstream/packages/types/src/Modifier.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{ModifierKind, ModifierSlot};

// Source: upstream/packages/types/src/Modifier.ts:18 (sha256:796da4037514e9798f33666107ab84b3e7a4d1656e87ad4354d2508e6a10dd38)
#[derive(Clone, Default)]
pub struct Modifier {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub kind: ModifierKind,
    pub slot: ModifierSlot,
}
impl PartialEq for Modifier {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
