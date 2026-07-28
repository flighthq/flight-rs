// @generated from upstream/packages/effects/src/boxBlurMath.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/effects/src/boxBlurMath.ts:19 (sha256:9ccf98a8cee1bc22c53de0ab9910e2277cb727865aedcc211e6c5b9c2b3f097e)
pub fn compute_box_blur_pass_radius(sigma: f64, passes: f64, pass: f64) -> f64 {
    if (sigma <= 0.0_f64) {
        return 0.0_f64;
    }
    let lower_width = compute_box_blur_lower_width(sigma, passes);
    let lower_count = compute_box_blur_lower_pass_count(sigma, passes, lower_width);
    let width = if (pass < lower_count) {
        lower_width
    } else {
        (lower_width + 2.0_f64)
    };
    return (0.0_f64).max(((width - 1.0_f64) / 2.0_f64));
}

// Source: upstream/packages/effects/src/boxBlurMath.ts:34 (sha256:e0a44882ee8d4b820da49705d888fc4d608ec3c3e5e80b84764a4ecb700273e2)
pub fn compute_box_blur_radius(sigma: f64, passes: f64) -> f64 {
    if (sigma <= 0.0_f64) {
        return 0.0_f64;
    }
    return (0.0_f64).max(
        (((-1.0_f64) + (1.0_f64 + (((12.0_f64 * sigma) * sigma) / passes)).sqrt()) / 2.0_f64)
            .round(),
    );
}

// Source: upstream/packages/effects/src/boxBlurMath.ts:45 (sha256:16c08b9275383dd1ce12f4e5d50966dd076fa284c6f2a914dd442093bd6c3eb5)
pub fn compute_gaussian_sigma_for_blur_radius(radius: f64, passes: f64) -> f64 {
    if ((radius <= 0.0_f64) || (passes <= 0.0_f64)) {
        return 0.0_f64;
    }
    let width = ((2.0_f64 * radius) + 1.0_f64);
    return (((passes * width) * width) / 12.0_f64).sqrt();
}

// Source: upstream/packages/effects/src/boxBlurMath.ts:53 (sha256:feea8fcdbbc1af4244f6d21c6a5059fae6ad8552aca26bbe2a27bb33d4f99723)
fn compute_box_blur_lower_width(sigma: f64, passes: f64) -> f64 {
    let mut width = (((((12.0_f64 * sigma) * sigma) / passes) + 1.0_f64).sqrt()).floor();
    if ((width % 2.0_f64) == 0.0_f64) {
        width -= 1.0_f64;
    }
    return width;
}

// Source: upstream/packages/effects/src/boxBlurMath.ts:62 (sha256:9c0931a24cc07a05627829f02a496bff1d0e41162345f146c3c60583aacc29a2)
fn compute_box_blur_lower_pass_count(sigma: f64, passes: f64, lower_width: f64) -> f64 {
    return ((((12.0_f64 * sigma) * sigma)
        - (passes * (((lower_width * lower_width) + (4.0_f64 * lower_width)) + 3.0_f64)))
        / (((-4.0_f64) * lower_width) - 4.0_f64))
        .round();
}
