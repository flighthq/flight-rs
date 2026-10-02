// @generated from upstream/packages/geometry/src/transform3d.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{compose_matrix4, create_quaternion, create_vector3, decompose_matrix4};
use flighthq_entity::{allocate_entity, finish_entity};
use flighthq_types::{
    EntityConstruction, Matrix4Like, Quaternion, QuaternionLike, Transform3D, Transform3DLike,
    Vector3, Vector3Like,
};

// Source: upstream/packages/geometry/src/transform3d.ts:16 (sha256:ae6085c8e34872730f39053f3dc2a1fbd4786461b4447caa01e39ce0c0c4d3f5)
pub fn compose_matrix4_from_transform3_d(out: &mut Matrix4Like, source: &Transform3DLike) -> () {
    compose_matrix4(
        out,
        &{
            let __flight_source = &(source.position);
            Vector3Like {
                __flight_identity: std::sync::Arc::clone(&__flight_source.__flight_identity),
                __flight_entity_runtime: std::sync::Arc::clone(
                    &__flight_source.__flight_entity_runtime,
                ),
                __flight_entity_snapshot: __flight_source.__flight_entity_snapshot.clone(),
                x: __flight_source.x,
                y: __flight_source.y,
                z: __flight_source.z,
            }
        },
        &{
            let __flight_source = &(source.rotation);
            QuaternionLike {
                __flight_identity: std::sync::Arc::clone(&__flight_source.__flight_identity),
                __flight_entity_runtime: std::sync::Arc::clone(
                    &__flight_source.__flight_entity_runtime,
                ),
                __flight_entity_snapshot: __flight_source.__flight_entity_snapshot.clone(),
                x: __flight_source.x,
                y: __flight_source.y,
                z: __flight_source.z,
                w: __flight_source.w,
            }
        },
        &{
            let __flight_source = &(source.scale);
            Vector3Like {
                __flight_identity: std::sync::Arc::clone(&__flight_source.__flight_identity),
                __flight_entity_runtime: std::sync::Arc::clone(
                    &__flight_source.__flight_entity_runtime,
                ),
                __flight_entity_snapshot: __flight_source.__flight_entity_snapshot.clone(),
                x: __flight_source.x,
                y: __flight_source.y,
                z: __flight_source.z,
            }
        },
    );
}

// Source: upstream/packages/geometry/src/transform3d.ts:22 (sha256:f109f38ae7e068b588187582120eaf74f7baccf0a88730e523764a46ee7cf16c)
pub fn create_transform3_d() -> Transform3D {
    let position = create_vector3(None, None, None);
    let rotation = create_quaternion(None, None, None, None);
    let scale = create_vector3(Some(1.0_f64), Some(1.0_f64), Some(1.0_f64));
    let mut out = allocate_entity();
    initialize_transform3_d((out).clone(), &position, &rotation, &scale);
    return finish_entity((out).clone());
}

// Source: upstream/packages/geometry/src/transform3d.ts:33 (sha256:a43de0257a1fec8cc23ca1854f7987ee3d40dc4858707633c9ef991cda28f511)
pub fn decompose_matrix4_to_transform3_d(out: &mut Transform3DLike, m: &Matrix4Like) -> () {
    decompose_matrix4(&mut out.position, &mut out.rotation, &mut out.scale, m);
}

// Source: upstream/packages/geometry/src/transform3d.ts:37 (sha256:664f7a6ea0d9fcdcd7aab9cef93bdd407958306dd7139acc5cbd40492b604156)
pub fn initialize_transform3_d(
    out: EntityConstruction<Transform3D>,
    position: &Vector3,
    rotation: &Quaternion,
    scale: &Vector3,
) -> () {
    crate::host_set("host.position", position);
    crate::host_set("host.rotation", rotation);
    crate::host_set("host.scale", scale);
}
