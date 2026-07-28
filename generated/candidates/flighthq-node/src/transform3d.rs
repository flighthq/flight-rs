// @generated from upstream/packages/node/src/transform3d.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{compute_node_world_transform_revision, invalidate_node_local_transform};
use flighthq_entity::get_entity_runtime;
use flighthq_geometry::{
    acquire_matrix4, compose_matrix4, copy_matrix4, copy_quaternion, copy_vector3, create_matrix4,
    decompose_matrix4, inverse_matrix4, matrix4_transform_point, multiply_matrix4, release_matrix4,
};
use flighthq_types::{
    Adjustment, ColorTransform, Entity, InteractionSignals, Matrix4, Matrix4Like, Node,
    NodeInteractionState, NodeSignals, NodeTraitsKey, QuaternionLike, Transform3DLike,
    Transform3DNode, Vector3Like,
};

#[derive(Clone, Default)]
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

// Source: upstream/packages/node/src/transform3d.ts:26 (sha256:c953371eff20a2e210300783a1f449ef4a377507d65d2316fa6f2f5b5beaefe2)
pub fn convert_node_vector3_global_to_local(
    out: &mut Vector3Like,
    source: &Transform3DNode,
    point: &Vector3Like,
) -> () {
    let mut inv = acquire_matrix4();
    inverse_matrix4(&mut inv, &get_node_world_matrix4(source));
    matrix4_transform_point(
        out,
        &{
            let __flight_source = &(inv);
            Matrix4Like {
                __flight_identity: std::sync::Arc::clone(&__flight_source.__flight_identity),
                m: (__flight_source.m).clone(),
            }
        },
        point,
    );
    release_matrix4(&inv);
}

// Source: upstream/packages/node/src/transform3d.ts:38 (sha256:107303f05e894702285baebe8f85e5661f1eacf98bbc0e98527aad8e6595096f)
pub fn convert_node_vector3_local_to_global(
    out: &mut Vector3Like,
    source: &Transform3DNode,
    point: &Vector3Like,
) -> () {
    matrix4_transform_point(out, &get_node_world_matrix4(source), point);
}

// Source: upstream/packages/node/src/transform3d.ts:49 (sha256:b63e92354299d5424a797977e349283c162594a0e9ac40124c5695ed60b6e0c5)
#[derive(Clone)]
struct EnsureNodeLocalMatrix4Record2 {
    __flight_identity: std::sync::Arc<()>,
    binding: Option<crate::OpaqueHostValue>,
    appearance_id: f64,
    bounds_using_local_bounds_id: f64,
    bounds_using_local_transform_id: f64,
    can_add_child:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(Node, Node) -> bool + Send + 'static>>>,
    children: Option<Vec<Node>>,
    color_adjustments: Option<Vec<Adjustment>>,
    resolved_color_transform: Option<ColorTransform>,
    color_adjustments_channel_mixing: bool,
    traits: Option<NodeTraitsKey>,
    interaction_signals: Option<InteractionSignals>,
    local_bounds_id: f64,
    local_bounds_using_local_bounds_id: f64,
    local_content_id: f64,
    local_transform_id: f64,
    local_transform_using_local_transform_id: f64,
    node_signals: Option<NodeSignals>,
    interaction_state: Option<NodeInteractionState>,
    parent: Option<Node>,
    world_bounds_using_local_bounds_id: f64,
    world_bounds_using_world_transform_id: f64,
    world_transform_id: f64,
    world_transform_using_local_transform_id: f64,
    world_transform_using_parent_transform_id: f64,
    local_matrix4: Option<Matrix4>,
    local_matrix4_detached: bool,
    world_matrix4: Option<Matrix4>,
}
impl PartialEq for EnsureNodeLocalMatrix4Record2 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn ensure_node_local_matrix4(target: &Transform3DNode) -> () {
    let mut runtime = get_entity_runtime(&{
        let __flight_source = &(target);
        Entity {
            __flight_identity: std::sync::Arc::clone(&__flight_source.__flight_identity),
        }
    });
    if (((runtime.local_matrix4).clone()).is_none())
        || (runtime.local_transform_using_local_transform_id != runtime.local_transform_id)
    {
        recompute_local_transform3_d(target, &mut runtime);
    }
}

