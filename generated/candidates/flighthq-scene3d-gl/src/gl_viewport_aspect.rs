// @generated from upstream/packages/scene3d-gl/src/glViewportAspect.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_render_gl::get_gl_render_state_runtime;
use flighthq_types::{BlendMode, GlRenderState, Matrix, Scene2DClipHooks, Scene3DGraphSyncPolicy};

#[derive(Clone, Default)]
pub struct FlightPartialRecord1 {
    pub __flight_identity: std::sync::Arc<()>,
    pub allow_smoothing: Option<bool>,
    pub background_color: Option<f64>,
    pub background_color_rgba: Option<Vec<f64>>,
    pub background_color_string: Option<String>,
    pub current_clip_depth: Option<f64>,
    pub display_object_clip_hooks: Option<Scene2DClipHooks>,
    pub pixel_ratio: Option<f64>,
    pub render_alpha: Option<f64>,
    pub render_blend_mode: Option<BlendMode>,
    pub render_transform2_d: Option<Matrix>,
    pub scene_graph_sync_policy: Option<Scene3DGraphSyncPolicy>,
    pub round_pixels: Option<bool>,
}
impl PartialEq for FlightPartialRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/scene3d-gl/src/glViewportAspect.ts:6 (sha256:a359770763df691d1893828a3dd60e16d9a7505987e0919812ac6693b907e63b)
pub fn get_gl_scene3_d_viewport_aspect(state: &GlRenderState) -> f64 {
    let viewport = ((get_gl_render_state_runtime(state)
        .inner
        .lock()
        .unwrap()
        .gl_render_state_runtime
        .render_target_viewport)
        .clone())
    .unwrap_or((state.canvas).clone());
    return if (viewport.width > 0.0_f64) && (viewport.height > 0.0_f64) {
        (viewport.width / viewport.height)
    } else {
        1.0_f64
    };
}
