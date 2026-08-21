// @generated from upstream/packages/effects/src/toneMapMath.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::{AgxToneMapOptions, FilmicToneMapOptions};

// Source: upstream/packages/effects/src/toneMapMath.ts:11 (sha256:17a1f2509c5556dbf56ea9b3feccf97d3c64a72bcd4db3e7dc37f6f8afd8ad59)
pub fn compute_aces_tone_map(x: f64) -> f64 {
    let a = 2.51_f64;
    let b = 0.03_f64;
    let c = 2.43_f64;
    let d = 0.59_f64;
    let e = 0.14_f64;
    return (0.0_f64).max((1.0_f64).min(((x * ((a * x) + b)) / ((x * ((c * x) + d)) + e))));
}

// Source: upstream/packages/effects/src/toneMapMath.ts:22 (sha256:72a038312367a9c28a547d500ce95ed217fefe832684b3ad367854c91b130ef9)
pub fn compute_agx_tone_map(x: f64, options: Option<AgxToneMapOptions>) -> f64 {
    let min_ev = (options.as_ref().and_then(|value| value.min_ev))
        .clone()
        .unwrap_or((-12.47393_f64));
    let max_ev = (options.as_ref().and_then(|value| value.max_ev))
        .clone()
        .unwrap_or(4.026069_f64);
    let val = (1e-10_f64).max(x);
    let log = (min_ev).max((max_ev).min((val).log2()));
    let normalized = ((log - min_ev) / (max_ev - min_ev));
    return agx_default_contrast_approx(normalized);
}

// Source: upstream/packages/effects/src/toneMapMath.ts:33 (sha256:e422d6c03bb0fecffd59c8ef91679fcec2fdbe859ddc713788257c77cf363941)
pub fn compute_exposure_scale(exposure: f64) -> f64 {
    return (2.0_f64).powf(exposure);
}

// Source: upstream/packages/effects/src/toneMapMath.ts:39 (sha256:b1814cb94f07678382cda784dd9607850097e19779614d0c18992715ea3d3b6e)
pub fn compute_filmic_tone_map(x: f64, options: Option<FilmicToneMapOptions>) -> f64 {
    let max_brightness = (options.as_ref().and_then(|value| value.max_brightness))
        .clone()
        .unwrap_or(1.0_f64);
    let contrast = (options.as_ref().and_then(|value| value.contrast))
        .clone()
        .unwrap_or(1.0_f64);
    let linear_start = (options.as_ref().and_then(|value| value.linear_start))
        .clone()
        .unwrap_or(0.22_f64);
    let linear_length = (options.as_ref().and_then(|value| value.linear_length))
        .clone()
        .unwrap_or(0.4_f64);
    let black_tighten = (options.as_ref().and_then(|value| value.black_tighten))
        .clone()
        .unwrap_or(1.33_f64);
    let pedestal = (options.as_ref().and_then(|value| value.pedestal))
        .clone()
        .unwrap_or(0.0_f64);
    let l0 = (((max_brightness - linear_start) * linear_length) / contrast);
    let l0 = (linear_start - (linear_start / contrast));
    let l1 = (linear_start + ((1.0_f64 - linear_start) / contrast));
    let s0 = (linear_start + l0);
    let s1 = (linear_start + (contrast * l0));
    let c2 = (contrast / (max_brightness - s1));
    let cp = ((-c2) / (2.0_f64).ln());
    let w0 = (1.0_f64 - smoothstep01(linear_start, s0, x));
    let t = ((linear_start * (x / linear_start).powf(black_tighten)) + pedestal);
    let l = (linear_start + (contrast * (x - linear_start)));
    let s = (max_brightness - ((max_brightness - s1) * (cp * (x - s0)).exp()));
    return (0.0_f64).max(
        ((((w0 * (1.0_f64 - smoothstep01(l0, l1, x))) * t) + (smoothstep01(l0, l1, x) * l))
            + ((1.0_f64 - w0) * s)),
    );
}

