// @generated from upstream/packages/types/src/CurlNoiseForce.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/CurlNoiseForce.ts:1 (sha256:89b594bb0f1664166cf9aff7b19dc58ab01f4cc2fe2171e2a343c204870ab4a7)
#[derive(Clone)]
pub struct CurlNoiseForce {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub kind: String,
    pub scale: f64,
    pub strength: f64,
    pub time: Option<f64>,
}
impl PartialEq for CurlNoiseForce {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/CurlNoiseForce.ts:11 (sha256:4074b9a41928bbf0bd03aab83041ee7669404a1c00831845342131bb2279d8b7)
pub const CURL_NOISE_FORCE_KIND: &'static str = "CurlNoiseForce";
