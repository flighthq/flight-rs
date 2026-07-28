// @generated from upstream/packages/particles/src/curve.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

pub use flighthq_types::{ColorKeyframe, CurveKeyframe, ParticleCurve};

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
pub struct SharedStructuralRecord1 {
    pub __flight_identity: std::sync::Arc<()>,
    pub time: f64,
}
impl PartialEq for SharedStructuralRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct SharedStructuralRecord2 {
    pub __flight_identity: std::sync::Arc<()>,
    pub f: f64,
    pub i: f64,
}
impl PartialEq for SharedStructuralRecord2 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct SharedStructuralRecord3 {
    pub __flight_identity: std::sync::Arc<()>,
}
impl PartialEq for SharedStructuralRecord3 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/particles/src/curve.ts:6 (sha256:783ff7a690c8c841b486c92aa530da277279f483891265bc824bbb646fbf97ce)
pub fn build_particle_color_curve(
    f: &mut impl FnMut(f64) -> Vec<f64>,
    samples: Option<f64>,
) -> Vec<f64> {
    let samples = samples.unwrap_or(33.0_f64);
    let n = (2.0_f64).max((__flight_js_to_i32(samples) | __flight_js_to_i32(0.0_f64)) as f64);
    let mut lut: Vec<f64> = vec![Default::default(); (n * 3.0_f64) as usize];
    {
        let mut i = 0.0_f64;
        while (i < n) {
            let __destructure0 = f((i / (n - 1.0_f64)));
            let r = __destructure0[0.0_f64 as usize].clone();
            let g = __destructure0[1.0_f64 as usize].clone();
            let b = __destructure0[2.0_f64 as usize].clone();
            {
                let __flight_index = (i * 3.0_f64) as usize;
                let __flight_value = r;
                if __flight_index == lut.len() {
                    lut.push(__flight_value);
                } else {
                    lut[__flight_index] = __flight_value;
                }
            };
            {
                let __flight_index = ((i * 3.0_f64) + 1.0_f64) as usize;
                let __flight_value = g;
                if __flight_index == lut.len() {
                    lut.push(__flight_value);
                } else {
                    lut[__flight_index] = __flight_value;
                }
            };
            {
                let __flight_index = ((i * 3.0_f64) + 2.0_f64) as usize;
                let __flight_value = b;
                if __flight_index == lut.len() {
                    lut.push(__flight_value);
                } else {
                    lut[__flight_index] = __flight_value;
                }
            };
            {
                i += 1.0;
                i
            };
        }
    }
    return lut;
}

// Source: upstream/packages/particles/src/curve.ts:19 (sha256:c16328d38b188b611eb9bf96e02bade98f8027d9c20be610f9317d0b35a19a44)
pub fn build_particle_curve(
    f: std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(f64) -> f64 + Send + 'static>>>,
    samples: Option<f64>,
) -> Vec<f64> {
    let samples = samples.unwrap_or(33.0_f64);
    let n = (2.0_f64).max((__flight_js_to_i32(samples) | __flight_js_to_i32(0.0_f64)) as f64);
    let mut lut: Vec<f64> = vec![Default::default(); (n) as usize];
    {
        let mut i = 0.0_f64;
        while (i < n) {
            {
                let __flight_index = (i) as usize;
                let __flight_value = {
                    let __flight_callback = (f).clone();
                    let __flight_result = __flight_callback.lock().unwrap()((i / (n - 1.0_f64)));
                    __flight_result
                };
                if __flight_index == lut.len() {
                    lut.push(__flight_value);
                } else {
                    lut[__flight_index] = __flight_value;
                }
            };
            {
                i += 1.0;
                i
            };
        }
    }
    return lut;
}

