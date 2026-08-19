// @generated from upstream/packages/effects/src/stylizeMath.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/effects/src/stylizeMath.ts:9 (sha256:0fdbe151d1fb7c7ec77eb14332bb0e1baa57904012c149452f668b2b67b57ea4)
pub fn compute_crt_mask_params(resolution: f64, curvature: f64, out: &mut Vec<f64>) -> () {
    let mask_scale = ((1.0_f64).max(resolution) / 360.0_f64);
    let curv = (0.0_f64).max((1.0_f64).min(curvature));
    {
        let __flight_index = (0.0_f64) as usize;
        let __flight_value = mask_scale;
        if __flight_index == out.len() {
            out.push(__flight_value);
        } else {
            out[__flight_index] = __flight_value;
        }
    };
    {
        let __flight_index = (1.0_f64) as usize;
        let __flight_value = (curv * 0.1_f64);
        if __flight_index == out.len() {
            out.push(__flight_value);
        } else {
            out[__flight_index] = __flight_value;
        }
    };
}

// Source: upstream/packages/effects/src/stylizeMath.ts:25 (sha256:aef8e20c7c98b213a98657f13e4d56129bb75de1d0bb89f40c056b05e8588bc1)
pub fn compute_halftone_cell_params(frequency: f64, angle: f64, out: &mut Vec<f64>) -> () {
    let cell_size = if (frequency > 1e-10_f64) {
        (1.0_f64 / frequency)
    } else {
        1.0_f64
    };
    {
        let __flight_index = (0.0_f64) as usize;
        let __flight_value = cell_size;
        if __flight_index == out.len() {
            out.push(__flight_value);
        } else {
            out[__flight_index] = __flight_value;
        }
    };
    {
        let __flight_index = (1.0_f64) as usize;
        let __flight_value = (angle).cos();
        if __flight_index == out.len() {
            out.push(__flight_value);
        } else {
            out[__flight_index] = __flight_value;
        }
    };
    {
        let __flight_index = (2.0_f64) as usize;
        let __flight_value = (angle).sin();
        if __flight_index == out.len() {
            out.push(__flight_value);
        } else {
            out[__flight_index] = __flight_value;
        }
    };
}

// Source: upstream/packages/effects/src/stylizeMath.ts:36 (sha256:beba6ccc1d5aac61f348f099076b0fcd6990624dd0232047bc26913e41a18968)
pub fn compute_scanline_params(resolution: f64, intensity: f64, out: &mut Vec<f64>) -> () {
    let scale = ((1.0_f64).max(resolution) / 480.0_f64);
    {
        let __flight_index = (0.0_f64) as usize;
        let __flight_value = scale;
        if __flight_index == out.len() {
            out.push(__flight_value);
        } else {
            out[__flight_index] = __flight_value;
        }
    };
    {
        let __flight_index = (1.0_f64) as usize;
        let __flight_value = (0.0_f64).max((1.0_f64).min(intensity));
        if __flight_index == out.len() {
            out.push(__flight_value);
        } else {
            out[__flight_index] = __flight_value;
        }
    };
}

// Source: upstream/packages/effects/src/stylizeMath.ts:50 (sha256:ac7cf9e9db0b8336dd5df9bd6f366d57cded72befb67f323f55444d040ad961e)
pub fn create_bayer_matrix(order: f64, out: &mut Vec<f32>) -> f64 {
    let size = (2.0_f64).powf((1.0_f64).max((order).round()));
    let size_sq = (size * size);
    let mut raw: Vec<f32> = vec![0.0_f32; (size_sq) as usize];
    raw[0.0_f64 as usize] = (0.0_f64) as f32;
    raw[1.0_f64 as usize] = (2.0_f64) as f32;
    raw[2.0_f64 as usize] = (3.0_f64) as f32;
    raw[3.0_f64 as usize] = (1.0_f64) as f32;
    let mut current_size = 2.0_f64;
    while (current_size < size) {
        let next = (current_size * 2.0_f64);
        let next_sq = (next * next);
        let mut tmp: Vec<f32> = vec![0.0_f32; (next_sq) as usize];
        {
            let mut y = 0.0_f64;
            while (y < current_size) {
                {
                    let mut x = 0.0_f64;
                    while (x < current_size) {
                        let base = ((raw[((y * current_size) + x) as usize] as f64) * 4.0_f64);
                        tmp[((y * next) + x) as usize] = (base) as f32;
                        tmp[((y * next) + (x + current_size)) as usize] = (base + 2.0_f64) as f32;
                        tmp[(((y + current_size) * next) + x) as usize] = (base + 3.0_f64) as f32;
                        tmp[(((y + current_size) * next) + (x + current_size)) as usize] =
                            (base + 1.0_f64) as f32;
                        {
                            x += 1.0;
                            x
                        };
                    }
                }
                {
                    y += 1.0;
                    y
                };
            }
        }
        {
            let __flight_offset = (0.0_f64) as usize;
            let __flight_values: Vec<f32> = ((tmp).clone())
                .iter()
                .map(|value| (*value) as f32)
                .collect();
            raw[__flight_offset..__flight_offset + __flight_values.len()]
                .copy_from_slice(&__flight_values);
        };
        current_size = next;
    }
    let inv_size_sq = (1.0_f64 / size_sq);
    {
        let mut i = 0.0_f64;
        while (i < size_sq) {
            out[i as usize] = ((raw[i as usize] as f64) * inv_size_sq) as f32;
            {
                i += 1.0;
                i
            };
        }
    }
    return size;
}
