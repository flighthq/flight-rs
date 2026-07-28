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
    pub morph: Option<MeshMorph>,
    pub skin: Option<Skin>,
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
    set_node_transform3_d(&mut clone, &{
        let __flight_source = &(source);
        Transform3DLike {
            __flight_identity: std::sync::Arc::clone(&__flight_source.__flight_identity),
            position: (__flight_source.position).clone(),
            rotation: (__flight_source.rotation).clone(),
            scale: (__flight_source.scale).clone(),
        }
    });
    if is_node_local_matrix4_detached(&{
        let __flight_source = &(source);
        Transform3DNode {
            __flight_identity: std::sync::Arc::clone(&__flight_source.__flight_identity),
            data: (__flight_source.data).clone(),
            enabled: __flight_source.enabled,
            kind: (__flight_source.kind).clone(),
            name: (__flight_source.name).clone(),
            position: (__flight_source.position).clone(),
            rotation: (__flight_source.rotation).clone(),
            scale: (__flight_source.scale).clone(),
        }
    }) {
        set_node_local_matrix4(
            &{
                let __flight_source = &(clone);
                Transform3DNode {
                    __flight_identity: std::sync::Arc::clone(&__flight_source.__flight_identity),
                    data: (__flight_source.data).clone(),
                    enabled: __flight_source.enabled,
                    kind: (__flight_source.kind).clone(),
                    name: (__flight_source.name).clone(),
                    position: (__flight_source.position).clone(),
                    rotation: (__flight_source.rotation).clone(),
                    scale: (__flight_source.scale).clone(),
                }
            },
            &get_node_local_matrix4(&{
                let __flight_source = &(source);
                Transform3DNode {
                    __flight_identity: std::sync::Arc::clone(&__flight_source.__flight_identity),
                    data: (__flight_source.data).clone(),
                    enabled: __flight_source.enabled,
                    kind: (__flight_source.kind).clone(),
                    name: (__flight_source.name).clone(),
                    position: (__flight_source.position).clone(),
                    rotation: (__flight_source.rotation).clone(),
                    scale: (__flight_source.scale).clone(),
                }
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
        MeshRuntime {
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

// Source: upstream/packages/scene/src/mesh.ts:78 (sha256:466ba13ecedfaf8075dc2fe6b2531259b77deb132787565db2b95555df9410c9)
pub fn get_mesh_signals(source: &Mesh) -> Option<NodeSignals> {
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

// Source: upstream/packages/scene/src/mesh.ts:85 (sha256:408835db9e498dd60caad0fe098ac5f58f6578b0f0e1801afb47505911eb1317)
pub fn is_mesh(source: crate::OpaqueHostValue) -> bool {
    return ((source.geometry).clone()).is_some();
}
