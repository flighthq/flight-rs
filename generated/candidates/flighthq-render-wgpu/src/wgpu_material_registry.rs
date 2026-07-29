// @generated from upstream/packages/render-wgpu/src/wgpuMaterialRegistry.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::get_wgpu_render_state_runtime;
use flighthq_types::{
    BlendMode, ColorTransform, DEFAULT_MATERIAL_KIND as default_material_kind_constant,
    DisplayObjectClipHooks, Kind, Material, Matrix, SceneGraphSyncPolicy, WgpuMaterialRenderer,
    WgpuRenderState,
};

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

#[derive(Clone, Default)]
pub struct FlightPartialRecord2 {
    pub __flight_identity: std::sync::Arc<()>,
    pub color_transform: Option<ColorTransform>,
}
impl PartialEq for FlightPartialRecord2 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/render-wgpu/src/wgpuMaterialRegistry.ts:6 (sha256:b7a6eb64938bf9fd51375f2368df7cb1d2ae3568bdd4892bb49c49c7ecf7835b)
pub fn get_wgpu_material_renderer(
    state: &WgpuRenderState,
    kind: Kind,
) -> Option<WgpuMaterialRenderer> {
    let runtime = get_wgpu_render_state_runtime(state);
    return runtime
        .inner
        .lock()
        .unwrap()
        .wgpu_render_state_runtime
        .material_renderer_map
        .as_mut()
        .unwrap()
        .iter()
        .find(|(key, _)| key == &(kind).clone())
        .map(|(_, value)| value.clone());
}

// Source: upstream/packages/render-wgpu/src/wgpuMaterialRegistry.ts:11 (sha256:1dafab0ff27e45efed35ace17ba59d2f7b2b8189916cfcdac42d2b904718e74f)
pub fn register_wgpu_material_renderer(
    state: &WgpuRenderState,
    kind: Kind,
    renderer: &WgpuMaterialRenderer,
) -> () {
    let mut runtime = get_wgpu_render_state_runtime(state);
    ({
        let __flight_runtime = runtime;
        let __flight_value = Some(Vec::new());
        let mut __flight_storage = __flight_runtime.inner.lock().unwrap();
        __flight_storage
            .wgpu_render_state_runtime
            .material_renderer_map?? = __flight_value;
        __flight_storage
            .wgpu_render_state_runtime
            .material_renderer_map
            .clone()
    }
    .set)(kind, renderer);
}

// Source: upstream/packages/render-wgpu/src/wgpuMaterialRegistry.ts:20 (sha256:44fc155d542ae59dc926d8f1ff570bd02063258e1202f71ef2d4a5c8d91ad9ac)
pub fn resolve_wgpu_material_renderer(
    state: &WgpuRenderState,
    material: Option<Material>,
) -> Option<WgpuMaterialRenderer> {
    let runtime = get_wgpu_render_state_runtime(state);
    let map = (runtime
        .inner
        .lock()
        .unwrap()
        .wgpu_render_state_runtime
        .material_renderer_map)
        .clone();
    if (map).is_none() {
        return None;
    }
    if (material).is_some() {
        let renderer = map
            .as_ref()
            .unwrap()
            .iter()
            .find(|(key, _)| key == &(material.as_ref().unwrap().kind).clone())
            .map(|(_, value)| value.clone());
        if (renderer).is_some() {
            return Some((renderer.as_ref().unwrap()).clone());
        }
    }
    return map
        .as_ref()
        .unwrap()
        .iter()
        .find(|(key, _)| key == &(default_material_kind_constant).to_owned())
        .map(|(_, value)| value.clone());
}
