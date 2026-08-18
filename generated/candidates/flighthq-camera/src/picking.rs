// @generated from upstream/packages/camera/src/picking.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::get_camera3_d_view_projection_matrix4;
use flighthq_geometry::{
    create_matrix4, create_vector3, inverse_matrix4, normalize_vector3, subtract_vector3,
};
use flighthq_types::{Camera3D, Matrix4, Matrix4Like, Ray3DLike, Vector3, Vector3Like};

// Source: upstream/packages/camera/src/picking.ts:22 (sha256:3880424ba27cd5b60e50d008a3f16a3042e9db69ffb375a352b8221e155f9274)
pub fn get_camera3_d_screen_to_world_ray(
    out: &mut Ray3DLike,
    camera: &Camera3D,
    ndc_x: f64,
    ndc_y: f64,
    aspect: f64,
) -> bool {
    get_camera3_d_view_projection_matrix4(
        &mut (*__SCRATCH_VIEW_PROJECTION.lock().unwrap()),
        camera,
        aspect,
    );
    if (!inverse_matrix4(&mut (*__SCRATCH_INVERSE_VP.lock().unwrap()), &{
        let __flight_source = &(*__SCRATCH_VIEW_PROJECTION.lock().unwrap());
        Matrix4Like {
            __flight_identity: std::sync::Arc::clone(&__flight_source.__flight_identity),
            __flight_entity_runtime: std::sync::Arc::clone(
                &__flight_source.__flight_entity_runtime,
            ),
            m: (__flight_source.m).clone(),
        }
    })) {
        return false;
    }
    let nx = ndc_x;
    let ny = ndc_y;
    let mut near_x = ((((((*__SCRATCH_INVERSE_VP.lock().unwrap()).m[0.0_f64 as usize] as f64)
        * nx)
        + (((*__SCRATCH_INVERSE_VP.lock().unwrap()).m[4.0_f64 as usize] as f64) * ny))
        + (((*__SCRATCH_INVERSE_VP.lock().unwrap()).m[8.0_f64 as usize] as f64) * (-1.0_f64)))
        + ((*__SCRATCH_INVERSE_VP.lock().unwrap()).m[12.0_f64 as usize] as f64));
    let mut near_y = ((((((*__SCRATCH_INVERSE_VP.lock().unwrap()).m[1.0_f64 as usize] as f64)
        * nx)
        + (((*__SCRATCH_INVERSE_VP.lock().unwrap()).m[5.0_f64 as usize] as f64) * ny))
        + (((*__SCRATCH_INVERSE_VP.lock().unwrap()).m[9.0_f64 as usize] as f64) * (-1.0_f64)))
        + ((*__SCRATCH_INVERSE_VP.lock().unwrap()).m[13.0_f64 as usize] as f64));
    let mut near_z = ((((((*__SCRATCH_INVERSE_VP.lock().unwrap()).m[2.0_f64 as usize] as f64)
        * nx)
        + (((*__SCRATCH_INVERSE_VP.lock().unwrap()).m[6.0_f64 as usize] as f64) * ny))
        + (((*__SCRATCH_INVERSE_VP.lock().unwrap()).m[10.0_f64 as usize] as f64) * (-1.0_f64)))
        + ((*__SCRATCH_INVERSE_VP.lock().unwrap()).m[14.0_f64 as usize] as f64));
    let near_w = ((((((*__SCRATCH_INVERSE_VP.lock().unwrap()).m[3.0_f64 as usize] as f64) * nx)
        + (((*__SCRATCH_INVERSE_VP.lock().unwrap()).m[7.0_f64 as usize] as f64) * ny))
        + (((*__SCRATCH_INVERSE_VP.lock().unwrap()).m[11.0_f64 as usize] as f64) * (-1.0_f64)))
        + ((*__SCRATCH_INVERSE_VP.lock().unwrap()).m[15.0_f64 as usize] as f64));
    if (near_w != 0.0_f64) {
        let inv_w = (1.0_f64 / near_w);
        near_x *= inv_w;
        near_y *= inv_w;
        near_z *= inv_w;
    }
    let mut far_x = ((((((*__SCRATCH_INVERSE_VP.lock().unwrap()).m[0.0_f64 as usize] as f64)
        * nx)
        + (((*__SCRATCH_INVERSE_VP.lock().unwrap()).m[4.0_f64 as usize] as f64) * ny))
        + ((*__SCRATCH_INVERSE_VP.lock().unwrap()).m[8.0_f64 as usize] as f64))
        + ((*__SCRATCH_INVERSE_VP.lock().unwrap()).m[12.0_f64 as usize] as f64));
    let mut far_y = ((((((*__SCRATCH_INVERSE_VP.lock().unwrap()).m[1.0_f64 as usize] as f64)
        * nx)
        + (((*__SCRATCH_INVERSE_VP.lock().unwrap()).m[5.0_f64 as usize] as f64) * ny))
        + ((*__SCRATCH_INVERSE_VP.lock().unwrap()).m[9.0_f64 as usize] as f64))
        + ((*__SCRATCH_INVERSE_VP.lock().unwrap()).m[13.0_f64 as usize] as f64));
    let mut far_z = ((((((*__SCRATCH_INVERSE_VP.lock().unwrap()).m[2.0_f64 as usize] as f64)
        * nx)
        + (((*__SCRATCH_INVERSE_VP.lock().unwrap()).m[6.0_f64 as usize] as f64) * ny))
        + ((*__SCRATCH_INVERSE_VP.lock().unwrap()).m[10.0_f64 as usize] as f64))
        + ((*__SCRATCH_INVERSE_VP.lock().unwrap()).m[14.0_f64 as usize] as f64));
    let far_w = ((((((*__SCRATCH_INVERSE_VP.lock().unwrap()).m[3.0_f64 as usize] as f64) * nx)
        + (((*__SCRATCH_INVERSE_VP.lock().unwrap()).m[7.0_f64 as usize] as f64) * ny))
        + ((*__SCRATCH_INVERSE_VP.lock().unwrap()).m[11.0_f64 as usize] as f64))
        + ((*__SCRATCH_INVERSE_VP.lock().unwrap()).m[15.0_f64 as usize] as f64));
    if (far_w != 0.0_f64) {
        let inv_w = (1.0_f64 / far_w);
        far_x *= inv_w;
        far_y *= inv_w;
        far_z *= inv_w;
    }
    (*__SCRATCH_NEAR.lock().unwrap()).x = near_x;
    (*__SCRATCH_NEAR.lock().unwrap()).y = near_y;
    (*__SCRATCH_NEAR.lock().unwrap()).z = near_z;
    (*__SCRATCH_FAR.lock().unwrap()).x = far_x;
    (*__SCRATCH_FAR.lock().unwrap()).y = far_y;
    (*__SCRATCH_FAR.lock().unwrap()).z = far_z;
    subtract_vector3(
        &mut (*__SCRATCH_DIR.lock().unwrap()),
        &{
            let __flight_source = &(*__SCRATCH_FAR.lock().unwrap());
            Vector3Like {
                __flight_identity: std::sync::Arc::clone(&__flight_source.__flight_identity),
                __flight_entity_runtime: std::sync::Arc::clone(
                    &__flight_source.__flight_entity_runtime,
                ),
                x: __flight_source.x,
                y: __flight_source.y,
                z: __flight_source.z,
            }
        },
        &{
            let __flight_source = &(*__SCRATCH_NEAR.lock().unwrap());
            Vector3Like {
                __flight_identity: std::sync::Arc::clone(&__flight_source.__flight_identity),
                __flight_entity_runtime: std::sync::Arc::clone(
                    &__flight_source.__flight_entity_runtime,
                ),
                x: __flight_source.x,
                y: __flight_source.y,
                z: __flight_source.z,
            }
        },
    );
    {
        let __flight_argument_1 = (*__SCRATCH_DIR.lock().unwrap()).clone();
        normalize_vector3(&mut (*__SCRATCH_DIR.lock().unwrap()), &__flight_argument_1)
    };
    out.origin.x = near_x;
    out.origin.y = near_y;
    out.origin.z = near_z;
    out.direction.x = (*__SCRATCH_DIR.lock().unwrap()).x;
    out.direction.y = (*__SCRATCH_DIR.lock().unwrap()).y;
    out.direction.z = (*__SCRATCH_DIR.lock().unwrap()).z;
    return true;
}