// Source: upstream/packages/node/src/transform3d.ts:56 (sha256:eef015ef90edf49f1c9a21e4cea8bd6b029870c639079c14a8560d26c135ccc9)
#[derive(Clone)]
struct EnsureNodeWorldMatrix4Record2 {
    __flight_identity: std::sync::Arc<()>,
    binding: Option<crate::OpaqueHostValue>,
    appearance_id: f64,
    bounds_using_local_bounds_id: f64,
    bounds_using_local_transform_id: f64,
    can_add_child:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(Node, Node) -> bool + Send + 'static>>>,
    children: Option<Vec<Node>>,
    color_adjustments: Option<Vec<Adjustment>>,
    resolved_color_transform: Option<ColorTransform>,
    color_adjustments_channel_mixing: bool,
    traits: Option<NodeTraitsKey>,
    interaction_signals: Option<InteractionSignals>,
    local_bounds_id: f64,
    local_bounds_using_local_bounds_id: f64,
    local_content_id: f64,
    local_transform_id: f64,
    local_transform_using_local_transform_id: f64,
    node_signals: Option<NodeSignals>,
    interaction_state: Option<NodeInteractionState>,
    parent: Option<Node>,
    world_bounds_using_local_bounds_id: f64,
    world_bounds_using_world_transform_id: f64,
    world_transform_id: f64,
    world_transform_using_local_transform_id: f64,
    world_transform_using_parent_transform_id: f64,
    local_matrix4: Option<Matrix4>,
    local_matrix4_detached: bool,
    world_matrix4: Option<Matrix4>,
}
impl PartialEq for EnsureNodeWorldMatrix4Record2 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn ensure_node_world_matrix4(target: &Transform3DNode) -> () {
    let mut runtime = get_entity_runtime(&{
        let __flight_source = &(target);
        Entity {
            __flight_identity: std::sync::Arc::clone(&__flight_source.__flight_identity),
        }
    });
    let parent = (runtime.parent).clone();
    let mut parent_runtime: Option<EnsureNodeWorldMatrix4Record2>;
    let mut parent_world_transform_id = 0.0_f64;
    if (parent).is_some() {
        ensure_node_world_matrix4(&parent.as_ref().unwrap());
        parent_runtime = Some(get_entity_runtime(&{
            let __flight_source = &(parent.as_ref().unwrap());
            Entity {
                __flight_identity: std::sync::Arc::clone(&__flight_source.__flight_identity),
            }
        }));
        parent_world_transform_id = parent_runtime.as_mut().unwrap().world_transform_id;
    }
    if (runtime.world_transform_using_local_transform_id != runtime.local_transform_id)
        || (runtime.world_transform_using_parent_transform_id != parent_world_transform_id)
    {
        recompute_world_transform3_d(
            target,
            &mut runtime,
            Some(((parent_runtime).clone().unwrap()).clone()),
        );
    }
}

// Source: upstream/packages/node/src/transform3d.ts:77 (sha256:3e2344902420b90cb3cb2ea767c006cff51e41cbb8c6d8cdfcab5e23a3365eb7)
#[derive(Clone)]
struct GetNodeLocalMatrix4Record2 {
    __flight_identity: std::sync::Arc<()>,
    binding: Option<crate::OpaqueHostValue>,
    appearance_id: f64,
    bounds_using_local_bounds_id: f64,
    bounds_using_local_transform_id: f64,
    can_add_child:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(Node, Node) -> bool + Send + 'static>>>,
    children: Option<Vec<Node>>,
    color_adjustments: Option<Vec<Adjustment>>,
    resolved_color_transform: Option<ColorTransform>,
    color_adjustments_channel_mixing: bool,
    traits: Option<NodeTraitsKey>,
    interaction_signals: Option<InteractionSignals>,
    local_bounds_id: f64,
    local_bounds_using_local_bounds_id: f64,
    local_content_id: f64,
    local_transform_id: f64,
    local_transform_using_local_transform_id: f64,
    node_signals: Option<NodeSignals>,
    interaction_state: Option<NodeInteractionState>,
    parent: Option<Node>,
    world_bounds_using_local_bounds_id: f64,
    world_bounds_using_world_transform_id: f64,
    world_transform_id: f64,
    world_transform_using_local_transform_id: f64,
    world_transform_using_parent_transform_id: f64,
    local_matrix4: Option<Matrix4>,
    local_matrix4_detached: bool,
    world_matrix4: Option<Matrix4>,
}
impl PartialEq for GetNodeLocalMatrix4Record2 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn get_node_local_matrix4(target: &Transform3DNode) -> Matrix4Like {
    ensure_node_local_matrix4(target);
    return ((get_entity_runtime(&{
        let __flight_source = &(target);
        Entity {
            __flight_identity: std::sync::Arc::clone(&__flight_source.__flight_identity),
        }
    })
    .local_matrix4)
        .clone())
    .unwrap();
}

