// @generated from upstream/packages/scene/src/sceneNodeAppearance.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::get_scene_node_runtime;
use flighthq_node::{get_node_appearance_revision, invalidate_node_appearance};
use flighthq_types::{
    Adjustment, ColorTransform, InteractionSignals, Kind, Node, NodeData, NodeInteractionState,
    NodeSignals, NodeTraitsKey, Quaternion, SceneNode, Vector3,
};

#[inline]
fn __flight_js_to_u32(value: f64) -> u32 {
    if !value.is_finite() || value == 0.0 {
        return 0;
    }
    value.trunc().rem_euclid(4294967296.0_f64) as u32
}

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
    pub data: Option<NodeData>,
    pub enabled: Option<bool>,
    pub kind: Option<Kind>,
    pub name: Option<String>,
    pub alpha: Option<f64>,
    pub visible: Option<bool>,
    pub position: Option<Vector3>,
    pub rotation: Option<Quaternion>,
    pub scale: Option<Vector3>,
}
impl PartialEq for FlightPartialRecord2 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone)]
pub struct FlightPartialRecord3 {
    pub __flight_identity: std::sync::Arc<()>,
    pub alpha: Option<f64>,
    pub visible: Option<bool>,
}
impl PartialEq for FlightPartialRecord3 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone)]
pub struct FlightPartialRecord4 {
    pub __flight_identity: std::sync::Arc<()>,
    pub position: Option<Vector3>,
    pub rotation: Option<Quaternion>,
    pub scale: Option<Vector3>,
}
impl PartialEq for FlightPartialRecord4 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/scene/src/sceneNodeAppearance.ts:10 (sha256:d707396363d5f5ea5dfe5eacd514358fca1e581f3c97d7e766b7941c29418fd4)
pub fn ensure_scene_node_world_alpha(source: &SceneNode) -> () {
    let mut runtime = get_scene_node_runtime(source);
    let parent = (runtime.parent).clone();
    let mut parent_world_alpha = 1.0_f64;
    let mut parent_world_appearance_id = 0.0_f64;
    if (parent).is_some() {
        ensure_scene_node_world_alpha(&parent.as_ref().unwrap());
        let parent_runtime = get_scene_node_runtime(&parent.as_ref().unwrap());
        parent_world_alpha = (parent_runtime.world_alpha).unwrap();
        parent_world_appearance_id = parent_runtime.world_appearance_id;
    }
    let appearance_id = get_node_appearance_revision(&Node {
        __flight_identity: std::sync::Arc::clone(&(source).__flight_identity),
        data: ((source).data).clone(),
        enabled: (source).enabled,
        kind: ((source).kind).clone(),
        name: ((source).name).clone(),
    });
    if (((runtime.world_alpha).is_none())
        || (runtime.world_alpha_using_appearance_id != appearance_id))
        || (runtime.world_alpha_using_parent_appearance_id != parent_world_appearance_id)
    {
        runtime.world_alpha = Some((parent_world_alpha * source.alpha));
        runtime.world_alpha_using_appearance_id = appearance_id;
        runtime.world_alpha_using_parent_appearance_id = parent_world_appearance_id;
        (*_WORLD_APPEARANCE_REVISION_COUNTER.lock().unwrap()) = (__flight_js_to_u32(
            ((*_WORLD_APPEARANCE_REVISION_COUNTER.lock().unwrap()).clone() + 1.0_f64),
        ) >> (__flight_js_to_u32(0.0_f64)
            & 31)) as f64;
        if ((*_WORLD_APPEARANCE_REVISION_COUNTER.lock().unwrap()).clone() == 0.0_f64) {
            (*_WORLD_APPEARANCE_REVISION_COUNTER.lock().unwrap()) = 1.0_f64;
        }
        runtime.world_appearance_id = (*_WORLD_APPEARANCE_REVISION_COUNTER.lock().unwrap()).clone();
    }
}

// Source: upstream/packages/scene/src/sceneNodeAppearance.ts:43 (sha256:9846a347a1ba8ea371f70760324e51470f8b449975907728697b05e13cef73be)
pub fn get_scene_node_world_alpha(source: &SceneNode) -> f64 {
    ensure_scene_node_world_alpha(source);
    return (get_scene_node_runtime(source).world_alpha).unwrap_or(1.0_f64);
}

// Source: upstream/packages/scene/src/sceneNodeAppearance.ts:51 (sha256:93a8b55e16993cd78df0d2d4522dc27b41c76caa39ecf77e121571ef67685df9)
pub fn set_scene_node_alpha(source: &mut SceneNode, alpha: f64) -> () {
    source.alpha = alpha;
    invalidate_node_appearance(&Node {
        __flight_identity: std::sync::Arc::clone(&(source).__flight_identity),
        data: ((source).data).clone(),
        enabled: (source).enabled,
        kind: ((source).kind).clone(),
        name: ((source).name).clone(),
    });
}

// Source: upstream/packages/scene/src/sceneNodeAppearance.ts:58 (sha256:f6439f3f62b61cf710dcc52f37dd2a7cd0333a471624dc3f07e562f4e694a5dd)
static _WORLD_APPEARANCE_REVISION_COUNTER: std::sync::LazyLock<std::sync::Mutex<f64>> =
    std::sync::LazyLock::new(|| std::sync::Mutex::new(0.0_f64));
