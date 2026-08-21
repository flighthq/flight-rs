// @generated from upstream/packages/camera/src/explainCamera3DView.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::{Camera3D, Camera3DViewExplanation};

// Source: upstream/packages/camera/src/explainCamera3DView.ts:13 (sha256:12f14c34d9ade920c7e77dbe8c74eb2acbed3ad639ca380db2874480b7454f36)
pub fn explain_camera3_d_view(camera: &Camera3D) -> Camera3DViewExplanation {
    let x = Vector3 {
        __flight_identity: std::sync::Arc::new(()),
        __flight_entity_runtime: Default::default(),
        x: (camera.view.m[0.0_f64 as usize] as f64),
        y: (camera.view.m[1.0_f64 as usize] as f64),
        z: (camera.view.m[2.0_f64 as usize] as f64),
    };
    let y = Vector3 {
        __flight_identity: std::sync::Arc::new(()),
        __flight_entity_runtime: Default::default(),
        x: (camera.view.m[4.0_f64 as usize] as f64),
        y: (camera.view.m[5.0_f64 as usize] as f64),
        z: (camera.view.m[6.0_f64 as usize] as f64),
    };
    let z = Vector3 {
        __flight_identity: std::sync::Arc::new(()),
        __flight_entity_runtime: Default::default(),
        x: (camera.view.m[8.0_f64 as usize] as f64),
        y: (camera.view.m[9.0_f64 as usize] as f64),
        z: (camera.view.m[10.0_f64 as usize] as f64),
    };
    let length_x = ((x.x).powi(2) + (x.y).powi(2) + (x.z).powi(2)).sqrt();
    let length_y = ((y.x).powi(2) + (y.y).powi(2) + (y.z).powi(2)).sqrt();
    let length_z = ((z.x).powi(2) + (z.y).powi(2) + (z.z).powi(2)).sqrt();
    let determinant = ((((camera.view.m[0.0_f64 as usize] as f64)
        * (((camera.view.m[5.0_f64 as usize] as f64)
            * (camera.view.m[10.0_f64 as usize] as f64))
            - ((camera.view.m[6.0_f64 as usize] as f64)
                * (camera.view.m[9.0_f64 as usize] as f64))))
        - ((camera.view.m[4.0_f64 as usize] as f64)
            * (((camera.view.m[1.0_f64 as usize] as f64)
                * (camera.view.m[10.0_f64 as usize] as f64))
                - ((camera.view.m[2.0_f64 as usize] as f64)
                    * (camera.view.m[9.0_f64 as usize] as f64)))))
        + ((camera.view.m[8.0_f64 as usize] as f64)
            * (((camera.view.m[1.0_f64 as usize] as f64)
                * (camera.view.m[6.0_f64 as usize] as f64))
                - ((camera.view.m[2.0_f64 as usize] as f64)
                    * (camera.view.m[5.0_f64 as usize] as f64)))));
    let scale_deviation = (((length_x - 1.0_f64).abs()).max((length_y - 1.0_f64).abs()))
        .max((length_z - 1.0_f64).abs());
    let shear_deviation = (((((x.x * y.x) + (x.y * y.y)) + (x.z * y.z)).abs())
        .max((((x.x * z.x) + (x.y * z.y)) + (x.z * z.z)).abs()))
    .max((((y.x * z.x) + (y.y * z.y)) + (y.z * z.z)).abs());
    return Camera3DViewExplanation {
        __flight_identity: std::sync::Arc::new(()),
        determinant: determinant,
        is_orthonormal: (scale_deviation <= ORTHONORMAL_TOLERANCE)
            && (shear_deviation <= ORTHONORMAL_TOLERANCE),
        is_reflection: (determinant < 0.0_f64),
        scale_deviation: scale_deviation,
        shear_deviation: shear_deviation,
    };
}

// Source: upstream/packages/camera/src/explainCamera3DView.ts:46 (sha256:64ffbdb91b9410d544fe5b763e186cf0f0af999bc9bfc0a1c080345e8ba78017)
const ORTHONORMAL_TOLERANCE: f64 = 0.001_f64;
