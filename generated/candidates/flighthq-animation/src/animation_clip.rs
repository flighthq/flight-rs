// @generated from upstream/packages/animation/src/animationClip.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{clone_animation_track, sample_animation_track};
use flighthq_entity::create_entity;
use flighthq_types::{AnimationChannel, AnimationClip, AnimationClipEvent, AnimationTrack};

// Source: upstream/packages/animation/src/animationClip.ts:8 (sha256:541904a49df3a4bc964fd29c0ffce4e4da570b682c8fb33280daa9786f930a48)
pub fn clone_animation_clip(clip: &AnimationClip) -> AnimationClip {
    let mut channels: Vec<AnimationChannel> = vec![];
    for channel in ((clip.channels).clone()).iter().cloned() {
        channels.push(create_animation_channel(
            &clone_animation_track(&channel.track),
            (channel.target_ref).clone(),
        ));
    }
    let events = ((clip.events).clone())
        .iter()
        .cloned()
        .map(|event: AnimationClipEvent| -> AnimationClipEvent {
            create_animation_clip_event(
                event.time,
                (event.name).clone(),
                Some(((event.payload).clone()).clone()),
            )
        })
        .collect::<Vec<_>>();
    return create_entity(Some(AnimationClip {
        __flight_identity: std::sync::Arc::new(()),
        __flight_entity_runtime: Default::default(),
        channels: (channels).clone(),
        duration: clip.duration,
        events: (events).clone(),
    }));
}

// Source: upstream/packages/animation/src/animationClip.ts:18 (sha256:c6735d6b8249c8f4c852b510de30afe1d63e6a88536e9821d1035d92b57bde54)
pub fn create_animation_channel(
    track: &AnimationTrack,
    target_ref: crate::FlightValue,
) -> AnimationChannel {
    return create_entity(Some(AnimationChannel {
        __flight_identity: std::sync::Arc::new(()),
        __flight_entity_runtime: Default::default(),
        target_ref: (target_ref).clone(),
        track: (*track).clone(),
    }));
}

// Source: upstream/packages/animation/src/animationClip.ts:24 (sha256:f9ff491120f4438ada9a0eb0b05c1d1f2d990ba1d54897f8e5b756b6b8fbd94d)
pub fn create_animation_clip(
    channels: &Vec<AnimationChannel>,
    duration: Option<f64>,
    mut events: Option<Vec<AnimationClipEvent>>,
) -> AnimationClip {
    let events = events.unwrap_or(vec![]);
    let copied_events = {
        let mut __flight_values = (events).clone();
        __flight_values.sort_by(|left, right| {
            let __flight_order =
                (|a: AnimationClipEvent, b: AnimationClipEvent| -> f64 { (a.time - b.time) })(
                    left.clone(),
                    right.clone(),
                );
            __flight_order
                .partial_cmp(&0.0_f64)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        __flight_values
    };
    validate_animation_clip_events(&copied_events);
    let computed_duration = (compute_channels_duration(channels))
        .max(compute_animation_clip_events_duration(&copied_events));
    if (((duration).is_some()) && ((copied_events.len() as f64) > 0.0_f64))
        && ((duration).as_ref().is_some_and(|value| {
            copied_events[((copied_events.len() as f64) - 1.0_f64) as usize].time > *value
        }))
    {
        panic!("{}", "generated Flight function threw");
    }
    return create_entity(Some(AnimationClip {
        __flight_identity: std::sync::Arc::new(()),
        __flight_entity_runtime: Default::default(),
        channels: (*channels).clone(),
        duration: (duration).clone().unwrap_or(computed_duration),
        events: (copied_events).clone(),
    }));
}

// Source: upstream/packages/animation/src/animationClip.ts:42 (sha256:3bdaa313dbe56d162d777b0ad9046080e77c09d52a8a4ae643a58b0169fdba89)
pub fn create_animation_clip_event(
    time: f64,
    name: String,
    payload: Option<crate::FlightValue>,
) -> AnimationClipEvent {
    let payload = payload.unwrap_or(crate::FlightValue::Null);
    return create_entity(Some(AnimationClipEvent {
        __flight_identity: std::sync::Arc::new(()),
        __flight_entity_runtime: Default::default(),
        name: (name).clone(),
        payload: (payload).clone(),
        time: time,
    }));
}

// Source: upstream/packages/animation/src/animationClip.ts:47 (sha256:ad743756bb2c6c2a2abf30874a8da56139ce42be09c1bd42e2bc63bdcfa3d0f2)
pub fn get_animation_clip_duration(clip: &AnimationClip) -> f64 {
    return clip.duration;
}

// Source: upstream/packages/animation/src/animationClip.ts:56 (sha256:1f0827db3b313ce1c970adcdedb0ab918edd3ef13cc272502d97479f217a7fa3)
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

// Source: upstream/packages/animation/src/animationClip.ts:70 (sha256:23b495cd45e3ca12fb6543dee0e21101ba8df07085564202305cf698f637265b)
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

// Source: upstream/packages/animation/src/animationClip.ts:80 (sha256:fc46d1a6a5b57c8a11eaa7799fc01e43255070499a5cb0b36a6076ea9ced5ecd)
fn compute_animation_clip_events_duration(events: &Vec<AnimationClipEvent>) -> f64 {
    return if ((events.len() as f64) > 0.0_f64) {
        events[((events.len() as f64) - 1.0_f64) as usize].time
    } else {
        0.0_f64
    };
}

// Source: upstream/packages/animation/src/animationClip.ts:84 (sha256:9df0ecf726061537d67f0cd4bd3c0947c0740c3f5537500a5a4a3298db0f979f)
fn validate_animation_clip_events(events: &Vec<AnimationClipEvent>) -> () {
    for event in (events).iter().cloned() {
        if (!(event.time).is_finite()) || (event.time < 0.0_f64) {
            panic!("{}", "generated Flight function threw");
        }
    }
}
