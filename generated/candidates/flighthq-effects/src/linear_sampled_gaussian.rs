// @generated from upstream/packages/effects/src/linearSampledGaussian.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{compute_gaussian_kernel_weights, get_gaussian_kernel_size};

// Source: upstream/packages/effects/src/linearSampledGaussian.ts:25 (sha256:e4f12c180e3c9d55b92722919cec5b444b0f81361e5513a0106be359f2ac9142)
pub fn compute_linear_sampled_gaussian(
    sigma: f64,
    out_weights: &mut Vec<f64>,
    out_offsets: &mut Vec<f64>,
) -> Vec<f64> {
    let size = get_gaussian_kernel_size(sigma);
    let radius = ((size - 1.0_f64) / 2.0_f64);
    compute_gaussian_kernel_weights(sigma, &mut (*SCRATCH_WEIGHTS.lock().unwrap()));
    let tap_count = get_linear_sampled_gaussian_tap_count(sigma);
    out_weights.truncate((tap_count) as usize);
    out_offsets.truncate((tap_count) as usize);
    {
        let mut tap = 0.0_f64;
        while (tap < tap_count) {
            let i = (tap * 2.0_f64);
            let pos_a = (i - radius);
            if ((i + 1.0_f64) < size) {
                let weight_a = (*SCRATCH_WEIGHTS.lock().unwrap())[i as usize].clone();
                let weight_b = (*SCRATCH_WEIGHTS.lock().unwrap())[(i + 1.0_f64) as usize].clone();
                let combined = (weight_a + weight_b);
                {
                    let __flight_index = (tap) as usize;
                    let __flight_value = combined;
                    if __flight_index == out_weights.len() {
                        out_weights.push(__flight_value);
                    } else {
                        out_weights[__flight_index] = __flight_value;
                    }
                };
                {
                    let __flight_index = (tap) as usize;
                    let __flight_value = if (combined == 0.0_f64) {
                        (pos_a + 0.5_f64)
                    } else {
                        (((pos_a * weight_a) + ((pos_a + 1.0_f64) * weight_b)) / combined)
                    };
                    if __flight_index == out_offsets.len() {
                        out_offsets.push(__flight_value);
                    } else {
                        out_offsets[__flight_index] = __flight_value;
                    }
                };
            } else {
                {
                    let __flight_index = (tap) as usize;
                    let __flight_value = (*SCRATCH_WEIGHTS.lock().unwrap())[i as usize].clone();
                    if __flight_index == out_weights.len() {
                        out_weights.push(__flight_value);
                    } else {
                        out_weights[__flight_index] = __flight_value;
                    }
                };
                {
                    let __flight_index = (tap) as usize;
                    let __flight_value = pos_a;
                    if __flight_index == out_offsets.len() {
                        out_offsets.push(__flight_value);
                    } else {
                        out_offsets[__flight_index] = __flight_value;
                    }
                };
            }
            {
                tap += 1.0;
                tap
            };
        }
    }
    return out_weights.clone();
}

// Source: upstream/packages/effects/src/linearSampledGaussian.ts:54 (sha256:351298c7be9d818614d6c98a5f27485cc710713f68c1cb0695813043b01f83c0)
pub fn get_linear_sampled_gaussian_tap_count(sigma: f64) -> f64 {
    return (get_gaussian_kernel_size(sigma) / 2.0_f64).ceil();
}

// Source: upstream/packages/effects/src/linearSampledGaussian.ts:58 (sha256:1c95668b7ae2fab1ba5fc6bc49a66fe3d5c089b10afa28fb83ae2d1250ac74ac)
static SCRATCH_WEIGHTS: std::sync::LazyLock<std::sync::Mutex<Vec<f64>>> =
    std::sync::LazyLock::new(|| std::sync::Mutex::new(vec![]));
