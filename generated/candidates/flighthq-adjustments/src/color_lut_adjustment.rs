// @generated from upstream/packages/adjustments/src/colorLutAdjustment.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::{AdjustmentKind, ColorTransformFunction};

#[derive(Clone)]
pub struct FlightPartialRecord1 {
    pub __flight_identity: std::sync::Arc<()>,
    pub kind: Option<AdjustmentKind>,
    pub transform: Option<ColorTransformFunction>,
}
impl PartialEq for FlightPartialRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone)]
pub struct FlightPartialRecord2 {
    pub __flight_identity: std::sync::Arc<()>,
    pub kind: Option<AdjustmentKind>,
    pub color_matrix: Option<Vec<f64>>,
}
impl PartialEq for FlightPartialRecord2 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/adjustments/src/colorLutAdjustment.ts:10 (sha256:706d5be3108fd619cffdcd0cbd8ced4ac47c2630d4a9b78839c64f3db0f634f2)
#[derive(Clone)]
struct GetAdjustmentColorTransformRecord3 {
    __flight_identity: std::sync::Arc<()>,
    kind: String,
}
impl PartialEq for GetAdjustmentColorTransformRecord3 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn get_adjustment_color_transform(
    operation: &GetAdjustmentColorTransformRecord3,
) -> Option<ColorTransformFunction> {
    let transform = (operation.transform).clone();
    return (transform).clone();
}

// Source: upstream/packages/adjustments/src/colorLutAdjustment.ts:20 (sha256:a3555604ecbc722928c2d782499cbed67c36de37dd76a702f84c347c44554b3d)
#[derive(Clone)]
struct IsColorLutAdjustmentRecord3 {
    __flight_identity: std::sync::Arc<()>,
    kind: String,
}
impl PartialEq for IsColorLutAdjustmentRecord3 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn is_color_lut_adjustment(operation: &IsColorLutAdjustmentRecord3) -> bool {
    return ("function" == "function");
}

// Source: upstream/packages/adjustments/src/colorLutAdjustment.ts:27 (sha256:653f829c8320921dd0e21c19733fb780fd327720e40b8fa9e742a4e9e5105e95)
fn color_matrix_transform(m: Vec<f64>) -> ColorTransformFunction {
    return std::sync::Arc::new(std::sync::Mutex::new(Box::new({
        let m = m.clone();
        move |mut out: Vec<f64>, r: f64, g: f64, b: f64| -> () {
            {
                let __flight_index = (0.0_f64) as usize;
                let __flight_value = (((((m[0.0_f64 as usize].clone() * r)
                    + (m[1.0_f64 as usize].clone() * g))
                    + (m[2.0_f64 as usize].clone() * b))
                    + m[3.0_f64 as usize].clone())
                    + (m[4.0_f64 as usize].clone() / 255.0_f64));
                if __flight_index == out.len() {
                    out.push(__flight_value);
                } else {
                    out[__flight_index] = __flight_value;
                }
            };
            {
                let __flight_index = (1.0_f64) as usize;
                let __flight_value = (((((m[5.0_f64 as usize].clone() * r)
                    + (m[6.0_f64 as usize].clone() * g))
                    + (m[7.0_f64 as usize].clone() * b))
                    + m[8.0_f64 as usize].clone())
                    + (m[9.0_f64 as usize].clone() / 255.0_f64));
                if __flight_index == out.len() {
                    out.push(__flight_value);
                } else {
                    out[__flight_index] = __flight_value;
                }
            };
            {
                let __flight_index = (2.0_f64) as usize;
                let __flight_value = (((((m[10.0_f64 as usize].clone() * r)
                    + (m[11.0_f64 as usize].clone() * g))
                    + (m[12.0_f64 as usize].clone() * b))
                    + m[13.0_f64 as usize].clone())
                    + (m[14.0_f64 as usize].clone() / 255.0_f64));
                if __flight_index == out.len() {
                    out.push(__flight_value);
                } else {
                    out[__flight_index] = __flight_value;
                }
            };
        }
    })
        as Box<dyn FnMut(Vec<f64>, f64, f64, f64) -> () + Send + 'static>));
}
