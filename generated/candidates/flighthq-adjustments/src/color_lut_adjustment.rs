// @generated from upstream/packages/adjustments/src/colorLutAdjustment.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::{AdjustmentKind, ColorTransformFunction};

#[derive(Clone, Default)]
pub struct SharedStructuralRecord1 {
    pub __flight_identity: std::sync::Arc<()>,
    pub kind: String,
}
impl PartialEq for SharedStructuralRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct FlightPartialRecord2 {
    pub __flight_identity: std::sync::Arc<()>,
    pub kind: Option<AdjustmentKind>,
    pub transform: Option<ColorTransformFunction>,
}
impl PartialEq for FlightPartialRecord2 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct FlightPartialRecord3 {
    pub __flight_identity: std::sync::Arc<()>,
    pub kind: Option<AdjustmentKind>,
    pub color_matrix: Option<Vec<f64>>,
}
impl PartialEq for FlightPartialRecord3 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/adjustments/src/colorLutAdjustment.ts:10 (sha256:706d5be3108fd619cffdcd0cbd8ced4ac47c2630d4a9b78839c64f3db0f634f2)
pub fn get_adjustment_color_transform(
    operation: &SharedStructuralRecord1,
) -> Option<ColorTransformFunction> {
    let transform = None::<ColorTransformFunction>;
    if ((transform).as_ref().map_or("undefined", |_| "function") == "function") {
        return (transform).clone();
    }
    let matrix = (|| -> Option<Vec<f64>> {
        let matrix = None::<Vec<f64>>;
        return if ((matrix).is_some())
            && ((matrix.as_ref().unwrap().len() as f64) == crate::COLOR_MATRIX_LENGTH)
        {
            (matrix).clone()
        } else {
            None
        };
    })();
    return if (matrix).is_none() {
        None
    } else {
        Some(color_matrix_transform(((matrix).clone().unwrap()).clone()))
    };
}

// Source: upstream/packages/adjustments/src/colorLutAdjustment.ts:20 (sha256:a3555604ecbc722928c2d782499cbed67c36de37dd76a702f84c347c44554b3d)
pub fn is_color_lut_adjustment(operation: &SharedStructuralRecord1) -> bool {
    return ((None::<ColorTransformFunction>)
        .as_ref()
        .map_or("undefined", |_| "function")
        == "function");
}

// Source: upstream/packages/adjustments/src/colorLutAdjustment.ts:27 (sha256:4b8b6191dcc732ab6f8a638e02033ad876d3297f4590c82beb66539098f98427)
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
                    + m[4.0_f64 as usize].clone());
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
                    + m[9.0_f64 as usize].clone());
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
                    + m[14.0_f64 as usize].clone());
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
