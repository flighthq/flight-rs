// @generated from upstream/packages/path-formats/src/svgPathData.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_path::{
    append_path_arc_to, append_path_close, append_path_cubic_curve_to, append_path_curve_to,
    append_path_line_to, append_path_move_to, create_path, for_each_path_segment,
};
use flighthq_types::{Path, PathSegment};

#[inline]

fn __flight_string_index_of(value: &str, search: &str, position: f64) -> f64 {
    let value: Vec<u16> = value.encode_utf16().collect();
    let search: Vec<u16> = search.encode_utf16().collect();
    let start = if position.is_nan() || position <= 0.0_f64 {
        0_usize
    } else if position >= value.len() as f64 {
        value.len()
    } else {
        position.trunc() as usize
    };
    if search.is_empty() {
        return start as f64;
    }
    value[start..]
        .windows(search.len())
        .position(|window| window == search)
        .map_or(-1.0_f64, |index| (start + index) as f64)
}

#[inline]

fn __flight_string_slice(value: &str, start: f64, end: Option<f64>) -> String {
    let value: Vec<u16> = value.encode_utf16().collect();
    let length = value.len();
    let relative = |index: f64| -> usize {
        if index.is_nan() {
            0
        } else if index < 0.0_f64 {
            length.saturating_sub((-index.trunc()) as usize)
        } else {
            (index.trunc() as usize).min(length)
        }
    };
    let start = relative(start);
    let end = end.map_or(length, relative);
    String::from_utf16_lossy(&value[start..end.max(start)])
}

#[inline]

fn __flight_parse_float(value: &str) -> f64 {
    let value = value.trim_start();
    if value.starts_with("Infinity") || value.starts_with("+Infinity") {
        return f64::INFINITY;
    }
    if value.starts_with("-Infinity") {
        return f64::NEG_INFINITY;
    }
    let bytes = value.as_bytes();
    let mut index = usize::from(matches!(bytes.first(), Some(b'+' | b'-')));
    let mut digits = 0_usize;
    while matches!(bytes.get(index), Some(b'0'..=b'9')) {
        index += 1;
        digits += 1;
    }
    if bytes.get(index) == Some(&b'.') {
        index += 1;
        while matches!(bytes.get(index), Some(b'0'..=b'9')) {
            index += 1;
            digits += 1;
        }
    }
    if digits == 0 {
        return f64::NAN;
    }
    let mantissa_end = index;
    if matches!(bytes.get(index), Some(b'e' | b'E')) {
        let exponent_start = index;
        index += 1;
        if matches!(bytes.get(index), Some(b'+' | b'-')) {
            index += 1;
        }
        let exponent_digits = index;
        while matches!(bytes.get(index), Some(b'0'..=b'9')) {
            index += 1;
        }
        if index == exponent_digits {
            index = exponent_start;
        }
    }
    value[..if index > mantissa_end {
        index
    } else {
        mantissa_end
    }]
        .parse::<f64>()
        .unwrap_or(f64::NAN)
}

