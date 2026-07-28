// @generated from upstream/packages/scene/src/sceneNode.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_node::{
    create_node, create_node_runtime, enable_node_signals, get_node_runtime, get_node_signals,
    init_appearance_trait, init_transform3_d_runtime_trait, init_transform3_d_trait,
};
use flighthq_types::{
    Adjustment, ColorTransform, InteractionSignals, Kind, Node, NodeData, NodeInteractionState,
    NodeSignals, NodeTraitsKey, Quaternion,
    SCENE_NODE_TRAITS_KEY as scene_node_traits_key_constant, Vector3,
};
pub use flighthq_types::{SCENE_NODE_KIND, SceneNode, SceneNodeRuntime, SceneNodeTraits};

#[derive(Clone)]
pub struct FlightPartialRecord1 {
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
impl PartialEq for FlightPartialRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone)]
pub struct FlightPartialRecord2 {
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

// Source: upstream/packages/scene/src/sceneNode.ts:17 (sha256:63226bd7e603e1382a282e143d894ddef46a663b327e255b2bce15010dbbd5a6)
pub fn create_scene_node(kind: Option<Kind>, obj: Option<FlightPartialRecord1>) -> SceneNode {
    let kind = kind.unwrap_or((SCENE_NODE_KIND).to_owned());
    let mut node = create_node(
        (kind).clone(),
        Some(((obj).clone().unwrap()).clone()),
        Some(undefined),
        Some(std::sync::Arc::new(std::sync::Mutex::new(Box::new(
            move |__flight_argument_0: Option<R>| -> R { create_scene_node_runtime() },
        )
            as Box<dyn FnMut(Option<R>) -> R + Send + 'static>))),
    );
    init_appearance_trait(&mut node, Some(((obj).clone().unwrap()).clone()));
    init_transform3_d_trait(&mut node, None);
    return node;
}

// Source: upstream/packages/scene/src/sceneNode.ts:27 (sha256:05626d2482d96e38657ad413f0f100b1d8aa907d6ec95e29d08bb82df8971097)
pub fn create_scene_node_runtime() -> SceneNodeRuntime {
    let mut out = create_node_runtime(None);
    out.traits = Some(scene_node_traits_key_constant);
    out.world_alpha = None;
    out.world_alpha_using_appearance_id = (-1.0_f64);
    out.world_alpha_using_parent_appearance_id = (-1.0_f64);
    out.world_appearance_id = 0.0_f64;
    init_transform3_d_runtime_trait(&mut out);
    return out;
}

// Source: upstream/packages/scene/src/sceneNode.ts:38 (sha256:79451d392d0d20a823df24637068db86a986ceca476c01d14fded4205cda79a1)
pub fn enable_scene_node_signals(source: &SceneNode) -> NodeSignals {
    return enable_node_signals(&Node {
        __flight_identity: std::sync::Arc::clone(&(source).__flight_identity),
        data: ((source).data).clone(),
        enabled: (source).enabled,
        kind: ((source).kind).clone(),
        name: ((source).name).clone(),
    });
}

// Source: upstream/packages/scene/src/sceneNode.ts:42 (sha256:dba286523d03c3be126b782754ac4af20e855dc47ba27121f6aa81a335a2299c)
pub fn get_scene_node_runtime(source: &SceneNode) -> SceneNodeRuntime {
    return get_node_runtime(&Node {
        __flight_identity: std::sync::Arc::clone(&(source).__flight_identity),
        data: ((source).data).clone(),
        enabled: (source).enabled,
        kind: ((source).kind).clone(),
        name: ((source).name).clone(),
    });
}

// Source: upstream/packages/scene/src/sceneNode.ts:46 (sha256:34a017800aae991e1ed4494dedde801a41f52d1e588f074c87cac81b89daca82)
pub fn get_scene_node_signals(source: &SceneNode) -> Option<NodeSignals> {
    return get_node_signals(&Node {
        __flight_identity: std::sync::Arc::clone(&(source).__flight_identity),
        data: ((source).data).clone(),
        enabled: (source).enabled,
        kind: ((source).kind).clone(),
        name: ((source).name).clone(),
    });
}
