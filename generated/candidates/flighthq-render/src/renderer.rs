// @generated from upstream/packages/render/src/renderer.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::get_render_state_runtime;
use flighthq_types::{
    BlendMode, DisplayObjectClipHooks, Kind, Matrix, RenderState, Renderable, Renderer,
    RendererData, SceneGraphSyncPolicy,
};

#[inline]
fn __flight_js_to_u32(value: f64) -> u32 {
    if !value.is_finite() || value == 0.0 {
        return 0;
    }
    value.trunc().rem_euclid(4294967296.0_f64) as u32
}

#[derive(Clone, Default)]
pub struct FlightPartialRecord1 {
    pub __flight_identity: std::sync::Arc<()>,
    pub allow_smoothing: Option<bool>,
    pub background_color: Option<f64>,
    pub background_color_rgba: Option<Vec<f64>>,
    pub background_color_string: Option<String>,
    pub current_clip_depth: Option<f64>,
    pub display_object_clip_hooks: Option<DisplayObjectClipHooks>,
    pub pixel_ratio: Option<f64>,
    pub render_alpha: Option<f64>,
    pub render_blend_mode: Option<BlendMode>,
    pub render_transform2_d: Option<Matrix>,
    pub scene_graph_sync_policy: Option<SceneGraphSyncPolicy>,
    pub round_pixels: Option<bool>,
}
impl PartialEq for FlightPartialRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/render/src/renderer.ts:7 (sha256:0599b7b2f4212b2aaf68f71cac956d3fa39b2ed817e3c6482ead6f4a79119606)
pub fn copy_all_renderers_from_render_state(target: &mut RenderState, source: &RenderState) -> () {
    copy_renderers_from_render_state((target).clone(), source);
    if ((source.display_object_clip_hooks).clone()).is_some() {
        target.display_object_clip_hooks = (source.display_object_clip_hooks).clone();
    }
}

// Source: upstream/packages/render/src/renderer.ts:12 (sha256:e7d1e50af268b2182a57fe107b2673ac9960946117dfe5a491a72dd40f3ff57a)
pub fn copy_renderers_from_render_state(target: RenderState, source: &RenderState) -> () {
    (get_render_state_runtime(source)
        .renderer_map
        .iter()
        .find(|(key, _)| key == &"forEach".to_owned())
        .map(|(_, value)| value.clone())
        .expect("TypeScript Record key was absent"))(std::sync::Arc::new(
        std::sync::Mutex::new(Box::new({
            let target = target.clone();
            move |renderer: crate::OpaqueHostValue, kind: crate::OpaqueHostValue| -> () {
                register_renderer(&target, (kind).clone(), &renderer);
            }
        })
            as Box<
                dyn FnMut(crate::OpaqueHostValue, crate::OpaqueHostValue) -> () + Send + 'static,
            >),
    ));
}

// Source: upstream/packages/render/src/renderer.ts:18 (sha256:f001e2b6917fc587d5a5a5628ec50cbb7368da55b1401b88426286f758f6d3ee)
pub fn noop_renderer_data(_state: &RenderState, _source: &Renderable) -> Option<RendererData> {
    return None;
}

// Source: upstream/packages/render/src/renderer.ts:22 (sha256:e3e87b076cba707aa9c4fd724c2d507318f6c1ddbf9ccc5edfd31592a63cae9a)
pub fn register_renderer(state: &RenderState, kind: Kind, renderer: &Renderer) -> () {
    let mut runtime = get_render_state_runtime(state);
    if (runtime
        .renderer_map
        .iter()
        .find(|(key, _)| key == &(kind).clone())
        .map(|(_, value)| value.clone()))
        == Some((*renderer).clone())
    {
        return;
    }
    runtime.renderer_map_id = (__flight_js_to_u32((runtime.renderer_map_id + 1.0_f64))
        >> (__flight_js_to_u32(0.0_f64) & 31)) as f64;
    {
        let __flight_key = (kind).clone();
        let __flight_value = (*renderer).clone();
        if let Some((_, value)) = runtime
            .renderer_map
            .iter_mut()
            .find(|(key, _)| key == &__flight_key)
        {
            *value = __flight_value;
        } else {
            runtime.renderer_map.push((__flight_key, __flight_value));
        }
    };
}
