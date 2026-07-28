// @generated from upstream/packages/scene/src/billboard.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{create_scene_node, get_scene_node_runtime};
use flighthq_node::{enable_node_signals, get_node_signals};
use flighthq_types::{
    Adjustment, ColorTransform, InteractionSignals, Kind, Material, MeshGeometry, Node, NodeData,
    NodeInteractionState, NodeSignals, NodeTraitsKey, Quaternion, SceneNode, Vector3,
};
pub use flighthq_types::{BILLBOARD_KIND, Billboard, BillboardMode, BillboardRuntime};

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
    pub geometry: Option<MeshGeometry>,
    pub materials: Option<Vec<Option<Material>>>,
    pub mode: Option<BillboardMode>,
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
impl PartialEq for FlightPartialRecord3 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone)]
pub struct FlightPartialRecord4 {
    pub __flight_identity: std::sync::Arc<()>,
    pub alpha: Option<f64>,
    pub visible: Option<bool>,
}
impl PartialEq for FlightPartialRecord4 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone)]
pub struct FlightPartialRecord5 {
    pub __flight_identity: std::sync::Arc<()>,
    pub position: Option<Vector3>,
    pub rotation: Option<Quaternion>,
    pub scale: Option<Vector3>,
}
impl PartialEq for FlightPartialRecord5 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/scene/src/billboard.ts:32 (sha256:a91e0b104fe82e4b0509ebff9f96b5de5916a22ac4be7b71686a310a068f2cda)
pub fn create_billboard(
    geometry: &MeshGeometry,
    materials: &Vec<Option<Material>>,
    mode: Option<BillboardMode>,
    kind: Option<Kind>,
    obj: Option<FlightPartialRecord1>,
) -> Billboard {
    let mode = mode.unwrap_or("full".to_owned());
    let kind = kind.unwrap_or((BILLBOARD_KIND).to_owned());
    let mut billboard = create_scene_node(
        Some(((kind).clone()).clone()),
        Some(((obj).clone().unwrap()).clone()),
    );
    billboard.geometry = (*geometry).clone();
    billboard.materials = (*materials).clone();
    billboard.mode = (mode).clone();
    return billboard;
}

// Source: upstream/packages/scene/src/billboard.ts:46 (sha256:ad62048d6887b1083246a862661fe8bc0eb2a769c4851f08185965764da8607d)
pub fn enable_billboard_signals(source: &Billboard) -> NodeSignals {
    return enable_node_signals(&Node {
        __flight_identity: std::sync::Arc::clone(&(source).__flight_identity),
        data: ((source).data).clone(),
        enabled: (source).enabled,
        kind: ((source).kind).clone(),
        name: ((source).name).clone(),
    });
}

// Source: upstream/packages/scene/src/billboard.ts:50 (sha256:b5ec90820db4c13ab3fe94f7cc74a6606d363091821d191b85f5501481255efc)
pub fn get_billboard_runtime(source: &Billboard) -> BillboardRuntime {
    return get_scene_node_runtime(&SceneNode {
        __flight_identity: std::sync::Arc::clone(&(source).__flight_identity),
        data: ((source).data).clone(),
        enabled: (source).enabled,
        kind: ((source).kind).clone(),
        name: ((source).name).clone(),
        alpha: (source).alpha,
        visible: (source).visible,
        position: ((source).position).clone(),
        rotation: ((source).rotation).clone(),
        scale: ((source).scale).clone(),
    });
}

// Source: upstream/packages/scene/src/billboard.ts:54 (sha256:7f4344bb9e2aee03de90f232d573ff202b13758de25c0363809cd6c3a07277c3)
pub fn get_billboard_signals(source: &Billboard) -> Option<NodeSignals> {
    return get_node_signals(&Node {
        __flight_identity: std::sync::Arc::clone(&(source).__flight_identity),
        data: ((source).data).clone(),
        enabled: (source).enabled,
        kind: ((source).kind).clone(),
        name: ((source).name).clone(),
    });
}

// Source: upstream/packages/scene/src/billboard.ts:61 (sha256:2df21ee89214648dc557fbf93bcf0025abe505634be62a03350b14933e694dd3)
pub fn is_billboard(source: &SceneNode) -> bool {
    let candidate = source;
    return (((candidate.geometry).clone()).is_some()) && (((candidate.mode).clone()).is_some());
}