// Source: upstream/packages/node/src/transform3d.ts:85 (sha256:e7180730187ed0bef79e4b777d3a21deeddcda466ada2770d2dc3388a54d8ad7)
pub fn get_node_transform3_d(out: &mut Transform3DLike, source: &Transform3DNode) -> () {
    copy_vector3(&mut out.position, &{
        let __flight_source = &(source.position);
        Vector3Like {
            __flight_identity: std::sync::Arc::clone(&__flight_source.__flight_identity),
            x: __flight_source.x,
            y: __flight_source.y,
            z: __flight_source.z,
        }
    });
    copy_quaternion(&mut out.rotation, &{
        let __flight_source = &(source.rotation);
        QuaternionLike {
            __flight_identity: std::sync::Arc::clone(&__flight_source.__flight_identity),
            x: __flight_source.x,
            y: __flight_source.y,
            z: __flight_source.z,
            w: __flight_source.w,
        }
    });
    copy_vector3(&mut out.scale, &{
        let __flight_source = &(source.scale);
        Vector3Like {
            __flight_identity: std::sync::Arc::clone(&__flight_source.__flight_identity),
            x: __flight_source.x,
            y: __flight_source.y,
            z: __flight_source.z,
        }
    });
}

// Source: upstream/packages/node/src/transform3d.ts:91 (sha256:7685bbe4e4fc9d7e0823567d0c9f544e3fd93ab36e29e8bb8dd6ef96695cad78)
#[derive(Clone)]
struct GetNodeWorldMatrix4Record2 {
    __flight_identity: std::sync::Arc<()>,
    binding: Option<crate::OpaqueHostValue>,
    appearance_id: f64,
    bounds_using_local_bounds_id: f64,
    bounds_using_local_transform_id: f64,
    can_add_child:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(Node, Node) -> bool + Send + 'static>>>,
    children: Option<Vec<Node>>,
    color_adjustments: Option<Vec<Adjustment>>,
    resolved_color_transform: Option<ColorTransform>,
    color_adjustments_channel_mixing: bool,
    traits: Option<NodeTraitsKey>,
    interaction_signals: Option<InteractionSignals>,
    local_bounds_id: f64,
    local_bounds_using_local_bounds_id: f64,
    local_content_id: f64,
    local_transform_id: f64,
    local_transform_using_local_transform_id: f64,
    node_signals: Option<NodeSignals>,
    interaction_state: Option<NodeInteractionState>,
    parent: Option<Node>,
    world_bounds_using_local_bounds_id: f64,
    world_bounds_using_world_transform_id: f64,
    world_transform_id: f64,
    world_transform_using_local_transform_id: f64,
    world_transform_using_parent_transform_id: f64,
    local_matrix4: Option<Matrix4>,
    local_matrix4_detached: bool,
    world_matrix4: Option<Matrix4>,
}
impl PartialEq for GetNodeWorldMatrix4Record2 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn get_node_world_matrix4(target: &Transform3DNode) -> Matrix4Like {
    ensure_node_world_matrix4(target);
    return ((get_entity_runtime(&{
        let __flight_source = &(target);
        Entity {
            __flight_identity: std::sync::Arc::clone(&__flight_source.__flight_identity),
        }
    })
    .world_matrix4)
        .clone())
    .unwrap();
}

