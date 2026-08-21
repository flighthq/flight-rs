// @generated from upstream/packages/effects/src/gaussianMath.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/effects/src/gaussianMath.ts:7 (sha256:78247158b3e33fcb0405c7dd33fb928ac482562e79185a6af66a7c3df111f85f)
pub fn compute_gaussian_radius_from_sigma(sigma: f64) -> f64 {
    return (3.0_f64 * (0.0_f64).max(sigma)).ceil();
}

// Source: upstream/packages/effects/src/gaussianMath.ts:13 (sha256:1e6d013c9d10f6711b0aab5bab7df536a66af99903ad6252e1eccd4fb5f42e09)
pub fn compute_gaussian_sigma_from_radius(radius: f64) -> f64 {
    return ((0.0_f64).max(radius) / 3.0_f64);
}

// Source: upstream/packages/effects/src/gaussianMath.ts:20 (sha256:613d520a49123fceea53f887a529dd67c361b1ccdcc2471bcc1cbf1fca888970)
pub fn compute_separable_blur_pass_count(samples: Option<f64>) -> f64 {
    return (1.0_f64).max(((samples).clone().unwrap_or(1.0_f64)).round());
}

// Source: upstream/packages/effects/src/gaussianMath.ts:29 (sha256:fb853d8aab4b36c16367c8dca22715665500fe9cf34879dc07df87bf4923bccd)
pub fn create_gaussian_kernel_weights(radius: f64, sigma: f64, out: &mut Vec<f32>) -> f64 {
    let r = (0.0_f64).max((radius).ceil());
    let s = (0.000001_f64).max(sigma);
    let two_sigma_sq = ((2.0_f64 * s) * s);
    let mut sum = 0.0_f64;
    {
        let mut i = 0.0_f64;
        while (i <= r) {
            let w = ((-(i * i)) / two_sigma_sq).exp();
            out[i as usize] = (w) as f32;
            sum += if (i == 0.0_f64) { w } else { (2.0_f64 * w) };
            {
                i += 1.0;
                i
            };
        }
    }
    let inv_sum = (1.0_f64 / sum);
    {
        let mut i = 0.0_f64;
        while (i <= r) {
            out[i as usize] *= (inv_sum) as f32;
            {
                i += 1.0;
                i
            };
        }
    }
    return (r + 1.0_f64);
}
