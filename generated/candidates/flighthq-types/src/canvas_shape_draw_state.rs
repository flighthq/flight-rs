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
    pub has_fill: bool,
    pub fill_style: crate::OpaqueHostValue,
    pub fill_matrix: Option<Matrix>,
    pub fill_matrix_inverse: Option<Matrix>,
    pub has_stroke: bool,
    pub stroke_style: crate::OpaqueHostValue,
    pub stroke_width: f64,
    pub has_pending_path: bool,
    pub has_current_point: bool,
    pub winding_rule: crate::OpaqueHostValue,
    pub bitmap_src: Option<crate::OpaqueHostValue>,
    pub bitmap_w: f64,
    pub bitmap_h: f64,
    pub flush: std::sync::Arc<dyn Fn() -> () + Send + Sync + 'static>,
}
