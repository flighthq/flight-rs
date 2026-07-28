// @generated from upstream/packages/types/src/AgxToneMapOptions.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/AgxToneMapOptions.ts:1 (sha256:05232ac34dd4349934a0f088c510d3ff4afaf9e0c95177de7d8970d2cdec04a2)
#[derive(Clone)]
pub struct AgxToneMapOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub min_ev: Option<f64>,
    pub max_ev: Option<f64>,
}
impl PartialEq for AgxToneMapOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
