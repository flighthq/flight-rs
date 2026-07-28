// @generated from upstream/packages/adjustments/src/lookupTableGradeAdjustment.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::sample_color_lut;
use flighthq_types::{ColorLut, ColorTransformFunction, LookupTableGradeAdjustment};

#[derive(Clone)]
pub struct FlightOmitRecord1 {
    pub __flight_identity: std::sync::Arc<()>,
    pub lut: Option<ColorLut>,
    pub strength: Option<f64>,
}
impl PartialEq for FlightOmitRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/adjustments/src/lookupTableGradeAdjustment.ts:10 (sha256:fecafa0e20c0bdf19d25cb014e6a956a309fcfe291f5ef451bf28f11cc1710a8)
#[derive(Clone)]
struct CreateLookupTableGradeAdjustmentRecord2 {
    __flight_identity: std::sync::Arc<()>,
}
impl PartialEq for CreateLookupTableGradeAdjustmentRecord2 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn create_lookup_table_grade_adjustment(
    options: Option<FlightOmitRecord1>,
) -> LookupTableGradeAdjustment {
    let options = options.unwrap_or(FlightOmitRecord1 {
        __flight_identity: std::sync::Arc::new(()),
        lut: None,
        strength: None,
    });
    let lut = (options.lut).clone();
    let strength = (options.strength).unwrap_or(1.0_f64);
    let mut transform: ColorTransformFunction = std::sync::Arc::new(std::sync::Mutex::new(
        Box::new(move |mut out: Vec<f64>, r: f64, g: f64, b: f64| -> () {
            if ((lut).is_none()) || (strength <= 0.0_f64) {
                {
                    let __flight_index = (0.0_f64) as usize;
                    let __flight_value = r;
                    if __flight_index == out.len() {
                        out.push(__flight_value);
                    } else {
                        out[__flight_index] = __flight_value;
                    }
                };
                {
                    let __flight_index = (1.0_f64) as usize;
                    let __flight_value = g;
                    if __flight_index == out.len() {
                        out.push(__flight_value);
                    } else {
                        out[__flight_index] = __flight_value;
                    }
                };
                {
                    let __flight_index = (2.0_f64) as usize;
                    let __flight_value = b;
                    if __flight_index == out.len() {
                        out.push(__flight_value);
                    } else {
                        out[__flight_index] = __flight_value;
                    }
                };
                return;
            }
            sample_color_lut(lut.as_ref().unwrap(), &mut out, r, g, b);
            {
                let __flight_index = (0.0_f64) as usize;
                let __flight_value = (r + ((out[0.0_f64 as usize].clone() - r) * strength));
                if __flight_index == out.len() {
                    out.push(__flight_value);
                } else {
                    out[__flight_index] = __flight_value;
                }
            };
            {
                let __flight_index = (1.0_f64) as usize;
                let __flight_value = (g + ((out[1.0_f64 as usize].clone() - g) * strength));
                if __flight_index == out.len() {
                    out.push(__flight_value);
                } else {
                    out[__flight_index] = __flight_value;
                }
            };
            {
                let __flight_index = (2.0_f64) as usize;
                let __flight_value = (b + ((out[2.0_f64 as usize].clone() - b) * strength));
                if __flight_index == out.len() {
                    out.push(__flight_value);
                } else {
                    out[__flight_index] = __flight_value;
                }
            };
        }) as Box<dyn FnMut(Vec<f64>, f64, f64, f64) -> () + Send + 'static>,
    ));
    return {
        let __flight_spread_1 = options;
        LookupTableGradeAdjustment {
            __flight_identity: std::sync::Arc::new(()),
            kind: "LookupTableGradeAdjustment".to_owned(),
            transform: (transform).clone(),
            lut: (__flight_spread_1.lut).clone(),
            strength: __flight_spread_1.strength,
        }
    };
}
