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
use flighthq_types::{Matrix4Like, Transform3DLike, Transform3DNode, Vector3Like};

// Source: upstream/packages/node/src/transform3d.ts:26 (sha256:c953371eff20a2e210300783a1f449ef4a377507d65d2316fa6f2f5b5beaefe2)
pub fn convert_node_vector3_global_to_local(
    out: &mut Vector3Like,
    source: &Transform3DNode,
    point: &Vector3Like,
) -> () {
    let mut inv = acquire_matrix4();
    inverse_matrix4(&mut inv, &get_node_world_matrix4(source));
    matrix4_transform_point(out, &inv, point);
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
struct EnsureNodeLocalMatrix4Record1 {
    __flight_identity: std::sync::Arc<()>,
}
impl PartialEq for EnsureNodeLocalMatrix4Record1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn ensure_node_local_matrix4(target: &Transform3DNode) -> () {
    let mut runtime = get_entity_runtime(target);
    if (((runtime.local_matrix4).clone()).is_none())
        || (runtime.local_transform_using_local_transform_id != runtime.local_transform_id)
    {
        recompute_local_transform3_d(target, &mut runtime);
    }
}

// Source: upstream/packages/node/src/transform3d.ts:56 (sha256:eef015ef90edf49f1c9a21e4cea8bd6b029870c639079c14a8560d26c135ccc9)
#[derive(Clone)]
struct EnsureNodeWorldMatrix4Record1 {
    __flight_identity: std::sync::Arc<()>,
}
impl PartialEq for EnsureNodeWorldMatrix4Record1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn ensure_node_world_matrix4(target: &Transform3DNode) -> () {
    let mut runtime = get_entity_runtime(target);
    let parent = (runtime.parent).clone();
    let mut parent_runtime: Option<EnsureNodeWorldMatrix4Record1>;
    let mut parent_world_transform_id = 0.0_f64;
    if (parent).is_some() {
        ensure_node_world_matrix4(&parent.as_ref().unwrap());
        parent_runtime = Some(get_entity_runtime(&parent.as_ref().unwrap()));
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
struct GetNodeLocalMatrix4Record1 {
    __flight_identity: std::sync::Arc<()>,
}
impl PartialEq for GetNodeLocalMatrix4Record1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn get_node_local_matrix4(target: &Transform3DNode) -> Matrix4Like {
    ensure_node_local_matrix4(target);
    return ((get_entity_runtime(target).local_matrix4).clone()).unwrap();
}

// Source: upstream/packages/node/src/transform3d.ts:85 (sha256:e7180730187ed0bef79e4b777d3a21deeddcda466ada2770d2dc3388a54d8ad7)
pub fn get_node_transform3_d(out: &mut Transform3DLike, source: &Transform3DNode) -> () {
    copy_vector3(&mut out.position, &source.position);
    copy_quaternion(&mut out.rotation, &source.rotation);
    copy_vector3(&mut out.scale, &source.scale);
}

// Source: upstream/packages/node/src/transform3d.ts:91 (sha256:7685bbe4e4fc9d7e0823567d0c9f544e3fd93ab36e29e8bb8dd6ef96695cad78)
#[derive(Clone)]
struct GetNodeWorldMatrix4Record1 {
    __flight_identity: std::sync::Arc<()>,
}
impl PartialEq for GetNodeWorldMatrix4Record1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn get_node_world_matrix4(target: &Transform3DNode) -> Matrix4Like {
    ensure_node_world_matrix4(target);
    return ((get_entity_runtime(target).world_matrix4).clone()).unwrap();
}

// Source: upstream/packages/node/src/transform3d.ts:99 (sha256:59f1dcee122778f01173614482e6cc8d8248607f7674e6de5fe3dd96a5f59cf3)
#[derive(Clone)]
struct IsNodeLocalMatrix4DetachedRecord1 {
    __flight_identity: std::sync::Arc<()>,
}
impl PartialEq for IsNodeLocalMatrix4DetachedRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn is_node_local_matrix4_detached(target: &Transform3DNode) -> bool {
    ensure_node_local_matrix4(target);
    return get_entity_runtime(target).local_matrix4_detached;
}

// Source: upstream/packages/node/src/transform3d.ts:108 (sha256:cef856672653057cd0d795c5cb386a7472ca125a25aba2f1adeb19b0251b43ea)
#[derive(Clone)]
struct SetNodeLocalMatrix4Record1 {
    __flight_identity: std::sync::Arc<()>,
}
impl PartialEq for SetNodeLocalMatrix4Record1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn set_node_local_matrix4(target: &Transform3DNode, source: &Matrix4Like) -> () {
    let mut runtime = get_entity_runtime(target);
    if ((runtime.local_matrix4).clone()).is_none() {
        runtime.local_matrix4 = Some(create_matrix4(
            None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            None, None,
        ));
    }
    copy_matrix4(&mut runtime.local_matrix4, source);
    invalidate_node_local_transform(target);
    runtime.local_transform_using_local_transform_id = runtime.local_transform_id;
    runtime.local_matrix4_detached = true;
}

// Source: upstream/packages/node/src/transform3d.ts:122 (sha256:016ead47d432103ce68af7e00c98e47350241182f909c3c60e0e13d6b860c31e)
pub fn set_node_transform3_d(target: &mut Transform3DNode, source: &Transform3DLike) -> () {
    copy_vector3(&mut target.position, &source.position);
    copy_quaternion(&mut target.rotation, &source.rotation);
    copy_vector3(&mut target.scale, &source.scale);
    invalidate_node_local_transform(target);
}

// Source: upstream/packages/node/src/transform3d.ts:135 (sha256:a39d643c79edb7bae1727e428a62a6aad99c36d8d7621a1abc2fa1520faf59e2)
#[derive(Clone)]
struct SyncNodeTransform3DFromMatrix4Record1 {
    __flight_identity: std::sync::Arc<()>,
}
impl PartialEq for SyncNodeTransform3DFromMatrix4Record1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn sync_node_transform3_d_from_matrix4(target: &mut Transform3DNode) -> () {
    let mut runtime = get_entity_runtime(target);
    ensure_node_local_matrix4(target);
    decompose_matrix4(
        &mut target.position,
        &mut target.rotation,
        &mut target.scale,
        &runtime.local_matrix4,
    );
    runtime.local_matrix4_detached = false;
}

// Source: upstream/packages/node/src/transform3d.ts:142 (sha256:4c8bb284a1533e1490d29dcd39d4d84ae47d3ba94c0d5a711c1c37812b3e7c10)
#[derive(Clone)]
struct RecomputeLocalTransform3DRecord1 {
    __flight_identity: std::sync::Arc<()>,
}
impl PartialEq for RecomputeLocalTransform3DRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

fn recompute_local_transform3_d(
    target: &Transform3DNode,
    runtime: &mut RecomputeLocalTransform3DRecord1,
) -> () {
    if ((runtime.local_matrix4).clone()).is_none() {
        runtime.local_matrix4 = Some(create_matrix4(
            None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            None, None,
        ));
    }
    compose_matrix4(
        &mut runtime.local_matrix4,
        &target.position,
        &target.rotation,
        &target.scale,
    );
    runtime.local_matrix4_detached = false;
    runtime.local_transform_using_local_transform_id = runtime.local_transform_id;
}

// Source: upstream/packages/node/src/transform3d.ts:152 (sha256:277953a1cc770832d264f5d42fea409b4236aa0896fe20a26da72e4be7fc8901)
#[derive(Clone)]
struct RecomputeWorldTransform3DRecord1 {
    __flight_identity: std::sync::Arc<()>,
}
impl PartialEq for RecomputeWorldTransform3DRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

fn recompute_world_transform3_d(
    target: &Transform3DNode,
    runtime: &mut RecomputeWorldTransform3DRecord1,
    parent_runtime: Option<RecomputeWorldTransform3DRecord1>,
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
                &mut runtime.world_matrix4,
                &parent_runtime.as_ref().unwrap().world_matrix4,
                &__flight_argument_2,
            )
        };
    } else {
        {
            let __flight_argument_1 = (runtime.local_matrix4).clone();
            copy_matrix4(&mut runtime.world_matrix4, &__flight_argument_1)
        };
    }
    compute_node_world_transform_revision(
        runtime,
        Some(((parent_runtime).clone().unwrap()).clone()),
    );
}
