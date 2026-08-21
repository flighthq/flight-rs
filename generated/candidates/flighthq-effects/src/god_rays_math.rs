// @generated from upstream/packages/effects/src/godRaysMath.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::GodRaysEffect;

// Source: upstream/packages/effects/src/godRaysMath.ts:11 (sha256:3e3cf5c493a3ad6a7d97f19db5a0575517d2ae88728f292a9191c2e78746c43b)
pub fn compute_god_rays_accumulation_scale(effect: &GodRaysEffect) -> f64 {
    let samples = (1.0_f64).max((effect.samples).clone().unwrap_or(100.0_f64));
    let weight = (0.000001_f64).max((effect.weight).clone().unwrap_or(0.4_f64));
    let exposure = (0.000001_f64).max((effect.exposure).clone().unwrap_or(0.1_f64));
    return (1.0_f64 / ((samples * weight) * exposure));
}

// Source: upstream/packages/effects/src/godRaysMath.ts:20 (sha256:d58a79704fc00c8f4bbffa1f4236706dbd2362a770610166750a699769922be3)
pub fn compute_god_rays_light_center(effect: &GodRaysEffect, out: &mut Vec<f64>) -> () {
    {
        let __flight_index = (0.0_f64) as usize;
        let __flight_value =
            (0.0_f64).max((1.0_f64).min((effect.center_x).clone().unwrap_or(0.5_f64)));
        if __flight_index == out.len() {
            out.push(__flight_value);
        } else {
            out[__flight_index] = __flight_value;
        }
    };
    {
        let __flight_index = (1.0_f64) as usize;
        let __flight_value =
            (0.0_f64).max((1.0_f64).min((effect.center_y).clone().unwrap_or(0.5_f64)));
        if __flight_index == out.len() {
            out.push(__flight_value);
        } else {
            out[__flight_index] = __flight_value;
        }
    };
}

// Source: upstream/packages/effects/src/godRaysMath.ts:29 (sha256:a2cbd9f9debdac82c52aa37832bfa9ffce57b711d350abca94c577af69e1d8e9)
pub fn compute_god_rays_sample_weight(effect: &GodRaysEffect, sample_index: f64) -> f64 {
    let decay = (effect.decay).clone().unwrap_or(0.96_f64);
    let weight = (effect.weight).clone().unwrap_or(0.4_f64);
    let exposure = (effect.exposure).clone().unwrap_or(0.1_f64);
    return (((decay).powf(sample_index) * weight) * exposure);
}

// Source: upstream/packages/effects/src/godRaysMath.ts:42 (sha256:de47c28e077c19697064e93bd08986789970517edba8b7516970bd661826195a)
pub fn compute_god_rays_step_size(
    effect: &GodRaysEffect,
    px: f64,
    py: f64,
    out: &mut Vec<f64>,
) -> () {
    let cx = (effect.center_x).clone().unwrap_or(0.5_f64);
    let cy = (effect.center_y).clone().unwrap_or(0.5_f64);
    let density = (effect.density).clone().unwrap_or(0.96_f64);
    let samples = (1.0_f64).max((effect.samples).clone().unwrap_or(100.0_f64));
    let dx = (((cx - px) * density) / samples);
    let dy = (((cy - py) * density) / samples);
    {
        let __flight_index = (0.0_f64) as usize;
        let __flight_value = dx;
        if __flight_index == out.len() {
            out.push(__flight_value);
        } else {
            out[__flight_index] = __flight_value;
        }
    };
    {
        let __flight_index = (1.0_f64) as usize;
        let __flight_value = dy;
        if __flight_index == out.len() {
            out.push(__flight_value);
        } else {
            out[__flight_index] = __flight_value;
        }
    };
}
