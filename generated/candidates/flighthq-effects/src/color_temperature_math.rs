// @generated from upstream/packages/effects/src/colorTemperatureMath.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/effects/src/colorTemperatureMath.ts:10 (sha256:9ffdbbd1f17fb91bdfeb64587c5789e0011c60ea06b6185ecb7c57c4ec8b705a)
pub fn compute_color_temperature_rgb(kelvin: f64, out: &mut Vec<f64>) -> () {
    let temp = ((1000.0_f64).max((40000.0_f64).min(kelvin)) / 100.0_f64);
    let mut r: f64;
    let mut g: f64;
    let mut b: f64;
    if (temp <= 66.0_f64) {
        r = 1.0_f64;
        g = (((99.4708025861_f64 * (temp).ln()) - 161.1195681661_f64) / 255.0_f64);
        b = if (temp <= 19.0_f64) {
            0.0_f64
        } else {
            (((138.5177312231_f64 * (temp - 10.0_f64).ln()) - 305.0447927307_f64) / 255.0_f64)
        };
    } else {
        r = ((329.698727446_f64 * (temp - 60.0_f64).powf((-0.1332047592_f64))) / 255.0_f64);
        g = ((288.1221695283_f64 * (temp - 60.0_f64).powf((-0.0755148492_f64))) / 255.0_f64);
        b = 1.0_f64;
    }
    {
        let __flight_index = (0.0_f64) as usize;
        let __flight_value = (0.0_f64).max((1.0_f64).min(r));
        if __flight_index == out.len() {
            out.push(__flight_value);
        } else {
            out[__flight_index] = __flight_value;
        }
    };
    {
        let __flight_index = (1.0_f64) as usize;
        let __flight_value = (0.0_f64).max((1.0_f64).min(g));
        if __flight_index == out.len() {
            out.push(__flight_value);
        } else {
            out[__flight_index] = __flight_value;
        }
    };
    {
        let __flight_index = (2.0_f64) as usize;
        let __flight_value = (0.0_f64).max((1.0_f64).min(b));
        if __flight_index == out.len() {
            out.push(__flight_value);
        } else {
            out[__flight_index] = __flight_value;
        }
    };
}

// Source: upstream/packages/effects/src/colorTemperatureMath.ts:33 (sha256:482b2767b70b041f0e93df6a1776a478afe3341dc09be20ec18bea780b6fa7dc)
pub fn compute_white_balance_multipliers(temperature: f64, tint: f64, out: &mut Vec<f64>) -> () {
    let kelvin = (6500.0_f64 - (temperature * 4500.0_f64));
    compute_color_temperature_rgb(kelvin, out);
    let green_shift = ((-tint) * 0.1_f64);
    {
        let __flight_index = (0.0_f64) as usize;
        let __flight_value = (0.0_f64).max(out[0.0_f64 as usize].clone());
        if __flight_index == out.len() {
            out.push(__flight_value);
        } else {
            out[__flight_index] = __flight_value;
        }
    };
    {
        let __flight_index = (1.0_f64) as usize;
        let __flight_value = (0.0_f64).max((out[1.0_f64 as usize].clone() + green_shift));
        if __flight_index == out.len() {
            out.push(__flight_value);
        } else {
            out[__flight_index] = __flight_value;
        }
    };
    {
        let __flight_index = (2.0_f64) as usize;
        let __flight_value = (0.0_f64).max(out[2.0_f64 as usize].clone());
        if __flight_index == out.len() {
            out.push(__flight_value);
        } else {
            out[__flight_index] = __flight_value;
        }
    };
}
