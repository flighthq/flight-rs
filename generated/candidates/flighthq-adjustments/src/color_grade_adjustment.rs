// @generated from upstream/packages/adjustments/src/colorGradeAdjustment.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::{ColorGradeAdjustment, ColorTransformFunction};

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
pub struct FlightOmitRecord1 {
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
impl PartialEq for FlightOmitRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/adjustments/src/colorGradeAdjustment.ts:9 (sha256:f8e6162f7dd7cca9b62f96e74d3925cc4e1e81bb8e378418ba53b91482587f1b)
#[derive(Clone, Default)]
struct CreateColorGradeAdjustmentRecord2 {
    __flight_identity: std::sync::Arc<()>,
}
impl PartialEq for CreateColorGradeAdjustmentRecord2 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn create_color_grade_adjustment(options: Option<FlightOmitRecord1>) -> ColorGradeAdjustment {
    let options = options.unwrap_or(FlightOmitRecord1 {
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
    let exposure = (2.0_f64).powf((options.exposure).clone().unwrap_or(0.0_f64));
    let brightness = (options.brightness).clone().unwrap_or(0.0_f64);
    let contrast = (options.contrast).clone().unwrap_or(1.0_f64);
    let saturation = (options.saturation).clone().unwrap_or(1.0_f64);
    let temperature = (options.temperature).clone().unwrap_or(0.0_f64);
    let tint = (options.tint).clone().unwrap_or(0.0_f64);
    let lift = unpack_rgb((options.lift).clone().unwrap_or(255.0_f64));
    let gamma_raw = unpack_rgb((options.gamma).clone().unwrap_or(2155905279.0_f64));
    let gain = unpack_rgb((options.gain).clone().unwrap_or(4294967295.0_f64));
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
    return {
        let __flight_spread_1 = options;
        ColorGradeAdjustment {
            __flight_identity: std::sync::Arc::new(()),
            kind: "ColorGradeAdjustment".to_owned(),
            transform: (transform).clone(),
            exposure: __flight_spread_1.exposure,
            brightness: __flight_spread_1.brightness,
            contrast: __flight_spread_1.contrast,
            saturation: __flight_spread_1.saturation,
            temperature: __flight_spread_1.temperature,
            tint: __flight_spread_1.tint,
            lift: __flight_spread_1.lift,
            gamma: __flight_spread_1.gamma,
            gain: __flight_spread_1.gain,
        }
    };
}

// Source: upstream/packages/adjustments/src/colorGradeAdjustment.ts:48 (sha256:92c4452839ded0362c28adef5c15154deeaad9b404aff5129f0596af7fea21ad)
fn clamp01(v: f64) -> f64 {
    return if (v < 0.0_f64) {
        0.0_f64
    } else {
        if (v > 1.0_f64) { 1.0_f64 } else { v }
    };
}

// Source: upstream/packages/adjustments/src/colorGradeAdjustment.ts:52 (sha256:28931813b5294ff30eb0603843143223641a413123fb98ae5206353a2ee44bb2)
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
