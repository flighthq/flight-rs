// @generated from upstream/packages/geometry/src/transform3d.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{compose_matrix4, create_quaternion, create_vector3, decompose_matrix4};
use flighthq_entity::create_entity;
use flighthq_types::{Matrix4Like, QuaternionLike, Transform3D, Transform3DLike, Vector3Like};

// Source: upstream/packages/geometry/src/transform3d.ts:9 (sha256:ae6085c8e34872730f39053f3dc2a1fbd4786461b4447caa01e39ce0c0c4d3f5)
pub fn compose_matrix4_from_transform3_d(out: &mut Matrix4Like, source: &Transform3DLike) -> () {
    compose_matrix4(
        out,
        &{
            let __flight_source = &(source.position);
            Vector3Like {
                __flight_identity: std::sync::Arc::clone(&__flight_source.__flight_identity),
                x: __flight_source.x,
                y: __flight_source.y,
                z: __flight_source.z,
            }
        },
        &{
            let __flight_source = &(source.rotation);
            QuaternionLike {
                __flight_identity: std::sync::Arc::clone(&__flight_source.__flight_identity),
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
                x: __flight_source.x,
                y: __flight_source.y,
                z: __flight_source.z,
            }
        },
    );
}

// Source: upstream/packages/geometry/src/transform3d.ts:15 (sha256:55986cc73a04e07fb0cde6bbc77e2cc699f7ea64b84db2fe9e7970c9c5c5d732)
pub fn create_transform3_d() -> Transform3D {
    return create_entity(Some(Transform3D {
        __flight_identity: std::sync::Arc::new(()),
        rotation: create_quaternion(None, None, None, None),
        scale: create_vector3(Some(1.0_f64), Some(1.0_f64), Some(1.0_f64)),
        position: create_vector3(None, None, None),
    }));
}

// Source: upstream/packages/geometry/src/transform3d.ts:25 (sha256:a43de0257a1fec8cc23ca1854f7987ee3d40dc4858707633c9ef991cda28f511)
pub fn decompose_matrix4_to_transform3_d(out: &mut Transform3DLike, m: &Matrix4Like) -> () {
    decompose_matrix4(&mut out.position, &mut out.rotation, &mut out.scale, m);
}