// Source: upstream/packages/node/src/transform3d.ts:99 (sha256:59f1dcee122778f01173614482e6cc8d8248607f7674e6de5fe3dd96a5f59cf3)
#[derive(Clone)]
struct IsNodeLocalMatrix4DetachedRecord2 {
    __flight_identity: std::sync::Arc<()>,
    binding: Option<crate::OpaqueHostValue>,
    appearance_id: f64,
    bounds_using_local_bounds_id: f64,
    bounds_using_local_transform_id: f64,
    can_add_child:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(Node, Node) -> bool + Send + 'static>>>,
    children: Option<Vec<Node>>,
    color_adjustments: Option<Vec<Adjustment>>,
    resolved_color_transform: Option<ColorTransform>,
    color_adjustments_channel_mixing: bool,
    traits: Option<NodeTraitsKey>,
    interaction_signals: Option<InteractionSignals>,
    local_bounds_id: f64,
    local_bounds_using_local_bounds_id: f64,
    local_content_id: f64,
    local_transform_id: f64,
    local_transform_using_local_transform_id: f64,
    node_signals: Option<NodeSignals>,
    interaction_state: Option<NodeInteractionState>,
    parent: Option<Node>,
    world_bounds_using_local_bounds_id: f64,
    world_bounds_using_world_transform_id: f64,
    world_transform_id: f64,
    world_transform_using_local_transform_id: f64,
    world_transform_using_parent_transform_id: f64,
    local_matrix4: Option<Matrix4>,
    local_matrix4_detached: bool,
    world_matrix4: Option<Matrix4>,
}
impl PartialEq for IsNodeLocalMatrix4DetachedRecord2 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn is_node_local_matrix4_detached(target: &Transform3DNode) -> bool {
    ensure_node_local_matrix4(target);
    return get_entity_runtime(&{
        let __flight_source = &(target);
        Entity {
            __flight_identity: std::sync::Arc::clone(&__flight_source.__flight_identity),
        }
    })
    .local_matrix4_detached;
}

// Source: upstream/packages/node/src/transform3d.ts:108 (sha256:cef856672653057cd0d795c5cb386a7472ca125a25aba2f1adeb19b0251b43ea)
#[derive(Clone)]
struct SetNodeLocalMatrix4Record2 {
    __flight_identity: std::sync::Arc<()>,
    binding: Option<crate::OpaqueHostValue>,
    appearance_id: f64,
    bounds_using_local_bounds_id: f64,
    bounds_using_local_transform_id: f64,
    can_add_child:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(Node, Node) -> bool + Send + 'static>>>,
    children: Option<Vec<Node>>,
    color_adjustments: Option<Vec<Adjustment>>,
    resolved_color_transform: Option<ColorTransform>,
    color_adjustments_channel_mixing: bool,
    traits: Option<NodeTraitsKey>,
    interaction_signals: Option<InteractionSignals>,
    local_bounds_id: f64,
    local_bounds_using_local_bounds_id: f64,
    local_content_id: f64,
    local_transform_id: f64,
    local_transform_using_local_transform_id: f64,
    node_signals: Option<NodeSignals>,
    interaction_state: Option<NodeInteractionState>,
    parent: Option<Node>,
    world_bounds_using_local_bounds_id: f64,
    world_bounds_using_world_transform_id: f64,
    world_transform_id: f64,
    world_transform_using_local_transform_id: f64,
    world_transform_using_parent_transform_id: f64,
    local_matrix4: Option<Matrix4>,
    local_matrix4_detached: bool,
    world_matrix4: Option<Matrix4>,
}
impl PartialEq for SetNodeLocalMatrix4Record2 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn set_node_local_matrix4(target: &Transform3DNode, source: &Matrix4Like) -> () {
    let mut runtime = get_entity_runtime(&{
        let __flight_source = &(target);
        Entity {
            __flight_identity: std::sync::Arc::clone(&__flight_source.__flight_identity),
        }
    });
    if ((runtime.local_matrix4).clone()).is_none() {
        runtime.local_matrix4 = Some(create_matrix4(
            None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            None, None,
        ));
    }
    copy_matrix4(runtime.local_matrix4.as_mut().unwrap(), source);
    invalidate_node_local_transform(&{
        let __flight_source = &(target);
        Node {
            __flight_identity: std::sync::Arc::clone(&__flight_source.__flight_identity),
            data: (__flight_source.data).clone(),
            enabled: __flight_source.enabled,
            kind: (__flight_source.kind).clone(),
            name: (__flight_source.name).clone(),
        }
    });
    runtime.local_transform_using_local_transform_id = runtime.local_transform_id;
    runtime.local_matrix4_detached = true;
}

