// @generated from upstream/packages/render/src/renderCache.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{get_render_proxy_adapter, register_renderer, set_render_proxy_adapter};
use flighthq_entity::create_entity;
use flighthq_geometry::{create_matrix, multiply_matrix};
use flighthq_signals::create_signal;
use flighthq_types::{
    Adjustment, BlendMode, ColorTransform, DisplayObjectClipHooks, InteractionSignals, Material,
    MaterialData, Matrix, Node, NodeInteractionState, NodeSignals, NodeTraitsKey,
    RenderCacheAdapterSignals, RenderProxy2D, RenderState, Renderable, Renderer,
    SceneGraphSyncPolicy,
};
pub use flighthq_types::{RENDER_CACHE_KIND, RenderCache, RenderCacheAdapter};

#[derive(Clone)]
pub struct FlightPartialRecord1 {
    pub __flight_identity: std::sync::Arc<()>,
    pub binding: Option<crate::OpaqueHostValue>,
    pub appearance_id: Option<f64>,
    pub bounds_using_local_bounds_id: Option<f64>,
    pub bounds_using_local_transform_id: Option<f64>,
    pub can_add_child: Option<
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(Node, Node) -> bool + Send + 'static>>>,
    >,
    pub children: Option<Vec<Node>>,
    pub color_adjustments: Option<Vec<Adjustment>>,
    pub resolved_color_transform: Option<ColorTransform>,
    pub color_adjustments_channel_mixing: Option<bool>,
    pub traits: Option<NodeTraitsKey>,
    pub interaction_signals: Option<InteractionSignals>,
    pub local_bounds_id: Option<f64>,
    pub local_bounds_using_local_bounds_id: Option<f64>,
    pub local_content_id: Option<f64>,
    pub local_transform_id: Option<f64>,
    pub local_transform_using_local_transform_id: Option<f64>,
    pub node_signals: Option<NodeSignals>,
    pub interaction_state: Option<NodeInteractionState>,
    pub parent: Option<Node>,
    pub world_bounds_using_local_bounds_id: Option<f64>,
    pub world_bounds_using_world_transform_id: Option<f64>,
    pub world_transform_id: Option<f64>,
    pub world_transform_using_local_transform_id: Option<f64>,
    pub world_transform_using_parent_transform_id: Option<f64>,
}
impl PartialEq for FlightPartialRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone)]
pub struct FlightPartialRecord2 {
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
impl PartialEq for FlightPartialRecord2 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone)]
pub struct FlightPartialRecord3 {
    pub __flight_identity: std::sync::Arc<()>,
    pub material: Option<Material>,
    pub material_data: Option<MaterialData>,
}
impl PartialEq for FlightPartialRecord3 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/render/src/renderCache.ts:18 (sha256:15bc3d4f9c3522579206fcb8eba5bae4335150b9c611f59b34f893bb08ccc8b6)
pub fn create_render_cache() -> RenderCache {
    return create_entity(Some(RenderCache {
        __flight_identity: std::sync::Arc::new(()),
        kind: RENDER_CACHE_KIND,
        transform: create_matrix(None, None, None, None, None, None),
    }));
}

// Source: upstream/packages/render/src/renderCache.ts:27 (sha256:81774f8819df365c91d00ca1d01852138bebff64559a5b004036d7c110e41b76)
pub fn create_render_cache_adapter(cache: Option<RenderCache>) -> RenderCacheAdapter {
    let adapter: RenderCacheAdapter = RenderCacheAdapter {
        __flight_identity: std::sync::Arc::new(()),
        cache: (cache).clone(),
        signals: None,
        adapt: std::sync::Arc::new(std::sync::Mutex::new(Box::new({
            let adapter = adapter.clone();
            move |_state: RenderState,
                  _source: Renderable,
                  mut node: RenderProxy2D|
                  -> Option<bool> {
                (adapter
                    .signals
                    .as_ref()
                    .unwrap()
                    .on_prepare
                    .as_ref()
                    .map(|value| (value.emit).clone()))();
                let attached = (adapter.cache).clone();
                if (attached).is_none() {
                    return None;
                }
                node.kind = (RENDER_CACHE_KIND).to_owned();
                {
                    let __flight_argument_1 = (node.transform2_d).clone();
                    multiply_matrix(
                        &mut node.transform2_d,
                        &__flight_argument_1,
                        &attached.as_ref().unwrap().transform,
                    )
                };
                return Some(false);
            }
        })
            as Box<
                dyn FnMut(RenderState, Renderable, RenderProxy2D) -> Option<bool> + Send + 'static,
            >)),
    };
    return adapter;
}