// Source: upstream/packages/particles/src/curve.ts:28 (sha256:2d3a5c547e45933a4028ee26d0c2c55a60c44b074cbf46e22a56f0e09562b540)
pub fn lerp_hsv_direct(
    out: &mut crate::FlightUnion2<Vec<f32>, Vec<f64>>,
    offset: f64,
    r0: f64,
    g0: f64,
    b0: f64,
    r1: f64,
    g1: f64,
    b1: f64,
    t: f64,
) -> () {
    let __destructure1 = rgb_to_hsv(r0, g0, b0);
    let h0 = __destructure1[0.0_f64 as usize].clone();
    let s0 = __destructure1[1.0_f64 as usize].clone();
    let v0 = __destructure1[2.0_f64 as usize].clone();
    let __destructure2 = rgb_to_hsv(r1, g1, b1);
    let h1 = __destructure2[0.0_f64 as usize].clone();
    let s1 = __destructure2[1.0_f64 as usize].clone();
    let v1 = __destructure2[2.0_f64 as usize].clone();
    let mut dh = (h1 - h0);
    if (dh > 0.5_f64) {
        dh -= 1.0_f64;
    } else {
        if (dh < (-0.5_f64)) {
            dh += 1.0_f64;
        }
    }
    let __destructure3 = hsv_to_rgb(
        (h0 + (dh * t)),
        (s0 + ((s1 - s0) * t)),
        (v0 + ((v1 - v0) * t)),
    );
    let r = __destructure3[0.0_f64 as usize].clone();
    let g = __destructure3[1.0_f64 as usize].clone();
    let b = __destructure3[2.0_f64 as usize].clone();
    out[offset as usize] = r;
    out[(offset + 1.0_f64) as usize] = g;
    out[(offset + 2.0_f64) as usize] = b;
}

// Source: upstream/packages/particles/src/curve.ts:52 (sha256:7c936f8f88bc7b7098dc9374cffe9ffed54e343105ca991e817354c944458110)
pub fn lerp_hsv_in_place(
    colors_out: &mut crate::FlightUnion2<Vec<f32>, Vec<f64>>,
    offset: f64,
    birth: &Vec<f32>,
    death: &Vec<f32>,
    t: f64,
) -> () {
    lerp_hsv_direct(
        &((*colors_out).clone()),
        offset,
        (birth[offset as usize] as f64),
        (birth[(offset + 1.0_f64) as usize] as f64),
        (birth[(offset + 2.0_f64) as usize] as f64),
        (death[offset as usize] as f64),
        (death[(offset + 1.0_f64) as usize] as f64),
        (death[(offset + 2.0_f64) as usize] as f64),
        t,
    );
}

