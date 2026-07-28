// @generated from upstream/packages/skeleton3d/src/skinVertices.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

#[inline]
fn __flight_js_to_u32(value: f64) -> u32 {
    if !value.is_finite() || value == 0.0 {
        return 0;
    }
    value.trunc().rem_euclid(4294967296.0_f64) as u32
}

#[inline]
fn __flight_js_to_i32(value: f64) -> i32 {
    __flight_js_to_u32(value) as i32
}

// Source: upstream/packages/skeleton3d/src/skinVertices.ts:16 (sha256:25a7356f61601c0361522ea115d8757322c63e135c0396accd3bba395045e0b2)
pub fn skin_vertices(
    out_positions: &mut Vec<f32>,
    out_normals: &mut Vec<f32>,
    positions: &Vec<f32>,
    normals: &Vec<f32>,
    joints: crate::OpaqueHostValue,
    weights: &Vec<f32>,
    joint_matrices: &Vec<f32>,
) -> () {
    let vertex_count = (__flight_js_to_i32(((positions.len() as f64) / 3.0_f64))
        | __flight_js_to_i32(0.0_f64)) as f64;
    {
        let mut v = 0.0_f64;
        while (v < vertex_count) {
            let p = (v * 3.0_f64);
            let px = (positions[p as usize] as f64);
            let py = (positions[(p + 1.0_f64) as usize] as f64);
            let pz = (positions[(p + 2.0_f64) as usize] as f64);
            let nx = (normals[p as usize] as f64);
            let ny = (normals[(p + 1.0_f64) as usize] as f64);
            let nz = (normals[(p + 2.0_f64) as usize] as f64);
            let mut opx = 0.0_f64;
            let mut opy = 0.0_f64;
            let mut opz = 0.0_f64;
            let mut onx = 0.0_f64;
            let mut ony = 0.0_f64;
            let mut onz = 0.0_f64;
            let w = (v * 4.0_f64);
            {
                let mut k = 0.0_f64;
                while (k < 4.0_f64) {
                    let weight = (weights[(w + k) as usize] as f64);
                    if (weight == 0.0_f64) {
                        {
                            k += 1.0;
                            k
                        };
                        continue;
                    }
                    let m = (joints[(w + k) as usize].clone() * 16.0_f64);
                    let m0 = (joint_matrices[m as usize] as f64);
                    let m1 = (joint_matrices[(m + 1.0_f64) as usize] as f64);
                    let m2 = (joint_matrices[(m + 2.0_f64) as usize] as f64);
                    let m4 = (joint_matrices[(m + 4.0_f64) as usize] as f64);
                    let m5 = (joint_matrices[(m + 5.0_f64) as usize] as f64);
                    let m6 = (joint_matrices[(m + 6.0_f64) as usize] as f64);
                    let m8 = (joint_matrices[(m + 8.0_f64) as usize] as f64);
                    let m9 = (joint_matrices[(m + 9.0_f64) as usize] as f64);
                    let m10 = (joint_matrices[(m + 10.0_f64) as usize] as f64);
                    opx += (weight
                        * ((((m0 * px) + (m4 * py)) + (m8 * pz))
                            + (joint_matrices[(m + 12.0_f64) as usize] as f64)));
                    opy += (weight
                        * ((((m1 * px) + (m5 * py)) + (m9 * pz))
                            + (joint_matrices[(m + 13.0_f64) as usize] as f64)));
                    opz += (weight
                        * ((((m2 * px) + (m6 * py)) + (m10 * pz))
                            + (joint_matrices[(m + 14.0_f64) as usize] as f64)));
                    onx += (weight * (((m0 * nx) + (m4 * ny)) + (m8 * nz)));
                    ony += (weight * (((m1 * nx) + (m5 * ny)) + (m9 * nz)));
                    onz += (weight * (((m2 * nx) + (m6 * ny)) + (m10 * nz)));
                    {
                        k += 1.0;
                        k
                    };
                }
            }
            out_positions[p as usize] = (opx) as f32;
            out_positions[(p + 1.0_f64) as usize] = (opy) as f32;
            out_positions[(p + 2.0_f64) as usize] = (opz) as f32;
            out_normals[p as usize] = (onx) as f32;
            out_normals[(p + 1.0_f64) as usize] = (ony) as f32;
            out_normals[(p + 2.0_f64) as usize] = (onz) as f32;
            {
                v += 1.0;
                v
            };
        }
    }
}
