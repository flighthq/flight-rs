// @generated from upstream/packages/types/src/WebcamConstraints.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/WebcamConstraints.ts:1 (sha256:7eb44a726269aabe8168241ccb3838d727d91d974673cb660d0d612ba2fcf4cc)
#[derive(Clone, Default)]
pub struct WebcamConstraints {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub exposure_compensation: Option<f64>,
    pub exposure_mode: Option<String>,
    pub focus_distance: Option<f64>,
    pub focus_mode: Option<String>,
    pub torch: Option<bool>,
    pub white_balance_mode: Option<String>,
    pub zoom: Option<f64>,
}
impl PartialEq for WebcamConstraints {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