// Source: upstream/packages/particles/src/curve.ts:74 (sha256:ec995439038deda264344784bb5d95a0528ee293b0d0abcfa3cd124ffdd1d4c5)
pub fn particle_color_curve_from_keyframes(
    keys: &mut Vec<ColorKeyframe>,
    samples: Option<f64>,
) -> Vec<f64> {
    let samples = samples.unwrap_or(33.0_f64);
    if ((keys.len() as f64) == 0.0_f64) {
        return vec![0.0_f64, 0.0_f64, 0.0_f64, 0.0_f64, 0.0_f64, 0.0_f64];
    }
    let sorted = {
        let mut __flight_values = (keys).clone();
        __flight_values.sort_by(|left, right| {
            let __flight_order =
                (|a: ColorKeyframe, b: ColorKeyframe| -> f64 { (a.time - b.time) })(
                    left.clone(),
                    right.clone(),
                );
            __flight_order
                .partial_cmp(&0.0_f64)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        __flight_values
    };
    return build_particle_color_curve(
        &mut |t: f64| -> Vec<f64> {
            let seg = locate_keyframe(&sorted, t);
            if (seg.f == 0.0_f64) {
                return vec![
                    sorted[seg.i as usize].r,
                    sorted[seg.i as usize].g,
                    sorted[seg.i as usize].b,
                ];
            }
            let a = sorted[seg.i as usize].clone();
            let b = sorted[(seg.i + 1.0_f64) as usize].clone();
            return vec![
                (a.r + ((b.r - a.r) * seg.f)),
                (a.g + ((b.g - a.g) * seg.f)),
                (a.b + ((b.b - a.b) * seg.f)),
            ];
        },
        Some(samples),
    );
}

// Source: upstream/packages/particles/src/curve.ts:88 (sha256:bfcf16be19906a5be170adefe9905f0abe1c8ac14995174a56c16c9997d4f47a)
pub fn particle_color_curve_to_keyframes(lut: &ParticleCurve) -> Vec<ColorKeyframe> {
    let n = (lut.length / 3.0_f64).floor();
    if (n == 0.0_f64) {
        return vec![];
    }
    if (n == 1.0_f64) {
        return vec![ColorKeyframe {
            __flight_identity: std::sync::Arc::new(()),
            time: 0.0_f64,
            r: lut[0.0_f64 as usize].clone(),
            g: lut[1.0_f64 as usize].clone(),
            b: lut[2.0_f64 as usize].clone(),
        }];
    }
    let mut keys: Vec<ColorKeyframe> = vec![Default::default(); (n) as usize];
    {
        let mut i = 0.0_f64;
        while (i < n) {
            {
                let __flight_index = (i) as usize;
                let __flight_value = ColorKeyframe {
                    __flight_identity: std::sync::Arc::new(()),
                    time: (i / (n - 1.0_f64)),
                    r: lut[(i * 3.0_f64) as usize].clone(),
                    g: lut[((i * 3.0_f64) + 1.0_f64) as usize].clone(),
                    b: lut[((i * 3.0_f64) + 2.0_f64) as usize].clone(),
                };
                if __flight_index == keys.len() {
                    keys.push(__flight_value);
                } else {
                    keys[__flight_index] = __flight_value;
                }
            };
            {
                i += 1.0;
                i
            };
        }
    }
    return keys;
}

// Source: upstream/packages/particles/src/curve.ts:100 (sha256:cc9e7c9b98ab1da548a15786de0b264e61e4cc7c8dfe97a7cdca0e7e0cce849b)
pub fn particle_curve_from_keyframes(
    keys: &mut Vec<CurveKeyframe>,
    samples: Option<f64>,
) -> Vec<f64> {
    let samples = samples.unwrap_or(33.0_f64);
    if ((keys.len() as f64) == 0.0_f64) {
        return vec![0.0_f64, 0.0_f64];
    }
    let sorted = {
        let mut __flight_values = (keys).clone();
        __flight_values.sort_by(|left, right| {
            let __flight_order =
                (|a: CurveKeyframe, b: CurveKeyframe| -> f64 { (a.time - b.time) })(
                    left.clone(),
                    right.clone(),
                );
            __flight_order
                .partial_cmp(&0.0_f64)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        __flight_values
    };
    return build_particle_curve(
        std::sync::Arc::new(std::sync::Mutex::new(Box::new({
            let sorted = sorted.clone();
            move |t: f64| -> f64 { interp_keyframe(&sorted, t) }
        })
            as Box<dyn FnMut(f64) -> f64 + Send + 'static>)),
        Some(samples),
    );
}

// Source: upstream/packages/particles/src/curve.ts:109 (sha256:b3beb7b5b8224995369d409cf3715319f965f886b22b376f7851aaa0d4127c2f)
pub fn particle_curve_to_keyframes(lut: &ParticleCurve) -> Vec<CurveKeyframe> {
    let n = lut.length;
    if (n == 0.0_f64) {
        return vec![];
    }
    if (n == 1.0_f64) {
        return vec![CurveKeyframe {
            __flight_identity: std::sync::Arc::new(()),
            time: 0.0_f64,
            value: lut[0.0_f64 as usize].clone(),
        }];
    }
    let mut keys: Vec<CurveKeyframe> = vec![Default::default(); (n) as usize];
    {
        let mut i = 0.0_f64;
        while (i < n) {
            {
                let __flight_index = (i) as usize;
                let __flight_value = CurveKeyframe {
                    __flight_identity: std::sync::Arc::new(()),
                    time: (i / (n - 1.0_f64)),
                    value: lut[i as usize].clone(),
                };
                if __flight_index == keys.len() {
                    keys.push(__flight_value);
                } else {
                    keys[__flight_index] = __flight_value;
                }
            };
            {
                i += 1.0;
                i
            };
        }
    }
    return keys;
}

// Source: upstream/packages/particles/src/curve.ts:119 (sha256:2bc6b7fe45a1b3570b160a6c2ccd33245f2938eac3c2959e58a79d8b3bbd934e)
fn hsv_to_rgb(h: f64, s: f64, v: f64) -> Vec<f64> {
    let c = (v * s);
    let x = (c * (1.0_f64 - (((h * 6.0_f64) % 2.0_f64) - 1.0_f64).abs()));
    let m = (v - c);
    let hi = ((h * 6.0_f64).floor() % 6.0_f64);
    let mut r = 0.0_f64;
    let mut g = 0.0_f64;
    let mut b = 0.0_f64;
    {
        let __switch_value = hi;
        let __flight_case = if __switch_value == 0.0_f64 {
            0_usize
        } else if __switch_value == 1.0_f64 {
            1_usize
        } else if __switch_value == 2.0_f64 {
            2_usize
        } else if __switch_value == 3.0_f64 {
            3_usize
        } else if __switch_value == 4.0_f64 {
            4_usize
        } else {
            5_usize
        };
        '__flight_switch: {
            if __flight_case <= 0_usize {
                r = c;
                g = x;
                b = 0.0_f64;
                break '__flight_switch;
            }
            if __flight_case <= 1_usize {
                r = x;
                g = c;
                b = 0.0_f64;
                break '__flight_switch;
            }
            if __flight_case <= 2_usize {
                r = 0.0_f64;
                g = c;
                b = x;
                break '__flight_switch;
            }
            if __flight_case <= 3_usize {
                r = 0.0_f64;
                g = x;
                b = c;
                break '__flight_switch;
            }
            if __flight_case <= 4_usize {
                r = x;
                g = 0.0_f64;
                b = c;
                break '__flight_switch;
            }
            if __flight_case <= 5_usize {
                r = c;
                g = 0.0_f64;
                b = x;
                break '__flight_switch;
            }
        }
    }
    return vec![(r + m), (g + m), (b + m)];
}

// Source: upstream/packages/particles/src/curve.ts:162 (sha256:f8249bf1d6f2f7f93ae47f2339963831d9730ba016a67060c864af0676c66945)
fn interp_keyframe(sorted: &Vec<CurveKeyframe>, t: f64) -> f64 {
    let seg = locate_keyframe(sorted, t);
    if (seg.f == 0.0_f64) {
        return sorted[seg.i as usize].value;
    }
    let a = sorted[seg.i as usize].value;
    let b = sorted[(seg.i + 1.0_f64) as usize].value;
    return (a + ((b - a) * seg.f));
}

// Source: upstream/packages/particles/src/curve.ts:172 (sha256:51e0bc24bd5b3d4de372d255864ad4cd7140fbe32de9a647dccf4bb261bb0ae1)
fn locate_keyframe(sorted: &Vec<SharedStructuralRecord1>, t: f64) -> SharedStructuralRecord2 {
    let n = (sorted.len() as f64);
    if (t <= sorted[0.0_f64 as usize].time) {
        return SharedStructuralRecord2 {
            __flight_identity: std::sync::Arc::new(()),
            f: 0.0_f64,
            i: 0.0_f64,
        };
    }
    if (t >= sorted[(n - 1.0_f64) as usize].time) {
        return SharedStructuralRecord2 {
            __flight_identity: std::sync::Arc::new(()),
            f: 0.0_f64,
            i: (n - 1.0_f64),
        };
    }
    {
        let mut i = 0.0_f64;
        while (i < (n - 1.0_f64)) {
            let t0 = sorted[i as usize].time;
            let t1 = sorted[(i + 1.0_f64) as usize].time;
            if (t <= t1) {
                let span = (t1 - t0);
                return SharedStructuralRecord2 {
                    __flight_identity: std::sync::Arc::new(()),
                    f: if (span <= 0.0_f64) {
                        0.0_f64
                    } else {
                        ((t - t0) / span)
                    },
                    i: i,
                };
            }
            {
                i += 1.0;
                i
            };
        }
    }
    return SharedStructuralRecord2 {
        __flight_identity: std::sync::Arc::new(()),
        f: 0.0_f64,
        i: (n - 1.0_f64),
    };
}

// Source: upstream/packages/particles/src/curve.ts:188 (sha256:16443d3a99585a2bdfdf4f3eeb1c686d3faf9bc7a880d81e408ce0c170e8af72)
fn rgb_to_hsv(r: f64, g: f64, b: f64) -> Vec<f64> {
    let max = ((r).max(g)).max(b);
    let min = ((r).min(g)).min(b);
    let delta = (max - min);
    let mut h = 0.0_f64;
    if (delta > 0.0_f64) {
        if (max == r) {
            h = (((g - b) / delta) % 6.0_f64);
        } else {
            if (max == g) {
                h = (((b - r) / delta) + 2.0_f64);
            } else {
                h = (((r - g) / delta) + 4.0_f64);
            }
        }
        h /= 6.0_f64;
        if (h < 0.0_f64) {
            h += 1.0_f64;
        }
    }
    return vec![
        h,
        if (max == 0.0_f64) {
            0.0_f64
        } else {
            (delta / max)
        },
        max,
    ];
}

// Source: upstream/packages/particles/src/curve.ts:205 (sha256:3452760874a3ff7d3f0fcc1950e46641661c7dd60e111240f3559e0ebb0357ff)
pub fn sample_particle_color_curve(
    out: &mut SharedStructuralRecord3,
    offset: f64,
    lut: &ParticleCurve,
    t: f64,
) -> () {
    let n = (lut.length / 3.0_f64);
    if (n <= 0.0_f64) {
        out[offset as usize] = 0.0_f64;
        out[(offset + 1.0_f64) as usize] = 0.0_f64;
        out[(offset + 2.0_f64) as usize] = 0.0_f64;
        return;
    }
    if (n == 1.0_f64) {
        out[offset as usize] = lut[0.0_f64 as usize].clone();
        out[(offset + 1.0_f64) as usize] = lut[1.0_f64 as usize].clone();
        out[(offset + 2.0_f64) as usize] = lut[2.0_f64 as usize].clone();
        return;
    }
    let x = (if (t <= 0.0_f64) {
        0.0_f64
    } else {
        if (t >= 1.0_f64) { 1.0_f64 } else { t }
    } * (n - 1.0_f64));
    let i = (__flight_js_to_i32(x) | __flight_js_to_i32(0.0_f64)) as f64;
    if (i >= (n - 1.0_f64)) {
        let base = ((n - 1.0_f64) * 3.0_f64);
        out[offset as usize] = lut[base as usize].clone();
        out[(offset + 1.0_f64) as usize] = lut[(base + 1.0_f64) as usize].clone();
        out[(offset + 2.0_f64) as usize] = lut[(base + 2.0_f64) as usize].clone();
        return;
    }
    let f = (x - i);
    let a = (i * 3.0_f64);
    let b = (a + 3.0_f64);
    out[offset as usize] =
        (lut[a as usize].clone() + ((lut[b as usize].clone() - lut[a as usize].clone()) * f));
    out[(offset + 1.0_f64) as usize] = (lut[(a + 1.0_f64) as usize].clone()
        + ((lut[(b + 1.0_f64) as usize].clone() - lut[(a + 1.0_f64) as usize].clone()) * f));
    out[(offset + 2.0_f64) as usize] = (lut[(a + 2.0_f64) as usize].clone()
        + ((lut[(b + 2.0_f64) as usize].clone() - lut[(a + 2.0_f64) as usize].clone()) * f));
}

// Source: upstream/packages/particles/src/curve.ts:244 (sha256:a3c267546bba9017a4cd5cd4eceb5cecf4acdb9fc668708ed22f5e436741a123)
pub fn sample_particle_curve(lut: &ParticleCurve, t: f64) -> f64 {
    let n = lut.length;
    if (n == 0.0_f64) {
        return 0.0_f64;
    }
    if (n == 1.0_f64) {
        return lut[0.0_f64 as usize].clone();
    }
    let x = (if (t <= 0.0_f64) {
        0.0_f64
    } else {
        if (t >= 1.0_f64) { 1.0_f64 } else { t }
    } * (n - 1.0_f64));
    let i = (__flight_js_to_i32(x) | __flight_js_to_i32(0.0_f64)) as f64;
    if (i >= (n - 1.0_f64)) {
        return lut[(n - 1.0_f64) as usize].clone();
    }
    return (lut[i as usize].clone()
        + ((lut[(i + 1.0_f64) as usize].clone() - lut[i as usize].clone()) * (x - i)));
}
