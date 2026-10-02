// @generated from upstream/packages/adjustments/src/colorGradeAdjustment.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::initialize_color_lut_adjustment;
use flighthq_entity::{allocate_entity, finish_entity};
use flighthq_types::{ColorGradeAdjustment, ColorTransformFunction, EntityConstruction};

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

#[derive(Clone, Default)]
pub struct FlightOmitRecord1550996253 {
    pub __flight_identity: std::sync::Arc<()>,
    pub exposure: Option<f64>,
    pub brightness: Option<f64>,
    pub contrast: Option<f64>,
    pub saturation: Option<f64>,
    pub temperature: Option<f64>,
    pub tint: Option<f64>,
    pub lift: Option<f64>,
    pub gamma: Option<f64>,
    pub gain: Option<f64>,
}
impl PartialEq for FlightOmitRecord1550996253 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/adjustments/src/colorGradeAdjustment.ts:11 (sha256:199595052d1e1c36dccf3a3a05f40d348db9803b4b637f99b656e546a28a8cd2)
#[derive(Clone, Default)]
struct CreateColorGradeAdjustmentRecord2 {
    __flight_identity: std::sync::Arc<()>,
}
impl PartialEq for CreateColorGradeAdjustmentRecord2 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn create_color_grade_adjustment(
    options: Option<FlightOmitRecord1550996253>,
) -> ColorGradeAdjustment {
    let options = options.unwrap_or(FlightOmitRecord1550996253 {
        __flight_identity: std::sync::Arc::new(()),
        exposure: None,
        brightness: None,
        contrast: None,
        saturation: None,
        temperature: None,
        tint: None,
        lift: None,
        gamma: None,
        gain: None,
    });
    let mut out = allocate_entity();
    initialize_color_grade_adjustment(
        (out).clone(),
        Some({
            let __flight_source = &((options).clone());
            FlightOmitRecord1550996253 {
                __flight_identity: std::sync::Arc::clone(&__flight_source.__flight_identity),
                exposure: __flight_source.exposure,
                brightness: __flight_source.brightness,
                contrast: __flight_source.contrast,
                saturation: __flight_source.saturation,
                temperature: __flight_source.temperature,
                tint: __flight_source.tint,
                lift: __flight_source.lift,
                gamma: __flight_source.gamma,
                gain: __flight_source.gain,
            }
        }),
    );
    return finish_entity((out).clone());
}

