// @generated from upstream/packages/types/src/CanvasShapeDrawState.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::Matrix;

// Source: upstream/packages/types/src/CanvasShapeDrawState.ts:3 (sha256:111455f0a80c8e3621d07bd7a906d671391ba9906e08fd916931c47d961c4cfb)
#[derive(Clone)]
pub struct CanvasShapeDrawState {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub has_fill: bool,
    pub fill_style: crate::FlightUnion2<String, crate::OpaqueHostValue>,
    pub fill_matrix: Option<Matrix>,
    pub fill_matrix_inverse: Option<Matrix>,
    pub has_stroke: bool,
    pub stroke_style: crate::FlightUnion2<String, crate::OpaqueHostValue>,
    pub stroke_width: f64,
    pub has_pending_path: bool,
    pub has_current_point: bool,
    pub winding_rule: crate::OpaqueHostValue,
    pub bitmap_src: Option<crate::OpaqueHostValue>,
    pub bitmap_w: f64,
    pub bitmap_h: f64,
    pub flush: std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>,
}
impl PartialEq for CanvasShapeDrawState {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
