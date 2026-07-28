// @generated from upstream/packages/render/src/renderColor.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::RenderState;

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

#[inline]

fn __flight_number_to_string(value: f64, radix: f64) -> String {
    let radix = radix.trunc().clamp(2.0_f64, 36.0_f64) as u32;
    let mut value = value.trunc().rem_euclid(4294967296.0_f64) as u32;
    if value == 0 {
        return "0".to_owned();
    }
    let mut digits = Vec::new();
    while value > 0 {
        let digit = value % radix;
        digits.push(char::from_digit(digit, radix).unwrap());
        value /= radix;
    }
    digits.iter().rev().collect()
}

#[inline]

fn __flight_pad_start(value: String, width: f64, pad: String) -> String {
    let length = value.chars().count();
    let width = width.max(0.0_f64).trunc() as usize;
    if length >= width || pad.is_empty() {
        return value;
    }
    let prefix: String = pad.chars().cycle().take(width - length).collect();
    prefix + &value
}

// Source: upstream/packages/render/src/renderColor.ts:3 (sha256:d8c772704d055919d21675d74b973cd67f49bff35e7cffe13636885aca00e00c)
#[derive(Clone, Default)]
struct SetRenderStateBackgroundColorRecord1 {
    __flight_identity: std::sync::Arc<()>,
    background_color: f64,
    background_color_rgba: Vec<f64>,
    background_color_string: String,
}
impl PartialEq for SetRenderStateBackgroundColorRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn set_render_state_background_color(state: &mut RenderState, color: f64) -> () {
    let mut _state = {
        let __flight_source = &((*state).clone());
        SetRenderStateBackgroundColorRecord1 {
            __flight_identity: std::sync::Arc::clone(&__flight_source.__flight_identity),
            background_color: __flight_source.background_color,
            background_color_rgba: (__flight_source.background_color_rgba).clone(),
            background_color_string: (__flight_source.background_color_string).clone(),
        }
    };
    let uint = (__flight_js_to_u32(color) >> (__flight_js_to_u32(0.0_f64) & 31)) as f64;
    _state.background_color = uint;
    let r = (__flight_js_to_u32(
        (__flight_js_to_i32(uint) & __flight_js_to_i32(4278190080.0_f64)) as f64,
    ) >> (__flight_js_to_u32(24.0_f64) & 31)) as f64;
    let g =
        (__flight_js_to_u32((__flight_js_to_i32(uint) & __flight_js_to_i32(16711680.0_f64)) as f64)
            >> (__flight_js_to_u32(16.0_f64) & 31)) as f64;
    let b =
        (__flight_js_to_u32((__flight_js_to_i32(uint) & __flight_js_to_i32(65280.0_f64)) as f64)
            >> (__flight_js_to_u32(8.0_f64) & 31)) as f64;
    let a = (__flight_js_to_i32(uint) & __flight_js_to_i32(255.0_f64)) as f64;
    {
        let __flight_index = (0.0_f64) as usize;
        let __flight_value = (r / 255.0_f64);
        if __flight_index == _state.background_color_rgba.len() {
            _state.background_color_rgba.push(__flight_value);
        } else {
            _state.background_color_rgba[__flight_index] = __flight_value;
        }
    };
    {
        let __flight_index = (1.0_f64) as usize;
        let __flight_value = (g / 255.0_f64);
        if __flight_index == _state.background_color_rgba.len() {
            _state.background_color_rgba.push(__flight_value);
        } else {
            _state.background_color_rgba[__flight_index] = __flight_value;
        }
    };
    {
        let __flight_index = (2.0_f64) as usize;
        let __flight_value = (b / 255.0_f64);
        if __flight_index == _state.background_color_rgba.len() {
            _state.background_color_rgba.push(__flight_value);
        } else {
            _state.background_color_rgba[__flight_index] = __flight_value;
        }
    };
    {
        let __flight_index = (3.0_f64) as usize;
        let __flight_value = (a / 255.0_f64);
        if __flight_index == _state.background_color_rgba.len() {
            _state.background_color_rgba.push(__flight_value);
        } else {
            _state.background_color_rgba[__flight_index] = __flight_value;
        }
    };
    _state.background_color_string = ("#"
        + (__flight_pad_start(
            __flight_number_to_string(uint, 16.0_f64),
            8.0_f64,
            "0".to_owned(),
        ))
        .to_uppercase());
}
