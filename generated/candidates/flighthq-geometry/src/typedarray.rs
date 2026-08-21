// @generated from upstream/packages/geometry/src/typedarray.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/geometry/src/typedarray.ts:1 (sha256:3f8413b01aa226113a558cbf5651bd5b3e3300fbcbac63490f79dcc9d7058e0f)
pub fn reserve_float32_array(array: &Vec<f32>, capacity: f64) -> Vec<f32> {
    if ((array.len() as f64) >= capacity) {
        return array.clone();
    }
    let mut out: Vec<f32> = vec![0.0_f32; (capacity) as usize];
    {
        let __flight_offset = (0.0_f64) as usize;
        let __flight_values: Vec<f32> = (array).iter().map(|value| (*value) as f32).collect();
        out[__flight_offset..__flight_offset + __flight_values.len()]
            .copy_from_slice(&__flight_values);
    };
    return out;
}

// Source: upstream/packages/geometry/src/typedarray.ts:8 (sha256:897ca85420c7f5f4f8a9aee55e903dba692233443e09f316d6bc30106de4d435)
pub fn reserve_int16_array(array: &Vec<i16>, capacity: f64) -> Vec<i16> {
    if ((array.len() as f64) >= capacity) {
        return array.clone();
    }
    let mut out: Vec<i16> = vec![0_i16; (capacity) as usize];
    {
        let __flight_offset = (0.0_f64) as usize;
        let __flight_values: Vec<i16> = (array).iter().map(|value| (*value) as i16).collect();
        out[__flight_offset..__flight_offset + __flight_values.len()]
            .copy_from_slice(&__flight_values);
    };
    return out;
}

// Source: upstream/packages/geometry/src/typedarray.ts:15 (sha256:b60c4fa6c63fdcc27226fd3f70d14a4d96efc1614fc06ed411a6a6ba1281e23f)
pub fn reserve_uint16_array(array: &Vec<u16>, capacity: f64) -> Vec<u16> {
    if ((array.len() as f64) >= capacity) {
        return array.clone();
    }
    let mut out: Vec<u16> = vec![0_u16; (capacity) as usize];
    {
        let __flight_offset = (0.0_f64) as usize;
        let __flight_values: Vec<u16> = (array).iter().map(|value| (*value) as u16).collect();
        out[__flight_offset..__flight_offset + __flight_values.len()]
            .copy_from_slice(&__flight_values);
    };
    return out;
}
