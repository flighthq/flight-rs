// @generated from upstream/packages/animation/src/animationBlend.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_entity::create_entity;
use flighthq_types::AnimationSampleAccumulator;

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

// Source: upstream/packages/animation/src/animationBlend.ts:7 (sha256:7deed8cfe7398bc422ee6f31dc94e59abf7b3cba53b2bd7372df8a000f827e2f)
pub fn accumulate_animation_sample(
    accumulator: &mut AnimationSampleAccumulator,
    sample: &Vec<f64>,
    weight: f64,
) -> () {
    if (!(weight > 0.0_f64)) {
        return;
    }
    let components = ((accumulator.components).min((accumulator.values.len() as f64)))
        .min((sample.len() as f64));
    let mut sign = 1.0_f64;
    if ((accumulator.quaternion) && (components >= 4.0_f64)) && (accumulator.weight > 0.0_f64) {
        let dot = (((((accumulator.values[0.0_f64 as usize] as f64)
            * sample[0.0_f64 as usize].clone())
            + ((accumulator.values[1.0_f64 as usize] as f64) * sample[1.0_f64 as usize].clone()))
            + ((accumulator.values[2.0_f64 as usize] as f64) * sample[2.0_f64 as usize].clone()))
            + ((accumulator.values[3.0_f64 as usize] as f64) * sample[3.0_f64 as usize].clone()));
        if (dot < 0.0_f64) {
            sign = (-1.0_f64);
        }
    }
    {
        let mut component = 0.0_f64;
        while (component < components) {
            accumulator.values[component as usize] +=
                ((sample[component as usize].clone() * weight) * sign) as f32;
            {
                component += 1.0;
                component
            };
        }
    }
    accumulator.weight += weight;
}

// Source: upstream/packages/animation/src/animationBlend.ts:30 (sha256:db33594c7717490951ec552d0a805af794c347bb0b1c31c86ec324dac26f2fa6)
pub fn add_animation_sample(
    out: &mut crate::FlightUnion2<Vec<f64>, Vec<f32>>,
    base: &Vec<f64>,
    delta: &Vec<f64>,
    weight: f64,
    quaternion: Option<bool>,
) -> () {
    let quaternion = quaternion.unwrap_or(false);
    if (((quaternion)
        && (match &*(out) {
            crate::FlightUnion2::A(values) => (values.len() as f64),
            crate::FlightUnion2::B(values) => (values.len() as f64),
        } >= 4.0_f64))
        && ((base.len() as f64) >= 4.0_f64))
        && ((delta.len() as f64) >= 4.0_f64)
    {
        write_weighted_quaternion(&mut (*_QUATERNION.lock().unwrap()), delta, weight);
        let ax = base[0.0_f64 as usize].clone();
        let ay = base[1.0_f64 as usize].clone();
        let az = base[2.0_f64 as usize].clone();
        let aw = base[3.0_f64 as usize].clone();
        let bx = ((*_QUATERNION.lock().unwrap())[0.0_f64 as usize] as f64);
        let by = ((*_QUATERNION.lock().unwrap())[1.0_f64 as usize] as f64);
        let bz = ((*_QUATERNION.lock().unwrap())[2.0_f64 as usize] as f64);
        let bw = ((*_QUATERNION.lock().unwrap())[3.0_f64 as usize] as f64);
        write_normalized_quaternion(
            out,
            ((((aw * bx) + (ax * bw)) + (ay * bz)) - (az * by)),
            ((((aw * by) - (ax * bz)) + (ay * bw)) + (az * bx)),
            ((((aw * bz) + (ax * by)) - (ay * bx)) + (az * bw)),
            ((((aw * bw) - (ax * bx)) - (ay * by)) - (az * bz)),
        );
        return;
    }
    let components = ((match &*(out) {
        crate::FlightUnion2::A(values) => (values.len() as f64),
        crate::FlightUnion2::B(values) => (values.len() as f64),
    })
    .min((base.len() as f64)))
    .min((delta.len() as f64));
    {
        let mut component = 0.0_f64;
        while (component < components) {
            {
                let __flight_index = (component) as usize;
                let __flight_value = (base[component as usize].clone()
                    + (delta[component as usize].clone() * weight));
                match out {
                    crate::FlightUnion2::A(values) => {
                        values[__flight_index] = __flight_value;
                    }
                    crate::FlightUnion2::B(values) => {
                        values[__flight_index] = (__flight_value) as f32;
                    }
                };
            };
            {
                component += 1.0;
                component
            };
        }
    }
}

