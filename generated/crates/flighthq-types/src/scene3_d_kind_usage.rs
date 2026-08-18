// @generated from upstream/packages/types/src/Scene3DKindUsage.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{Kind, TextureSourceKind};

// Source: upstream/packages/types/src/Scene3DKindUsage.ts:18 (sha256:6221bdcad721b47767821e41a9d71f9e1d6766b425ec20a430f0ee643b761ab6)
#[derive(Clone, Default)]
pub struct Scene3DKindUsage {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub material_kinds: Vec<Kind>,
    pub modifier_kinds: Vec<Kind>,
    pub node_kinds: Vec<Kind>,
    pub resource_mime_types: Vec<String>,
    pub texture_source_kinds: Vec<TextureSourceKind>,
}
impl PartialEq for Scene3DKindUsage {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
