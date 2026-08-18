// @generated from upstream/packages/camera/src/culling.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::get_camera3_d_view_projection_matrix4;
use flighthq_geometry::{
    create_frustum, create_matrix4, is_frustum_containing_point, is_frustum_intersecting_aabb,
    is_frustum_intersecting_sphere, set_frustum_from_matrix4,
};
use flighthq_types::{
    AabbLike, BoundingSphereLike, Camera3D, Frustum, FrustumLike, Matrix4, Matrix4Like, Vector3Like,
};

// Source: upstream/packages/camera/src/culling.ts:21 (sha256:c3ab00b6457993b3ee74d7f20738277e4436ede744875fa9e79cf04207285baa)
pub fn get_camera3_d_frustum(out: &mut FrustumLike, camera: &Camera3D, aspect: f64) -> () {
    get_camera3_d_view_projection_matrix4(
        &mut (*__SCRATCH_VIEW_PROJECTION.lock().unwrap()),
        camera,
        aspect,
    );
    set_frustum_from_matrix4(out, &{
        let __flight_source = &(*__SCRATCH_VIEW_PROJECTION.lock().unwrap());
        Matrix4Like {
            __flight_identity: std::sync::Arc::clone(&__flight_source.__flight_identity),
            __flight_entity_runtime: std::sync::Arc::clone(
                &__flight_source.__flight_entity_runtime,
            ),
            m: (__flight_source.m).clone(),
        }
    });
}

// Source: upstream/packages/camera/src/culling.ts:31 (sha256:3ffbf9f063ded81083fcb7f409a1c75a57a567f5170aa093d559dd245bbb9ac0)
pub fn is_box_in_camera3_d_frustum(camera: &Camera3D, aabb: &AabbLike, aspect: f64) -> bool {
    get_camera3_d_frustum(&mut (*__SCRATCH_FRUSTUM.lock().unwrap()), camera, aspect);
    return is_frustum_intersecting_aabb(
        &{
            let __flight_source = &(*__SCRATCH_FRUSTUM.lock().unwrap());
            FrustumLike {
                __flight_identity: std::sync::Arc::clone(&__flight_source.__flight_identity),
                __flight_entity_runtime: std::sync::Arc::clone(
                    &__flight_source.__flight_entity_runtime,
                ),
                bottom: (__flight_source.bottom).clone(),
                far: (__flight_source.far).clone(),
                left: (__flight_source.left).clone(),
                near: (__flight_source.near).clone(),
                right: (__flight_source.right).clone(),
                top: (__flight_source.top).clone(),
            }
        },
        aabb,
    );
}

// Source: upstream/packages/camera/src/culling.ts:38 (sha256:d6a0971345b7dfffb298d7c93cfd89f15183e1aa604c8b1923d96e773909caba)
pub fn is_point_in_camera3_d_frustum(camera: &Camera3D, point: &Vector3Like, aspect: f64) -> bool {
    get_camera3_d_frustum(&mut (*__SCRATCH_FRUSTUM.lock().unwrap()), camera, aspect);
    return is_frustum_containing_point(
        &{
            let __flight_source = &(*__SCRATCH_FRUSTUM.lock().unwrap());
            FrustumLike {
                __flight_identity: std::sync::Arc::clone(&__flight_source.__flight_identity),
                __flight_entity_runtime: std::sync::Arc::clone(
                    &__flight_source.__flight_entity_runtime,
                ),
                bottom: (__flight_source.bottom).clone(),
                far: (__flight_source.far).clone(),
                left: (__flight_source.left).clone(),
                near: (__flight_source.near).clone(),
                right: (__flight_source.right).clone(),
                top: (__flight_source.top).clone(),
            }
        },
        point,
    );
}

// Source: upstream/packages/camera/src/culling.ts:49 (sha256:f25ca5bf02754beb4795d241d4b86b30342454cc1e53e3d87eebcf67544722a0)
pub fn is_sphere_in_camera3_d_frustum(
    camera: &Camera3D,
    sphere: &BoundingSphereLike,
    aspect: f64,
) -> bool {
    get_camera3_d_frustum(&mut (*__SCRATCH_FRUSTUM.lock().unwrap()), camera, aspect);
    return is_frustum_intersecting_sphere(
        &{
            let __flight_source = &(*__SCRATCH_FRUSTUM.lock().unwrap());
            FrustumLike {
                __flight_identity: std::sync::Arc::clone(&__flight_source.__flight_identity),
                __flight_entity_runtime: std::sync::Arc::clone(
                    &__flight_source.__flight_entity_runtime,
                ),
                bottom: (__flight_source.bottom).clone(),
                far: (__flight_source.far).clone(),
                left: (__flight_source.left).clone(),
                near: (__flight_source.near).clone(),
                right: (__flight_source.right).clone(),
                top: (__flight_source.top).clone(),
            }
        },
        sphere,
    );
}

// Source: upstream/packages/camera/src/culling.ts:59 (sha256:ea1bce46bff5117486aa66f0bc0c33f5ba239247bd135b339e34f20358b60428)
static __SCRATCH_VIEW_PROJECTION: std::sync::LazyLock<std::sync::Mutex<Matrix4>> =
    std::sync::LazyLock::new(|| {
        std::sync::Mutex::new(create_matrix4(
            None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            None, None,
        ))
    });

// Source: upstream/packages/camera/src/culling.ts:60 (sha256:575e321cdd91797eb27cb92e6741e245ac48dd589abcfae81c51b2e49e2aa832)
static __SCRATCH_FRUSTUM: std::sync::LazyLock<std::sync::Mutex<Frustum>> =
    std::sync::LazyLock::new(|| std::sync::Mutex::new(create_frustum()));