// Source: upstream/packages/effects/src/toneMapMath.ts:62 (sha256:252a35404843f6bf24567d244f2a65831f03de033b2f0045ae0918f2eeeefbe5)
pub fn compute_reinhard_extended_tone_map(x: f64, white: f64) -> f64 {
    let w2 = (white * white);
    return ((x * (1.0_f64 + (x / w2))) / (1.0_f64 + x));
}

// Source: upstream/packages/effects/src/toneMapMath.ts:68 (sha256:00dd72f982f3ac4bd86ec78b893b8f470791c0f0aa551a49deb4f47fd3720a3e)
pub fn compute_reinhard_tone_map(x: f64) -> f64 {
    return (x / (1.0_f64 + x));
}

// Source: upstream/packages/effects/src/toneMapMath.ts:74 (sha256:23a2fab6739b6471d9c6b466017ce0526022e10b2d41b6c33ed776ce4a6ea296)
pub fn compute_uncharted2_tone_map(x: f64) -> f64 {
    let a = 0.15_f64;
    let b = 0.5_f64;
    let c = 0.1_f64;
    let d = 0.2_f64;
    let e = 0.02_f64;
    let f = 0.3_f64;
    return ((((x * ((a * x) + (c * b))) + (d * e)) / ((x * ((a * x) + b)) + (d * f))) - (e / f));
}

// Source: upstream/packages/effects/src/toneMapMath.ts:86 (sha256:6fe0d5922c5891f85a09a5cacbdda98f06c2c08d65691c0c0b45af572506b3ed)
pub fn get_aces_input_matrix(out: &mut Vec<f32>) -> () {
    out[0.0_f64 as usize] = (0.59719_f64) as f32;
    out[1.0_f64 as usize] = (0.076_f64) as f32;
    out[2.0_f64 as usize] = (0.0284_f64) as f32;
    out[3.0_f64 as usize] = (0.35458_f64) as f32;
    out[4.0_f64 as usize] = (0.90834_f64) as f32;
    out[5.0_f64 as usize] = (0.13383_f64) as f32;
    out[6.0_f64 as usize] = (0.04823_f64) as f32;
    out[7.0_f64 as usize] = (0.01566_f64) as f32;
    out[8.0_f64 as usize] = (0.83777_f64) as f32;
}

// Source: upstream/packages/effects/src/toneMapMath.ts:101 (sha256:093298753706eb4508c88f2c6d92025cb45a2d1674688e644f69717ddec008a2)
pub fn get_aces_output_matrix(out: &mut Vec<f32>) -> () {
    out[0.0_f64 as usize] = (1.60475_f64) as f32;
    out[1.0_f64 as usize] = (-0.10208_f64) as f32;
    out[2.0_f64 as usize] = (-0.00327_f64) as f32;
    out[3.0_f64 as usize] = (-0.53108_f64) as f32;
    out[4.0_f64 as usize] = (1.10813_f64) as f32;
    out[5.0_f64 as usize] = (-0.07276_f64) as f32;
    out[6.0_f64 as usize] = (-0.07367_f64) as f32;
    out[7.0_f64 as usize] = (-0.00605_f64) as f32;
    out[8.0_f64 as usize] = (1.07602_f64) as f32;
}

// Source: upstream/packages/effects/src/toneMapMath.ts:114 (sha256:c0390113bbd19cacd7cde9db0a2fd166edb036ea6c1a130f5087a3acf382190c)
fn agx_default_contrast_approx(x: f64) -> f64 {
    let x2 = (x * x);
    let x4 = (x2 * x2);
    return ((((((((15.5_f64 * x4) * x2) - ((40.14_f64 * x4) * x)) + (31.96_f64 * x4))
        - ((6.868_f64 * x2) * x))
        + (0.4298_f64 * x2))
        + (0.1191_f64 * x))
        - 0.00232_f64);
}

// Source: upstream/packages/effects/src/toneMapMath.ts:120 (sha256:62f316a6bc83c3ddcc98bff3772d358b67babccfeb544526c39fce7aa27b1d24)
fn smoothstep01(edge0: f64, edge1: f64, x: f64) -> f64 {
    let t = (0.0_f64).max((1.0_f64).min(((x - edge0) / (edge1 - edge0))));
    return ((t * t) * (3.0_f64 - (2.0_f64 * t)));
}