// Source: upstream/packages/camera/src/picking.ts:84 (sha256:bf8ca45258f1437d26754bbeeaff9b3e8a4b06bd00eb9fdcf0552ab51b5b8a9c)
pub fn get_camera3_d_world_to_screen(
    out: &mut Vector3Like,
    camera: &Camera3D,
    world_point: &Vector3Like,
    aspect: f64,
) -> bool {
    get_camera3_d_view_projection_matrix4(
        &mut (*__SCRATCH_VIEW_PROJECTION.lock().unwrap()),
        camera,
        aspect,
    );
    let wx = world_point.x;
    let wy = world_point.y;
    let wz = world_point.z;
    let clip_x = ((((((*__SCRATCH_VIEW_PROJECTION.lock().unwrap()).m[0.0_f64 as usize] as f64)
        * wx)
        + (((*__SCRATCH_VIEW_PROJECTION.lock().unwrap()).m[4.0_f64 as usize] as f64) * wy))
        + (((*__SCRATCH_VIEW_PROJECTION.lock().unwrap()).m[8.0_f64 as usize] as f64) * wz))
        + ((*__SCRATCH_VIEW_PROJECTION.lock().unwrap()).m[12.0_f64 as usize] as f64));
    let clip_y = ((((((*__SCRATCH_VIEW_PROJECTION.lock().unwrap()).m[1.0_f64 as usize] as f64)
        * wx)
        + (((*__SCRATCH_VIEW_PROJECTION.lock().unwrap()).m[5.0_f64 as usize] as f64) * wy))
        + (((*__SCRATCH_VIEW_PROJECTION.lock().unwrap()).m[9.0_f64 as usize] as f64) * wz))
        + ((*__SCRATCH_VIEW_PROJECTION.lock().unwrap()).m[13.0_f64 as usize] as f64));
    let clip_z = ((((((*__SCRATCH_VIEW_PROJECTION.lock().unwrap()).m[2.0_f64 as usize] as f64)
        * wx)
        + (((*__SCRATCH_VIEW_PROJECTION.lock().unwrap()).m[6.0_f64 as usize] as f64) * wy))
        + (((*__SCRATCH_VIEW_PROJECTION.lock().unwrap()).m[10.0_f64 as usize] as f64) * wz))
        + ((*__SCRATCH_VIEW_PROJECTION.lock().unwrap()).m[14.0_f64 as usize] as f64));
    let clip_w = ((((((*__SCRATCH_VIEW_PROJECTION.lock().unwrap()).m[3.0_f64 as usize] as f64)
        * wx)
        + (((*__SCRATCH_VIEW_PROJECTION.lock().unwrap()).m[7.0_f64 as usize] as f64) * wy))
        + (((*__SCRATCH_VIEW_PROJECTION.lock().unwrap()).m[11.0_f64 as usize] as f64) * wz))
        + ((*__SCRATCH_VIEW_PROJECTION.lock().unwrap()).m[15.0_f64 as usize] as f64));
    if (clip_w <= 0.0_f64) {
        return false;
    }
    let inv_w = (1.0_f64 / clip_w);
    out.x = (clip_x * inv_w);
    out.y = (clip_y * inv_w);
    out.z = (clip_z * inv_w);
    return true;
}

