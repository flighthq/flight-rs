// @generated from upstream/packages/node/src/transform2d.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::invalidate_node_local_transform;
use flighthq_entity::get_entity_runtime;
use flighthq_geometry::{
    copy_matrix, create_matrix, decompose_matrix_to_transform2_d,
    inverse_matrix_transform_point_xy, matrix_transform_point_xy, multiply_matrix,
};
use flighthq_types::{Matrix, MatrixLike, Transform2DLike, Transform2DNode, Vector2Like};

// Source: upstream/packages/node/src/transform2d.ts:27 (sha256:ca91bb995efe853efdeb9d1be141829c817052921c2bce255cecf43e1946e58c)
pub fn convert_node_vector2_global_to_local(
    out: &mut Vector2Like,
    source: &Transform2DNode,
    vector: &Vector2Like,
) -> () {
    inverse_matrix_transform_point_xy(out, &get_node_world_matrix(source), vector.x, vector.y);
}

// Source: upstream/packages/node/src/transform2d.ts:39 (sha256:78974095e131a641e6700333e776e3652017af5a8e408f867743be7cef35418a)
pub fn convert_node_vector2_local_to_global(
    out: &mut Vector2Like,
    source: &Transform2DNode,
    vector: &Vector2Like,
) -> () {
    matrix_transform_point_xy(out, &get_node_world_matrix(source), vector.x, vector.y);
}

// Source: upstream/packages/node/src/transform2d.ts:47 (sha256:3be4e134897216f7c00602c3134fddc788d273a7166eb061db80252a17cd2fc2)
#[derive(Clone)]
struct EnsureNodeLocalMatrixRecord1 {
    __flight_identity: std::sync::Arc<()>,
}
impl PartialEq for EnsureNodeLocalMatrixRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn ensure_node_local_matrix(target: &Transform2DNode) -> () {
    let mut runtime = get_entity_runtime(target);
    if (runtime.local_transform_using_local_transform_id != runtime.local_transform_id) {
        recompute_local_transform2_d(target, &mut runtime);
    }
}

// Source: upstream/packages/node/src/transform2d.ts:54 (sha256:9bae50d0441fca8a8c6ec3a77be1016a4542508eb60f0928690c0da6c49e5f64)
#[derive(Clone)]
struct EnsureNodeWorldMatrixRecord1 {
    __flight_identity: std::sync::Arc<()>,
}
impl PartialEq for EnsureNodeWorldMatrixRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn ensure_node_world_matrix(target: &Transform2DNode) -> () {
    let mut runtime = get_entity_runtime(target);
    let parent = ((runtime.parent).clone()).unwrap();
    let mut parent_runtime: Option<EnsureNodeWorldMatrixRecord1>;
    let mut parent_world_transform_id = 0.0_f64;
    if (parent).is_some() {
        ensure_node_world_matrix(&parent);
        parent_runtime = Some(get_entity_runtime(&parent));
        parent_world_transform_id = parent_runtime.as_mut().unwrap().world_transform_id;
    }
    if (runtime.world_transform_using_local_transform_id != runtime.local_transform_id)
        || (runtime.world_transform_using_parent_transform_id != parent_world_transform_id)
    {
        recompute_world_transform2_d(
            target,
            &mut runtime,
            Some(((parent_runtime).clone().unwrap()).clone()),
        );
    }
}

// Source: upstream/packages/node/src/transform2d.ts:75 (sha256:cffd0eb1275e77e62c22a7e7c9d56232394966534ad0a4add4c5fed0950acd3b)
#[derive(Clone)]
struct GetNodeLocalMatrixRecord1 {
    __flight_identity: std::sync::Arc<()>,
}
impl PartialEq for GetNodeLocalMatrixRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn get_node_local_matrix(target: &Transform2DNode) -> Matrix {
    ensure_node_local_matrix(target);
    return ((get_entity_runtime(target).local_matrix).clone()).unwrap();
}

// Source: upstream/packages/node/src/transform2d.ts:81 (sha256:32fc97f3ecfad0a510480da5cf0235aee5dfde79666c816f715a67191e972761)
pub fn get_node_transform2_d(out: &mut Transform2DLike, source: &Transform2DNode) -> () {
    out.pivot_x = source.pivot_x;
    out.pivot_y = source.pivot_y;
    out.rotation = source.rotation;
    out.scale_x = source.scale_x;
    out.scale_y = source.scale_y;
    out.skew_x = source.skew_x;
    out.skew_y = source.skew_y;
    out.x = source.x;
    out.y = source.y;
}

// Source: upstream/packages/node/src/transform2d.ts:93 (sha256:002d8726d0a49dedd8c19ead0a29b7fd9ead67f2b309d7bee110598302320f77)
#[derive(Clone)]
struct GetNodeWorldMatrixRecord1 {
    __flight_identity: std::sync::Arc<()>,
}
impl PartialEq for GetNodeWorldMatrixRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn get_node_world_matrix(target: &Transform2DNode) -> Matrix {
    ensure_node_world_matrix(target);
    return ((get_entity_runtime(target).world_matrix).clone()).unwrap();
}

// Source: upstream/packages/node/src/transform2d.ts:101 (sha256:430ca47f27b161924ddb55dd30da83d4b7a3117e19fb42ccbcecf84ba3084e1c)
pub fn set_node_local_matrix(target: &mut Transform2DNode, source: &MatrixLike) -> () {
    decompose_matrix_to_transform2_d(target, source);
    invalidate_node_local_transform(target);
}