#[derive(Clone, Default)]
pub struct SharedStructuralRecord1 {
    pub __flight_identity: std::sync::Arc<()>,
    pub precision: Option<f64>,
}
impl PartialEq for SharedStructuralRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/path-formats/src/svgPathData.ts:26 (sha256:47740f889abfd44be81e63395000f16345b23c75821a0adc73f58ce86f7c11e3)
pub fn append_svg_path_data(path: &mut Path, d: String) -> bool {
    let __flight_utf16_d: std::sync::Arc<Vec<u16>> =
        std::sync::Arc::new(d.encode_utf16().collect());
    let length = (__flight_utf16_d.len() as f64);
    let pos: std::sync::Arc<std::sync::Mutex<f64>> =
        std::sync::Arc::new(std::sync::Mutex::new(0.0_f64));
    let mut current_x = 0.0_f64;
    let mut current_y = 0.0_f64;
    let mut start_x = 0.0_f64;
    let mut start_y = 0.0_f64;
    let mut last_control2_x = 0.0_f64;
    let mut last_control2_y = 0.0_f64;
    let mut last_quad_control_x = 0.0_f64;
    let mut last_quad_control_y = 0.0_f64;
    let mut last_kind = "".to_owned();
    let mut skip_separators: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>,
    > = std::sync::Arc::new(std::sync::Mutex::new(Box::new({
        let __flight_utf16_d = __flight_utf16_d.clone();
        let mut pos = pos.clone();
        move || -> () {
            while ((*pos.lock().unwrap()).clone() < length) {
                let c = {
                    let __flight_units: &[u16] = &__flight_utf16_d;
                    let __flight_raw_index = (*pos.lock().unwrap()).clone();
                    let __flight_index = if __flight_raw_index.is_nan() {
                        0_i64
                    } else if __flight_raw_index.is_finite() {
                        __flight_raw_index.trunc() as i64
                    } else {
                        -1_i64
                    };
                    if __flight_index < 0 {
                        f64::NAN
                    } else {
                        __flight_units
                            .get(__flight_index as usize)
                            .map_or(f64::NAN, |unit| f64::from(*unit))
                    }
                };
                if (((((c == 32.0_f64) || (c == 9.0_f64)) || (c == 10.0_f64)) || (c == 13.0_f64))
                    || (c == 12.0_f64))
                    || (c == 44.0_f64)
                {
                    {
                        (*pos.lock().unwrap()) += 1.0;
                        (*pos.lock().unwrap())
                    };
                } else {
                    break;
                }
            }
        }
    })
        as Box<dyn FnMut() -> () + Send + 'static>));
    let mut read_number: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut() -> Option<f64> + Send + 'static>>,
    > = std::sync::Arc::new(std::sync::Mutex::new(Box::new({
        let __flight_utf16_d = __flight_utf16_d.clone();
        let mut pos = pos.clone();
        let skip_separators = skip_separators.clone();
        move || -> Option<f64> {
            {
                let __flight_callback = (skip_separators).clone();
                let __flight_result = __flight_callback.lock().unwrap()();
                __flight_result
            };
            let start = (*pos.lock().unwrap()).clone();
            if ((*pos.lock().unwrap()).clone() < length)
                && (({
                    let __flight_units: &[u16] = &__flight_utf16_d;
                    let __flight_raw_index = (*pos.lock().unwrap()).clone();
                    if __flight_raw_index.is_finite()
                        && __flight_raw_index >= 0.0_f64
                        && __flight_raw_index.fract() == 0.0_f64
                    {
                        __flight_units
                            .get(__flight_raw_index as usize)
                            .map_or_else(String::new, |unit| String::from_utf16_lossy(&[*unit]))
                    } else {
                        String::new()
                    }
                } == "+")
                    || ({
                        let __flight_units: &[u16] = &__flight_utf16_d;
                        let __flight_raw_index = (*pos.lock().unwrap()).clone();
                        if __flight_raw_index.is_finite()
                            && __flight_raw_index >= 0.0_f64
                            && __flight_raw_index.fract() == 0.0_f64
                        {
                            __flight_units
                                .get(__flight_raw_index as usize)
                                .map_or_else(String::new, |unit| String::from_utf16_lossy(&[*unit]))
                        } else {
                            String::new()
                        }
                    } == "-"))
            {
                {
                    (*pos.lock().unwrap()) += 1.0;
                    (*pos.lock().unwrap())
                };
            }
            let mut saw_digit = false;
            while (((*pos.lock().unwrap()).clone() < length)
                && ({
                    let __flight_units: &[u16] = &__flight_utf16_d;
                    let __flight_raw_index = (*pos.lock().unwrap()).clone();
                    if __flight_raw_index.is_finite()
                        && __flight_raw_index >= 0.0_f64
                        && __flight_raw_index.fract() == 0.0_f64
                    {
                        __flight_units
                            .get(__flight_raw_index as usize)
                            .map_or_else(String::new, |unit| String::from_utf16_lossy(&[*unit]))
                    } else {
                        String::new()
                    }
                } >= "0".to_owned()))
                && ({
                    let __flight_units: &[u16] = &__flight_utf16_d;
                    let __flight_raw_index = (*pos.lock().unwrap()).clone();
                    if __flight_raw_index.is_finite()
                        && __flight_raw_index >= 0.0_f64
                        && __flight_raw_index.fract() == 0.0_f64
                    {
                        __flight_units
                            .get(__flight_raw_index as usize)
                            .map_or_else(String::new, |unit| String::from_utf16_lossy(&[*unit]))
                    } else {
                        String::new()
                    }
                } <= "9".to_owned())
            {
                {
                    (*pos.lock().unwrap()) += 1.0;
                    (*pos.lock().unwrap())
                };
                saw_digit = true;
            }
            if ((*pos.lock().unwrap()).clone() < length)
                && ({
                    let __flight_units: &[u16] = &__flight_utf16_d;
                    let __flight_raw_index = (*pos.lock().unwrap()).clone();
                    if __flight_raw_index.is_finite()
                        && __flight_raw_index >= 0.0_f64
                        && __flight_raw_index.fract() == 0.0_f64
                    {
                        __flight_units
                            .get(__flight_raw_index as usize)
                            .map_or_else(String::new, |unit| String::from_utf16_lossy(&[*unit]))
                    } else {
                        String::new()
                    }
                } == ".")
            {
                {
                    (*pos.lock().unwrap()) += 1.0;
                    (*pos.lock().unwrap())
                };
                while (((*pos.lock().unwrap()).clone() < length)
                    && ({
                        let __flight_units: &[u16] = &__flight_utf16_d;
                        let __flight_raw_index = (*pos.lock().unwrap()).clone();
                        if __flight_raw_index.is_finite()
                            && __flight_raw_index >= 0.0_f64
                            && __flight_raw_index.fract() == 0.0_f64
                        {
                            __flight_units
                                .get(__flight_raw_index as usize)
                                .map_or_else(String::new, |unit| String::from_utf16_lossy(&[*unit]))
                        } else {
                            String::new()
                        }
                    } >= "0".to_owned()))
                    && ({
                        let __flight_units: &[u16] = &__flight_utf16_d;
                        let __flight_raw_index = (*pos.lock().unwrap()).clone();
                        if __flight_raw_index.is_finite()
                            && __flight_raw_index >= 0.0_f64
                            && __flight_raw_index.fract() == 0.0_f64
                        {
                            __flight_units
                                .get(__flight_raw_index as usize)
                                .map_or_else(String::new, |unit| String::from_utf16_lossy(&[*unit]))
                        } else {
                            String::new()
                        }
                    } <= "9".to_owned())
                {
                    {
                        (*pos.lock().unwrap()) += 1.0;
                        (*pos.lock().unwrap())
                    };
                    saw_digit = true;
                }
            }
            if (!saw_digit) {
                (*pos.lock().unwrap()) = start;
                return None;
            }
            if ((*pos.lock().unwrap()).clone() < length)
                && (({
                    let __flight_units: &[u16] = &__flight_utf16_d;
                    let __flight_raw_index = (*pos.lock().unwrap()).clone();
                    if __flight_raw_index.is_finite()
                        && __flight_raw_index >= 0.0_f64
                        && __flight_raw_index.fract() == 0.0_f64
                    {
                        __flight_units
                            .get(__flight_raw_index as usize)
                            .map_or_else(String::new, |unit| String::from_utf16_lossy(&[*unit]))
                    } else {
                        String::new()
                    }
                } == "e")
                    || ({
                        let __flight_units: &[u16] = &__flight_utf16_d;
                        let __flight_raw_index = (*pos.lock().unwrap()).clone();
                        if __flight_raw_index.is_finite()
                            && __flight_raw_index >= 0.0_f64
                            && __flight_raw_index.fract() == 0.0_f64
                        {
                            __flight_units
                                .get(__flight_raw_index as usize)
                                .map_or_else(String::new, |unit| String::from_utf16_lossy(&[*unit]))
                        } else {
                            String::new()
                        }
                    } == "E"))
            {
                let exp_start = (*pos.lock().unwrap()).clone();
                {
                    (*pos.lock().unwrap()) += 1.0;
                    (*pos.lock().unwrap())
                };
                if ((*pos.lock().unwrap()).clone() < length)
                    && (({
                        let __flight_units: &[u16] = &__flight_utf16_d;
                        let __flight_raw_index = (*pos.lock().unwrap()).clone();
                        if __flight_raw_index.is_finite()
                            && __flight_raw_index >= 0.0_f64
                            && __flight_raw_index.fract() == 0.0_f64
                        {
                            __flight_units
                                .get(__flight_raw_index as usize)
                                .map_or_else(String::new, |unit| String::from_utf16_lossy(&[*unit]))
                        } else {
                            String::new()
                        }
                    } == "+")
                        || ({
                            let __flight_units: &[u16] = &__flight_utf16_d;
                            let __flight_raw_index = (*pos.lock().unwrap()).clone();
                            if __flight_raw_index.is_finite()
                                && __flight_raw_index >= 0.0_f64
                                && __flight_raw_index.fract() == 0.0_f64
                            {
                                __flight_units
                                    .get(__flight_raw_index as usize)
                                    .map_or_else(String::new, |unit| {
                                        String::from_utf16_lossy(&[*unit])
                                    })
                            } else {
                                String::new()
                            }
                        } == "-"))
                {
                    {
                        (*pos.lock().unwrap()) += 1.0;
                        (*pos.lock().unwrap())
                    };
                }
                let mut exp_digit = false;
                while (((*pos.lock().unwrap()).clone() < length)
                    && ({
                        let __flight_units: &[u16] = &__flight_utf16_d;
                        let __flight_raw_index = (*pos.lock().unwrap()).clone();
                        if __flight_raw_index.is_finite()
                            && __flight_raw_index >= 0.0_f64
                            && __flight_raw_index.fract() == 0.0_f64
                        {
                            __flight_units
                                .get(__flight_raw_index as usize)
                                .map_or_else(String::new, |unit| String::from_utf16_lossy(&[*unit]))
                        } else {
                            String::new()
                        }
                    } >= "0".to_owned()))
                    && ({
                        let __flight_units: &[u16] = &__flight_utf16_d;
                        let __flight_raw_index = (*pos.lock().unwrap()).clone();
                        if __flight_raw_index.is_finite()
                            && __flight_raw_index >= 0.0_f64
                            && __flight_raw_index.fract() == 0.0_f64
                        {
                            __flight_units
                                .get(__flight_raw_index as usize)
                                .map_or_else(String::new, |unit| String::from_utf16_lossy(&[*unit]))
                        } else {
                            String::new()
                        }
                    } <= "9".to_owned())
                {
                    {
                        (*pos.lock().unwrap()) += 1.0;
                        (*pos.lock().unwrap())
                    };
                    exp_digit = true;
                }
                if (!exp_digit) {
                    (*pos.lock().unwrap()) = exp_start;
                }
            }
            let value = __flight_parse_float(
                &(__flight_string_slice(&(d), start, Some((*pos.lock().unwrap()).clone()))),
            );
            return if (value).is_finite() {
                Some(value)
            } else {
                None
            };
        }
    })
        as Box<dyn FnMut() -> Option<f64> + Send + 'static>));
    let mut read_flag: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut() -> Option<f64> + Send + 'static>>,
    > = std::sync::Arc::new(std::sync::Mutex::new(Box::new({
        let __flight_utf16_d = __flight_utf16_d.clone();
        let mut pos = pos.clone();
        let skip_separators = skip_separators.clone();
        move || -> Option<f64> {
            {
                let __flight_callback = (skip_separators).clone();
                let __flight_result = __flight_callback.lock().unwrap()();
                __flight_result
            };
            if ((*pos.lock().unwrap()).clone() < length)
                && ({
                    let __flight_units: &[u16] = &__flight_utf16_d;
                    let __flight_raw_index = (*pos.lock().unwrap()).clone();
                    if __flight_raw_index.is_finite()
                        && __flight_raw_index >= 0.0_f64
                        && __flight_raw_index.fract() == 0.0_f64
                    {
                        __flight_units
                            .get(__flight_raw_index as usize)
                            .map_or_else(String::new, |unit| String::from_utf16_lossy(&[*unit]))
                    } else {
                        String::new()
                    }
                } == "0")
            {
                {
                    (*pos.lock().unwrap()) += 1.0;
                    (*pos.lock().unwrap())
                };
                return Some(0.0_f64);
            }
            if ((*pos.lock().unwrap()).clone() < length)
                && ({
                    let __flight_units: &[u16] = &__flight_utf16_d;
                    let __flight_raw_index = (*pos.lock().unwrap()).clone();
                    if __flight_raw_index.is_finite()
                        && __flight_raw_index >= 0.0_f64
                        && __flight_raw_index.fract() == 0.0_f64
                    {
                        __flight_units
                            .get(__flight_raw_index as usize)
                            .map_or_else(String::new, |unit| String::from_utf16_lossy(&[*unit]))
                    } else {
                        String::new()
                    }
                } == "1")
            {
                {
                    (*pos.lock().unwrap()) += 1.0;
                    (*pos.lock().unwrap())
                };
                return Some(1.0_f64);
            }
            return None;
        }
    })
        as Box<dyn FnMut() -> Option<f64> + Send + 'static>));
    while true {
        {
            let __flight_callback = (skip_separators).clone();
            let __flight_result = __flight_callback.lock().unwrap()();
            __flight_result
        };
        if ((*pos.lock().unwrap()).clone() >= length) {
            break;
        }
        let mut command_letter = {
            let __flight_units: &[u16] = &__flight_utf16_d;
            let __flight_raw_index = (*pos.lock().unwrap()).clone();
            if __flight_raw_index.is_finite()
                && __flight_raw_index >= 0.0_f64
                && __flight_raw_index.fract() == 0.0_f64
            {
                __flight_units
                    .get(__flight_raw_index as usize)
                    .map_or_else(String::new, |unit| String::from_utf16_lossy(&[*unit]))
            } else {
                String::new()
            }
        };
        if (!is_svg_command_letter((command_letter).clone())) {
            return false;
        }
        {
            (*pos.lock().unwrap()) += 1.0;
            (*pos.lock().unwrap())
        };
        if ((last_kind == "") && ((command_letter).clone() != "M"))
            && ((command_letter).clone() != "m")
        {
            return false;
        }
        if ((command_letter).clone() == "Z") || ((command_letter).clone() == "z") {
            append_path_close(path);
            current_x = start_x;
            current_y = start_y;
            last_kind = "Z".to_owned();
            continue;
        }
        let mut active = (command_letter).clone();
        let mut first = true;
        while true {
            if (!first) {
                {
                    let __flight_callback = (skip_separators).clone();
                    let __flight_result = __flight_callback.lock().unwrap()();
                    __flight_result
                };
                if ((*pos.lock().unwrap()).clone() >= length) {
                    break;
                }
                if is_svg_command_letter({
                    let __flight_units: &[u16] = &__flight_utf16_d;
                    let __flight_raw_index = (*pos.lock().unwrap()).clone();
                    if __flight_raw_index.is_finite()
                        && __flight_raw_index >= 0.0_f64
                        && __flight_raw_index.fract() == 0.0_f64
                    {
                        __flight_units
                            .get(__flight_raw_index as usize)
                            .map_or_else(String::new, |unit| String::from_utf16_lossy(&[*unit]))
                    } else {
                        String::new()
                    }
                }) {
                    break;
                }
            }
            let relative = ((active).clone() >= "a".to_owned());
            let upper = if relative {
                ((active).clone()).to_uppercase()
            } else {
                (active).clone()
            };
            if (upper == "M") {
                let nx = {
                    let __flight_callback = (read_number).clone();
                    let __flight_result = __flight_callback.lock().unwrap()();
                    __flight_result
                };
                let ny = {
                    let __flight_callback = (read_number).clone();
                    let __flight_result = __flight_callback.lock().unwrap()();
                    __flight_result
                };
                if ((nx).is_none()) || ((ny).is_none()) {
                    return false;
                }
                current_x = if relative {
                    (current_x + *(nx.as_ref().unwrap()))
                } else {
                    *(nx.as_ref().unwrap())
                };
                current_y = if relative {
                    (current_y + *(ny.as_ref().unwrap()))
                } else {
                    *(ny.as_ref().unwrap())
                };
                start_x = current_x;
                start_y = current_y;
                append_path_move_to(path, current_x, current_y);
                last_kind = "M".to_owned();
            } else {
                if (upper == "L") {
                    let nx = {
                        let __flight_callback = (read_number).clone();
                        let __flight_result = __flight_callback.lock().unwrap()();
                        __flight_result
                    };
                    let ny = {
                        let __flight_callback = (read_number).clone();
                        let __flight_result = __flight_callback.lock().unwrap()();
                        __flight_result
                    };
                    if ((nx).is_none()) || ((ny).is_none()) {
                        return false;
                    }
                    current_x = if relative {
                        (current_x + *(nx.as_ref().unwrap()))
                    } else {
                        *(nx.as_ref().unwrap())
                    };
                    current_y = if relative {
                        (current_y + *(ny.as_ref().unwrap()))
                    } else {
                        *(ny.as_ref().unwrap())
                    };
                    append_path_line_to(path, current_x, current_y);
                    last_kind = "L".to_owned();
                } else {
                    if (upper == "H") {
                        let nx = {
                            let __flight_callback = (read_number).clone();
                            let __flight_result = __flight_callback.lock().unwrap()();
                            __flight_result
                        };
                        if (nx).is_none() {
                            return false;
                        }
                        current_x = if relative {
                            (current_x + *(nx.as_ref().unwrap()))
                        } else {
                            *(nx.as_ref().unwrap())
                        };
                        append_path_line_to(path, current_x, current_y);
                        last_kind = "L".to_owned();
                    } else {
                        if (upper == "V") {
                            let ny = {
                                let __flight_callback = (read_number).clone();
                                let __flight_result = __flight_callback.lock().unwrap()();
                                __flight_result
                            };
                            if (ny).is_none() {
                                return false;
                            }
                            current_y = if relative {
                                (current_y + *(ny.as_ref().unwrap()))
                            } else {
                                *(ny.as_ref().unwrap())
                            };
                            append_path_line_to(path, current_x, current_y);
                            last_kind = "L".to_owned();
                        } else {
                            if (upper == "C") {
                                let x1 = {
                                    let __flight_callback = (read_number).clone();
                                    let __flight_result = __flight_callback.lock().unwrap()();
                                    __flight_result
                                };
                                let y1 = {
                                    let __flight_callback = (read_number).clone();
                                    let __flight_result = __flight_callback.lock().unwrap()();
                                    __flight_result
                                };
                                let x2 = {
                                    let __flight_callback = (read_number).clone();
                                    let __flight_result = __flight_callback.lock().unwrap()();
                                    __flight_result
                                };
                                let y2 = {
                                    let __flight_callback = (read_number).clone();
                                    let __flight_result = __flight_callback.lock().unwrap()();
                                    __flight_result
                                };
                                let x = {
                                    let __flight_callback = (read_number).clone();
                                    let __flight_result = __flight_callback.lock().unwrap()();
                                    __flight_result
                                };
                                let y = {
                                    let __flight_callback = (read_number).clone();
                                    let __flight_result = __flight_callback.lock().unwrap()();
                                    __flight_result
                                };
                                if ((((((x1).is_none()) || ((y1).is_none())) || ((x2).is_none()))
                                    || ((y2).is_none()))
                                    || ((x).is_none()))
                                    || ((y).is_none())
                                {
                                    return false;
                                }
                                let c1x = if relative {
                                    (current_x + *(x1.as_ref().unwrap()))
                                } else {
                                    *(x1.as_ref().unwrap())
                                };
                                let c1y = if relative {
                                    (current_y + *(y1.as_ref().unwrap()))
                                } else {
                                    *(y1.as_ref().unwrap())
                                };
                                let c2x = if relative {
                                    (current_x + *(x2.as_ref().unwrap()))
                                } else {
                                    *(x2.as_ref().unwrap())
                                };
                                let c2y = if relative {
                                    (current_y + *(y2.as_ref().unwrap()))
                                } else {
                                    *(y2.as_ref().unwrap())
                                };
                                let ax = if relative {
                                    (current_x + *(x.as_ref().unwrap()))
                                } else {
                                    *(x.as_ref().unwrap())
                                };
                                let ay = if relative {
                                    (current_y + *(y.as_ref().unwrap()))
                                } else {
                                    *(y.as_ref().unwrap())
                                };
                                append_path_cubic_curve_to(path, c1x, c1y, c2x, c2y, ax, ay);
                                last_control2_x = c2x;
                                last_control2_y = c2y;
                                current_x = ax;
                                current_y = ay;
                                last_kind = "C".to_owned();
                            } else {
                                if (upper == "S") {
                                    let x2 = {
                                        let __flight_callback = (read_number).clone();
                                        let __flight_result = __flight_callback.lock().unwrap()();
                                        __flight_result
                                    };
                                    let y2 = {
                                        let __flight_callback = (read_number).clone();
                                        let __flight_result = __flight_callback.lock().unwrap()();
                                        __flight_result
                                    };
                                    let x = {
                                        let __flight_callback = (read_number).clone();
                                        let __flight_result = __flight_callback.lock().unwrap()();
                                        __flight_result
                                    };
                                    let y = {
                                        let __flight_callback = (read_number).clone();
                                        let __flight_result = __flight_callback.lock().unwrap()();
                                        __flight_result
                                    };
                                    if ((((x2).is_none()) || ((y2).is_none())) || ((x).is_none()))
                                        || ((y).is_none())
                                    {
                                        return false;
                                    }
                                    let reflect = (last_kind == "C") || (last_kind == "S");
                                    let c1x = if reflect {
                                        ((2.0_f64 * current_x) - last_control2_x)
                                    } else {
                                        current_x
                                    };
                                    let c1y = if reflect {
                                        ((2.0_f64 * current_y) - last_control2_y)
                                    } else {
                                        current_y
                                    };
                                    let c2x = if relative {
                                        (current_x + *(x2.as_ref().unwrap()))
                                    } else {
                                        *(x2.as_ref().unwrap())
                                    };
                                    let c2y = if relative {
                                        (current_y + *(y2.as_ref().unwrap()))
                                    } else {
                                        *(y2.as_ref().unwrap())
                                    };
                                    let ax = if relative {
                                        (current_x + *(x.as_ref().unwrap()))
                                    } else {
                                        *(x.as_ref().unwrap())
                                    };
                                    let ay = if relative {
                                        (current_y + *(y.as_ref().unwrap()))
                                    } else {
                                        *(y.as_ref().unwrap())
                                    };
                                    append_path_cubic_curve_to(path, c1x, c1y, c2x, c2y, ax, ay);
                                    last_control2_x = c2x;
                                    last_control2_y = c2y;
                                    current_x = ax;
                                    current_y = ay;
                                    last_kind = "S".to_owned();
                                } else {
                                    if (upper == "Q") {
                                        let x1 = {
                                            let __flight_callback = (read_number).clone();
                                            let __flight_result =
                                                __flight_callback.lock().unwrap()();
                                            __flight_result
                                        };
                                        let y1 = {
                                            let __flight_callback = (read_number).clone();
                                            let __flight_result =
                                                __flight_callback.lock().unwrap()();
                                            __flight_result
                                        };
                                        let x = {
                                            let __flight_callback = (read_number).clone();
                                            let __flight_result =
                                                __flight_callback.lock().unwrap()();
                                            __flight_result
                                        };
                                        let y = {
                                            let __flight_callback = (read_number).clone();
                                            let __flight_result =
                                                __flight_callback.lock().unwrap()();
                                            __flight_result
                                        };
                                        if ((((x1).is_none()) || ((y1).is_none()))
                                            || ((x).is_none()))
                                            || ((y).is_none())
                                        {
                                            return false;
                                        }
                                        let cx = if relative {
                                            (current_x + *(x1.as_ref().unwrap()))
                                        } else {
                                            *(x1.as_ref().unwrap())
                                        };
                                        let cy = if relative {
                                            (current_y + *(y1.as_ref().unwrap()))
                                        } else {
                                            *(y1.as_ref().unwrap())
                                        };
                                        let ax = if relative {
                                            (current_x + *(x.as_ref().unwrap()))
                                        } else {
                                            *(x.as_ref().unwrap())
                                        };
                                        let ay = if relative {
                                            (current_y + *(y.as_ref().unwrap()))
                                        } else {
                                            *(y.as_ref().unwrap())
                                        };
                                        append_path_curve_to(path, cx, cy, ax, ay);
                                        last_quad_control_x = cx;
                                        last_quad_control_y = cy;
                                        current_x = ax;
                                        current_y = ay;
                                        last_kind = "Q".to_owned();
                                    } else {
                                        if (upper == "T") {
                                            let x = {
                                                let __flight_callback = (read_number).clone();
                                                let __flight_result =
                                                    __flight_callback.lock().unwrap()();
                                                __flight_result
                                            };
                                            let y = {
                                                let __flight_callback = (read_number).clone();
                                                let __flight_result =
                                                    __flight_callback.lock().unwrap()();
                                                __flight_result
                                            };
                                            if ((x).is_none()) || ((y).is_none()) {
                                                return false;
                                            }
                                            let reflect = (last_kind == "Q") || (last_kind == "T");
                                            let cx = if reflect {
                                                ((2.0_f64 * current_x) - last_quad_control_x)
                                            } else {
                                                current_x
                                            };
                                            let cy = if reflect {
                                                ((2.0_f64 * current_y) - last_quad_control_y)
                                            } else {
                                                current_y
                                            };
                                            let ax = if relative {
                                                (current_x + *(x.as_ref().unwrap()))
                                            } else {
                                                *(x.as_ref().unwrap())
                                            };
                                            let ay = if relative {
                                                (current_y + *(y.as_ref().unwrap()))
                                            } else {
                                                *(y.as_ref().unwrap())
                                            };
                                            append_path_curve_to(path, cx, cy, ax, ay);
                                            last_quad_control_x = cx;
                                            last_quad_control_y = cy;
                                            current_x = ax;
                                            current_y = ay;
                                            last_kind = "T".to_owned();
                                        } else {
                                            if (upper == "A") {
                                                let rx = {
                                                    let __flight_callback = (read_number).clone();
                                                    let __flight_result =
                                                        __flight_callback.lock().unwrap()();
                                                    __flight_result
                                                };
                                                let ry = {
                                                    let __flight_callback = (read_number).clone();
                                                    let __flight_result =
                                                        __flight_callback.lock().unwrap()();
                                                    __flight_result
                                                };
                                                let rotation_degrees = {
                                                    let __flight_callback = (read_number).clone();
                                                    let __flight_result =
                                                        __flight_callback.lock().unwrap()();
                                                    __flight_result
                                                };
                                                let large_arc = {
                                                    let __flight_callback = (read_flag).clone();
                                                    let __flight_result =
                                                        __flight_callback.lock().unwrap()();
                                                    __flight_result
                                                };
                                                let sweep = {
                                                    let __flight_callback = (read_flag).clone();
                                                    let __flight_result =
                                                        __flight_callback.lock().unwrap()();
                                                    __flight_result
                                                };
                                                let x = {
                                                    let __flight_callback = (read_number).clone();
                                                    let __flight_result =
                                                        __flight_callback.lock().unwrap()();
                                                    __flight_result
                                                };
                                                let y = {
                                                    let __flight_callback = (read_number).clone();
                                                    let __flight_result =
                                                        __flight_callback.lock().unwrap()();
                                                    __flight_result
                                                };
                                                if (((((((rx).is_none()) || ((ry).is_none()))
                                                    || ((rotation_degrees).is_none()))
                                                    || ((large_arc).is_none()))
                                                    || ((sweep).is_none()))
                                                    || ((x).is_none()))
                                                    || ((y).is_none())
                                                {
                                                    return false;
                                                }
                                                let ax = if relative {
                                                    (current_x + *(x.as_ref().unwrap()))
                                                } else {
                                                    *(x.as_ref().unwrap())
                                                };
                                                let ay = if relative {
                                                    (current_y + *(y.as_ref().unwrap()))
                                                } else {
                                                    *(y.as_ref().unwrap())
                                                };
                                                append_path_arc_to(
                                                    path,
                                                    *(rx.as_ref().unwrap()),
                                                    *(ry.as_ref().unwrap()),
                                                    ((*(rotation_degrees.as_ref().unwrap())
                                                        * std::f64::consts::PI)
                                                        / 180.0_f64),
                                                    (*(large_arc.as_ref().unwrap()) == 1.0_f64),
                                                    (*(sweep.as_ref().unwrap()) == 1.0_f64),
                                                    ax,
                                                    ay,
                                                );
                                                current_x = ax;
                                                current_y = ay;
                                                last_kind = "A".to_owned();
                                            } else {
                                                return false;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            first = false;
            if ((active).clone() == "M") {
                active = "L".to_owned();
            } else {
                if ((active).clone() == "m") {
                    active = "l".to_owned();
                }
            }
        }
    }
    return true;
}

// Source: upstream/packages/path-formats/src/svgPathData.ts:283 (sha256:55264b4768f768b2ea3800cf3ca542d138e2f3b9614d0a30a9a7c965bbd9e588)
pub fn format_svg_path_data(path: &Path, options: Option<SharedStructuralRecord1>) -> String {
    let precision = options.as_ref().and_then(|value| value.precision);
    let parts: std::sync::Arc<std::sync::Mutex<Vec<String>>> =
        std::sync::Arc::new(std::sync::Mutex::new(vec![]));
    for_each_path_segment(path, &mut |segment: PathSegment| -> () {
        if matches!(&(segment), flighthq_types::PathSegment::A(_)) {
            (*parts.lock().unwrap()).push(format!(
                "M{} {}",
                format_svg_number(
                    (match (segment).clone() {
                        flighthq_types::PathSegment::A(value) => value,
                        flighthq_types::PathSegment::B(_) =>
                            panic!("TypeScript union narrowing failed"),
                    })
                    .x,
                    Some((precision).clone().unwrap())
                ),
                format_svg_number(
                    (match (segment).clone() {
                        flighthq_types::PathSegment::A(value) => value,
                        flighthq_types::PathSegment::B(_) =>
                            panic!("TypeScript union narrowing failed"),
                    })
                    .y,
                    Some((precision).clone().unwrap())
                )
            ));
        } else {
            if matches!(
                &(segment),
                crate::FlightUnion2::B(crate::FlightUnion2::A(_))
            ) {
                (*parts.lock().unwrap()).push(format!(
                    "L{} {}",
                    format_svg_number(
                        (match (segment).clone() {
                            flighthq_types::PathSegment::A(_) =>
                                panic!("TypeScript union narrowing failed"),
                            flighthq_types::PathSegment::B(value) => match value {
                                crate::FlightUnion2::A(value) => value,
                                crate::FlightUnion2::B(_) =>
                                    panic!("TypeScript union narrowing failed"),
                            },
                        })
                        .x,
                        Some((precision).clone().unwrap())
                    ),
                    format_svg_number(
                        (match (segment).clone() {
                            flighthq_types::PathSegment::A(_) =>
                                panic!("TypeScript union narrowing failed"),
                            flighthq_types::PathSegment::B(value) => match value {
                                crate::FlightUnion2::A(value) => value,
                                crate::FlightUnion2::B(_) =>
                                    panic!("TypeScript union narrowing failed"),
                            },
                        })
                        .y,
                        Some((precision).clone().unwrap())
                    )
                ));
            } else {
                if matches!(
                    &(segment),
                    crate::FlightUnion2::B(crate::FlightUnion2::B(crate::FlightUnion2::A(_)))
                ) {
                    (*parts.lock().unwrap()).push(format!(
                        "{}{}",
                        format!(
                            "Q{} {} ",
                            format_svg_number(
                                (match (segment).clone() {
                                    flighthq_types::PathSegment::A(_) =>
                                        panic!("TypeScript union narrowing failed"),
                                    flighthq_types::PathSegment::B(value) => match value {
                                        crate::FlightUnion2::A(_) =>
                                            panic!("TypeScript union narrowing failed"),
                                        crate::FlightUnion2::B(value) => match value {
                                            crate::FlightUnion2::A(value) => value,
                                            crate::FlightUnion2::B(_) =>
                                                panic!("TypeScript union narrowing failed"),
                                        },
                                    },
                                })
                                .control_x,
                                Some((precision).clone().unwrap())
                            ),
                            format_svg_number(
                                (match (segment).clone() {
                                    flighthq_types::PathSegment::A(_) =>
                                        panic!("TypeScript union narrowing failed"),
                                    flighthq_types::PathSegment::B(value) => match value {
                                        crate::FlightUnion2::A(_) =>
                                            panic!("TypeScript union narrowing failed"),
                                        crate::FlightUnion2::B(value) => match value {
                                            crate::FlightUnion2::A(value) => value,
                                            crate::FlightUnion2::B(_) =>
                                                panic!("TypeScript union narrowing failed"),
                                        },
                                    },
                                })
                                .control_y,
                                Some((precision).clone().unwrap())
                            )
                        ),
                        format!(
                            "{} {}",
                            format_svg_number(
                                (match (segment).clone() {
                                    flighthq_types::PathSegment::A(_) =>
                                        panic!("TypeScript union narrowing failed"),
                                    flighthq_types::PathSegment::B(value) => match value {
                                        crate::FlightUnion2::A(_) =>
                                            panic!("TypeScript union narrowing failed"),
                                        crate::FlightUnion2::B(value) => match value {
                                            crate::FlightUnion2::A(value) => value,
                                            crate::FlightUnion2::B(_) =>
                                                panic!("TypeScript union narrowing failed"),
                                        },
                                    },
                                })
                                .x,
                                Some((precision).clone().unwrap())
                            ),
                            format_svg_number(
                                (match (segment).clone() {
                                    flighthq_types::PathSegment::A(_) =>
                                        panic!("TypeScript union narrowing failed"),
                                    flighthq_types::PathSegment::B(value) => match value {
                                        crate::FlightUnion2::A(_) =>
                                            panic!("TypeScript union narrowing failed"),
                                        crate::FlightUnion2::B(value) => match value {
                                            crate::FlightUnion2::A(value) => value,
                                            crate::FlightUnion2::B(_) =>
                                                panic!("TypeScript union narrowing failed"),
                                        },
                                    },
                                })
                                .y,
                                Some((precision).clone().unwrap())
                            )
                        )
                    ));
                } else {
                    if matches!(
                        &(segment),
                        crate::FlightUnion2::B(crate::FlightUnion2::B(crate::FlightUnion2::B(
                            crate::FlightUnion2::A(_)
                        )))
                    ) {
                        (*parts.lock().unwrap()).push(format!(
                            "{}{}",
                            format!(
                                "{}{}",
                                format!(
                                    "C{} {} ",
                                    format_svg_number(
                                        (match (segment).clone() {
                                            flighthq_types::PathSegment::A(_) =>
                                                panic!("TypeScript union narrowing failed"),
                                            flighthq_types::PathSegment::B(value) => match value {
                                                crate::FlightUnion2::A(_) =>
                                                    panic!("TypeScript union narrowing failed"),
                                                crate::FlightUnion2::B(value) => match value {
                                                    crate::FlightUnion2::A(_) =>
                                                        panic!("TypeScript union narrowing failed"),
                                                    crate::FlightUnion2::B(value) => match value {
                                                        crate::FlightUnion2::A(value) => value,
                                                        crate::FlightUnion2::B(_) => panic!(
                                                            "TypeScript union narrowing failed"
                                                        ),
                                                    },
                                                },
                                            },
                                        })
                                        .control1_x,
                                        Some((precision).clone().unwrap())
                                    ),
                                    format_svg_number(
                                        (match (segment).clone() {
                                            flighthq_types::PathSegment::A(_) =>
                                                panic!("TypeScript union narrowing failed"),
                                            flighthq_types::PathSegment::B(value) => match value {
                                                crate::FlightUnion2::A(_) =>
                                                    panic!("TypeScript union narrowing failed"),
                                                crate::FlightUnion2::B(value) => match value {
                                                    crate::FlightUnion2::A(_) =>
                                                        panic!("TypeScript union narrowing failed"),
                                                    crate::FlightUnion2::B(value) => match value {
                                                        crate::FlightUnion2::A(value) => value,
                                                        crate::FlightUnion2::B(_) => panic!(
                                                            "TypeScript union narrowing failed"
                                                        ),
                                                    },
                                                },
                                            },
                                        })
                                        .control1_y,
                                        Some((precision).clone().unwrap())
                                    )
                                ),
                                format!(
                                    "{} {} ",
                                    format_svg_number(
                                        (match (segment).clone() {
                                            flighthq_types::PathSegment::A(_) =>
                                                panic!("TypeScript union narrowing failed"),
                                            flighthq_types::PathSegment::B(value) => match value {
                                                crate::FlightUnion2::A(_) =>
                                                    panic!("TypeScript union narrowing failed"),
                                                crate::FlightUnion2::B(value) => match value {
                                                    crate::FlightUnion2::A(_) =>
                                                        panic!("TypeScript union narrowing failed"),
                                                    crate::FlightUnion2::B(value) => match value {
                                                        crate::FlightUnion2::A(value) => value,
                                                        crate::FlightUnion2::B(_) => panic!(
                                                            "TypeScript union narrowing failed"
                                                        ),
                                                    },
                                                },
                                            },
                                        })
                                        .control2_x,
                                        Some((precision).clone().unwrap())
                                    ),
                                    format_svg_number(
                                        (match (segment).clone() {
                                            flighthq_types::PathSegment::A(_) =>
                                                panic!("TypeScript union narrowing failed"),
                                            flighthq_types::PathSegment::B(value) => match value {
                                                crate::FlightUnion2::A(_) =>
                                                    panic!("TypeScript union narrowing failed"),
                                                crate::FlightUnion2::B(value) => match value {
                                                    crate::FlightUnion2::A(_) =>
                                                        panic!("TypeScript union narrowing failed"),
                                                    crate::FlightUnion2::B(value) => match value {
                                                        crate::FlightUnion2::A(value) => value,
                                                        crate::FlightUnion2::B(_) => panic!(
                                                            "TypeScript union narrowing failed"
                                                        ),
                                                    },
                                                },
                                            },
                                        })
                                        .control2_y,
                                        Some((precision).clone().unwrap())
                                    )
                                )
                            ),
                            format!(
                                "{} {}",
                                format_svg_number(
                                    (match (segment).clone() {
                                        flighthq_types::PathSegment::A(_) =>
                                            panic!("TypeScript union narrowing failed"),
                                        flighthq_types::PathSegment::B(value) => match value {
                                            crate::FlightUnion2::A(_) =>
                                                panic!("TypeScript union narrowing failed"),
                                            crate::FlightUnion2::B(value) => match value {
                                                crate::FlightUnion2::A(_) =>
                                                    panic!("TypeScript union narrowing failed"),
                                                crate::FlightUnion2::B(value) => match value {
                                                    crate::FlightUnion2::A(value) => value,
                                                    crate::FlightUnion2::B(_) =>
                                                        panic!("TypeScript union narrowing failed"),
                                                },
                                            },
                                        },
                                    })
                                    .x,
                                    Some((precision).clone().unwrap())
                                ),
                                format_svg_number(
                                    (match (segment).clone() {
                                        flighthq_types::PathSegment::A(_) =>
                                            panic!("TypeScript union narrowing failed"),
                                        flighthq_types::PathSegment::B(value) => match value {
                                            crate::FlightUnion2::A(_) =>
                                                panic!("TypeScript union narrowing failed"),
                                            crate::FlightUnion2::B(value) => match value {
                                                crate::FlightUnion2::A(_) =>
                                                    panic!("TypeScript union narrowing failed"),
                                                crate::FlightUnion2::B(value) => match value {
                                                    crate::FlightUnion2::A(value) => value,
                                                    crate::FlightUnion2::B(_) =>
                                                        panic!("TypeScript union narrowing failed"),
                                                },
                                            },
                                        },
                                    })
                                    .y,
                                    Some((precision).clone().unwrap())
                                )
                            )
                        ));
                    } else {
                        if (((match (segment).clone() {
                            flighthq_types::PathSegment::A(_) => {
                                panic!("TypeScript union narrowing failed")
                            }
                            flighthq_types::PathSegment::B(value) => match value {
                                crate::FlightUnion2::A(_) => {
                                    panic!("TypeScript union narrowing failed")
                                }
                                crate::FlightUnion2::B(value) => match value {
                                    crate::FlightUnion2::A(_) => {
                                        panic!("TypeScript union narrowing failed")
                                    }
                                    crate::FlightUnion2::B(value) => match value {
                                        crate::FlightUnion2::A(_) => {
                                            panic!("TypeScript union narrowing failed")
                                        }
                                        crate::FlightUnion2::B(value) => value,
                                    },
                                },
                            },
                        })
                        .kind)
                            .clone()
                            == "close")
                        {
                            (*parts.lock().unwrap()).push("Z".to_owned());
                        }
                    }
                }
            }
        }
    });
    return ((*parts.lock().unwrap()).clone())
        .iter()
        .map(|value| value.to_string())
        .collect::<Vec<_>>()
        .join(("".to_owned()).as_str());
}

// Source: upstream/packages/path-formats/src/svgPathData.ts:314 (sha256:53f9355169e9a9fa6c28fe78487246e5ed91738942fde2c3834d0f527bcad355)
pub fn parse_svg_path_data(d: String) -> Option<Path> {
    let mut path = create_path(None);
    if (!append_svg_path_data(&mut path, (d).clone())) {
        return None;
    }
    return Some((path).clone());
}

// Source: upstream/packages/path-formats/src/svgPathData.ts:320 (sha256:3d55b5b9648b702cfeeefab18e92d5fe18956a550ceca1e8cd07b8810e636fef)
fn format_svg_number(value: f64, precision: Option<f64>) -> String {
    if (precision).is_none() {
        return {
            let __flight_value = {
                let __flight_portable_source = value;
                crate::FlightValue::Number(*(&__flight_portable_source) as f64)
            };
            crate::flight_value_to_string(&__flight_value)
        };
    }
    let factor = (10.0_f64).powf(*(precision.as_ref().unwrap()));
    return {
        let __flight_value = {
            let __flight_portable_source = ((value * factor).round() / factor);
            crate::FlightValue::Number(*(&__flight_portable_source) as f64)
        };
        crate::flight_value_to_string(&__flight_value)
    };
}

// Source: upstream/packages/path-formats/src/svgPathData.ts:327 (sha256:b96909a1054a73f2443e51c64f02be6673290494c302b7d1aa6b231cb00d229a)
fn is_svg_command_letter(c: String) -> bool {
    return (__flight_string_index_of(&("MmLlHhVvCcSsQqTtAaZz"), &((c).clone()), 0.0_f64)
        != (-1.0_f64));
}
