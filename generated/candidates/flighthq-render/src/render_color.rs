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

// Source: upstream/packages/render/src/renderColor.ts:3 (sha256:d8c772704d055919d21675d74b973cd67f49bff35e7cffe13636885aca00e00c)
#[derive(Clone)]
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
    let mut _state = state;
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
    _state.background_color_string =
        ("#" + (((uint.to_string)(16.0_f64).pad_start)(8.0_f64, "0").to_upper_case)());
}
