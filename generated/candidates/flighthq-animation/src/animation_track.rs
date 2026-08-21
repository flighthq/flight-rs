// @generated from upstream/packages/animation/src/animationTrack.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_entity::create_entity;
use flighthq_types::{
    ANIMATION_INTERPOLATION_LINEAR as animation_interpolation_linear_constant,
    AnimationInterpolation, AnimationTrack, AnimationTrackValidationDiagnostic, EasingFunction,
};

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
    pub times: Vec<f64>,
    pub values: Vec<f64>,
    pub components: Option<f64>,
    pub interpolation: Option<AnimationInterpolation>,
    pub quaternion: Option<bool>,
    pub easing: Option<EasingFunction>,
    pub segment_easings: Option<Vec<Option<EasingFunction>>>,
}
impl PartialEq for SharedStructuralRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/animation/src/animationTrack.ts:12 (sha256:b89676b0200ed02f6e2355a6601fd928b66ffd6efc99c288be4f19b288955d15)
pub fn clone_animation_track(track: &AnimationTrack) -> AnimationTrack {
    return create_entity(Some(AnimationTrack {
        __flight_identity: std::sync::Arc::new(()),
        __flight_entity_runtime: Default::default(),
        components: track.components,
        easing: (track.easing).clone(),
        interpolation: (track.interpolation).clone(),
        quaternion: track.quaternion,
        segment_easings: if ((track.segment_easings).clone()).is_none() {
            None
        } else {
            Some(
                (track.segment_easings.as_ref().unwrap())
                    .as_ref()
                    .unwrap()
                    .clone(),
            )
        },
        times: clone_number_buffer(&track.times),
        values: clone_number_buffer(&track.values),
    }));
}

// Source: upstream/packages/animation/src/animationTrack.ts:27 (sha256:fcf9940824510f421234ddad37994d5cb4932508f752b6f89a60ec7b41ed471f)
pub fn create_animation_track(opts: &SharedStructuralRecord1) -> AnimationTrack {
    return create_entity(Some(AnimationTrack {
        __flight_identity: std::sync::Arc::new(()),
        __flight_entity_runtime: Default::default(),
        components: (opts.components).clone().unwrap_or(1.0_f64),
        easing: (opts.easing).clone(),
        interpolation: ((opts.interpolation).clone())
            .clone()
            .unwrap_or((animation_interpolation_linear_constant).to_owned()),
        quaternion: (opts.quaternion).clone().unwrap_or(false),
        segment_easings: (opts.segment_easings).clone(),
        times: (opts.times).clone(),
        values: (opts.values).clone(),
    }));
}

// Source: upstream/packages/animation/src/animationTrack.ts:52 (sha256:6505b73c5885152dfad1ec6e26da4e1c84d3f388880610a9506828b2315ed977)
pub fn sample_animation_track(
    out: &mut crate::FlightUnion2<Vec<f64>, Vec<f32>>,
    track: &mut AnimationTrack,
    t: f64,
) -> () {
    let components = track.components;
    let count = (track.times.len() as f64);
    if (count == 0.0_f64) {
        {
            let mut c = 0.0_f64;
            while (c < components) {
                out[c as usize] = 0.0_f64;
                {
                    c += 1.0;
                    c
                };
            }
        }
        return;
    }
    if (count == 1.0_f64) || (t <= track.times[0.0_f64 as usize].clone()) {
        copy_keyframe_value(&((*out).clone()), track, 0.0_f64);
        return;
    }
    if (t >= track.times[(count - 1.0_f64) as usize].clone()) {
        copy_keyframe_value(&((*out).clone()), track, (count - 1.0_f64));
        return;
    }
    let mut lo = 0.0_f64;
    let mut hi = (count - 1.0_f64);
    while (lo < hi) {
        let mid = (__flight_js_to_i32(((lo + hi) + 1.0_f64)) >> (__flight_js_to_u32(1.0_f64) & 31))
            as f64;
        if (track.times[mid as usize].clone() <= t) {
            lo = mid;
        } else {
            hi = (mid - 1.0_f64);
        }
    }
    let i = lo;
    let t0 = track.times[i as usize].clone();
    let dt = (track.times[(i + 1.0_f64) as usize].clone() - t0);
    let mut alpha = if (dt > 0.0_f64) {
        ((t - t0) / dt)
    } else {
        0.0_f64
    };
    let easing = (track
        .segment_easings
        .as_ref()
        .and_then(|values| values.get(i as usize).cloned()))
    .clone()
    .or((track.easing).clone());
    if (easing).is_some() {
        alpha = {
            let __flight_callback = (easing.as_ref().unwrap()).clone();
            let __flight_result = __flight_callback.lock().unwrap()(alpha);
            __flight_result
        };
    }
    if ((track.interpolation).clone() == "Step") {
        copy_keyframe_value(&((*out).clone()), track, i);
        return;
    }
    if ((track.interpolation).clone() == "Cubic") {
        sample_cubic_segment(&((*out).clone()), track, i, alpha, dt);
        return;
    }
    let oi = keyframe_value_offset(track, i);
    let oj = keyframe_value_offset(track, (i + 1.0_f64));
    if (track.quaternion) && (components == 4.0_f64) {
        slerp_flat_quaternion(&((*out).clone()), &mut track.values, oi, oj, alpha);
        return;
    }
    {
        let mut c = 0.0_f64;
        while (c < components) {
            let a = track.values[(oi + c) as usize].clone();
            out[c as usize] = (a + ((track.values[(oj + c) as usize].clone() - a) * alpha));
            {
                c += 1.0;
                c
            };
        }
    }
}

