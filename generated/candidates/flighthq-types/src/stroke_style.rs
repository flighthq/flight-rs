// @generated from upstream/packages/types/src/StrokeStyle.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/StrokeStyle.ts:1 (sha256:ec1ee1a0110859d8a51e6fef7add0524114a018e5ce4bb456d8f0d1707c3d278)
#[derive(Clone, Default)]
pub struct StrokeStyle {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub cap: Option<String>,
    pub dash: Option<Vec<f64>>,
    pub dash_offset: Option<f64>,
    pub join: Option<String>,
    pub miter_limit: Option<f64>,
    pub width: Option<f64>,
}
impl PartialEq for StrokeStyle {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
