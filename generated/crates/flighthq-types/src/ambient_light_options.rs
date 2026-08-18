// @generated from upstream/packages/types/src/AmbientLightOptions.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/AmbientLightOptions.ts:1 (sha256:327e7902f53591c7b64ef38c05213c1a455523299f0a1843b732f97800f973b9)
#[derive(Clone, Default)]
pub struct AmbientLightOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub color: Option<f64>,
    pub intensity: Option<f64>,
}
impl PartialEq for AmbientLightOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
