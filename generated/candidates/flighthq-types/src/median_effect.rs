// @generated from upstream/packages/types/src/MedianEffect.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/MedianEffect.ts:6 (sha256:b86b3761a9d86faa5aed9c9f0ebbab702ef27c1d92fa29e0683abba81217a7a6)
#[derive(Clone)]
pub struct MedianEffect {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub kind: String,
    pub radius: Option<f64>,
}
impl PartialEq for MedianEffect {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
