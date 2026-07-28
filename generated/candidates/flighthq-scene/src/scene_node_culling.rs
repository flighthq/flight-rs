// @generated from upstream/packages/scene/src/sceneNodeCulling.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::is_mesh;
use flighthq_geometry::{
    create_aabb, is_frustum_intersecting_aabb, set_frustum_from_matrix4, transform_aabb_by_matrix4,
};
use flighthq_mesh::compute_mesh_geometry_bounds;
use flighthq_node::{ensure_node_world_matrix4, get_node_runtime, get_node_world_matrix4};
use flighthq_types::{
    Aabb, AabbLike, Adjustment, ColorTransform, FrustumLike, InteractionSignals, Kind, Material,
    Matrix4Like, MeshGeometry, MeshMorph, Node, NodeData, NodeInteractionState, NodeSignals,
    NodeTraitsKey, Quaternion, SceneNode, Skin, Transform3DNode, Vector3,
};

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

// Source: upstream/packages/scene/src/sceneNodeCulling.ts:21 (sha256:08fd8e219668794c3132f8d707e85efb9e69395f3da88a1e5e04dc8c66141f3c)
pub fn build_scene_frustum(out: &mut FrustumLike, view_projection: &Matrix4Like) -> () {
    set_frustum_from_matrix4(out, view_projection);
}

// Source: upstream/packages/scene/src/sceneNodeCulling.ts:36 (sha256:827626e8a96ed470571982e11cb52a39c8b91e1317c4d91776e9878db285c815)
pub fn cull_scene_node_by_frustum(
    out: &mut Vec<SceneNode>,
    root: &mut SceneNode,
    frustum: &FrustumLike,
) -> Vec<SceneNode> {
    _cull_node(out, root, frustum);
    return out.clone();
}

// Source: upstream/packages/scene/src/sceneNodeCulling.ts:45 (sha256:3a7da157b950ded849fdd9f2eefb5d0856ccc9152490a57f11ec30f28583242b)
fn _cull_node(out: &mut Vec<SceneNode>, node: &mut SceneNode, frustum: &FrustumLike) -> () {
    if (!node.enabled) {
        return;
    }
    if is_mesh((node).clone()) {
        let mut geom = node.geometry;
        let mut local_bounds = geom.bounds;
        if (local_bounds).is_none() {
            compute_mesh_geometry_bounds(&mut (*_SCRATCH_LOCAL_AABB.lock().unwrap()), &geom);
            local_bounds = (*_SCRATCH_LOCAL_AABB.lock().unwrap()).clone();
        }
        if (local_bounds.min.x <= local_bounds.max.x) {
            ensure_node_world_matrix4(&{
                let __flight_source = &(node);
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
            });
            let world_matrix = get_node_world_matrix4(&{
                let __flight_source = &(node);
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
            });
            transform_aabb_by_matrix4(
                &mut (*_SCRATCH_WORLD_AABB.lock().unwrap()),
                &{
                    let __flight_source = &(local_bounds);
                    AabbLike {
                        __flight_identity: std::sync::Arc::clone(
                            &__flight_source.__flight_identity,
                        ),
                        max: (__flight_source.max).clone(),
                        min: (__flight_source.min).clone(),
                    }
                },
                &world_matrix,
            );
            if is_frustum_intersecting_aabb(frustum, &{
                let __flight_source = &(*_SCRATCH_WORLD_AABB.lock().unwrap());
                AabbLike {
                    __flight_identity: std::sync::Arc::clone(&__flight_source.__flight_identity),
                    max: (__flight_source.max).clone(),
                    min: (__flight_source.min).clone(),
                }
            }) {
                out.push((*node).clone());
            }
        }
    }
    let mut children = (get_node_runtime(&{
        let __flight_source = &(node);
        Node {
            __flight_identity: std::sync::Arc::clone(&__flight_source.__flight_identity),
            data: (__flight_source.data).clone(),
            enabled: __flight_source.enabled,
            kind: (__flight_source.kind).clone(),
            name: (__flight_source.name).clone(),
        }
    })
    .children)
        .clone();
    if (children).is_some() {
        {
            let mut i = 0.0_f64;
            while (i < (children.as_mut().unwrap().len() as f64)) {
                _cull_node(
                    out,
                    &mut children.as_mut().unwrap()[i as usize].clone(),
                    frustum,
                );
                {
                    i += 1.0;
                    i
                };
            }
        }
    }
}

// Source: upstream/packages/scene/src/sceneNodeCulling.ts:74 (sha256:6fe5187a5a345646b6e1b1b86592ae73ec716e3ddd479281737882422c0e848d)
static _SCRATCH_LOCAL_AABB: std::sync::LazyLock<std::sync::Mutex<Aabb>> =
    std::sync::LazyLock::new(|| {
        std::sync::Mutex::new(create_aabb(None, None, None, None, None, None))
    });

// Source: upstream/packages/scene/src/sceneNodeCulling.ts:75 (sha256:f4adb2a3cb6957f412db3d2b35ccb6c2409c20971edfb289edbd9458ad7af15d)
static _SCRATCH_WORLD_AABB: std::sync::LazyLock<std::sync::Mutex<Aabb>> =
    std::sync::LazyLock::new(|| {
        std::sync::Mutex::new(create_aabb(None, None, None, None, None, None))
    });
