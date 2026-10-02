// @generated from upstream/packages/camera/src/reflection.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_geometry::{create_matrix4, multiply_matrix4, set_matrix4};
use flighthq_types::{Camera3D, Matrix4, Matrix4Like, PlaneLike};

// Source: upstream/packages/camera/src/reflection.ts:14 (sha256:c06eb23a96bc285d11d9b302b0eb019fe6e5d904d2718d7294b0f129c103cc41)
pub fn apply_oblique_near_clip_plane(
    projection: &mut Matrix4Like,
    view_space_plane: &PlaneLike,
) -> () {
    let cx = view_space_plane.a;
    let cy = view_space_plane.b;
    let cz = view_space_plane.c;
    let cw = view_space_plane.d;
    let qx = ((_sgn(cx) + (projection.m[8.0_f64 as usize] as f64))
        / (projection.m[0.0_f64 as usize] as f64));
    let qy = ((_sgn(cy) + (projection.m[9.0_f64 as usize] as f64))
        / (projection.m[5.0_f64 as usize] as f64));
    let qz = (-1.0_f64);
    let qw = ((1.0_f64 + (projection.m[10.0_f64 as usize] as f64))
        / (projection.m[14.0_f64 as usize] as f64));
    let k = (2.0_f64 / ((((cx * qx) + (cy * qy)) + (cz * qz)) + (cw * qw)));
    projection.m[2.0_f64 as usize] = ((k * cx) - (projection.m[3.0_f64 as usize] as f64)) as f32;
    projection.m[6.0_f64 as usize] = ((k * cy) - (projection.m[7.0_f64 as usize] as f64)) as f32;
    projection.m[10.0_f64 as usize] = ((k * cz) - (projection.m[11.0_f64 as usize] as f64)) as f32;
    projection.m[14.0_f64 as usize] = ((k * cw) - (projection.m[15.0_f64 as usize] as f64)) as f32;
}

// Source: upstream/packages/camera/src/reflection.ts:40 (sha256:fe0ad89698f2341ccf47805bb4b00a7d961a48052883e7a1cbff9e50820affc7)
pub fn reflect_camera3_d_by_plane(out: &mut Camera3D, camera: &Camera3D, plane: &PlaneLike) -> () {
    let pa = plane.a;
    let pb = plane.b;
    let pc = plane.c;
    let pd = plane.d;
    let near = camera.near;
    let far = camera.far;
    let jx = camera.jitter.x;
    let jy = camera.jitter.y;
    let near_clip_plane = (camera.near_clip_plane).clone();
    set_matrix4(
        &mut (*__SCRATCH_REFLECTION.lock().unwrap()),
        (1.0_f64 - ((2.0_f64 * pa) * pa)),
        (((-2.0_f64) * pa) * pb),
        (((-2.0_f64) * pa) * pc),
        0.0_f64,
        (((-2.0_f64) * pa) * pb),
        (1.0_f64 - ((2.0_f64 * pb) * pb)),
        (((-2.0_f64) * pb) * pc),
        0.0_f64,
        (((-2.0_f64) * pa) * pc),
        (((-2.0_f64) * pb) * pc),
        (1.0_f64 - ((2.0_f64 * pc) * pc)),
        0.0_f64,
        (((-2.0_f64) * pa) * pd),
        (((-2.0_f64) * pb) * pd),
        (((-2.0_f64) * pc) * pd),
        1.0_f64,
    );
    multiply_matrix4(
        &mut out.view,
        &{
            let __flight_source = &(camera.view);
            Matrix4Like {
                __flight_identity: std::sync::Arc::clone(&__flight_source.__flight_identity),
                __flight_entity_runtime: std::sync::Arc::clone(
                    &__flight_source.__flight_entity_runtime,
                ),
                __flight_entity_snapshot: __flight_source.__flight_entity_snapshot.clone(),
                m: (__flight_source.m).clone(),
            }
        },
        &{
            let __flight_source = &(*__SCRATCH_REFLECTION.lock().unwrap());
            Matrix4Like {
                __flight_identity: std::sync::Arc::clone(&__flight_source.__flight_identity),
                __flight_entity_runtime: std::sync::Arc::clone(
                    &__flight_source.__flight_entity_runtime,
                ),
                __flight_entity_snapshot: __flight_source.__flight_entity_snapshot.clone(),
                m: (__flight_source.m).clone(),
            }
        },
    );
    out.near = near;
    out.far = far;
    out.jitter.x = jx;
    out.jitter.y = jy;
    out.near_clip_plane = (near_clip_plane).clone();
    out.projection = (camera.projection).clone();
}

// Source: upstream/packages/camera/src/reflection.ts:81 (sha256:bdb8ba1e63537280c586eccb825da2777ae896558de0f5c7aec8b532067aad6e)
fn _sgn(x: f64) -> f64 {
    return if (x < 0.0_f64) { (-1.0_f64) } else { 1.0_f64 };
}

// Source: upstream/packages/camera/src/reflection.ts:85 (sha256:97f4c78b5e2bcd987d1f015bea97d32f7d4547bf0354cf0ec4a0a2eac21108ce)
static __SCRATCH_REFLECTION: std::sync::LazyLock<std::sync::Mutex<Matrix4>> =
    std::sync::LazyLock::new(|| {
        std::sync::Mutex::new(create_matrix4(
            None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            None, None,
        ))
    });
