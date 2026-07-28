// @generated from upstream/packages/render/src/explainDisplayObjectRender.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{get_render_proxy2_d, get_render_state_runtime};
use flighthq_types::{
    Adjustment, BlendMode, ColorTransform, DisplayObjectClipHooks, InteractionSignals, Kind,
    Material, MaterialData, Matrix, Node, NodeInteractionState, NodeSignals, NodeTraitsKey,
    RenderState, Renderable, SceneGraphSyncPolicy,
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

// Source: upstream/packages/render/src/explainDisplayObjectRender.ts:10 (sha256:3a24a8aa2b14a0155d209fd31e930b1ff8445fa77105537639d87c7c613d2f13)
#[derive(Clone)]
pub struct DisplayObjectRenderExplanation {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub kind: Kind,
    pub has_renderer: bool,
    pub prepared: bool,
    pub visible: bool,
    pub effective_alpha: f64,
    pub reason: DisplayObjectRenderBlankReason,
}
impl PartialEq for DisplayObjectRenderExplanation {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/render/src/explainDisplayObjectRender.ts:34 (sha256:d07d5486c0d8adc26d6f2e942ba8f02a5e9aaf309154518d7c2b6a0e7151cb49)
pub type DisplayObjectRenderBlankReason = String;

// Source: upstream/packages/render/src/explainDisplayObjectRender.ts:52 (sha256:2d836f9c7a4ed9f82616680bcfdddeed20f718cf94b0e629abab5f7aed888794)
pub fn explain_display_object_render(
    state: &RenderState,
    source: &Renderable,
) -> DisplayObjectRenderExplanation {
    let kind = source.kind;
    let has_renderer = (get_render_state_runtime(state)
        .renderer_map
        .iter()
        .find(|(key, _)| key == &kind)
        .map(|(_, value)| value.clone()))
    .is_some();
    let proxy = get_render_proxy2_d(state, source);
    let prepared = (proxy).is_some();
    let appearance = source;
    let visible = if (proxy).is_some() {
        proxy.as_ref().unwrap().visible
    } else {
        appearance.visible
    };
    let effective_alpha = if (proxy).is_some() {
        proxy.as_ref().unwrap().alpha
    } else {
        appearance.alpha
    };
    let mut reason: Option<DisplayObjectRenderBlankReason> = None;
    if (!has_renderer) {
        reason = Some("no-renderer".to_owned());
    } else {
        if (!prepared) {
            reason = Some("not-prepared".to_owned());
        } else {
            if (!visible) {
                reason = Some("not-visible".to_owned());
            } else {
                if (effective_alpha <= 0.0_f64) {
                    reason = Some("zero-alpha".to_owned());
                } else {
                    reason = Some("ok".to_owned());
                }
            }
        }
    }
    return DisplayObjectRenderExplanation {
        __flight_identity: std::sync::Arc::new(()),
        kind: kind,
        has_renderer: has_renderer,
        prepared: prepared,
        visible: visible,
        effective_alpha: effective_alpha,
        reason: (reason).clone().unwrap(),
    };
}
