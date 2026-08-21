// @generated from upstream/packages/animation/src/animationRootMotion.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::sample_animation_track;
use flighthq_entity::create_entity;
use flighthq_types::{AnimationClip, AnimationRootMotionExtractor};

// Source: upstream/packages/animation/src/animationRootMotion.ts:8 (sha256:c3bcd3d0589d9217e3e52cf3b44c278ee1b9e565afc3f03150493bfe73bcfc17)
pub fn create_animation_root_motion_extractor(
    clip: &mut AnimationClip,
    channel_index: f64,
) -> AnimationRootMotionExtractor {
    if ((!(channel_index).is_finite() && (channel_index).fract() == 0.0_f64)
        || (channel_index < 0.0_f64))
        || (channel_index >= (clip.channels.len() as f64))
    {
        panic!("{}", "generated Flight function threw");
    }
    let mut channel = clip.channels[channel_index as usize].clone();
    let width = channel.track.components;
    if (channel.track.quaternion) && (width != 4.0_f64) {
        panic!("{}", "generated Flight function threw");
    }
    let mut extractor = create_entity(Some(AnimationRootMotionExtractor {
        __flight_identity: std::sync::Arc::new(()),
        __flight_entity_runtime: Default::default(),
        channel: (channel).clone(),
        channel_index: channel_index,
        clip: (*clip).clone(),
        cycle_delta: vec![0.0_f32; (width) as usize],
        from_motion: vec![0.0_f32; (width) as usize],
        from_sample: vec![0.0_f32; (width) as usize],
        power_scratch: vec![0.0_f32; (width) as usize],
        start_sample: vec![0.0_f32; (width) as usize],
        to_motion: vec![0.0_f32; (width) as usize],
        to_sample: vec![0.0_f32; (width) as usize],
    }));
    sample_animation_track(
        &((extractor.start_sample).clone()),
        &mut extractor.channel.track,
        0.0_f64,
    );
    sample_animation_track(
        &((extractor.to_sample).clone()),
        &mut extractor.channel.track,
        extractor.clip.duration,
    );
    {
        let __flight_argument_1 = (extractor.start_sample).clone();
        let __flight_argument_2 = (extractor.to_sample).clone();
        let __flight_result = write_animation_root_motion_delta(
            &((extractor.cycle_delta).clone()),
            &__flight_argument_1,
            &__flight_argument_2,
            extractor.channel.track.quaternion,
        );
        __flight_result
    };
    return extractor;
}

// Source: upstream/packages/animation/src/animationRootMotion.ts:46 (sha256:1218a5d8bf544e15c0180df66ae1cb7db451d01c4a77d443f1bf91b07841c86a)
pub fn extract_animation_root_motion(
    out: &mut crate::FlightUnion2<Vec<f64>, Vec<f32>>,
    extractor: &mut AnimationRootMotionExtractor,
    start_time: f64,
    end_time: f64,
) -> bool {
    if (!(start_time).is_finite()) || (!(end_time).is_finite()) {
        panic!("{}", "generated Flight function threw");
    }
    let components = extractor.channel.track.components;
    if (out.length < components) {
        return false;
    }
    write_animation_root_motion_at(
        &mut extractor.from_motion,
        extractor,
        start_time,
        &mut extractor.from_sample,
    );
    write_animation_root_motion_at(
        &mut extractor.to_motion,
        extractor,
        end_time,
        &mut extractor.to_sample,
    );
    write_animation_root_motion_delta(
        &((*out).clone()),
        &extractor.from_motion,
        &extractor.to_motion,
        extractor.channel.track.quaternion,
    );
    return true;
}

// Source: upstream/packages/animation/src/animationRootMotion.ts:63 (sha256:f521fb68f66cbfe6a36629e8c1fae8c73be455438304686823cb95ba964d1028)
fn multiply_animation_root_motion_quaternion(
    out: &mut crate::FlightUnion2<Vec<f64>, Vec<f32>>,
    a: &Vec<f64>,
    b: &Vec<f64>,
) -> () {
    let ax = a[0.0_f64 as usize].clone();
    let ay = a[1.0_f64 as usize].clone();
    let az = a[2.0_f64 as usize].clone();
    let aw = a[3.0_f64 as usize].clone();
    let bx = b[0.0_f64 as usize].clone();
    let by = b[1.0_f64 as usize].clone();
    let bz = b[2.0_f64 as usize].clone();
    let bw = b[3.0_f64 as usize].clone();
    write_normalized_animation_root_motion_quaternion(
        &((*out).clone()),
        ((((aw * bx) + (ax * bw)) + (ay * bz)) - (az * by)),
        ((((aw * by) - (ax * bz)) + (ay * bw)) + (az * bx)),
        ((((aw * bz) + (ax * by)) - (ay * bx)) + (az * bw)),
        ((((aw * bw) - (ax * bx)) - (ay * by)) - (az * bz)),
    );
}