// Source: upstream/packages/animation/src/animationBlend.ts:65 (sha256:3d40d63846c46db688ffc95bc35599137a3a88cf871d36813a655ff98a0e35eb)
pub fn blend_animation_samples(
    out: &mut crate::FlightUnion2<Vec<f64>, Vec<f32>>,
    a: &Vec<f64>,
    b: &Vec<f64>,
    alpha: f64,
    quaternion: Option<bool>,
) -> () {
    let quaternion = quaternion.unwrap_or(false);
    let t = if (alpha < 0.0_f64) {
        0.0_f64
    } else {
        if (alpha > 1.0_f64) { 1.0_f64 } else { alpha }
    };
    if (((quaternion)
        && (match &*(out) {
            crate::FlightUnion2::A(values) => (values.len() as f64),
            crate::FlightUnion2::B(values) => (values.len() as f64),
        } >= 4.0_f64))
        && ((a.len() as f64) >= 4.0_f64))
        && ((b.len() as f64) >= 4.0_f64)
    {
        slerp_quaternion(out, a, b, t);
        return;
    }
    let components = ((match &*(out) {
        crate::FlightUnion2::A(values) => (values.len() as f64),
        crate::FlightUnion2::B(values) => (values.len() as f64),
    })
    .min((a.len() as f64)))
    .min((b.len() as f64));
    {
        let mut component = 0.0_f64;
        while (component < components) {
            {
                let __flight_index = (component) as usize;
                let __flight_value = (a[component as usize].clone()
                    + ((b[component as usize].clone() - a[component as usize].clone()) * t));
                match out {
                    crate::FlightUnion2::A(values) => {
                        values[__flight_index] = __flight_value;
                    }
                    crate::FlightUnion2::B(values) => {
                        values[__flight_index] = (__flight_value) as f32;
                    }
                };
            };
            {
                component += 1.0;
                component
            };
        }
    }
}

// Source: upstream/packages/animation/src/animationBlend.ts:85 (sha256:54117e1c545acf0251ec3a10ac65eafc0266bfacba88144ea12b21efb20f9ee0)
pub fn create_animation_sample_accumulator(
    components: f64,
    quaternion: Option<bool>,
) -> AnimationSampleAccumulator {
    let quaternion = quaternion.unwrap_or(false);
    let width =
        (0.0_f64).max((__flight_js_to_i32(components) | __flight_js_to_i32(0.0_f64)) as f64);
    return create_entity(Some(AnimationSampleAccumulator {
        __flight_identity: std::sync::Arc::new(()),
        __flight_entity_snapshot: Default::default(),
        __flight_entity_runtime: Default::default(),
        components: width,
        quaternion: quaternion,
        values: vec![0.0_f32; (width) as usize],
        weight: 0.0_f64,
    }));
}

// Source: upstream/packages/animation/src/animationBlend.ts:92 (sha256:9d6e25355bfc2c24394f80ec8d9cadb1e947a7d452319454ade7857d1120b6ed)
pub fn finish_animation_sample(
    out: &mut crate::FlightUnion2<Vec<f64>, Vec<f32>>,
    accumulator: &AnimationSampleAccumulator,
) -> bool {
    if (!(accumulator.weight > 0.0_f64)) {
        return false;
    }
    let components = ((match &*(out) {
        crate::FlightUnion2::A(values) => (values.len() as f64),
        crate::FlightUnion2::B(values) => (values.len() as f64),
    })
    .min(accumulator.components))
    .min((accumulator.values.len() as f64));
    if (accumulator.quaternion) && (components >= 4.0_f64) {
        write_normalized_quaternion(
            out,
            (accumulator.values[0.0_f64 as usize] as f64),
            (accumulator.values[1.0_f64 as usize] as f64),
            (accumulator.values[2.0_f64 as usize] as f64),
            (accumulator.values[3.0_f64 as usize] as f64),
        );
        return true;
    }
    let inverse_weight = (1.0_f64 / accumulator.weight);
    {
        let mut component = 0.0_f64;
        while (component < components) {
            {
                let __flight_index = (component) as usize;
                let __flight_value =
                    ((accumulator.values[component as usize] as f64) * inverse_weight);
                match out {
                    crate::FlightUnion2::A(values) => {
                        values[__flight_index] = __flight_value;
                    }
                    crate::FlightUnion2::B(values) => {
                        values[__flight_index] = (__flight_value) as f32;
                    }
                };
            };
            {
                component += 1.0;
                component
            };
        }
    }
    return true;
}

