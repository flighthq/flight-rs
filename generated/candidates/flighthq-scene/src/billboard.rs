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

#[derive(Clone, Default)]
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

#[derive(Clone, Default)]
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

#[derive(Clone, Default)]
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

#[derive(Clone, Default)]
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

#[derive(Clone, Default)]
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
    return enable_node_signals(&{
        let __flight_source = &(source);
        Node {
            __flight_identity: std::sync::Arc::clone(&__flight_source.__flight_identity),
            data: (__flight_source.data).clone(),
            enabled: __flight_source.enabled,
            kind: (__flight_source.kind).clone(),
            name: (__flight_source.name).clone(),
        }
    });
}

// Source: upstream/packages/scene/src/billboard.ts:50 (sha256:b5ec90820db4c13ab3fe94f7cc74a6606d363091821d191b85f5501481255efc)
pub fn get_billboard_runtime(source: &Billboard) -> BillboardRuntime {
    return {
        let __flight_source = &(get_scene_node_runtime(&{
            let __flight_source = &(source);
            SceneNode {
                __flight_identity: std::sync::Arc::clone(&__flight_source.__flight_identity),
                data: (__flight_source.data).clone(),
                enabled: __flight_source.enabled,
                kind: (__flight_source.kind).clone(),
                name: (__flight_source.name).clone(),
                alpha: __flight_source.alpha,
                visible: __flight_source.visible,
                position: (__flight_source.position).clone(),
                rotation: (__flight_source.rotation).clone(),
                scale: (__flight_source.scale).clone(),
            }
        }));
        BillboardRuntime {
            __flight_identity: std::sync::Arc::clone(&__flight_source.__flight_identity),
            binding: (__flight_source.binding).clone(),
            appearance_id: __flight_source.appearance_id,
            bounds_using_local_bounds_id: __flight_source.bounds_using_local_bounds_id,
            bounds_using_local_transform_id: __flight_source.bounds_using_local_transform_id,
            can_add_child: (__flight_source.can_add_child).clone(),
            children: (__flight_source.children).clone(),
            color_adjustments: (__flight_source.color_adjustments).clone(),
            resolved_color_transform: (__flight_source.resolved_color_transform).clone(),
            color_adjustments_channel_mixing: __flight_source.color_adjustments_channel_mixing,
            traits: (__flight_source.traits).clone(),
            interaction_signals: (__flight_source.interaction_signals).clone(),
            local_bounds_id: __flight_source.local_bounds_id,
            local_bounds_using_local_bounds_id: __flight_source.local_bounds_using_local_bounds_id,
            local_content_id: __flight_source.local_content_id,
            local_transform_id: __flight_source.local_transform_id,
            local_transform_using_local_transform_id: __flight_source
                .local_transform_using_local_transform_id,
            node_signals: (__flight_source.node_signals).clone(),
            interaction_state: (__flight_source.interaction_state).clone(),
            parent: (__flight_source.parent).clone(),
            world_bounds_using_local_bounds_id: __flight_source.world_bounds_using_local_bounds_id,
            world_bounds_using_world_transform_id: __flight_source
                .world_bounds_using_world_transform_id,
            world_transform_id: __flight_source.world_transform_id,
            world_transform_using_local_transform_id: __flight_source
                .world_transform_using_local_transform_id,
            world_transform_using_parent_transform_id: __flight_source
                .world_transform_using_parent_transform_id,
            world_alpha: __flight_source.world_alpha,
            world_alpha_using_appearance_id: __flight_source.world_alpha_using_appearance_id,
            world_alpha_using_parent_appearance_id: __flight_source
                .world_alpha_using_parent_appearance_id,
            world_appearance_id: __flight_source.world_appearance_id,
            local_matrix4: (__flight_source.local_matrix4).clone(),
            local_matrix4_detached: __flight_source.local_matrix4_detached,
            world_matrix4: (__flight_source.world_matrix4).clone(),
        }
    };
}

// Source: upstream/packages/scene/src/billboard.ts:54 (sha256:7f4344bb9e2aee03de90f232d573ff202b13758de25c0363809cd6c3a07277c3)
pub fn get_billboard_signals(source: &Billboard) -> Option<NodeSignals> {
    return get_node_signals(&{
        let __flight_source = &(source);
        Node {
            __flight_identity: std::sync::Arc::clone(&__flight_source.__flight_identity),
            data: (__flight_source.data).clone(),
            enabled: __flight_source.enabled,
            kind: (__flight_source.kind).clone(),
            name: (__flight_source.name).clone(),
        }
    });
}

// Source: upstream/packages/scene/src/billboard.ts:61 (sha256:2df21ee89214648dc557fbf93bcf0025abe505634be62a03350b14933e694dd3)
pub fn is_billboard(source: &SceneNode) -> bool {
    let candidate = {
        let __flight_source = &((*source).clone());
        FlightPartialRecord1 {
            __flight_identity: std::sync::Arc::clone(&__flight_source.__flight_identity),
            data: (__flight_source.data).clone(),
            enabled: Some(__flight_source.enabled),
            kind: Some((__flight_source.kind).clone()),
            name: (__flight_source.name).clone(),
            alpha: Some(__flight_source.alpha),
            visible: Some(__flight_source.visible),
            position: Some((__flight_source.position).clone()),
            rotation: Some((__flight_source.rotation).clone()),
            scale: Some((__flight_source.scale).clone()),
            geometry: None,
            materials: None,
            mode: None,
        }
    };
    return (((candidate.geometry).clone()).is_some()) && (((candidate.mode).clone()).is_some());
}