// Source: upstream/packages/animation/src/animationTrack.ts:111 (sha256:988484a98be2b5acef5a9cd68f8f1b11c09e0b31cd257e951c96731fd2baaa39)
pub fn trim_animation_track(
    track: &AnimationTrack,
    start_time: f64,
    end_time: f64,
) -> AnimationTrack {
    let components = track.components;
    let count = (track.times.len() as f64);
    let stride = keyframe_stride(track);
    let mut out_times: Vec<f64> = vec![];
    let mut out_values: Vec<f64> = vec![];
    let mut source_keyframes: Vec<f64> = vec![];
    {
        let mut k = 0.0_f64;
        while (k < count) {
            let time = track.times[k as usize].clone();
            if (time < start_time) || (time > end_time) {
                {
                    k += 1.0;
                    k
                };
                continue;
            }
            out_times.push((time - start_time));
            source_keyframes.push(k);
            let off = (k * stride);
            {
                let mut c = 0.0_f64;
                while (c < stride) {
                    out_values.push(track.values[(off + c) as usize].clone());
                    {
                        c += 1.0;
                        c
                    };
                }
            }
            {
                k += 1.0;
                k
            };
        }
    }
    return create_entity(Some(AnimationTrack {
        __flight_identity: std::sync::Arc::new(()),
        __flight_entity_runtime: Default::default(),
        components: components,
        easing: (track.easing).clone(),
        interpolation: (track.interpolation).clone(),
        quaternion: track.quaternion,
        segment_easings: Some(
            if (((track.segment_easings).clone()).is_none())
                || ((source_keyframes.len() as f64) < 2.0_f64)
            {
                if ((track.segment_easings).clone()).is_none() {
                    None
                } else {
                    vec![]
                }
            } else {
                (track.segment_easings.as_ref().unwrap())[(source_keyframes[0.0_f64 as usize]
                    .clone()) as usize
                    ..(source_keyframes[((source_keyframes.len() as f64) - 1.0_f64) as usize]
                        .clone()) as usize]
                    .to_vec()
            },
        ),
        times: (out_times).clone(),
        values: (out_values).clone(),
    }));
}