// Source: upstream/packages/animation/src/animationBlend.ts:111 (sha256:301031f070d76c28530f66e4fa9fcd6f8a276f4f66ab3bb0680a5882f0567700)
pub fn reset_animation_sample_accumulator(accumulator: &mut AnimationSampleAccumulator) -> () {
    {
        let __flight_value = (0.0_f64) as f32;
        let __flight_collection = &mut accumulator.values;
        __flight_collection.fill(__flight_value);
        __flight_collection.clone()
    };
    accumulator.weight = 0.0_f64;
}

// Source: upstream/packages/animation/src/animationBlend.ts:116 (sha256:1513ec2ac24c3c0337ad9074c0405341266697e33e2fc85e67d734ae3461d0e6)
fn slerp_quaternion(
    out: &mut crate::FlightUnion2<Vec<f64>, Vec<f32>>,
    a: &Vec<f64>,
    b: &Vec<f64>,
    alpha: f64,
) -> () {
    let ax = a[0.0_f64 as usize].clone();
    let ay = a[1.0_f64 as usize].clone();
    let az = a[2.0_f64 as usize].clone();
    let aw = a[3.0_f64 as usize].clone();
    let mut bx = b[0.0_f64 as usize].clone();
    let mut by = b[1.0_f64 as usize].clone();
    let mut bz = b[2.0_f64 as usize].clone();
    let mut bw = b[3.0_f64 as usize].clone();
    let mut dot = ((((ax * bx) + (ay * by)) + (az * bz)) + (aw * bw));
    if (dot < 0.0_f64) {
        dot = (-dot);
        bx = (-bx);
        by = (-by);
        bz = (-bz);
        bw = (-bw);
    }
    let mut scale_a: f64;
    let mut scale_b: f64;
    if ((1.0_f64 - dot) > 0.000001_f64) {
        let angle = ((1.0_f64).min(dot)).acos();
        let inverse_sin = (1.0_f64 / (angle).sin());
        scale_a = (((1.0_f64 - alpha) * angle).sin() * inverse_sin);
        scale_b = ((alpha * angle).sin() * inverse_sin);
    } else {
        scale_a = (1.0_f64 - alpha);
        scale_b = alpha;
    }
    write_normalized_quaternion(
        out,
        ((scale_a * ax) + (scale_b * bx)),
        ((scale_a * ay) + (scale_b * by)),
        ((scale_a * az) + (scale_b * bz)),
        ((scale_a * aw) + (scale_b * bw)),
    );
}

