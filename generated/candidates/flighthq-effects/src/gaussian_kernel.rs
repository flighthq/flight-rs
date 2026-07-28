// @generated from upstream/packages/effects/src/gaussianKernel.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/effects/src/gaussianKernel.ts:17 (sha256:0c59f79442255b26aa3aedc310358d473bfe4eef91d896d7afdda0908417fc53)
pub fn compute_gaussian_kernel_weights(sigma: f64, out: &mut Vec<f64>) -> Vec<f64> {
    let size = get_gaussian_kernel_size(sigma);
    out.truncate((size) as usize);
    if (size == 1.0_f64) {
        {
            let __flight_index = (0.0_f64) as usize;
            let __flight_value = 1.0_f64;
            if __flight_index == out.len() {
                out.push(__flight_value);
            } else {
                out[__flight_index] = __flight_value;
            }
        };
        return out.clone();
    }
    let radius = ((size - 1.0_f64) / 2.0_f64);
    let two_sigma_squared = ((2.0_f64 * sigma) * sigma);
    let mut sum = 0.0_f64;
    {
        let mut i = 0.0_f64;
        while (i < size) {
            let x = (i - radius);
            let weight = ((-(x * x)) / two_sigma_squared).exp();
            {
                let __flight_index = (i) as usize;
                let __flight_value = weight;
                if __flight_index == out.len() {
                    out.push(__flight_value);
                } else {
                    out[__flight_index] = __flight_value;
                }
            };
            sum += weight;
            {
                i += 1.0;
                i
            };
        }
    }
    let inverse_sum = (1.0_f64 / sum);
    {
        let mut i = 0.0_f64;
        while (i < size) {
            out[i as usize] *= inverse_sum;
            {
                i += 1.0;
                i
            };
        }
    }
    return out.clone();
}

// Source: upstream/packages/effects/src/gaussianKernel.ts:45 (sha256:279b73f133113d05cc166c6964c36691e87e0c3eb2cf3a7dde118b306b435ead)
pub fn get_gaussian_kernel_size(sigma: f64) -> f64 {
    if (sigma <= 0.0_f64) {
        return 1.0_f64;
    }
    let radius = (3.0_f64 * sigma).ceil();
    return ((radius * 2.0_f64) + 1.0_f64);
}
