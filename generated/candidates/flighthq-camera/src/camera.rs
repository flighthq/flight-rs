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
use flighthq_types::{Camera, Matrix4, Matrix4Like, Projection, Vector3Like};

// Source: upstream/packages/camera/src/camera.ts:13 (sha256:1fba96bfa30be86b4f72c54ae12a2ae0982043161e44b1ce04eb30311aec305d)
pub fn create_camera(opts: &CameraOptions) -> Camera {
    return create_entity(Some(Camera {
        __flight_identity: std::sync::Arc::new(()),
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

// Source: upstream/packages/camera/src/camera.ts:32 (sha256:db741dbbe7f9529c6df98f8f34c7638922af42eb927bc5d796cb1756fa62e844)
pub fn get_camera_inverse_view_projection_matrix4(
    out: &mut Matrix4Like,
    camera: &Camera,
    aspect: f64,
) -> bool {
    get_camera_view_projection_matrix4(
        &mut (*__SCRATCH_VIEW_PROJECTION.lock().unwrap()),
        camera,
        aspect,
    );
    return inverse_matrix4(out, &{
        let __flight_source = &(*__SCRATCH_VIEW_PROJECTION.lock().unwrap());
        Matrix4Like {
            __flight_identity: std::sync::Arc::clone(&__flight_source.__flight_identity),
            m: (__flight_source.m).clone(),
        }
    });
}

// Source: upstream/packages/camera/src/camera.ts:47 (sha256:f297dd8a29f983503919530244ded9bb913147e260aa4575b43161154fff4210)
pub fn get_camera_view_projection_matrix4(
    out: &mut Matrix4Like,
    camera: &Camera,
    aspect: f64,
) -> () {
    set_projection_matrix4(
        &mut (*__SCRATCH_PROJECTION.lock().unwrap()),
        &camera.projection,
        aspect,
        camera.near,
        camera.far,
    );
    multiply_matrix4(
        out,
        &{
            let __flight_source = &(*__SCRATCH_PROJECTION.lock().unwrap());
            Matrix4Like {
                __flight_identity: std::sync::Arc::clone(&__flight_source.__flight_identity),
                m: (__flight_source.m).clone(),
            }
        },
        &{
            let __flight_source = &(camera.view);
            Matrix4Like {
                __flight_identity: std::sync::Arc::clone(&__flight_source.__flight_identity),
                m: (__flight_source.m).clone(),
            }
        },
    );
}

// Source: upstream/packages/camera/src/camera.ts:54 (sha256:d664784b9b748d6677868774dd9f0fa3b5dfd9b8507c83fda461b16850f92bf5)
pub fn set_camera_jitter(camera: &mut Camera, x: f64, y: f64) -> () {
    camera.jitter.x = x;
    camera.jitter.y = y;
}

// Source: upstream/packages/camera/src/camera.ts:64 (sha256:a62836bcf741dc96294dd2068b2ec85e6bc882b5081805db412d0c6ebf77381f)
pub fn set_camera_view_matrix4_from_look_at(
    camera: &mut Camera,
    eye: &Vector3Like,
    target: &Vector3Like,
    up: &Vector3Like,
) -> () {
    set_matrix4_look_at(&mut camera.view, eye, target, up);
}

// Source: upstream/packages/camera/src/camera.ts:75 (sha256:0f44286840f426aeaa333d62d4990af5d349651c9bb67157de6e428ac573f23b)
pub fn set_camera_view_matrix4_from_matrix4(camera: &mut Camera, view: &Matrix4Like) -> () {
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

// Source: upstream/packages/camera/src/camera.ts:85 (sha256:04b1094a606a36166ccef2bcdec2ffdc1d333141ff2e711eb6ef0e4bbc3ee3da)
pub fn update_camera_inverse_view_projection(camera: &mut Camera, aspect: f64) -> bool {
    let ok = get_camera_inverse_view_projection_matrix4(
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

// Source: upstream/packages/camera/src/camera.ts:95 (sha256:22118ce33ca6e70dccedd29eb2eec28a141e76d3f5613818b25d37f258d4e6a6)
#[derive(Clone)]
pub struct CameraOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub far: f64,
    pub near: f64,
    pub projection: Projection,
}
impl PartialEq for CameraOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/camera/src/camera.ts:102 (sha256:4ad6a35d9104577101cd617015c99a42397b2a539dbed3a36ad1c58c3356133d)
static __SCRATCH_INVERSE: std::sync::LazyLock<std::sync::Mutex<Matrix4>> =
    std::sync::LazyLock::new(|| {
        std::sync::Mutex::new(create_matrix4(
            None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            None, None,
        ))
    });

// Source: upstream/packages/camera/src/camera.ts:103 (sha256:25f0673e0ee20250bdc4881d41975acf1c9d47c88eec611dac8217aedc5ded65)
static __SCRATCH_PROJECTION: std::sync::LazyLock<std::sync::Mutex<Matrix4>> =
    std::sync::LazyLock::new(|| {
        std::sync::Mutex::new(create_matrix4(
            None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            None, None,
        ))
    });

// Source: upstream/packages/camera/src/camera.ts:104 (sha256:ea1bce46bff5117486aa66f0bc0c33f5ba239247bd135b339e34f20358b60428)
static __SCRATCH_VIEW_PROJECTION: std::sync::LazyLock<std::sync::Mutex<Matrix4>> =
    std::sync::LazyLock::new(|| {
        std::sync::Mutex::new(create_matrix4(
            None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            None, None,
        ))
    });