// Source: upstream/packages/node/src/transform3d.ts:122 (sha256:016ead47d432103ce68af7e00c98e47350241182f909c3c60e0e13d6b860c31e)
pub fn set_node_transform3_d(target: &mut Transform3DNode, source: &Transform3DLike) -> () {
    copy_vector3(&mut target.position, &{
        let __flight_source = &(source.position);
        Vector3Like {
            __flight_identity: std::sync::Arc::clone(&__flight_source.__flight_identity),
            x: __flight_source.x,
            y: __flight_source.y,
            z: __flight_source.z,
        }
    });
    copy_quaternion(&mut target.rotation, &{
        let __flight_source = &(source.rotation);
        QuaternionLike {
            __flight_identity: std::sync::Arc::clone(&__flight_source.__flight_identity),
            x: __flight_source.x,
            y: __flight_source.y,
            z: __flight_source.z,
            w: __flight_source.w,
        }
    });
    copy_vector3(&mut target.scale, &{
        let __flight_source = &(source.scale);
        Vector3Like {
            __flight_identity: std::sync::Arc::clone(&__flight_source.__flight_identity),
            x: __flight_source.x,
            y: __flight_source.y,
            z: __flight_source.z,
        }
    });
    invalidate_node_local_transform(&{
        let __flight_source = &(target);
        Node {
            __flight_identity: std::sync::Arc::clone(&__flight_source.__flight_identity),
            data: (__flight_source.data).clone(),
            enabled: __flight_source.enabled,
            kind: (__flight_source.kind).clone(),
            name: (__flight_source.name).clone(),
        }
    });
}

// Source: upstream/packages/node/src/transform3d.ts:135 (sha256:a39d643c79edb7bae1727e428a62a6aad99c36d8d7621a1abc2fa1520faf59e2)
#[derive(Clone)]
struct SyncNodeTransform3DFromMatrix4Record2 {
    __flight_identity: std::sync::Arc<()>,
    binding: Option<crate::OpaqueHostValue>,
    appearance_id: f64,
    bounds_using_local_bounds_id: f64,
    bounds_using_local_transform_id: f64,
    can_add_child:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(Node, Node) -> bool + Send + 'static>>>,
    children: Option<Vec<Node>>,
    color_adjustments: Option<Vec<Adjustment>>,
    resolved_color_transform: Option<ColorTransform>,
    color_adjustments_channel_mixing: bool,
    traits: Option<NodeTraitsKey>,
    interaction_signals: Option<InteractionSignals>,
    local_bounds_id: f64,
    local_bounds_using_local_bounds_id: f64,
    local_content_id: f64,
    local_transform_id: f64,
    local_transform_using_local_transform_id: f64,
    node_signals: Option<NodeSignals>,
    interaction_state: Option<NodeInteractionState>,
    parent: Option<Node>,
    world_bounds_using_local_bounds_id: f64,
    world_bounds_using_world_transform_id: f64,
    world_transform_id: f64,
    world_transform_using_local_transform_id: f64,
    world_transform_using_parent_transform_id: f64,
    local_matrix4: Option<Matrix4>,
    local_matrix4_detached: bool,
    world_matrix4: Option<Matrix4>,
}
impl PartialEq for SyncNodeTransform3DFromMatrix4Record2 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn sync_node_transform3_d_from_matrix4(target: &mut Transform3DNode) -> () {
    let mut runtime = get_entity_runtime(&{
        let __flight_source = &(target);
        Entity {
            __flight_identity: std::sync::Arc::clone(&__flight_source.__flight_identity),
        }
    });
    ensure_node_local_matrix4(target);
    decompose_matrix4(
        &mut target.position,
        &mut target.rotation,
        &mut target.scale,
        runtime.local_matrix4.as_ref().unwrap(),
    );
    runtime.local_matrix4_detached = false;
}

// Source: upstream/packages/node/src/transform3d.ts:142 (sha256:4c8bb284a1533e1490d29dcd39d4d84ae47d3ba94c0d5a711c1c37812b3e7c10)
#[derive(Clone)]
struct RecomputeLocalTransform3DRecord2 {
    __flight_identity: std::sync::Arc<()>,
    binding: Option<crate::OpaqueHostValue>,
    appearance_id: f64,
    bounds_using_local_bounds_id: f64,
    bounds_using_local_transform_id: f64,
    can_add_child:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(Node, Node) -> bool + Send + 'static>>>,
    children: Option<Vec<Node>>,
    color_adjustments: Option<Vec<Adjustment>>,
    resolved_color_transform: Option<ColorTransform>,
    color_adjustments_channel_mixing: bool,
    traits: Option<NodeTraitsKey>,
    interaction_signals: Option<InteractionSignals>,
    local_bounds_id: f64,
    local_bounds_using_local_bounds_id: f64,
    local_content_id: f64,
    local_transform_id: f64,
    local_transform_using_local_transform_id: f64,
    node_signals: Option<NodeSignals>,
    interaction_state: Option<NodeInteractionState>,
    parent: Option<Node>,
    world_bounds_using_local_bounds_id: f64,
    world_bounds_using_world_transform_id: f64,
    world_transform_id: f64,
    world_transform_using_local_transform_id: f64,
    world_transform_using_parent_transform_id: f64,
    local_matrix4: Option<Matrix4>,
    local_matrix4_detached: bool,
    world_matrix4: Option<Matrix4>,
}
impl PartialEq for RecomputeLocalTransform3DRecord2 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