// Source: upstream/packages/adjustments/src/colorGradeAdjustment.ts:25 (sha256:f929ecb579301c9632087542d4be92c8a71bd194898fafccdbf796c32a49ff5a)
#[derive(Clone, Default)]
struct InitializeColorGradeAdjustmentRecord2 {
    __flight_identity: std::sync::Arc<()>,
}
impl PartialEq for InitializeColorGradeAdjustmentRecord2 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn initialize_color_grade_adjustment(
    out: EntityConstruction<ColorGradeAdjustment>,
    options: Option<FlightOmitRecord1550996253>,
) -> () {
    let options = options.unwrap_or(FlightOmitRecord1550996253 {
        __flight_identity: std::sync::Arc::new(()),
        exposure: None,
        brightness: None,
        contrast: None,
        saturation: None,
        temperature: None,
        tint: None,
        lift: None,
        gamma: None,
        gain: None,
    });
    let exposure = (2.0_f64).powf((options.exposure).unwrap_or(0.0_f64));
    let brightness = (options.brightness).unwrap_or(0.0_f64);
    let contrast = (options.contrast).unwrap_or(1.0_f64);
    let saturation = (options.saturation).unwrap_or(1.0_f64);
    let temperature = (options.temperature).unwrap_or(0.0_f64);
    let tint = (options.tint).unwrap_or(0.0_f64);
    let lift = unpack_rgb((options.lift).unwrap_or(255.0_f64));
    let gamma_raw = unpack_rgb((options.gamma).unwrap_or(2155905279.0_f64));
    let gain = unpack_rgb((options.gain).unwrap_or(4294967295.0_f64));
    let gamma_exp: Vec<f64> = vec![
        (1.0_f64 / (gamma_raw[0.0_f64 as usize].clone() * 2.0_f64).max(0.001_f64)),
        (1.0_f64 / (gamma_raw[1.0_f64 as usize].clone() * 2.0_f64).max(0.001_f64)),
        (1.0_f64 / (gamma_raw[2.0_f64 as usize].clone() * 2.0_f64).max(0.001_f64)),
    ];
    let mut transform: ColorTransformFunction =
        std::sync::Arc::new(std::sync::Mutex::new(Box::new({
            let gain = gain.clone();
            let gamma_exp = gamma_exp.clone();
            let lift = lift.clone();
            move |mut out: Vec<f64>, r: f64, g: f64, b: f64| -> () {
                let mut cr = (((r * exposure) + brightness) + (temperature * 0.5_f64));
                let mut cg = (((g * exposure) + brightness) + (tint * 0.5_f64));
                let mut cb = (((b * exposure) + brightness) - (temperature * 0.5_f64));
                let luma = (((cr * 0.2126_f64) + (cg * 0.7152_f64)) + (cb * 0.0722_f64));
                cr = (luma + ((cr - luma) * saturation));
                cg = (luma + ((cg - luma) * saturation));
                cb = (luma + ((cb - luma) * saturation));
                cr = (((cr - 0.5_f64) * contrast) + 0.5_f64);
                cg = (((cg - 0.5_f64) * contrast) + 0.5_f64);
                cb = (((cb - 0.5_f64) * contrast) + 0.5_f64);
                cr = (((cr * gain[0.0_f64 as usize].clone())
                    + (lift[0.0_f64 as usize].clone() * (1.0_f64 - cr)))
                    .max(0.0_f64))
                .powf(gamma_exp[0.0_f64 as usize].clone());
                cg = (((cg * gain[1.0_f64 as usize].clone())
                    + (lift[1.0_f64 as usize].clone() * (1.0_f64 - cg)))
                    .max(0.0_f64))
                .powf(gamma_exp[1.0_f64 as usize].clone());
                cb = (((cb * gain[2.0_f64 as usize].clone())
                    + (lift[2.0_f64 as usize].clone() * (1.0_f64 - cb)))
                    .max(0.0_f64))
                .powf(gamma_exp[2.0_f64 as usize].clone());
                {
                    let __flight_index = (0.0_f64) as usize;
                    let __flight_value = clamp01(cr);
                    if __flight_index == out.len() {
                        out.push(__flight_value);
                    } else {
                        out[__flight_index] = __flight_value;
                    }
                };
                {
                    let __flight_index = (1.0_f64) as usize;
                    let __flight_value = clamp01(cg);
                    if __flight_index == out.len() {
                        out.push(__flight_value);
                    } else {
                        out[__flight_index] = __flight_value;
                    }
                };
                {
                    let __flight_index = (2.0_f64) as usize;
                    let __flight_value = clamp01(cb);
                    if __flight_index == out.len() {
                        out.push(__flight_value);
                    } else {
                        out[__flight_index] = __flight_value;
                    }
                };
            }
        })
            as Box<dyn FnMut(Vec<f64>, f64, f64, f64) -> () + Send + 'static>));
    initialize_color_lut_adjustment(
        (out).clone(),
        "ColorGradeAdjustment".to_owned(),
        (transform).clone(),
    );
    crate::host_set("host.exposure", (options.exposure).unwrap_or(0.0_f64));
    crate::host_set("host.brightness", (options.brightness).unwrap_or(0.0_f64));
    crate::host_set("host.contrast", (options.contrast).unwrap_or(1.0_f64));
    crate::host_set("host.saturation", (options.saturation).unwrap_or(1.0_f64));
    crate::host_set("host.temperature", (options.temperature).unwrap_or(0.0_f64));
    crate::host_set("host.tint", (options.tint).unwrap_or(0.0_f64));
    crate::host_set("host.lift", (options.lift).unwrap_or(255.0_f64));
    crate::host_set("host.gamma", (options.gamma).unwrap_or(2155905279.0_f64));
    crate::host_set("host.gain", (options.gain).unwrap_or(4294967295.0_f64));
}

// Source: upstream/packages/adjustments/src/colorGradeAdjustment.ts:74 (sha256:92c4452839ded0362c28adef5c15154deeaad9b404aff5129f0596af7fea21ad)
fn clamp01(v: f64) -> f64 {
    return if (v < 0.0_f64) {
        0.0_f64
    } else {
        if (v > 1.0_f64) { 1.0_f64 } else { v }
    };
}

// Source: upstream/packages/adjustments/src/colorGradeAdjustment.ts:78 (sha256:28931813b5294ff30eb0603843143223641a413123fb98ae5206353a2ee44bb2)
fn unpack_rgb(c: f64) -> Vec<f64> {
    return vec![
        ((__flight_js_to_i32((__flight_js_to_u32(c) >> (__flight_js_to_u32(24.0_f64) & 31)) as f64)
            & __flight_js_to_i32(255.0_f64)) as f64
            / 255.0_f64),
        ((__flight_js_to_i32((__flight_js_to_u32(c) >> (__flight_js_to_u32(16.0_f64) & 31)) as f64)
            & __flight_js_to_i32(255.0_f64)) as f64
            / 255.0_f64),
        ((__flight_js_to_i32((__flight_js_to_u32(c) >> (__flight_js_to_u32(8.0_f64) & 31)) as f64)
            & __flight_js_to_i32(255.0_f64)) as f64
            / 255.0_f64),
    ];
}
