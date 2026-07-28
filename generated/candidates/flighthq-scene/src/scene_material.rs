// @generated from upstream/packages/scene/src/sceneMaterial.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_node::find_node;
use flighthq_types::{
    Adjustment, ColorTransform, InteractionSignals, Kind, Material, MeshGeometry, MeshMorph, Node,
    NodeData, NodeInteractionState, NodeSignals, NodeTraitsKey, Quaternion, SceneNode, Skin,
    Vector3,
};

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
    pub morph: Option<MeshMorph>,
    pub skin: Option<Skin>,
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

// Source: upstream/packages/scene/src/sceneMaterial.ts:11 (sha256:dc30425b669d69d9e52afea629f1b3304a07e5e88c1b2b410be39451bff2c322)
pub fn find_scene_material_by_name(root: &SceneNode, name: String) -> Option<Material> {
    let root_match = get_named_node_material(root, (name).clone());
    if (root_match).is_some() {
        return Some((root_match.as_ref().unwrap()).clone());
    }
    let found: std::sync::Arc<std::sync::Mutex<Option<Material>>> =
        std::sync::Arc::new(std::sync::Mutex::new(None));
    find_node(
        &Node {
            __flight_identity: std::sync::Arc::clone(&(root).__flight_identity),
            data: ((root).data).clone(),
            enabled: (root).enabled,
            kind: ((root).kind).clone(),
            name: ((root).name).clone(),
        },
        &mut |node: Node| -> bool {
            let match_ = get_named_node_material(&node, (name).clone());
            if (match_).is_none() {
                return false;
            }
            (*found.lock().unwrap()) = Some((match_.as_ref().unwrap()).clone());
            return true;
        },
    );
    return (*found.lock().unwrap()).clone();
}

// Source: upstream/packages/scene/src/sceneMaterial.ts:26 (sha256:551ccf8e51b997ca62bb0e397eb0c657199dfc5ec63d835c9c931d5fea5eec5d)
fn get_named_node_material(node: &SceneNode, name: String) -> Option<Material> {
    let materials = (node.materials).clone();
    if (materials).is_none() {
        return None;
    }
    {
        let mut i = 0.0_f64;
        while (i < (materials.as_ref().unwrap().len() as f64)) {
            let material = materials.as_ref().unwrap()[i as usize].clone();
            if ((material).is_some())
                && (((material.as_ref().unwrap().name).clone()) == Some((name).clone()))
            {
                return (material).clone();
            }
            {
                i += 1.0;
                i
            };
        }
    }
    return None;
}