fn recompute_local_transform3_d(
    target: &Transform3DNode,
    runtime: &mut RecomputeLocalTransform3DRecord2,
) -> () {
    if ((runtime.local_matrix4).clone()).is_none() {
        runtime.local_matrix4 = Some(create_matrix4(
            None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            None, None,
        ));
    }
    compose_matrix4(
        runtime.local_matrix4.as_mut().unwrap(),
        &{
            let __flight_source = &(target.position);
            Vector3Like {
                __flight_identity: std::sync::Arc::clone(&__flight_source.__flight_identity),
                x: __flight_source.x,
                y: __flight_source.y,
                z: __flight_source.z,
            }
        },
        &{
            let __flight_source = &(target.rotation);
            QuaternionLike {
                __flight_identity: std::sync::Arc::clone(&__flight_source.__flight_identity),
                x: __flight_source.x,
                y: __flight_source.y,
                z: __flight_source.z,
                w: __flight_source.w,
            }
        },
        &{
            let __flight_source = &(target.scale);
            Vector3Like {
                __flight_identity: std::sync::Arc::clone(&__flight_source.__flight_identity),
                x: __flight_source.x,
                y: __flight_source.y,
                z: __flight_source.z,
            }
        },
    );
    runtime.local_matrix4_detached = false;
    runtime.local_transform_using_local_transform_id = runtime.local_transform_id;
}

// Source: upstream/packages/node/src/transform3d.ts:152 (sha256:277953a1cc770832d264f5d42fea409b4236aa0896fe20a26da72e4be7fc8901)
#[derive(Clone)]
struct RecomputeWorldTransform3DRecord2 {
    __flight_identity: std::sync::Arc<()>,
    binding: Option<crate::OpaqueHostValue>,
    appearance_id: f64,
    bounds_using_local_bounds_id: f64,
    bounds_using_local_transform_id: f64,
    can_add_child:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(Node, Node) -> bool + Send + 'static>>>,
    children: Option<Vec<Node>>,
    color_adjustments: Option<Vec<Adjustment>>,
    resolved_color_transform: Option<ColorTransform>,
    color_adjustments_channel_mixing: bool,
    traits: Option<NodeTraitsKey>,
    interaction_signals: Option<InteractionSignals>,
    local_bounds_id: f64,
    local_bounds_using_local_bounds_id: f64,
    local_content_id: f64,
    local_transform_id: f64,
    local_transform_using_local_transform_id: f64,
    node_signals: Option<NodeSignals>,
    interaction_state: Option<NodeInteractionState>,
    parent: Option<Node>,
    world_bounds_using_local_bounds_id: f64,
    world_bounds_using_world_transform_id: f64,
    world_transform_id: f64,
    world_transform_using_local_transform_id: f64,
    world_transform_using_parent_transform_id: f64,
    local_matrix4: Option<Matrix4>,
    local_matrix4_detached: bool,
    world_matrix4: Option<Matrix4>,
}
impl PartialEq for RecomputeWorldTransform3DRecord2 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

fn recompute_world_transform3_d(
    target: &Transform3DNode,
    runtime: &mut RecomputeWorldTransform3DRecord2,
    parent_runtime: Option<RecomputeWorldTransform3DRecord2>,
) -> () {
    if ((runtime.world_matrix4).clone()).is_none() {
        runtime.world_matrix4 = Some(create_matrix4(
            None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            None, None,
        ));
    }
    ensure_node_local_matrix4(target);
    if (parent_runtime).is_some() {
        {
            let __flight_argument_2 = (runtime.local_matrix4).clone();
            multiply_matrix4(
                runtime.world_matrix4.as_mut().unwrap(),
                parent_runtime
                    .as_ref()
                    .unwrap()
                    .world_matrix4
                    .as_ref()
                    .unwrap(),
                &__flight_argument_2,
            )
        };
    } else {
        {
            let __flight_argument_1 = (runtime.local_matrix4).clone();
            copy_matrix4(
                runtime.world_matrix4.as_mut().unwrap(),
                &__flight_argument_1,
            )
        };
    }
    compute_node_world_transform_revision(
        runtime,
        Some(((parent_runtime).clone().unwrap()).clone()),
    );
}
