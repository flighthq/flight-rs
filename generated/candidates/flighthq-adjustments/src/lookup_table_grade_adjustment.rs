// @generated from upstream/packages/adjustments/src/lookupTableGradeAdjustment.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{initialize_color_lut_adjustment, sample_color_lut};
use flighthq_entity::{allocate_entity, finish_entity};
use flighthq_types::{
    ColorLut, ColorTransformFunction, EntityConstruction, LookupTableGradeAdjustment,
};

#[derive(Clone, Default)]
pub struct FlightOmitRecord64912180 {
    pub __flight_identity: std::sync::Arc<()>,
    pub lut: Option<ColorLut>,
    pub strength: Option<f64>,
}
impl PartialEq for FlightOmitRecord64912180 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/adjustments/src/lookupTableGradeAdjustment.ts:12 (sha256:c2f5bbe2e78e8606cd4b3fd7dd4e2d8a4f981668dc68c72251851c68fec51e1a)
#[derive(Clone, Default)]
struct CreateLookupTableGradeAdjustmentRecord2 {
    __flight_identity: std::sync::Arc<()>,
}
impl PartialEq for CreateLookupTableGradeAdjustmentRecord2 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn create_lookup_table_grade_adjustment(
    options: Option<FlightOmitRecord64912180>,
) -> LookupTableGradeAdjustment {
    let options = options.unwrap_or(FlightOmitRecord64912180 {
        __flight_identity: std::sync::Arc::new(()),
        lut: None,
        strength: None,
    });
    let mut out = allocate_entity();
    initialize_lookup_table_grade_adjustment(
        (out).clone(),
        Some({
            let __flight_source = &((options).clone());
            FlightOmitRecord64912180 {
                __flight_identity: std::sync::Arc::clone(&__flight_source.__flight_identity),
                lut: (__flight_source.lut).clone(),
                strength: __flight_source.strength,
            }
        }),
    );
    return finish_entity((out).clone());
}

// Source: upstream/packages/adjustments/src/lookupTableGradeAdjustment.ts:25 (sha256:6ddd3158990b9caf0e2c466de9b7b4d9ded6ef2a4a72e0d889043915d61c1209)
#[derive(Clone, Default)]
struct InitializeLookupTableGradeAdjustmentRecord2 {
    __flight_identity: std::sync::Arc<()>,
}
impl PartialEq for InitializeLookupTableGradeAdjustmentRecord2 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn initialize_lookup_table_grade_adjustment(
    out: EntityConstruction<LookupTableGradeAdjustment>,
    options: Option<FlightOmitRecord64912180>,
) -> () {
    let options = options.unwrap_or(FlightOmitRecord64912180 {
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
            sample_color_lut(&lut.as_ref().unwrap(), &mut out, r, g, b);
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
    initialize_color_lut_adjustment(
        (out).clone(),
        "LookupTableGradeAdjustment".to_owned(),
        (transform).clone(),
    );
    crate::host_set("host.lut", (options.lut).clone());
    crate::host_set("host.strength", (options.strength).unwrap_or(1.0_f64));
}
