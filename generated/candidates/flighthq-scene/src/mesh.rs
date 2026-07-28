// @generated from upstream/packages/scene/src/mesh.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{create_scene_node, get_scene_node_runtime};
use flighthq_node::{
    enable_node_signals, get_node_local_matrix4, get_node_signals, is_node_local_matrix4_detached,
    set_node_local_matrix4, set_node_transform3_d,
};
use flighthq_types::{
    Adjustment, ColorTransform, InteractionSignals, Kind,
    MESH_DEFORMER_MORPH as mesh_deformer_morph_constant,
    MESH_DEFORMER_NONE as mesh_deformer_none_constant,
    MESH_DEFORMER_SKELETAL as mesh_deformer_skeletal_constant, Material, MeshDeformer,
    MeshGeometry, MeshMorph, Node, NodeData, NodeInteractionState, NodeSignals, NodeTraitsKey,
    Quaternion, SceneNode, Skin, Transform3DLike, Transform3DNode, Vector3,
};
pub use flighthq_types::{MESH_KIND, Mesh, MeshRuntime};

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

// Source: upstream/packages/scene/src/mesh.ts:27 (sha256:277fcef755c1a9cd8ec6c38994524888dc06d449d304767d88375391d4aa9076)
pub fn clone_mesh(source: &Mesh) -> Mesh {
    let mut clone = create_mesh(
        &source.geometry,
        &((source.materials).clone()).clone(),
        Some(((source.kind).clone()).clone()),
        Some(FlightPartialRecord1 {
            __flight_identity: std::sync::Arc::new(()),
            enabled: Some(source.enabled),
            name: (source.name).clone(),
            data: None,
            kind: None,
            alpha: None,
            visible: None,
            position: None,
            rotation: None,
            scale: None,
            geometry: None,
            materials: None,
            morph: None,
            skin: None,
        }),
    );
    clone.alpha = source.alpha;
    set_node_transform3_d(
        &mut clone,
        &Transform3DLike {
            __flight_identity: std::sync::Arc::clone(&(source).__flight_identity),
            position: ((source).position).clone(),
            rotation: ((source).rotation).clone(),
            scale: ((source).scale).clone(),
        },
    );
    if is_node_local_matrix4_detached(&Transform3DNode {
        __flight_identity: std::sync::Arc::clone(&(source).__flight_identity),
        data: ((source).data).clone(),
        enabled: (source).enabled,
        kind: ((source).kind).clone(),
        name: ((source).name).clone(),
        position: ((source).position).clone(),
        rotation: ((source).rotation).clone(),
        scale: ((source).scale).clone(),
    }) {
        set_node_local_matrix4(
            &Transform3DNode {
                __flight_identity: std::sync::Arc::clone(&(clone).__flight_identity),
                data: ((clone).data).clone(),
                enabled: (clone).enabled,
                kind: ((clone).kind).clone(),
                name: ((clone).name).clone(),
                position: ((clone).position).clone(),
                rotation: ((clone).rotation).clone(),
                scale: ((clone).scale).clone(),
            },
            &get_node_local_matrix4(&Transform3DNode {
                __flight_identity: std::sync::Arc::clone(&(source).__flight_identity),
                data: ((source).data).clone(),
                enabled: (source).enabled,
                kind: ((source).kind).clone(),
                name: ((source).name).clone(),
                position: ((source).position).clone(),
                rotation: ((source).rotation).clone(),
                scale: ((source).scale).clone(),
            }),
        );
    }
    if ((source.skin).clone()).is_some() {
        clone.skin = (source.skin).clone();
    }
    if ((source.morph).clone()).is_some() {
        clone.morph = (source.morph).clone();
    }
    return clone;
}

// Source: upstream/packages/scene/src/mesh.ts:46 (sha256:4fbd8df439f616ec0665150fd447de8d79e1ff6b28bcfd083df5d9954a20f3bc)
pub fn create_mesh(
    geometry: &MeshGeometry,
    materials: &Vec<Option<Material>>,
    kind: Option<Kind>,
    obj: Option<FlightPartialRecord1>,
) -> Mesh {
    let kind = kind.unwrap_or((MESH_KIND).to_owned());
    let mut mesh = create_scene_node(
        Some(((kind).clone()).clone()),
        Some(((obj).clone().unwrap()).clone()),
    );
    mesh.geometry = (*geometry).clone();
    mesh.materials = (*materials).clone();
    return mesh;
}

// Source: upstream/packages/scene/src/mesh.ts:58 (sha256:3075d4aa4f8277ac28e3a3c15c21c3b9e4f7403ec139088ca5b0afc9a5e0e2b5)
pub fn enable_mesh_signals(source: &Mesh) -> NodeSignals {
    return enable_node_signals(&Node {
        __flight_identity: std::sync::Arc::clone(&(source).__flight_identity),
        data: ((source).data).clone(),
        enabled: (source).enabled,
        kind: ((source).kind).clone(),
        name: ((source).name).clone(),
    });
}

// Source: upstream/packages/scene/src/mesh.ts:68 (sha256:1c56f3460bc3d04c757d8c00e42698868b33fdeda60b5ecb831ce35c206de8e1)
pub fn get_mesh_deformer(source: &Mesh) -> MeshDeformer {
    if ((source.skin).clone()).is_some() {
        return (mesh_deformer_skeletal_constant).to_owned();
    }
    if ((source.morph).clone()).is_some() {
        return (mesh_deformer_morph_constant).to_owned();
    }
    return (mesh_deformer_none_constant).to_owned();
}

// Source: upstream/packages/scene/src/mesh.ts:74 (sha256:c7679192eae905074e4e69117bcd5251d816c92261d0ce8146a129a121728f1c)
pub fn get_mesh_runtime(source: &Mesh) -> MeshRuntime {
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

// Source: upstream/packages/scene/src/mesh.ts:78 (sha256:466ba13ecedfaf8075dc2fe6b2531259b77deb132787565db2b95555df9410c9)
pub fn get_mesh_signals(source: &Mesh) -> Option<NodeSignals> {
    return get_node_signals(&Node {
        __flight_identity: std::sync::Arc::clone(&(source).__flight_identity),
        data: ((source).data).clone(),
        enabled: (source).enabled,
        kind: ((source).kind).clone(),
        name: ((source).name).clone(),
    });
}

// Source: upstream/packages/scene/src/mesh.ts:85 (sha256:408835db9e498dd60caad0fe098ac5f58f6578b0f0e1801afb47505911eb1317)
pub fn is_mesh(source: crate::OpaqueHostValue) -> bool {
    return ((source.geometry).clone()).is_some();
}