// Source: upstream/packages/animation/src/animationTrack.ts:149 (sha256:f9479860c1212366ddcfc872bb3f15dc3b70144887b5a6ee98610a1201ad837b)
pub fn validate_animation_track(
    track: &AnimationTrack,
) -> Option<Vec<AnimationTrackValidationDiagnostic>> {
    let mut diagnostics: Vec<AnimationTrackValidationDiagnostic> = vec![];
    let count = (track.times.len() as f64);
    {
        let mut k = 1.0_f64;
        while (k < count) {
            if (track.times[k as usize].clone() <= track.times[(k - 1.0_f64) as usize].clone()) {
                diagnostics.push(AnimationTrackValidationDiagnostic {
          __flight_identity: std::sync::Arc::new(()),
          code: "nonAscendingTimes".to_owned(),
          index: Some(k),
          message: format!("times[{}] ({}) is not greater than times[{}] ({}); times must be strictly ascending.", k, track.times[k as usize].clone(), (k - 1.0_f64), track.times[(k - 1.0_f64) as usize].clone()),
        });
            }
            {
                k += 1.0;
                k
            };
        }
    }
    let expected = (count * keyframe_stride(track));
    if ((track.values.len() as f64) != expected) {
        diagnostics.push(AnimationTrackValidationDiagnostic {
            __flight_identity: std::sync::Arc::new(()),
            code: "valuesLengthMismatch".to_owned(),
            index: None,
            message: format!(
                "values.length ({}) must equal keyCount * componentsPerKeyframe ({}).",
                (track.values.len() as f64),
                expected
            ),
        });
    }
    let expected_easings = (0.0_f64).max((count - 1.0_f64));
    if (((track.segment_easings).clone()).is_some())
        && ((track.segment_easings.as_ref().unwrap().len() as f64) != expected_easings)
    {
        diagnostics.push(AnimationTrackValidationDiagnostic {
            __flight_identity: std::sync::Arc::new(()),
            code: "segmentEasingsLengthMismatch".to_owned(),
            index: None,
            message: format!(
                "segmentEasings.length ({}) must equal keyCount - 1 ({}).",
                (track.segment_easings.as_ref().unwrap().len() as f64),
                expected_easings
            ),
        });
    }
    return if ((diagnostics.len() as f64) > 0.0_f64) {
        Some((diagnostics).clone())
    } else {
        None
    };
}

// Source: upstream/packages/animation/src/animationTrack.ts:183 (sha256:18109128439552d43471e9ed58704c9022c0384a02c401632c6fd0ac388e6f58)
fn clone_number_buffer(src: &Vec<f64>) -> crate::FlightUnion2<Vec<f64>, Vec<f32>> {
    if false {
        return crate::FlightUnion2::<Vec<f64>, Vec<f32>>::A((src).clone());
    }
    let mut out: Vec<f64> = vec![Default::default(); (src.len() as f64) as usize];
    {
        let mut i = 0.0_f64;
        while (i < (src.len() as f64)) {
            {
                let __flight_index = (i) as usize;
                let __flight_value = src[i as usize].clone();
                if __flight_index == out.len() {
                    out.push(__flight_value);
                } else {
                    out[__flight_index] = __flight_value;
                }
            };
            {
                i += 1.0;
                i
            };
        }
    }
    return crate::FlightUnion2::<Vec<f64>, Vec<f32>>::A((out).clone());
}

// Source: upstream/packages/animation/src/animationTrack.ts:190 (sha256:4fdf472a9d796ad467f2bd6a87d180467025444138194da9a584cf917e4f8df0)
fn copy_keyframe_value(
    out: &mut crate::FlightUnion2<Vec<f64>, Vec<f32>>,
    track: &AnimationTrack,
    k: f64,
) -> () {
    let off = keyframe_value_offset(track, k);
    {
        let mut c = 0.0_f64;
        while (c < track.components) {
            out[c as usize] = track.values[(off + c) as usize].clone();
            {
                c += 1.0;
                c
            };
        }
    }
}

// Source: upstream/packages/animation/src/animationTrack.ts:197 (sha256:d800b0a1f4c15b53b6886ef0fb38fdc942c1d325e3aeff4d952f9c37bcb46e65)
fn keyframe_stride(track: &AnimationTrack) -> f64 {
    return if ((track.interpolation).clone() == "Cubic") {
        (track.components * 3.0_f64)
    } else {
        track.components
    };
}

// Source: upstream/packages/animation/src/animationTrack.ts:203 (sha256:ec1397cdd5b6485d345beaf22e682e83d49bf05b67b2aacd1b04ded58451af29)
fn keyframe_value_offset(track: &AnimationTrack, k: f64) -> f64 {
    let stride = keyframe_stride(track);
    return if ((track.interpolation).clone() == "Cubic") {
        ((k * stride) + track.components)
    } else {
        (k * stride)
    };
}

