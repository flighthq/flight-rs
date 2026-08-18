// @generated from upstream/packages/camera/src/camera.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::set_projection_matrix4;
use flighthq_entity::create_entity;
use flighthq_geometry::{
    create_matrix4, create_vector2, inverse_matrix4, multiply_matrix4, set_matrix4_look_at,
};
use flighthq_types::{Camera3D, Camera3DOptions, Matrix4, Matrix4Like, Projection, Vector3Like};

// Source: upstream/packages/camera/src/camera.ts:19 (sha256:f444bf68bb008a5cd44a273e6244eac4add2ab2c35bc99d720c1301d7cc6827d)
pub fn create_camera3_d(opts: &Camera3DOptions) -> Camera3D {
    return create_entity(Some(Camera3D {
        __flight_identity: std::sync::Arc::new(()),
        __flight_entity_runtime: Default::default(),
        far: opts.far,
        inverse_view_projection: create_matrix4(
            None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            None, None,
        ),
        jitter: create_vector2(Some(0.0_f64), Some(0.0_f64)),
        near: opts.near,
        projection: (opts.projection).clone(),
        view: create_matrix4(
            None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            None, None,
        ),
    }));
}

// Source: upstream/packages/camera/src/camera.ts:37 (sha256:b391dbfbff4df4ff85b3d3a6a40b10b13c0b4479a2c2818679d99587a51ba801)
pub fn get_camera3_d_inverse_view_projection_matrix4(
    out: &mut Matrix4Like,
    camera: &Camera3D,
    aspect: f64,
) -> bool {
    get_camera3_d_view_projection_matrix4(
        &mut (*__SCRATCH_VIEW_PROJECTION.lock().unwrap()),
        camera,
        aspect,
    );
    return inverse_matrix4(out, &{
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

// Source: upstream/packages/camera/src/camera.ts:52 (sha256:7ee87804c11ab798faba13113df8cad7d13e3d24ab7233c6c926d3158f267997)
pub fn get_camera3_d_view_projection_matrix4(
    out: &mut Matrix4Like,
    camera: &Camera3D,
    aspect: f64,
) -> () {
    set_projection_matrix4(
        &mut (*__SCRATCH_PROJECTION.lock().unwrap()),
        &camera.projection,
        aspect,
        camera.near,
        camera.far,
    );
    apply_camera3_d_projection_jitter(
        &mut (*__SCRATCH_PROJECTION.lock().unwrap()),
        camera.jitter.x,
        camera.jitter.y,
    );
    multiply_matrix4(
        out,
        &{
            let __flight_source = &(*__SCRATCH_PROJECTION.lock().unwrap());
            Matrix4Like {
                __flight_identity: std::sync::Arc::clone(&__flight_source.__flight_identity),
                __flight_entity_runtime: std::sync::Arc::clone(
                    &__flight_source.__flight_entity_runtime,
                ),
                m: (__flight_source.m).clone(),
            }
        },
        &{
            let __flight_source = &(camera.view);
            Matrix4Like {
                __flight_identity: std::sync::Arc::clone(&__flight_source.__flight_identity),
                __flight_entity_runtime: std::sync::Arc::clone(
                    &__flight_source.__flight_entity_runtime,
                ),
                m: (__flight_source.m).clone(),
            }
        },
    );
}

// Source: upstream/packages/camera/src/camera.ts:64 (sha256:fdad39b4d9c8174aac7ab3346322007d2af9edb7fbda13b0eba8b7f6415b73b6)
pub fn set_camera3_d_aspect(camera: &mut Camera3D, aspect: f64) -> () {
    if matches!(&(camera.projection), flighthq_types::Projection::B(_)) {
        (match (camera.projection).clone() {
            flighthq_types::Projection::A(_) => panic!("TypeScript union narrowing failed"),
            flighthq_types::Projection::B(value) => value,
        })
        .aspect = aspect;
        return;
    }
    (match (camera.projection).clone() {
        flighthq_types::Projection::A(value) => value,
        flighthq_types::Projection::B(_) => panic!("TypeScript union narrowing failed"),
    })
    .half_width = ((match (camera.projection).clone() {
        flighthq_types::Projection::A(value) => value,
        flighthq_types::Projection::B(_) => panic!("TypeScript union narrowing failed"),
    })
    .half_height
        * aspect);
}

// Source: upstream/packages/camera/src/camera.ts:75 (sha256:bfb927850796b081447fa35ebbbb118b6fc5923dfc60f5e2f704422bd1c5534d)
pub fn set_camera3_d_jitter(camera: &mut Camera3D, x: f64, y: f64) -> () {
    camera.jitter.x = x;
    camera.jitter.y = y;
}

// Source: upstream/packages/camera/src/camera.ts:85 (sha256:ad72d8902dbe544369f6f78a21fed72fbd73f8d20cb6d5b1ff37fc11a03992c8)
pub fn set_camera3_d_view_matrix4_from_look_at(
    camera: &mut Camera3D,
    eye: &Vector3Like,
    target: &Vector3Like,
    up: &Vector3Like,
) -> () {
    set_matrix4_look_at(&mut camera.view, eye, target, up);
}

// Source: upstream/packages/camera/src/camera.ts:96 (sha256:a0e61b2bf4214f423ea6f3c3de9b77900e5c23f2f4cf6b3532994734dfde2d3c)
pub fn set_camera3_d_view_matrix4_from_matrix4(camera: &mut Camera3D, view: &Matrix4Like) -> () {
    {
        let __flight_offset = (0.0_f64) as usize;
        let __flight_values: Vec<f32> = ((view.m).clone())
            .iter()
            .map(|value| (*value) as f32)
            .collect();
        camera.view.m[__flight_offset..__flight_offset + __flight_values.len()]
            .copy_from_slice(&__flight_values);
    };
}

// Source: upstream/packages/camera/src/camera.ts:104 (sha256:aba0c8297b05d3db0a1586783d6cd8b5caa3ac4353d3cff7db19632f419cd778)
pub fn update_camera3_d_inverse_view_projection(camera: &mut Camera3D, aspect: f64) -> bool {
    let ok = get_camera3_d_inverse_view_projection_matrix4(
        &mut (*__SCRATCH_INVERSE.lock().unwrap()),
        camera,
        aspect,
    );
    if ok {
        {
            let __flight_offset = (0.0_f64) as usize;
            let __flight_values: Vec<f32> = (((*__SCRATCH_INVERSE.lock().unwrap()).m).clone())
                .iter()
                .map(|value| (*value) as f32)
                .collect();
            camera.inverse_view_projection.m
                [__flight_offset..__flight_offset + __flight_values.len()]
                .copy_from_slice(&__flight_values);
        };
    }
    return ok;
}

// Source: upstream/packages/camera/src/camera.ts:113 (sha256:7ec7d78ecc2375afdd5fdace3128662b774baf283c6d578b2e34d225a67cf60b)
fn apply_camera3_d_projection_jitter(out: &mut Matrix4Like, x: f64, y: f64) -> () {
    out.m[0.0_f64 as usize] += (x * (out.m[3.0_f64 as usize] as f64)) as f32;
    out.m[4.0_f64 as usize] += (x * (out.m[7.0_f64 as usize] as f64)) as f32;
    out.m[8.0_f64 as usize] += (x * (out.m[11.0_f64 as usize] as f64)) as f32;
    out.m[12.0_f64 as usize] += (x * (out.m[15.0_f64 as usize] as f64)) as f32;
    out.m[1.0_f64 as usize] += (y * (out.m[3.0_f64 as usize] as f64)) as f32;
    out.m[5.0_f64 as usize] += (y * (out.m[7.0_f64 as usize] as f64)) as f32;
    out.m[9.0_f64 as usize] += (y * (out.m[11.0_f64 as usize] as f64)) as f32;
    out.m[13.0_f64 as usize] += (y * (out.m[15.0_f64 as usize] as f64)) as f32;
}

// Source: upstream/packages/camera/src/camera.ts:126 (sha256:4ad6a35d9104577101cd617015c99a42397b2a539dbed3a36ad1c58c3356133d)
static __SCRATCH_INVERSE: std::sync::LazyLock<std::sync::Mutex<Matrix4>> =
    std::sync::LazyLock::new(|| {
        std::sync::Mutex::new(create_matrix4(
            None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            None, None,
        ))
    });

// Source: upstream/packages/camera/src/camera.ts:127 (sha256:25f0673e0ee20250bdc4881d41975acf1c9d47c88eec611dac8217aedc5ded65)
static __SCRATCH_PROJECTION: std::sync::LazyLock<std::sync::Mutex<Matrix4>> =
    std::sync::LazyLock::new(|| {
        std::sync::Mutex::new(create_matrix4(
            None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            None, None,
        ))
    });

// Source: upstream/packages/camera/src/camera.ts:128 (sha256:ea1bce46bff5117486aa66f0bc0c33f5ba239247bd135b339e34f20358b60428)
static __SCRATCH_VIEW_PROJECTION: std::sync::LazyLock<std::sync::Mutex<Matrix4>> =
    std::sync::LazyLock::new(|| {
        std::sync::Mutex::new(create_matrix4(
            None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            None, None,
        ))
    });
