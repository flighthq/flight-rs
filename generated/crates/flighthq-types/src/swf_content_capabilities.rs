// @generated from upstream/packages/types/src/SwfContentCapabilities.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/SwfContentCapabilities.ts:14 (sha256:89d54c04924e09c1d13a1b4bf328fb1df5af7502152cae447d91f50d374b82ff)
#[derive(Clone, Default)]
pub struct SwfContentCapabilities {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub tag_counts: Vec<(f64, f64)>,
    pub uses_blend_mode: bool,
    pub uses_filters: bool,
    pub uses_bitmap_fills: bool,
}
impl PartialEq for SwfContentCapabilities {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
