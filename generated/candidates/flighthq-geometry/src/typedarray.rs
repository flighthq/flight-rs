// @generated from upstream/packages/geometry/src/typedarray.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/geometry/src/typedarray.ts:1 (sha256:efe1ee36b04015ef1a36ba08fd6d7a0b960cd7d2be239ed0599c58be3fcca97f)
pub fn reserve_float32_array(array: &Vec<f32>, capacity: f64) -> Vec<f32> {
    if ((array.len() as f64) >= capacity) {
        return array.clone();
    }
    let mut out: Vec<f32> = vec![0.0_f32; (capacity) as usize];
    if true {
        {
            let __flight_offset = (0.0_f64) as usize;
            let __flight_values: Vec<f32> = (array).iter().map(|value| (*value) as f32).collect();
            out[__flight_offset..__flight_offset + __flight_values.len()]
                .copy_from_slice(&__flight_values);
        };
    }
    return out;
}

// Source: upstream/packages/geometry/src/typedarray.ts:8 (sha256:e016d560876b7f7754504a4aa2b3b7597f82be40eaf9bee5e464a276d5ed8c3c)
pub fn reserve_int16_array(array: &Vec<i16>, capacity: f64) -> Vec<i16> {
    if ((array.len() as f64) >= capacity) {
        return array.clone();
    }
    let mut out: Vec<i16> = vec![0_i16; (capacity) as usize];
    if true {
        {
            let __flight_offset = (0.0_f64) as usize;
            let __flight_values: Vec<i16> = (array).iter().map(|value| (*value) as i16).collect();
            out[__flight_offset..__flight_offset + __flight_values.len()]
                .copy_from_slice(&__flight_values);
        };
    }
    return out;
}

// Source: upstream/packages/geometry/src/typedarray.ts:15 (sha256:10855d09ca0303ec17162812a90dc2b71b1c466153a1cadd76aef47ed82eb1ff)
pub fn reserve_uint16_array(array: &Vec<u16>, capacity: f64) -> Vec<u16> {
    if ((array.len() as f64) >= capacity) {
        return array.clone();
    }
    let mut out: Vec<u16> = vec![0_u16; (capacity) as usize];
    if true {
        {
            let __flight_offset = (0.0_f64) as usize;
            let __flight_values: Vec<u16> = (array).iter().map(|value| (*value) as u16).collect();
            out[__flight_offset..__flight_offset + __flight_values.len()]
                .copy_from_slice(&__flight_values);
        };
    }
    return out;
}
