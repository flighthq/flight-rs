// @generated from upstream/packages/camera/src/cubeCapture.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::create_perspective_projection;
use flighthq_geometry::set_matrix4_look_at;
use flighthq_types::{
    Camera3D, PerspectiveProjection, PerspectiveProjectionOptions, Projection, RawProjection,
    Vector3Like,
};

// Source: upstream/packages/camera/src/cubeCapture.ts:12 (sha256:3bbddf59b0328197b3433c85706308d8b368a63c5af1101449aa74beecc67c0d)
pub fn get_cube_capture_face_camera3_d(
    out: &mut Camera3D,
    position: &Vector3Like,
    face: f64,
) -> () {
    let px = position.x;
    let py = position.y;
    let pz = position.z;
    let dir = _CUBE_FACE_DIRECTIONS[face as usize].clone();
    (*_TARGET.lock().unwrap()).x = (px + dir[0.0_f64 as usize].clone());
    (*_TARGET.lock().unwrap()).y = (py + dir[1.0_f64 as usize].clone());
    (*_TARGET.lock().unwrap()).z = (pz + dir[2.0_f64 as usize].clone());
    (*_EYE.lock().unwrap()).x = px;
    (*_EYE.lock().unwrap()).y = py;
    (*_EYE.lock().unwrap()).z = pz;
    (*_UP.lock().unwrap()).x = dir[3.0_f64 as usize].clone();
    (*_UP.lock().unwrap()).y = dir[4.0_f64 as usize].clone();
    (*_UP.lock().unwrap()).z = dir[5.0_f64 as usize].clone();
    set_matrix4_look_at(
        &mut out.view,
        &(*_EYE.lock().unwrap()),
        &(*_TARGET.lock().unwrap()),
        &(*_UP.lock().unwrap()),
    );
    out.projection = flighthq_types::Projection::B(crate::FlightUnion2::<
        PerspectiveProjection,
        RawProjection,
    >::A(create_perspective_projection(
        &PerspectiveProjectionOptions {
            __flight_identity: std::sync::Arc::new(()),
            aspect: Some(1.0_f64),
            fov_y: (std::f64::consts::PI * 0.5_f64),
        },
    )));
}

// Source: upstream/packages/camera/src/cubeCapture.ts:33 (sha256:ae71110ca348339e307a10c915d22e2dc7fcce4ada597f761535a5f5475feff6)
static _CUBE_FACE_DIRECTIONS: std::sync::LazyLock<Vec<Vec<f64>>> = std::sync::LazyLock::new(|| {
    vec![
        vec![1.0_f64, 0.0_f64, 0.0_f64, 0.0_f64, (-1.0_f64), 0.0_f64],
        vec![(-1.0_f64), 0.0_f64, 0.0_f64, 0.0_f64, (-1.0_f64), 0.0_f64],
        vec![0.0_f64, 1.0_f64, 0.0_f64, 0.0_f64, 0.0_f64, 1.0_f64],
        vec![0.0_f64, (-1.0_f64), 0.0_f64, 0.0_f64, 0.0_f64, (-1.0_f64)],
        vec![0.0_f64, 0.0_f64, 1.0_f64, 0.0_f64, (-1.0_f64), 0.0_f64],
        vec![0.0_f64, 0.0_f64, (-1.0_f64), 0.0_f64, (-1.0_f64), 0.0_f64],
    ]
});

// Source: upstream/packages/camera/src/cubeCapture.ts:42 (sha256:f905567836d6d1525a8a8a4c8f583b89fe0b92ac550f2ea16e2ae37619f7b39e)
static _EYE: std::sync::LazyLock<std::sync::Mutex<Vector3Like>> = std::sync::LazyLock::new(|| {
    std::sync::Mutex::new(Vector3Like {
        __flight_identity: std::sync::Arc::new(()),
        __flight_entity_snapshot: Default::default(),
        __flight_entity_runtime: Default::default(),
        x: 0.0_f64,
        y: 0.0_f64,
        z: 0.0_f64,
    })
});

// Source: upstream/packages/camera/src/cubeCapture.ts:43 (sha256:08ea600988417111f59dbabd52f237be39d8058cf35921fdea2f739efe83cc41)
static _TARGET: std::sync::LazyLock<std::sync::Mutex<Vector3Like>> =
    std::sync::LazyLock::new(|| {
        std::sync::Mutex::new(Vector3Like {
            __flight_identity: std::sync::Arc::new(()),
            __flight_entity_snapshot: Default::default(),
            __flight_entity_runtime: Default::default(),
            x: 0.0_f64,
            y: 0.0_f64,
            z: 0.0_f64,
        })
    });

// Source: upstream/packages/camera/src/cubeCapture.ts:44 (sha256:65c7f982d9b5174143f784580db15e85ae40cc086cb9887b2de2331c7b6dec69)
static _UP: std::sync::LazyLock<std::sync::Mutex<Vector3Like>> = std::sync::LazyLock::new(|| {
    std::sync::Mutex::new(Vector3Like {
        __flight_identity: std::sync::Arc::new(()),
        __flight_entity_snapshot: Default::default(),
        __flight_entity_runtime: Default::default(),
        x: 0.0_f64,
        y: 0.0_f64,
        z: 0.0_f64,
    })
});
