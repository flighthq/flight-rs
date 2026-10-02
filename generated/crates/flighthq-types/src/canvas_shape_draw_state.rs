// @generated from upstream/packages/types/src/CanvasShapeDrawState.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{CanvasTextureResolvers, LineScaleMode, Matrix};

// Source: upstream/packages/types/src/CanvasShapeDrawState.ts:5 (sha256:2c020a5e63911c5e9dc760bd9d9ee9f671211d47afe2debb824e6f1f49ea5eb5)
#[derive(Clone)]
pub struct CanvasShapeDrawState {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub canvas_texture_resolvers: CanvasTextureResolvers,
    pub allow_smoothing: bool,
    pub has_fill: bool,
    pub fill_style: crate::FlightUnion2<String, crate::OpaqueHostValue>,
    pub fill_matrix: Option<Matrix>,
    pub fill_matrix_inverse: Option<Matrix>,
    pub has_stroke: bool,
    pub line_scale_mode: LineScaleMode,
    pub stroke_style: crate::FlightUnion2<String, crate::OpaqueHostValue>,
    pub stroke_width: f64,
    pub current_x: f64,
    pub current_y: f64,
    pub has_pending_path: bool,
    pub has_current_point: bool,
    pub subpath_start_x: f64,
    pub subpath_start_y: f64,
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