// Source: upstream/packages/render/src/renderCache.ts:52 (sha256:1e84e6fdcbb63ee3df7cc41893b09fa1a345c648f7e3de91229e48cfecc61ba3)
pub fn enable_render_cache_adapter_signals(adapter: &mut RenderCacheAdapter) -> () {
    adapter.signals?? = Some(RenderCacheAdapterSignals {
        __flight_identity: std::sync::Arc::new(()),
        on_prepare: create_signal(),
    });
}

// Source: upstream/packages/render/src/renderCache.ts:60 (sha256:ea102352571a2280cd139e663503229090035a2ae044f000616d9aa2b2a7d249)
pub fn get_render_proxy_cache(state: &RenderState, source: &Renderable) -> Option<RenderCache> {
    let adapter = get_render_proxy_adapter(state, source);
    return if is_render_cache_adapter((adapter).clone().unwrap()) {
        Some(adapter.as_ref().unwrap().cache)
    } else {
        None
    };
}

// Source: upstream/packages/render/src/renderCache.ts:65 (sha256:b0a0bbc734fa7eb3db7af7019e279d872645fbdc178ec8c95c2d8229e8659b97)
pub fn is_render_cache(source: crate::OpaqueHostValue) -> bool {
    return ((match &(source) {
        crate::OpaqueHostValue::Undefined => "undefined",
        crate::OpaqueHostValue::Null | crate::OpaqueHostValue::Object => "object",
        crate::OpaqueHostValue::Bool(_) => "boolean",
        crate::OpaqueHostValue::Number(_) => "number",
        crate::OpaqueHostValue::String(_) => "string",
    } == "object")
        && ((source).is_some()))
        && ((source.kind).clone() == RENDER_CACHE_KIND);
}

// Source: upstream/packages/render/src/renderCache.ts:69 (sha256:216567bd53551c28473b89d338a46992c20150732618d319da67befa0d38b163)
pub fn is_render_cache_adapter(value: crate::OpaqueHostValue) -> bool {
    return (((match &(value) {
        crate::OpaqueHostValue::Undefined => "undefined",
        crate::OpaqueHostValue::Null | crate::OpaqueHostValue::Object => "object",
        crate::OpaqueHostValue::Bool(_) => "boolean",
        crate::OpaqueHostValue::Number(_) => "number",
        crate::OpaqueHostValue::String(_) => "string",
    } == "object")
        && ((value).is_some()))
        && ("function" == "function"))
        && (true);
}

// Source: upstream/packages/render/src/renderCache.ts:78 (sha256:8aef0238110d37a38b99c4b35262978d3867efcb7d9750ddb9ca96d106ca4bec)
pub fn register_render_cache_renderer(state: &RenderState, renderer: &Renderer) -> () {
    register_renderer(state, (RENDER_CACHE_KIND).to_owned(), renderer);
}

// Source: upstream/packages/render/src/renderCache.ts:87 (sha256:f87cf7011483d238f3c791ba37fc7211c42cd9b344ef9faa149c5fe233b8dabf)
pub fn use_render_cache(
    state: &RenderState,
    source: &Renderable,
    cache: &RenderCache,
) -> RenderCacheAdapter {
    let mut existing = get_render_proxy_adapter(state, source);
    if is_render_cache_adapter((existing).clone().unwrap()) {
        existing.as_mut().unwrap().cache = cache;
        return (existing).clone().unwrap();
    }
    let adapter = create_render_cache_adapter(Some(((*cache).clone()).clone()));
    set_render_proxy_adapter(state, source, Some((adapter).clone()));
    return adapter;
}
