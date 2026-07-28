// @generated from upstream/packages/effects/src/depthMath.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/effects/src/depthMath.ts:9 (sha256:e7fdffc50a11a8a86e161f21896cea74d44926443f4b6e99e3d3ff37d6057ee3)
pub fn compute_depth_of_field_coc(
    depth: f64,
    focus_distance: f64,
    aperture: f64,
    focal_length: f64,
) -> f64 {
    let fd = (0.00001_f64).max(focus_distance);
    let fl = ((0.00001_f64).max(focal_length) / 1000.0_f64);
    let d = (0.00001_f64).max(depth);
    let a = (fl / (0.00001_f64).max(aperture));
    return ((a * (d - fd)) / (d * (fd - fl)));
}

// Source: upstream/packages/effects/src/depthMath.ts:25 (sha256:6bcf1ccbb9c8ef0613cf642b21f8cd44ed06a3e8fa233c6c3c1debd9e061ebe9)
pub fn compute_linear_depth_from_nonlinear(depth: f64, near: f64, far: f64) -> f64 {
    return ((near * far) / (far - (depth * (far - near))));
}

// Source: upstream/packages/effects/src/depthMath.ts:33 (sha256:db33ab2f1ed4f3bcc8bfe2f6b4a11fc8dddcbef90713ff9abd414c8844544364)
pub fn compute_ssao_sample_kernel(samples: f64, out: &mut Vec<f32>) -> f64 {
    let n = (1.0_f64).max((samples).round());
    {
        let mut i = 0.0_f64;
        while (i < n) {
            let h2 = halton((i + 1.0_f64), 2.0_f64);
            let h3 = halton((i + 1.0_f64), 3.0_f64);
            let theta = ((h2 * 2.0_f64) * std::f64::consts::PI);
            let phi = (1.0_f64 - h3).acos();
            let scale = (i / n);
            let dist = (0.1_f64 + ((0.9_f64 * scale) * scale));
            out[((i * 3.0_f64) + 0.0_f64) as usize] = (((phi).sin() * (theta).cos()) * dist) as f32;
            out[((i * 3.0_f64) + 1.0_f64) as usize] = (((phi).sin() * (theta).sin()) * dist) as f32;
            out[((i * 3.0_f64) + 2.0_f64) as usize] = ((phi).cos() * dist) as f32;
            {
                i += 1.0;
                i
            };
        }
    }
    return n;
}

// Source: upstream/packages/effects/src/depthMath.ts:53 (sha256:9465e644f3e3503376d061d750a647aa2d7b4f0b7a1458ed0780b1b9125d4068)
fn halton(index: f64, base: f64) -> f64 {
    let mut result = 0.0_f64;
    let mut f = 1.0_f64;
    let mut i = index;
    while (i > 0.0_f64) {
        f /= base;
        result += (f * (i % base));
        i = (i / base).floor();
    }
    return result;
}
