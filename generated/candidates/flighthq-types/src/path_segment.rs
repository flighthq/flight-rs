// @generated from upstream/packages/types/src/PathSegment.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/PathSegment.ts:12 (sha256:db16b55842026cbb291940c65ffaefa52646bfcbdbc43d08be6790754e4c8b20)
#[derive(Clone)]
pub struct PathSegment {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub kind: String,
    pub x: Option<f64>,
    pub y: Option<f64>,
    pub control_x: Option<f64>,
    pub control_y: Option<f64>,
    pub control1_x: Option<f64>,
    pub control1_y: Option<f64>,
    pub control2_x: Option<f64>,
    pub control2_y: Option<f64>,
}
impl PartialEq for PathSegment {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
