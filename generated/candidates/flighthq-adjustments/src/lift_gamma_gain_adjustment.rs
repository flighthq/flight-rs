// @generated from upstream/packages/adjustments/src/liftGammaGainAdjustment.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::initialize_color_lut_adjustment;
use flighthq_entity::{allocate_entity, finish_entity};
use flighthq_types::{ColorTransformFunction, EntityConstruction, LiftGammaGainAdjustment};

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
pub struct FlightOmitRecord2651849460 {
    pub __flight_identity: std::sync::Arc<()>,
    pub lift: Option<f64>,
    pub gamma: Option<f64>,
    pub gain: Option<f64>,
}
impl PartialEq for FlightOmitRecord2651849460 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/adjustments/src/liftGammaGainAdjustment.ts:11 (sha256:301a676520b4dd4b4793e7ca6a91c31675cf90bb20e9da136d304c33b370c43c)
#[derive(Clone, Default)]
struct CreateLiftGammaGainAdjustmentRecord2 {
    __flight_identity: std::sync::Arc<()>,
}
impl PartialEq for CreateLiftGammaGainAdjustmentRecord2 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn create_lift_gamma_gain_adjustment(
    options: Option<FlightOmitRecord2651849460>,
) -> LiftGammaGainAdjustment {
    let options = options.unwrap_or(FlightOmitRecord2651849460 {
        __flight_identity: std::sync::Arc::new(()),
        lift: None,
        gamma: None,
        gain: None,
    });
    let mut out = allocate_entity();
    initialize_lift_gamma_gain_adjustment(
        (out).clone(),
        Some({
            let __flight_source = &((options).clone());
            FlightOmitRecord2651849460 {
                __flight_identity: std::sync::Arc::clone(&__flight_source.__flight_identity),
                lift: __flight_source.lift,
                gamma: __flight_source.gamma,
                gain: __flight_source.gain,
            }
        }),
    );
    return finish_entity((out).clone());
}

// Source: upstream/packages/adjustments/src/liftGammaGainAdjustment.ts:22 (sha256:cb59e456d29b93e4d9a898dc973e09206dbe6e4f3f0b55170e1bf4fbfd6383d0)
#[derive(Clone, Default)]
struct InitializeLiftGammaGainAdjustmentRecord2 {
    __flight_identity: std::sync::Arc<()>,
}
impl PartialEq for InitializeLiftGammaGainAdjustmentRecord2 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn initialize_lift_gamma_gain_adjustment(
    out: EntityConstruction<LiftGammaGainAdjustment>,
    options: Option<FlightOmitRecord2651849460>,
) -> () {
    let options = options.unwrap_or(FlightOmitRecord2651849460 {
        __flight_identity: std::sync::Arc::new(()),
        lift: None,
        gamma: None,
        gain: None,
    });
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
                {
                    let __flight_index = (0.0_f64) as usize;
                    let __flight_value = clamp01(
                        (((r * gain[0.0_f64 as usize].clone())
                            + (lift[0.0_f64 as usize].clone() * (1.0_f64 - r)))
                            .max(0.0_f64))
                        .powf(gamma_exp[0.0_f64 as usize].clone()),
                    );
                    if __flight_index == out.len() {
                        out.push(__flight_value);
                    } else {
                        out[__flight_index] = __flight_value;
                    }
                };
                {
                    let __flight_index = (1.0_f64) as usize;
                    let __flight_value = clamp01(
                        (((g * gain[1.0_f64 as usize].clone())
                            + (lift[1.0_f64 as usize].clone() * (1.0_f64 - g)))
                            .max(0.0_f64))
                        .powf(gamma_exp[1.0_f64 as usize].clone()),
                    );
                    if __flight_index == out.len() {
                        out.push(__flight_value);
                    } else {
                        out[__flight_index] = __flight_value;
                    }
                };
                {
                    let __flight_index = (2.0_f64) as usize;
                    let __flight_value = clamp01(
                        (((b * gain[2.0_f64 as usize].clone())
                            + (lift[2.0_f64 as usize].clone() * (1.0_f64 - b)))
                            .max(0.0_f64))
                        .powf(gamma_exp[2.0_f64 as usize].clone()),
                    );
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
        "LiftGammaGainAdjustment".to_owned(),
        (transform).clone(),
    );
    crate::host_set("host.lift", (options.lift).unwrap_or(255.0_f64));
    crate::host_set("host.gamma", (options.gamma).unwrap_or(2155905279.0_f64));
    crate::host_set("host.gain", (options.gain).unwrap_or(4294967295.0_f64));
}

// Source: upstream/packages/adjustments/src/liftGammaGainAdjustment.ts:46 (sha256:92c4452839ded0362c28adef5c15154deeaad9b404aff5129f0596af7fea21ad)
fn clamp01(v: f64) -> f64 {
    return if (v < 0.0_f64) {
        0.0_f64
    } else {
        if (v > 1.0_f64) { 1.0_f64 } else { v }
    };
}

// Source: upstream/packages/adjustments/src/liftGammaGainAdjustment.ts:52 (sha256:28931813b5294ff30eb0603843143223641a413123fb98ae5206353a2ee44bb2)
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