// Source: upstream/packages/animation/src/animationRootMotion.ts:85 (sha256:6674e183e37ef0890bc5dd92522bbbd173017f5f1224d689ab2bc17c31725333)
fn write_animation_root_motion_at(
    out: &mut Vec<f32>,
    extractor: &mut AnimationRootMotionExtractor,
    time: f64,
    sample: &mut Vec<f32>,
) -> () {
    let duration = extractor.clip.duration;
    if (!(duration > 0.0_f64)) {
        write_animation_root_motion_identity(
            &(crate::FlightUnion2::<Vec<f64>, Vec<f32>>::B((*out).clone())),
            extractor.channel.track.components,
            extractor.channel.track.quaternion,
        );
        return;
    }
    let cycle = (time / duration).floor();
    let local_time = (time - (cycle * duration));
    sample_animation_track(
        &(crate::FlightUnion2::<Vec<f64>, Vec<f32>>::B((*sample).clone())),
        &mut extractor.channel.track,
        local_time,
    );
    if extractor.channel.track.quaternion {
        write_animation_root_motion_quaternion_power(out, extractor, cycle);
        {
            let __flight_argument_1 = (extractor.start_sample).clone();
            let __flight_result = write_animation_root_motion_delta(
                &(crate::FlightUnion2::<Vec<f64>, Vec<f32>>::B((extractor.power_scratch).clone())),
                &__flight_argument_1,
                sample,
                true,
            );
            __flight_result
        };
        {
            let __flight_argument_1 = (out).clone();
            let __flight_result = multiply_animation_root_motion_quaternion(
                &(crate::FlightUnion2::<Vec<f64>, Vec<f32>>::B((*out).clone())),
                &__flight_argument_1,
                &extractor.power_scratch,
            );
            __flight_result
        };
        return;
    }
    {
        let mut component = 0.0_f64;
        while (component < extractor.channel.track.components) {
            out[component as usize] = (((sample[component as usize] as f64)
                - (extractor.start_sample[component as usize] as f64))
                + ((extractor.cycle_delta[component as usize] as f64) * cycle))
                as f32;
            {
                component += 1.0;
                component
            };
        }
    }
}

// Source: upstream/packages/animation/src/animationRootMotion.ts:111 (sha256:9934eaef8e234e3d0964a1750c9065e925b405865f145796e425d7ad7c2990f3)
fn write_animation_root_motion_delta(
    out: &mut crate::FlightUnion2<Vec<f64>, Vec<f32>>,
    from: &Vec<f64>,
    to: &Vec<f64>,
    quaternion: bool,
) -> () {
    if (!quaternion) {
        let width = ((out.length).min((from.len() as f64))).min((to.len() as f64));
        {
            let mut component = 0.0_f64;
            while (component < width) {
                out[component as usize] =
                    (to[component as usize].clone() - from[component as usize].clone());
                {
                    component += 1.0;
                    component
                };
            }
        }
        return;
    }
    write_normalized_animation_root_motion_quaternion(
        &((*out).clone()),
        ((((from[3.0_f64 as usize].clone() * to[0.0_f64 as usize].clone())
            - (from[0.0_f64 as usize].clone() * to[3.0_f64 as usize].clone()))
            - (from[1.0_f64 as usize].clone() * to[2.0_f64 as usize].clone()))
            + (from[2.0_f64 as usize].clone() * to[1.0_f64 as usize].clone())),
        ((((from[3.0_f64 as usize].clone() * to[1.0_f64 as usize].clone())
            + (from[0.0_f64 as usize].clone() * to[2.0_f64 as usize].clone()))
            - (from[1.0_f64 as usize].clone() * to[3.0_f64 as usize].clone()))
            - (from[2.0_f64 as usize].clone() * to[0.0_f64 as usize].clone())),
        ((((from[3.0_f64 as usize].clone() * to[2.0_f64 as usize].clone())
            - (from[0.0_f64 as usize].clone() * to[1.0_f64 as usize].clone()))
            + (from[1.0_f64 as usize].clone() * to[0.0_f64 as usize].clone()))
            - (from[2.0_f64 as usize].clone() * to[3.0_f64 as usize].clone())),
        ((((from[3.0_f64 as usize].clone() * to[3.0_f64 as usize].clone())
            + (from[0.0_f64 as usize].clone() * to[0.0_f64 as usize].clone()))
            + (from[1.0_f64 as usize].clone() * to[1.0_f64 as usize].clone()))
            + (from[2.0_f64 as usize].clone() * to[2.0_f64 as usize].clone())),
    );
}

