// @generated from upstream/packages/types/src/StlFeatures.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/StlFeatures.ts:8 (sha256:62094796bb5c03e9694e5d105c0753f47dfd4d2076b8ca8e4cbde3f98f446df7)
pub type StlVariant = String;

// Source: upstream/packages/types/src/StlFeatures.ts:17 (sha256:b5a76bb7dfbdef61f162e5c26f97b15b0a99a181946f5a324cc347d2d1b0728b)
#[derive(Clone, Default)]
pub struct StlFeatures {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub triangle_count: f64,
    pub variant: StlVariant,
}
impl PartialEq for StlFeatures {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