// Source: upstream/packages/camera/src/picking.ts:110 (sha256:ea1bce46bff5117486aa66f0bc0c33f5ba239247bd135b339e34f20358b60428)
static __SCRATCH_VIEW_PROJECTION: std::sync::LazyLock<std::sync::Mutex<Matrix4>> =
    std::sync::LazyLock::new(|| {
        std::sync::Mutex::new(create_matrix4(
            None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            None, None,
        ))
    });

// Source: upstream/packages/camera/src/picking.ts:111 (sha256:9533d4b350fea262c12d05798919dbd7e09488baad44ff9ced667d0a21cfe6ff)
static __SCRATCH_INVERSE_VP: std::sync::LazyLock<std::sync::Mutex<Matrix4>> =
    std::sync::LazyLock::new(|| {
        std::sync::Mutex::new(create_matrix4(
            None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            None, None,
        ))
    });

// Source: upstream/packages/camera/src/picking.ts:112 (sha256:744e95910d21eaeb616e6b459dec7d0e200cd0cb6ecc7d6780a449d490dab0d4)
static __SCRATCH_NEAR: std::sync::LazyLock<std::sync::Mutex<Vector3>> =
    std::sync::LazyLock::new(|| std::sync::Mutex::new(create_vector3(None, None, None)));

// Source: upstream/packages/camera/src/picking.ts:113 (sha256:507b197cd038c26584a0dfc58842e84347024fe0530e472fc9449e996e6862e9)
static __SCRATCH_FAR: std::sync::LazyLock<std::sync::Mutex<Vector3>> =
    std::sync::LazyLock::new(|| std::sync::Mutex::new(create_vector3(None, None, None)));

// Source: upstream/packages/camera/src/picking.ts:114 (sha256:dd60ed1ea48d08bc8c0df044acfd4467ebcc6485ec4d0accd2c7efe1ced96983)
static __SCRATCH_DIR: std::sync::LazyLock<std::sync::Mutex<Vector3>> =
    std::sync::LazyLock::new(|| std::sync::Mutex::new(create_vector3(None, None, None)));
