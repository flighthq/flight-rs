// @generated from upstream/packages/effects/src/kuwaharaMath.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::KuwaharaEffect;

// Source: upstream/packages/effects/src/kuwaharaMath.ts:13 (sha256:aaaa7ac1fee8edbd04aa0472ac050c57d4bbfd2ea988d1d9b1addc502a0341a2)
pub fn compute_kuwahara_gaussian_weights(radius: f64, out: &mut Vec<f32>) -> f64 {
    let r = (1.0_f64).max((radius).floor());
    let size = (r + 1.0_f64);
    let sigma = (r / 2.0_f64);
    let two_sigma_sq = ((2.0_f64 * sigma) * sigma);
    let mut sum = 0.0_f64;
    {
        let mut y = 0.0_f64;
        while (y < size) {
            {
                let mut x = 0.0_f64;
                while (x < size) {
                    let d = ((x * x) + (y * y));
                    out[((y * size) + x) as usize] = (((-d) / two_sigma_sq).exp()) as f32;
                    sum += (out[((y * size) + x) as usize] as f64);
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
    let inv_sum = if (sum > 1e-10_f64) {
        (1.0_f64 / sum)
    } else {
        1.0_f64
    };
    {
        let mut i = 0.0_f64;
        while (i < (size * size)) {
            out[i as usize] *= (inv_sum) as f32;
            {
                i += 1.0;
                i
            };
        }
    }
    return (size * size);
}

// Source: upstream/packages/effects/src/kuwaharaMath.ts:39 (sha256:57bff1cbab2af40dc9f73b5e3f072469f90878fc1c5e1aa4abc112100f9a983b)
pub fn compute_kuwahara_sector_offsets(radius: f64, out: &mut Vec<f64>) -> () {
    let r = (1.0_f64).max((radius).floor());
    let half = r;
    let v0 = (-half);
    let v1 = (-half);
    let v2 = 0.0_f64;
    let v3 = (-half);
    let v4 = (-half);
    let v5 = 0.0_f64;
    let v6 = 0.0_f64;
    let v7 = 0.0_f64;
    {
        let __flight_index = (0.0_f64) as usize;
        let __flight_value = v0;
        if __flight_index == out.len() {
            out.push(__flight_value);
        } else {
            out[__flight_index] = __flight_value;
        }
    };
    {
        let __flight_index = (1.0_f64) as usize;
        let __flight_value = v1;
        if __flight_index == out.len() {
            out.push(__flight_value);
        } else {
            out[__flight_index] = __flight_value;
        }
    };
    {
        let __flight_index = (2.0_f64) as usize;
        let __flight_value = v2;
        if __flight_index == out.len() {
            out.push(__flight_value);
        } else {
            out[__flight_index] = __flight_value;
        }
    };
    {
        let __flight_index = (3.0_f64) as usize;
        let __flight_value = v3;
        if __flight_index == out.len() {
            out.push(__flight_value);
        } else {
            out[__flight_index] = __flight_value;
        }
    };
    {
        let __flight_index = (4.0_f64) as usize;
        let __flight_value = v4;
        if __flight_index == out.len() {
            out.push(__flight_value);
        } else {
            out[__flight_index] = __flight_value;
        }
    };
    {
        let __flight_index = (5.0_f64) as usize;
        let __flight_value = v5;
        if __flight_index == out.len() {
            out.push(__flight_value);
        } else {
            out[__flight_index] = __flight_value;
        }
    };
    {
        let __flight_index = (6.0_f64) as usize;
        let __flight_value = v6;
        if __flight_index == out.len() {
            out.push(__flight_value);
        } else {
            out[__flight_index] = __flight_value;
        }
    };
    {
        let __flight_index = (7.0_f64) as usize;
        let __flight_value = v7;
        if __flight_index == out.len() {
            out.push(__flight_value);
        } else {
            out[__flight_index] = __flight_value;
        }
    };
}

// Source: upstream/packages/effects/src/kuwaharaMath.ts:69 (sha256:d04f08f3560bd8403f6f4bc66d2e1e1a46f651cbdc2f2055e829440b14b780ed)
pub fn compute_kuwahara_sector_pixel_count(effect: &KuwaharaEffect) -> f64 {
    let s = compute_kuwahara_sector_size(effect);
    return (s * s);
}

// Source: upstream/packages/effects/src/kuwaharaMath.ts:76 (sha256:2914125bb1a7fc1c2a0ab9996d298bca9b7d8d53bd2554d5eb8bfc06856129be)
pub fn compute_kuwahara_sector_size(effect: &KuwaharaEffect) -> f64 {
    return ((1.0_f64).max(((effect.radius).clone().unwrap_or(3.0_f64)).floor()) + 1.0_f64);
}
