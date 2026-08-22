// @generated from upstream/packages/adjustments/src/colorLutCache.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{COLOR_LUT_DEFAULT_SIZE as color_lut_default_size_constant, bake_color_lut};
use flighthq_types::{ColorLut, ColorLutCache, ColorTransformFunction};

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

// Source: upstream/packages/adjustments/src/colorLutCache.ts:20 (sha256:43b7051bda2b12fe9ccb47c39eaebd4ec760dff96ad22db73055c349ecefe0c7)
pub fn bake_color_lut_for_run(
    cache: &mut ColorLutCache,
    run: &Vec<SharedStructuralRecord1>,
    size: Option<f64>,
) -> ColorLut {
    let size = size.unwrap_or(color_lut_default_size_constant);
    let signature = color_lut_run_signature(run, size);
    if (((cache.signature).clone()) == Some((signature).clone()))
        && (((cache.lut).clone()).is_some())
    {
        return ((cache.lut).clone()).unwrap();
    }
    let mut transforms: Vec<ColorTransformFunction> = vec![];
    for operation in (run).iter().cloned() {
        let transform = (|| -> Option<ColorTransformFunction> {
            let transform = None::<ColorTransformFunction>;
            if (((transform).as_ref().map_or("undefined", |_| "function")).to_owned() == "function")
            {
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
                Some((|| -> ColorTransformFunction {
                    return std::sync::Arc::new(std::sync::Mutex::new(Box::new(
                        move |mut out: Vec<f64>, r: f64, g: f64, b: f64| -> () {
                            {
                                let __flight_index = (0.0_f64) as usize;
                                let __flight_value = (((((matrix.as_ref().unwrap()
                                    [0.0_f64 as usize]
                                    .clone()
                                    * r)
                                    + (matrix.as_ref().unwrap()[1.0_f64 as usize].clone() * g))
                                    + (matrix.as_ref().unwrap()[2.0_f64 as usize].clone() * b))
                                    + matrix.as_ref().unwrap()[3.0_f64 as usize].clone())
                                    + matrix.as_ref().unwrap()[4.0_f64 as usize].clone());
                                if __flight_index == out.len() {
                                    out.push(__flight_value);
                                } else {
                                    out[__flight_index] = __flight_value;
                                }
                            };
                            {
                                let __flight_index = (1.0_f64) as usize;
                                let __flight_value = (((((matrix.as_ref().unwrap()
                                    [5.0_f64 as usize]
                                    .clone()
                                    * r)
                                    + (matrix.as_ref().unwrap()[6.0_f64 as usize].clone() * g))
                                    + (matrix.as_ref().unwrap()[7.0_f64 as usize].clone() * b))
                                    + matrix.as_ref().unwrap()[8.0_f64 as usize].clone())
                                    + matrix.as_ref().unwrap()[9.0_f64 as usize].clone());
                                if __flight_index == out.len() {
                                    out.push(__flight_value);
                                } else {
                                    out[__flight_index] = __flight_value;
                                }
                            };
                            {
                                let __flight_index = (2.0_f64) as usize;
                                let __flight_value = (((((matrix.as_ref().unwrap()
                                    [10.0_f64 as usize]
                                    .clone()
                                    * r)
                                    + (matrix.as_ref().unwrap()[11.0_f64 as usize].clone() * g))
                                    + (matrix.as_ref().unwrap()[12.0_f64 as usize].clone() * b))
                                    + matrix.as_ref().unwrap()[13.0_f64 as usize].clone())
                                    + matrix.as_ref().unwrap()[14.0_f64 as usize].clone());
                                if __flight_index == out.len() {
                                    out.push(__flight_value);
                                } else {
                                    out[__flight_index] = __flight_value;
                                }
                            };
                        },
                    )
                        as Box<dyn FnMut(Vec<f64>, f64, f64, f64) -> () + Send + 'static>));
                })())
            };
        })();
        if ((transform).clone()).is_some() {
            transforms.push(((transform.as_ref().unwrap()).clone()).clone());
        }
    }
    let lut = bake_color_lut(&transforms, Some(size));
    cache.signature = Some((signature).clone());
    cache.lut = Some((lut).clone());
    return lut;
}

// Source: upstream/packages/adjustments/src/colorLutCache.ts:40 (sha256:57f403fdf8d5bd7afff86bdc44e1c23863c0e2d90257ec4d19180469fd165e93)
pub fn create_color_lut_cache() -> ColorLutCache {
    return ColorLutCache {
        __flight_identity: std::sync::Arc::new(()),
        signature: None,
        lut: None,
    };
}

// Source: upstream/packages/adjustments/src/colorLutCache.ts:48 (sha256:15cbc958713f0e9b3b31ba476d3c5cb73ef8c6673ab1b45d8837bed603377581)
fn color_lut_run_signature(run: &Vec<SharedStructuralRecord1>, size: f64) -> String {
    return format!(
        "{}\n{}",
        size,
        crate::flight_json_stringify(
            &({
                let __flight_portable_source = (*run).clone();
                crate::FlightValue::Array(
                    (&__flight_portable_source)
                        .iter()
                        .map(|value| {
                            crate::FlightValue::Record({
                                let mut __flight_record = Vec::new();
                                __flight_record.push((
                                    "kind".to_owned(),
                                    crate::FlightValue::String((&((value).kind)).clone()),
                                ));
                                __flight_record
                            })
                        })
                        .collect(),
                )
            })
        )
        .expect("JSON.stringify encountered an opaque host object")
        .expect("JSON.stringify returned undefined where Rust requires String")
    );
}
