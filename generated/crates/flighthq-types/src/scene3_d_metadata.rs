// @generated from upstream/packages/types/src/Scene3DMetadata.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/Scene3DMetadata.ts:4 (sha256:90594e5397c8d83d4fbb2d391506532d2a78d0f3a4773d5e5f05a5b04eef081a)
#[derive(Clone, Default)]
pub struct Scene3DMetadata {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub copyright: Option<String>,
    pub generator: Option<String>,
    pub version: Option<String>,
}
impl PartialEq for Scene3DMetadata {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
