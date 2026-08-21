// @generated from upstream/packages/skeleton2d/src/skinAttachment2DPoints.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::report_skeleton2_d_deform_length_mismatch;
use flighthq_types::{Skeleton2D, Skin2D};

// Source: upstream/packages/skeleton2d/src/skinAttachment2DPoints.ts:6 (sha256:43430cb9fcc08b2602c28ac39599e5c4ef0b3c11cc5d023d40daacc2bdcf1160)
const MATRIX_STRIDE: f64 = 6.0_f64;

// Source: upstream/packages/skeleton2d/src/skinAttachment2DPoints.ts:36 (sha256:b7035a70387785d31c502703ac8e33ae0d321f4d723b7c0bd2038ca982159843)
pub fn skin_skeleton2_d_attachment_points(
    out: &mut crate::FlightUnion2<Vec<f32>, Vec<f64>>,
    skin: &Option<Skin2D>,
    vertices: &Option<Vec<f32>>,
    skeleton: &Skeleton2D,
    bone_index: f64,
    deform: &Option<Vec<f32>>,
    subject: String,
) -> () {
    if ((skin).is_some()) && ((skin).is_some()) {
        let offsets = if ((deform).is_some())
            && (((deform.as_ref().unwrap().len() as f64) * 2.0_f64)
                == (skin.as_ref().unwrap().influences.len() as f64))
        {
            Some((*deform.as_ref().unwrap()).clone())
        } else {
            None
        };
        if ((deform).is_some()) && ((offsets).is_none()) {
            report_skeleton2_d_deform_length_mismatch(
                (subject).clone(),
                (deform.as_ref().unwrap().len() as f64),
                ((skin.as_ref().unwrap().influences.len() as f64) / 2.0_f64),
            );
        }
        let mut vi = 0.0_f64;
        let mut di = 0.0_f64;
        let mut oi = 0.0_f64;
        {
            let mut v = 0.0_f64;
            while (v < (skin.as_ref().unwrap().influence_counts.len() as f64)) {
                let mut wx = 0.0_f64;
                let mut wy = 0.0_f64;
                let n = (skin.as_ref().unwrap().influence_counts[v as usize] as f64);
                {
                    let mut k = 0.0_f64;
                    while (k < n) {
                        let b = ((skin.as_ref().unwrap().influences[vi as usize] as f64)
                            * MATRIX_STRIDE);
                        let lx = if (offsets).is_none() {
                            (skin.as_ref().unwrap().influences[(vi + 1.0_f64) as usize] as f64)
                                as f32
                        } else {
                            ((skin.as_ref().unwrap().influences[(vi + 1.0_f64) as usize] as f64)
                                + (offsets.as_ref().unwrap()[di as usize] as f64))
                                as f32
                        };
                        let ly = if (offsets).is_none() {
                            (skin.as_ref().unwrap().influences[(vi + 2.0_f64) as usize] as f64)
                                as f32
                        } else {
                            ((skin.as_ref().unwrap().influences[(vi + 2.0_f64) as usize] as f64)
                                + (offsets.as_ref().unwrap()[(di + 1.0_f64) as usize] as f64))
                                as f32
                        };
                        let weight =
                            (skin.as_ref().unwrap().influences[(vi + 3.0_f64) as usize] as f64);
                        wx += (weight
                            * ((((skeleton.world_matrices[b as usize] as f64) * lx)
                                + ((skeleton.world_matrices[(b + 2.0_f64) as usize] as f64)
                                    * ly))
                                + (skeleton.world_matrices[(b + 4.0_f64) as usize] as f64)));
                        wy += (weight
                            * ((((skeleton.world_matrices[(b + 1.0_f64) as usize] as f64) * lx)
                                + ((skeleton.world_matrices[(b + 3.0_f64) as usize] as f64)
                                    * ly))
                                + (skeleton.world_matrices[(b + 5.0_f64) as usize] as f64)));
                        vi += 4.0_f64;
                        di += 2.0_f64;
                        {
                            k += 1.0;
                            k
                        };
                    }
                }
                {
                    let __flight_index = (oi) as usize;
                    let __flight_value = wx;
                    match out {
                        crate::FlightUnion2::A(values) => {
                            values[__flight_index] = (__flight_value) as f32;
                        }
                        crate::FlightUnion2::B(values) => {
                            values[__flight_index] = __flight_value;
                        }
                    };
                };
                {
                    let __flight_index = (oi + 1.0_f64) as usize;
                    let __flight_value = wy;
                    match out {
                        crate::FlightUnion2::A(values) => {
                            values[__flight_index] = (__flight_value) as f32;
                        }
                        crate::FlightUnion2::B(values) => {
                            values[__flight_index] = __flight_value;
                        }
                    };
                };
                oi += 2.0_f64;
                {
                    v += 1.0;
                    v
                };
            }
        }
        return;
    }
    if ((vertices).is_none()) || ((vertices).is_none()) {
        return;
    }
    if (bone_index < 0.0_f64)
        || ((bone_index * MATRIX_STRIDE) >= (skeleton.world_matrices.len() as f64))
    {
        return;
    }
    let offsets = if ((deform).is_some())
        && ((deform.as_ref().unwrap().len() as f64) == (vertices.as_ref().unwrap().len() as f64))
    {
        Some((*deform.as_ref().unwrap()).clone())
    } else {
        None
    };
    if ((deform).is_some()) && ((offsets).is_none()) {
        report_skeleton2_d_deform_length_mismatch(
            (subject).clone(),
            (deform.as_ref().unwrap().len() as f64),
            (vertices.as_ref().unwrap().len() as f64),
        );
    }
    let b = (bone_index * MATRIX_STRIDE);
    let a = (skeleton.world_matrices[b as usize] as f64);
    let bb = (skeleton.world_matrices[(b + 1.0_f64) as usize] as f64);
    let c = (skeleton.world_matrices[(b + 2.0_f64) as usize] as f64);
    let d = (skeleton.world_matrices[(b + 3.0_f64) as usize] as f64);
    let tx = (skeleton.world_matrices[(b + 4.0_f64) as usize] as f64);
    let ty = (skeleton.world_matrices[(b + 5.0_f64) as usize] as f64);
    {
        let mut i = 0.0_f64;
        while (i < (vertices.as_ref().unwrap().len() as f64)) {
            let vx = if (offsets).is_none() {
                (vertices.as_ref().unwrap()[i as usize] as f64) as f32
            } else {
                ((vertices.as_ref().unwrap()[i as usize] as f64)
                    + (offsets.as_ref().unwrap()[i as usize] as f64)) as f32
            };
            let vy = if (offsets).is_none() {
                (vertices.as_ref().unwrap()[(i + 1.0_f64) as usize] as f64) as f32
            } else {
                ((vertices.as_ref().unwrap()[(i + 1.0_f64) as usize] as f64)
                    + (offsets.as_ref().unwrap()[(i + 1.0_f64) as usize] as f64))
                    as f32
            };
            {
                let __flight_index = (i) as usize;
                let __flight_value = (((a * vx) + (c * vy)) + tx);
                match out {
                    crate::FlightUnion2::A(values) => {
                        values[__flight_index] = (__flight_value) as f32;
                    }
                    crate::FlightUnion2::B(values) => {
                        values[__flight_index] = __flight_value;
                    }
                };
            };
            {
                let __flight_index = (i + 1.0_f64) as usize;
                let __flight_value = (((bb * vx) + (d * vy)) + ty);
                match out {
                    crate::FlightUnion2::A(values) => {
                        values[__flight_index] = (__flight_value) as f32;
                    }
                    crate::FlightUnion2::B(values) => {
                        values[__flight_index] = __flight_value;
                    }
                };
            };
            {
                i += 2.0_f64;
                i.clone()
            };
        }
    }
}