// Source: upstream/packages/animation/src/animationRootMotion.ts:131 (sha256:2aad6639886573a4a3db81d4d023cc7f9e026113a367957fb49e32958bb9087c)
fn write_animation_root_motion_identity(
    out: &mut crate::FlightUnion2<Vec<f64>, Vec<f32>>,
    components: f64,
    quaternion: bool,
) -> () {
    let width = (out.length).min(components);
    {
        let mut component = 0.0_f64;
        while (component < width) {
            out[component as usize] = 0.0_f64;
            {
                component += 1.0;
                component
            };
        }
    }
    if (quaternion) && (width >= 4.0_f64) {
        out[3.0_f64 as usize] = 1.0_f64;
    }
}

// Source: upstream/packages/animation/src/animationRootMotion.ts:137 (sha256:f3b81ae55cd5a666e05f040741622ee2c6afd19924010079ee85b21abd3d2cc3)
fn write_animation_root_motion_quaternion_power(
    out: &mut Vec<f32>,
    extractor: &mut AnimationRootMotionExtractor,
    exponent: f64,
) -> () {
    write_animation_root_motion_identity(
        &(crate::FlightUnion2::<Vec<f64>, Vec<f32>>::B((*out).clone())),
        4.0_f64,
        true,
    );
    if (exponent == 0.0_f64) {
        return;
    }
    if (exponent > 0.0_f64) {
        {
            let __flight_offset = (0.0_f64) as usize;
            let __flight_values: Vec<f32> = ((extractor.cycle_delta).clone())
                .iter()
                .map(|value| (*value) as f32)
                .collect();
            extractor.power_scratch[__flight_offset..__flight_offset + __flight_values.len()]
                .copy_from_slice(&__flight_values);
        };
    } else {
        extractor.power_scratch[0.0_f64 as usize] =
            (-(extractor.cycle_delta[0.0_f64 as usize] as f64)) as f32;
        extractor.power_scratch[1.0_f64 as usize] =
            (-(extractor.cycle_delta[1.0_f64 as usize] as f64)) as f32;
        extractor.power_scratch[2.0_f64 as usize] =
            (-(extractor.cycle_delta[2.0_f64 as usize] as f64)) as f32;
        extractor.power_scratch[3.0_f64 as usize] =
            (extractor.cycle_delta[3.0_f64 as usize] as f64) as f32;
    }
    let mut remaining = (exponent).abs();
    while (remaining > 0.0_f64) {
        if ((remaining % 2.0_f64) == 1.0_f64) {
            {
                let __flight_argument_1 = (out).clone();
                let __flight_result = multiply_animation_root_motion_quaternion(
                    &(crate::FlightUnion2::<Vec<f64>, Vec<f32>>::B((*out).clone())),
                    &__flight_argument_1,
                    &extractor.power_scratch,
                );
                __flight_result
            };
        }
        remaining = (remaining / 2.0_f64).floor();
        if (remaining > 0.0_f64) {
            {
                let __flight_argument_1 = (extractor.power_scratch).clone();
                let __flight_argument_2 = (extractor.power_scratch).clone();
                let __flight_result = multiply_animation_root_motion_quaternion(
                    &(crate::FlightUnion2::<Vec<f64>, Vec<f32>>::B(
                        (extractor.power_scratch).clone(),
                    )),
                    &__flight_argument_1,
                    &__flight_argument_2,
                );
                __flight_result
            };
        }
    }
}

// Source: upstream/packages/animation/src/animationRootMotion.ts:161 (sha256:467e029d5ab2aaf9b1298a4a0ed621db2ce4a365dda3aa722bef99bdc29aa5c6)
fn write_normalized_animation_root_motion_quaternion(
    out: &mut crate::FlightUnion2<Vec<f64>, Vec<f32>>,
    x: f64,
    y: f64,
    z: f64,
    w: f64,
) -> () {
    let length = ((x).powi(2) + (y).powi(2) + (z).powi(2) + (w).powi(2)).sqrt();
    if (!(length > 0.0_f64)) {
        write_animation_root_motion_identity(&((*out).clone()), 4.0_f64, true);
        return;
    }
    let inverse_length = (1.0_f64 / length);
    out[0.0_f64 as usize] = (x * inverse_length);
    out[1.0_f64 as usize] = (y * inverse_length);
    out[2.0_f64 as usize] = (z * inverse_length);
    out[3.0_f64 as usize] = (w * inverse_length);
}