// Source: upstream/packages/animation/src/animationTrack.ts:208 (sha256:658a02c6e1ef46c1ec5fc85444fab4e2447d569b07edfc32f2ddb90c5f8dcdda)
fn normalize_flat_quaternion(out: &mut crate::FlightUnion2<Vec<f64>, Vec<f32>>) -> () {
    let x = out[0.0_f64 as usize].clone();
    let y = out[1.0_f64 as usize].clone();
    let z = out[2.0_f64 as usize].clone();
    let w = out[3.0_f64 as usize].clone();
    let len = ((x).powi(2) + (y).powi(2) + (z).powi(2) + (w).powi(2)).sqrt();
    if (len > 0.0_f64) {
        let inv = (1.0_f64 / len);
        out[0.0_f64 as usize] = (x * inv);
        out[1.0_f64 as usize] = (y * inv);
        out[2.0_f64 as usize] = (z * inv);
        out[3.0_f64 as usize] = (w * inv);
    }
}

// Source: upstream/packages/animation/src/animationTrack.ts:226 (sha256:141770b866450d5904f607b17c37e1395c24904708c8d12c2dba10ff0e2c4c24)
fn sample_cubic_segment(
    out: &mut crate::FlightUnion2<Vec<f64>, Vec<f32>>,
    track: &AnimationTrack,
    i: f64,
    alpha: f64,
    dt: f64,
) -> () {
    let components = track.components;
    let stride = (components * 3.0_f64);
    let a2 = (alpha * alpha);
    let a3 = (a2 * alpha);
    let h00 = (((2.0_f64 * a3) - (3.0_f64 * a2)) + 1.0_f64);
    let h10 = ((a3 - (2.0_f64 * a2)) + alpha);
    let h01 = (((-2.0_f64) * a3) + (3.0_f64 * a2));
    let h11 = (a3 - a2);
    let base0 = (i * stride);
    let base1 = ((i + 1.0_f64) * stride);
    {
        let mut c = 0.0_f64;
        while (c < components) {
            let p0 = track.values[((base0 + components) + c) as usize].clone();
            let m0 = track.values[((base0 + (components * 2.0_f64)) + c) as usize].clone();
            let p1 = track.values[((base1 + components) + c) as usize].clone();
            let m1 = track.values[(base1 + c) as usize].clone();
            out[c as usize] = ((((h00 * p0) + ((h10 * dt) * m0)) + (h01 * p1)) + ((h11 * dt) * m1));
            {
                c += 1.0;
                c
            };
        }
    }
    if (track.quaternion) && (components == 4.0_f64) {
        normalize_flat_quaternion(&((*out).clone()));
    }
}

// Source: upstream/packages/animation/src/animationTrack.ts:255 (sha256:3bf822cfd02a04a268843d4af724cf990bc59ab4d4db60c485ef3baa0364de2e)
fn slerp_flat_quaternion(
    out: &mut crate::FlightUnion2<Vec<f64>, Vec<f32>>,
    values: &mut Vec<f64>,
    oa: f64,
    ob: f64,
    alpha: f64,
) -> () {
    let ax = values[oa as usize].clone();
    let ay = values[(oa + 1.0_f64) as usize].clone();
    let az = values[(oa + 2.0_f64) as usize].clone();
    let aw = values[(oa + 3.0_f64) as usize].clone();
    let mut bx = values[ob as usize].clone();
    let mut by = values[(ob + 1.0_f64) as usize].clone();
    let mut bz = values[(ob + 2.0_f64) as usize].clone();
    let mut bw = values[(ob + 3.0_f64) as usize].clone();
    let mut cosom = ((((ax * bx) + (ay * by)) + (az * bz)) + (aw * bw));
    if (cosom < 0.0_f64) {
        cosom = (-cosom);
        bx = (-bx);
        by = (-by);
        bz = (-bz);
        bw = (-bw);
    }
    let mut scale0: f64;
    let mut scale1: f64;
    if ((1.0_f64 - cosom) > 0.000001_f64) {
        let omega = (cosom).acos();
        let sinom = (omega).sin();
        scale0 = (((1.0_f64 - alpha) * omega).sin() / sinom);
        scale1 = ((alpha * omega).sin() / sinom);
    } else {
        scale0 = (1.0_f64 - alpha);
        scale1 = alpha;
    }
    out[0.0_f64 as usize] = ((scale0 * ax) + (scale1 * bx));
    out[1.0_f64 as usize] = ((scale0 * ay) + (scale1 * by));
    out[2.0_f64 as usize] = ((scale0 * az) + (scale1 * bz));
    out[3.0_f64 as usize] = ((scale0 * aw) + (scale1 * bw));
}
