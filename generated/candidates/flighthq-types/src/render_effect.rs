// @generated from upstream/packages/types/src/RenderEffect.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::Kind;

// Source: upstream/packages/types/src/RenderEffect.ts:13 (sha256:a96c82182a4d8ea1a51b966ca3a55d7ee91cccf410fff7e917e2a5c0f0bba54b)
#[derive(Clone)]
pub struct RenderEffect {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub kind: Kind,
}
impl PartialEq for RenderEffect {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
