// @generated from upstream/packages/color/src/colorFromKelvin.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

#[inline]
fn __flight_js_to_u32(value: f64) -> u32 {
    if !value.is_finite() || value == 0.0 {
        return 0;
    }
    value.trunc().rem_euclid(4294967296.0_f64) as u32
}

#[inline]
fn __flight_js_to_i32(value: f64) -> i32 {
    __flight_js_to_u32(value) as i32
}

// Source: upstream/packages/color/src/colorFromKelvin.ts:8 (sha256:a560a4115d973fe9ccd272a3f1fc6b5305ff6fe52a693d414a78633546f3dccc)
pub fn create_color_from_kelvin(kelvin: f64) -> f64 {
    let temp = ((1000.0_f64).max((40000.0_f64).min(kelvin)) / 100.0_f64);
    let mut r: f64;
    let mut g: f64;
    let mut b: f64;
    if (temp <= 66.0_f64) {
        r = 255.0_f64;
    } else {
        r = (329.698727446_f64 * (temp - 60.0_f64).powf((-0.1332047592_f64)));
    }
    if (temp <= 66.0_f64) {
        g = ((99.4708025861_f64 * (temp).ln()) - 161.1195681661_f64);
    } else {
        g = (288.1221695283_f64 * (temp - 60.0_f64).powf((-0.0755148492_f64)));
    }
    if (temp >= 66.0_f64) {
        b = 255.0_f64;
    } else {
        if (temp <= 19.0_f64) {
            b = 0.0_f64;
        } else {
            b = ((138.5177312231_f64 * (temp - 10.0_f64).ln()) - 305.0447927307_f64);
        }
    }
    let ri = (0.0_f64).max((255.0_f64).min((r).round()));
    let gi = (0.0_f64).max((255.0_f64).min((g).round()));
    let bi = (0.0_f64).max((255.0_f64).min((b).round()));
    return (__flight_js_to_u32(
        (__flight_js_to_i32(
            (__flight_js_to_i32(
                (__flight_js_to_i32(
                    __flight_js_to_i32(ri).wrapping_shl((__flight_js_to_u32(24.0_f64) & 31)) as f64,
                ) | __flight_js_to_i32(
                    __flight_js_to_i32(gi).wrapping_shl((__flight_js_to_u32(16.0_f64) & 31)) as f64,
                )) as f64,
            ) | __flight_js_to_i32(
                __flight_js_to_i32(bi).wrapping_shl((__flight_js_to_u32(8.0_f64) & 31)) as f64,
            )) as f64,
        ) | __flight_js_to_i32(255.0_f64)) as f64,
    ) >> (__flight_js_to_u32(0.0_f64) & 31)) as f64;
}
