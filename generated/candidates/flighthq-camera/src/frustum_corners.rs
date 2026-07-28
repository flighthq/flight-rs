// @generated from upstream/packages/camera/src/frustumCorners.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::get_camera_view_projection_matrix4;
use flighthq_geometry::{create_matrix4, inverse_matrix4};
use flighthq_types::{Camera, Matrix4, Matrix4Like, Vector3Like};

// Source: upstream/packages/camera/src/frustumCorners.ts:22 (sha256:84e8e374cdb6014b2346b9ab7daad1b43da5454de01f39233945b8e260915b39)
pub fn get_camera_frustum_corners(
    out: &mut Vec<Vector3Like>,
    camera: &Camera,
    aspect: f64,
) -> bool {
    get_camera_view_projection_matrix4(
        &mut (*__SCRATCH_VIEW_PROJECTION.lock().unwrap()),
        camera,
        aspect,
    );
    if (!inverse_matrix4(&mut (*__SCRATCH_INVERSE_VP.lock().unwrap()), &{
        let __flight_source = &(*__SCRATCH_VIEW_PROJECTION.lock().unwrap());
        Matrix4Like {
            __flight_identity: std::sync::Arc::clone(&__flight_source.__flight_identity),
            m: (__flight_source.m).clone(),
        }
    })) {
        return false;
    }
    let ndc_corners: Vec<Vec<f64>> = vec![
        vec![(-1.0_f64), (-1.0_f64), (-1.0_f64)],
        vec![1.0_f64, (-1.0_f64), (-1.0_f64)],
        vec![(-1.0_f64), 1.0_f64, (-1.0_f64)],
        vec![1.0_f64, 1.0_f64, (-1.0_f64)],
        vec![(-1.0_f64), (-1.0_f64), 1.0_f64],
        vec![1.0_f64, (-1.0_f64), 1.0_f64],
        vec![(-1.0_f64), 1.0_f64, 1.0_f64],
        vec![1.0_f64, 1.0_f64, 1.0_f64],
    ];
    let mut results: Vec<Vec<f64>> = vec![];
    {
        let mut i = 0.0_f64;
        while (i < 8.0_f64) {
            let __destructure0 = ndc_corners[i as usize].clone();
            let nx = __destructure0[0.0_f64 as usize].clone();
            let ny = __destructure0[1.0_f64 as usize].clone();
            let nz = __destructure0[2.0_f64 as usize].clone();
            let mut wx = ((((((*__SCRATCH_INVERSE_VP.lock().unwrap()).m[0.0_f64 as usize]
                as f64)
                * nx)
                + (((*__SCRATCH_INVERSE_VP.lock().unwrap()).m[4.0_f64 as usize] as f64) * ny))
                + (((*__SCRATCH_INVERSE_VP.lock().unwrap()).m[8.0_f64 as usize] as f64) * nz))
                + ((*__SCRATCH_INVERSE_VP.lock().unwrap()).m[12.0_f64 as usize] as f64));
            let mut wy = ((((((*__SCRATCH_INVERSE_VP.lock().unwrap()).m[1.0_f64 as usize]
                as f64)
                * nx)
                + (((*__SCRATCH_INVERSE_VP.lock().unwrap()).m[5.0_f64 as usize] as f64) * ny))
                + (((*__SCRATCH_INVERSE_VP.lock().unwrap()).m[9.0_f64 as usize] as f64) * nz))
                + ((*__SCRATCH_INVERSE_VP.lock().unwrap()).m[13.0_f64 as usize] as f64));
            let mut wz = ((((((*__SCRATCH_INVERSE_VP.lock().unwrap()).m[2.0_f64 as usize]
                as f64)
                * nx)
                + (((*__SCRATCH_INVERSE_VP.lock().unwrap()).m[6.0_f64 as usize] as f64) * ny))
                + (((*__SCRATCH_INVERSE_VP.lock().unwrap()).m[10.0_f64 as usize] as f64) * nz))
                + ((*__SCRATCH_INVERSE_VP.lock().unwrap()).m[14.0_f64 as usize] as f64));
            let ww = ((((((*__SCRATCH_INVERSE_VP.lock().unwrap()).m[3.0_f64 as usize] as f64)
                * nx)
                + (((*__SCRATCH_INVERSE_VP.lock().unwrap()).m[7.0_f64 as usize] as f64) * ny))
                + (((*__SCRATCH_INVERSE_VP.lock().unwrap()).m[11.0_f64 as usize] as f64) * nz))
                + ((*__SCRATCH_INVERSE_VP.lock().unwrap()).m[15.0_f64 as usize] as f64));
            if (ww != 0.0_f64) {
                let inv_w = (1.0_f64 / ww);
                wx *= inv_w;
                wy *= inv_w;
                wz *= inv_w;
            }
            results.push(vec![wx, wy, wz]);
            {
                i += 1.0;
                i
            };
        }
    }
    {
        let mut i = 0.0_f64;
        while (i < 8.0_f64) {
            out[i as usize].x = results[i as usize][0.0_f64 as usize].clone();
            out[i as usize].y = results[i as usize][1.0_f64 as usize].clone();
            out[i as usize].z = results[i as usize][2.0_f64 as usize].clone();
            {
                i += 1.0;
                i
            };
        }
    }
    return true;
}

// Source: upstream/packages/camera/src/frustumCorners.ts:70 (sha256:ea1bce46bff5117486aa66f0bc0c33f5ba239247bd135b339e34f20358b60428)
static __SCRATCH_VIEW_PROJECTION: std::sync::LazyLock<std::sync::Mutex<Matrix4>> =
    std::sync::LazyLock::new(|| {
        std::sync::Mutex::new(create_matrix4(
            None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            None, None,
        ))
    });

// Source: upstream/packages/camera/src/frustumCorners.ts:71 (sha256:9533d4b350fea262c12d05798919dbd7e09488baad44ff9ced667d0a21cfe6ff)
static __SCRATCH_INVERSE_VP: std::sync::LazyLock<std::sync::Mutex<Matrix4>> =
    std::sync::LazyLock::new(|| {
        std::sync::Mutex::new(create_matrix4(
            None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            None, None,
        ))
    });
