// @generated from upstream/packages/adjustments/src/liftGammaGainAdjustment.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::{ColorTransformFunction, LiftGammaGainAdjustment};

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
    pub lift: Option<f64>,
    pub gamma: Option<f64>,
    pub gain: Option<f64>,
}
impl PartialEq for FlightOmitRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/adjustments/src/liftGammaGainAdjustment.ts:6 (sha256:e86a15b30666226017b1cb779869b1d6dd87c356e86675c76432e2d7dda3271d)
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
    options: Option<FlightOmitRecord1>,
) -> LiftGammaGainAdjustment {
    let options = options.unwrap_or(FlightOmitRecord1 {
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
    return {
        let __flight_spread_1 = options;
        LiftGammaGainAdjustment {
            __flight_identity: std::sync::Arc::new(()),
            kind: "LiftGammaGainAdjustment".to_owned(),
            transform: (transform).clone(),
            lift: __flight_spread_1.lift,
            gamma: __flight_spread_1.gamma,
            gain: __flight_spread_1.gain,
        }
    };
}

// Source: upstream/packages/adjustments/src/liftGammaGainAdjustment.ts:26 (sha256:92c4452839ded0362c28adef5c15154deeaad9b404aff5129f0596af7fea21ad)
fn clamp01(v: f64) -> f64 {
    return if (v < 0.0_f64) {
        0.0_f64
    } else {
        if (v > 1.0_f64) { 1.0_f64 } else { v }
    };
}

// Source: upstream/packages/adjustments/src/liftGammaGainAdjustment.ts:32 (sha256:28931813b5294ff30eb0603843143223641a413123fb98ae5206353a2ee44bb2)
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
