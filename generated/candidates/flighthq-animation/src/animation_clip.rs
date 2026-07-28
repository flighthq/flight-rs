// @generated from upstream/packages/animation/src/animationClip.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{clone_animation_track, sample_animation_track};
use flighthq_types::{AnimationChannel, AnimationClip, AnimationTrack};

// Source: upstream/packages/animation/src/animationClip.ts:7 (sha256:f4b32932b9d2455461c7c6867046571bc538f0be20caf87001745e650e48a336)
pub fn clone_animation_clip(clip: &AnimationClip) -> AnimationClip {
    let mut channels: Vec<AnimationChannel> = vec![];
    for channel in ((clip.channels).clone()).iter().cloned() {
        channels.push(create_animation_channel(
            &clone_animation_track(&channel.track),
            (channel.target_ref).clone(),
        ));
    }
    return AnimationClip {
        __flight_identity: std::sync::Arc::new(()),
        channels: (channels).clone(),
        duration: clip.duration,
    };
}

// Source: upstream/packages/animation/src/animationClip.ts:16 (sha256:170927c2ba647c17b6047691cf73cf2e132938c91d73208b0a2f43e2058898da)
pub fn create_animation_channel(
    track: &AnimationTrack,
    target_ref: crate::OpaqueHostValue,
) -> AnimationChannel {
    return AnimationChannel {
        __flight_identity: std::sync::Arc::new(()),
        target_ref: (target_ref).clone(),
        track: (*track).clone(),
    };
}

// Source: upstream/packages/animation/src/animationClip.ts:21 (sha256:f481f5ef3ee9588b476eee741ee64f21eea58722ddeac4f122971f18045552c8)
pub fn create_animation_clip(
    channels: &Vec<AnimationChannel>,
    duration: Option<f64>,
) -> AnimationClip {
    return AnimationClip {
        __flight_identity: std::sync::Arc::new(()),
        channels: (*channels).clone(),
        duration: (duration).unwrap_or(compute_channels_duration(channels)),
    };
}

// Source: upstream/packages/animation/src/animationClip.ts:26 (sha256:ad743756bb2c6c2a2abf30874a8da56139ce42be09c1bd42e2bc63bdcfa3d0f2)
pub fn get_animation_clip_duration(clip: &AnimationClip) -> f64 {
    return clip.duration;
}

// Source: upstream/packages/animation/src/animationClip.ts:35 (sha256:1f0827db3b313ce1c970adcdedb0ab918edd3ef13cc272502d97479f217a7fa3)
pub fn sample_animation_clip(
    out: &mut crate::FlightUnion2<Vec<f64>, Vec<f32>>,
    clip: &mut AnimationClip,
    time: f64,
    visit: &mut impl FnMut(crate::FlightUnion2<Vec<f64>, Vec<f32>>, AnimationChannel, f64) -> (),
) -> () {
    {
        let mut i = 0.0_f64;
        while (i < (clip.channels.len() as f64)) {
            let mut channel = clip.channels[i as usize].clone();
            sample_animation_track(&((*out).clone()), &mut channel.track, time);
            visit((*out).clone(), (channel).clone(), i);
            {
                i += 1.0;
                i
            };
        }
    }
}

// Source: upstream/packages/animation/src/animationClip.ts:49 (sha256:23b495cd45e3ca12fb6543dee0e21101ba8df07085564202305cf698f637265b)
fn compute_channels_duration(channels: &Vec<AnimationChannel>) -> f64 {
    let mut max = 0.0_f64;
    for channel in (channels).iter().cloned() {
        let last = (channel.track.times.len() as f64);
        if (last > 0.0_f64) && (channel.track.times[(last - 1.0_f64) as usize].clone() > max) {
            max = channel.track.times[(last - 1.0_f64) as usize].clone();
        }
    }
    return max;
}
