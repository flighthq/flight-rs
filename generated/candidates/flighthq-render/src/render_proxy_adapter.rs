// @generated from upstream/packages/render/src/renderProxyAdapter.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{get_render_state_runtime, install_render_adapt_hook, update_render_proxy_renderer};
use flighthq_node::invalidate_node_appearance;
use flighthq_types::{
    Adjustment, BlendMode, ColorTransform, DisplayObjectClipHooks, InteractionSignals, Material,
    MaterialData, Matrix, Node, NodeInteractionState, NodeSignals, NodeTraitsKey, RenderProxy2D,
    RenderProxyAdapter, RenderState, Renderable, SceneGraphSyncPolicy,
};

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

// Source: upstream/packages/render/src/renderProxyAdapter.ts:7 (sha256:530d65dcefc2ec1b91a7b5ba71ebfb9db7e1bff73c7f86fdaaac92f0bc17d6c2)
#[derive(Clone)]
struct ApplyRenderProxyAdapterRecord4 {
    __flight_identity: std::sync::Arc<()>,
    traverse_children: bool,
}
impl PartialEq for ApplyRenderProxyAdapterRecord4 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn apply_render_proxy_adapter(
    state: &RenderState,
    source: &Renderable,
    data: &mut ApplyRenderProxyAdapterRecord4,
) -> () {
    let render_adapter = get_render_state_runtime(state)
        .render_proxy_adapter_map
        .iter()
        .find(|(key, _)| key == &(*source).clone())
        .map(|(_, value)| value.clone());
    let mut traverse_children = true;
    if (render_adapter).is_some() {
        let result = {
            let __flight_callback = (render_adapter.as_ref().unwrap().adapt).clone();
            let __flight_result =
                __flight_callback.lock().unwrap()((*state).clone(), (*source).clone(), data);
            __flight_result
        };
        if (result).is_some() {
            traverse_children = *(result.as_ref().unwrap());
            update_render_proxy_renderer(state, data);
        }
    }
    data.traverse_children = traverse_children;
}

// Source: upstream/packages/render/src/renderProxyAdapter.ts:24 (sha256:d396f28bf6281fb87842ad414cd5530ecb213426553b4ab1570653fca44de6fa)
pub fn get_render_proxy_adapter(
    state: &RenderState,
    source: &Renderable,
) -> Option<RenderProxyAdapter> {
    return get_render_state_runtime(state)
        .render_proxy_adapter_map
        .iter()
        .find(|(key, _)| key == &(*source).clone())
        .map(|(_, value)| value.clone());
}

// Source: upstream/packages/render/src/renderProxyAdapter.ts:28 (sha256:78929be4bb073022ebe2aff4aa47002fe327dacfd9dffadb3479bcf438bf399c)
pub fn set_render_proxy_adapter(
    state: &RenderState,
    source: &Renderable,
    adapter: Option<RenderProxyAdapter>,
) -> () {
    if ((get_render_state_runtime(state).render_adapt_hook).clone() != apply_render_proxy_adapter) {
        install_render_adapt_hook(
            state,
            std::sync::Arc::new(std::sync::Mutex::new(Box::new(
                move |__flight_argument_0: RenderState,
                      __flight_argument_1: Renderable,
                      mut __flight_argument_2: RenderProxy2D|
                      -> () {
                    apply_render_proxy_adapter(
                        &__flight_argument_0,
                        &__flight_argument_1,
                        &mut __flight_argument_2,
                    )
                },
            )
                as Box<
                    dyn FnMut(RenderState, Renderable, RenderProxy2D) -> () + Send + 'static,
                >)),
        );
    }
    let mut runtime = get_render_state_runtime(state);
    if (adapter).is_none() {
        {
            let __flight_key = (*source).clone();
            if let Some(__flight_index) = runtime
                .render_proxy_adapter_map
                .iter()
                .position(|(key, _)| key == &__flight_key)
            {
                runtime.render_proxy_adapter_map.remove(__flight_index);
                true
            } else {
                false
            }
        };
    } else {
        {
            let __flight_key = (*source).clone();
            let __flight_value = (adapter.as_ref().unwrap()).clone();
            if let Some((_, value)) = runtime
                .render_proxy_adapter_map
                .iter_mut()
                .find(|(key, _)| key == &__flight_key)
            {
                *value = __flight_value;
            } else {
                runtime
                    .render_proxy_adapter_map
                    .push((__flight_key, __flight_value));
            }
        };
    }
    invalidate_node_appearance(&source);
}