// Source: upstream/packages/animation/src/animationBlend.ts:158 (sha256:40fd59a43bb82af7c12482e64dfe11783be23131f6d7dccee17b4c4426798107)
fn write_normalized_quaternion(
    out: &mut crate::FlightUnion2<Vec<f64>, Vec<f32>>,
    x: f64,
    y: f64,
    z: f64,
    w: f64,
) -> () {
    let length = ((x).powi(2) + (y).powi(2) + (z).powi(2) + (w).powi(2)).sqrt();
    if (length == 0.0_f64) {
        {
            let __flight_index = (0.0_f64) as usize;
            let __flight_value = 0.0_f64;
            match out {
                crate::FlightUnion2::A(values) => {
                    values[__flight_index] = __flight_value;
                }
                crate::FlightUnion2::B(values) => {
                    values[__flight_index] = (__flight_value) as f32;
                }
            };
        };
        {
            let __flight_index = (1.0_f64) as usize;
            let __flight_value = 0.0_f64;
            match out {
                crate::FlightUnion2::A(values) => {
                    values[__flight_index] = __flight_value;
                }
                crate::FlightUnion2::B(values) => {
                    values[__flight_index] = (__flight_value) as f32;
                }
            };
        };
        {
            let __flight_index = (2.0_f64) as usize;
            let __flight_value = 0.0_f64;
            match out {
                crate::FlightUnion2::A(values) => {
                    values[__flight_index] = __flight_value;
                }
                crate::FlightUnion2::B(values) => {
                    values[__flight_index] = (__flight_value) as f32;
                }
            };
        };
        {
            let __flight_index = (3.0_f64) as usize;
            let __flight_value = 1.0_f64;
            match out {
                crate::FlightUnion2::A(values) => {
                    values[__flight_index] = __flight_value;
                }
                crate::FlightUnion2::B(values) => {
                    values[__flight_index] = (__flight_value) as f32;
                }
            };
        };
        return;
    }
    let inverse_length = (1.0_f64 / length);
    {
        let __flight_index = (0.0_f64) as usize;
        let __flight_value = (x * inverse_length);
        match out {
            crate::FlightUnion2::A(values) => {
                values[__flight_index] = __flight_value;
            }
            crate::FlightUnion2::B(values) => {
                values[__flight_index] = (__flight_value) as f32;
            }
        };
    };
    {
        let __flight_index = (1.0_f64) as usize;
        let __flight_value = (y * inverse_length);
        match out {
            crate::FlightUnion2::A(values) => {
                values[__flight_index] = __flight_value;
            }
            crate::FlightUnion2::B(values) => {
                values[__flight_index] = (__flight_value) as f32;
            }
        };
    };
    {
        let __flight_index = (2.0_f64) as usize;
        let __flight_value = (z * inverse_length);
        match out {
            crate::FlightUnion2::A(values) => {
                values[__flight_index] = __flight_value;
            }
            crate::FlightUnion2::B(values) => {
                values[__flight_index] = (__flight_value) as f32;
            }
        };
    };
    {
        let __flight_index = (3.0_f64) as usize;
        let __flight_value = (w * inverse_length);
        match out {
            crate::FlightUnion2::A(values) => {
                values[__flight_index] = __flight_value;
            }
            crate::FlightUnion2::B(values) => {
                values[__flight_index] = (__flight_value) as f32;
            }
        };
    };
}

// Source: upstream/packages/animation/src/animationBlend.ts:174 (sha256:f80b5257469870e96f7c4f51b6926ab62e5085bc520f8f566c63bf81eab892cc)
fn write_weighted_quaternion(out: &mut Vec<f32>, delta: &Vec<f64>, weight: f64) -> () {
    {
        let mut __flight_argument_0 =
            crate::FlightUnion2::<Vec<f64>, Vec<f32>>::B(std::mem::take(out));
        let __flight_result = slerp_quaternion(
            &mut __flight_argument_0,
            &((((*IDENTITY_QUATERNION).clone()).clone())
                .iter()
                .map(|__flight_value| (*__flight_value) as f64)
                .collect::<Vec<_>>()),
            delta,
            weight,
        );
        *(out) = match __flight_argument_0 {
            crate::FlightUnion2::A(_) => panic!("TypeScript union narrowing failed"),
            crate::FlightUnion2::B(value) => value,
        };
        __flight_result
    };
}

// Source: upstream/packages/animation/src/animationBlend.ts:178 (sha256:d4446e824c6a6c52679953e6ac1b93f765a402647f27e9e12d5f5885b9ed1899)
static IDENTITY_QUATERNION: std::sync::LazyLock<Vec<f32>> = std::sync::LazyLock::new(|| {
    (vec![0.0_f64, 0.0_f64, 0.0_f64, 1.0_f64])
        .iter()
        .map(|value| (*value) as f32)
        .collect()
});

// Source: upstream/packages/animation/src/animationBlend.ts:179 (sha256:1c753a3cc9a9dc74d189e43dae74d64281d1448a36ab14ea5acd004e41c82a13)
static _QUATERNION: std::sync::LazyLock<std::sync::Mutex<Vec<f32>>> =
    std::sync::LazyLock::new(|| std::sync::Mutex::new(vec![0.0_f32; (4.0_f64) as usize]));
