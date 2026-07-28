// @generated from upstream/packages/types/src/RenderCache.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::Matrix;

// Source: upstream/packages/types/src/RenderCache.ts:15 (sha256:fbbab26591c975a56f0549917b0bf0eaa9b582875228a3a3f56270a09bf715a7)
#[derive(Clone)]
pub struct RenderCache {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub kind: RenderCacheKind,
    pub transform: Matrix,
}
impl PartialEq for RenderCache {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/RenderCache.ts:20 (sha256:6635360182c2b3057df34a9155f11d6864b7b9058b15348f25c4f2386261f1c5)
// TypeScript value namespace RenderCacheKind is represented by its generated Rust type.

// Source: upstream/packages/types/src/RenderCache.ts:21 (sha256:75a6755c719fdd03031052d9f81e51047022ed1aa09dfe2d66745548a451beeb)
pub type RenderCacheKind = crate::OpaqueHostValue;