// Source: upstream/packages/node/src/transform2d.ts:110 (sha256:d96787413eb8a910e47e6d00e7836e538509bbb809236a60dc690980223d9e48)
pub fn set_node_transform2_d(target: &mut Transform2DNode, source: &Transform2DLike) -> () {
    target.pivot_x = source.pivot_x;
    target.pivot_y = source.pivot_y;
    target.rotation = source.rotation;
    target.scale_x = source.scale_x;
    target.scale_y = source.scale_y;
    target.skew_x = source.skew_x;
    target.skew_y = source.skew_y;
    target.x = source.x;
    target.y = source.y;
    invalidate_node_local_transform(target);
}

// Source: upstream/packages/node/src/transform2d.ts:126 (sha256:8455a54fc438991d8c55c6ae3e5d0fca20a65b4f6b8ac7e56bd48ceb51ecfdfc)
#[derive(Clone)]
struct RecomputeLocalTransform2DRecord1 {
    __flight_identity: std::sync::Arc<()>,
}
impl PartialEq for RecomputeLocalTransform2DRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

fn recompute_local_transform2_d(
    target: &Transform2DNode,
    runtime: &mut RecomputeLocalTransform2DRecord1,
) -> () {
    if (target.rotation != runtime.rotation_angle) {
        let mut angle = (target.rotation % 360.0_f64);
        if (angle > 180.0_f64) {
            angle -= 360.0_f64;
        } else {
            if (angle < (-180.0_f64)) {
                angle += 360.0_f64;
            }
        }
        let rad = (angle * DEG_TO_RAD);
        let sin = (rad).sin();
        let cos = (rad).cos();
        runtime.rotation_angle = angle;
        runtime.rotation_sine = sin;
        runtime.rotation_cosine = cos;
    }
    if ((runtime.local_matrix).clone()).is_none() {
        runtime.local_matrix = Some(create_matrix(None, None, None, None, None, None));
    }
    let mut matrix = (runtime.local_matrix).clone();
    if (target.skew_x == 0.0_f64) && (target.skew_y == 0.0_f64) {
        matrix.as_mut().unwrap().a = (runtime.rotation_cosine * target.scale_x);
        matrix.as_mut().unwrap().b = (runtime.rotation_sine * target.scale_x);
        matrix.as_mut().unwrap().c = ((-runtime.rotation_sine) * target.scale_y);
        matrix.as_mut().unwrap().d = (runtime.rotation_cosine * target.scale_y);
    } else {
        let rad_y = ((runtime.rotation_angle + target.skew_y) * DEG_TO_RAD);
        let rad_x = ((runtime.rotation_angle + target.skew_x) * DEG_TO_RAD);
        matrix.as_mut().unwrap().a = ((rad_y).cos() * target.scale_x);
        matrix.as_mut().unwrap().b = ((rad_y).sin() * target.scale_x);
        matrix.as_mut().unwrap().c = ((-(rad_x).sin()) * target.scale_y);
        matrix.as_mut().unwrap().d = ((rad_x).cos() * target.scale_y);
    }
    matrix.as_mut().unwrap().tx = (target.x
        - ((matrix.as_mut().unwrap().a * target.pivot_x)
            + (matrix.as_mut().unwrap().c * target.pivot_y)));
    matrix.as_mut().unwrap().ty = (target.y
        - ((matrix.as_mut().unwrap().b * target.pivot_x)
            + (matrix.as_mut().unwrap().d * target.pivot_y)));
    runtime.local_transform_using_local_transform_id = runtime.local_transform_id;
}

// Source: upstream/packages/node/src/transform2d.ts:166 (sha256:2cd251e9cb6acca7f0c6c73e1ca4e2266937f7bd32a9f707f73f7e5307de0e47)
#[derive(Clone)]
struct RecomputeWorldTransform2DRecord1 {
    __flight_identity: std::sync::Arc<()>,
}
impl PartialEq for RecomputeWorldTransform2DRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

fn recompute_world_transform2_d(
    target: &Transform2DNode,
    runtime: &mut RecomputeWorldTransform2DRecord1,
    parent_runtime: Option<RecomputeWorldTransform2DRecord1>,
) -> () {
    if ((runtime.world_matrix).clone()).is_none() {
        runtime.world_matrix = Some(create_matrix(None, None, None, None, None, None));
    }
    ensure_node_local_matrix(target);
    if (parent_runtime).is_some() {
        {
            let __flight_argument_2 = (runtime.local_matrix).clone();
            multiply_matrix(
                &mut runtime.world_matrix,
                &parent_runtime.as_ref().unwrap().world_matrix,
                &__flight_argument_2,
            )
        };
    } else {
        {
            let __flight_argument_1 = (runtime.local_matrix).clone();
            copy_matrix(&mut runtime.world_matrix, &__flight_argument_1)
        };
    }
    compute_node_world_transform_revision(
        runtime,
        Some(((parent_runtime).clone().unwrap()).clone()),
    );
}

// Source: upstream/packages/node/src/transform2d.ts:181 (sha256:20f0ca1e133840394a2a40394cc19c0be291922e87a2b68980feb609e87508f4)
const DEG_TO_RAD: f64 = 0.017453292519943295_f64;
