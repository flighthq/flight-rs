// @generated from upstream/packages/animation/src/animationClip.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{clone_animation_track, sample_animation_track};
use flighthq_entity::{allocate_entity, finish_entity};
use flighthq_types::{
    AnimationChannel, AnimationClip, AnimationClipEvent, AnimationTrack, EntityConstruction,
};

// Source: upstream/packages/animation/src/animationClip.ts:14 (sha256:1ecdbe14f8de6b9d763e9bf22430e0168afcb9f3367ca91f72ec9aef803e148b)
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
    let mut out = allocate_entity();
    crate::host_set("host.channels", channels);
    crate::host_set("host.duration", clip.duration);
    crate::host_set("host.events", events);
    return finish_entity((out).clone());
}

// Source: upstream/packages/animation/src/animationClip.ts:27 (sha256:e0d593558f9bd36f2b8280bfeca5879d3a074121f91b333753c6b8e5a694dd86)
pub fn create_animation_channel(
    track: &AnimationTrack,
    target_ref: crate::FlightValue,
) -> AnimationChannel {
    let mut out = allocate_entity();
    initialize_animation_channel((out).clone(), track, (target_ref).clone());
    return finish_entity((out).clone());
}

// Source: upstream/packages/animation/src/animationClip.ts:33 (sha256:0108f7f0bc48471e621596a1056e1d9a3c0ec762e4551305b3658ab019f24aeb)
pub fn create_animation_clip(
    channels: &Vec<AnimationChannel>,
    duration: Option<f64>,
    mut events: Option<Vec<AnimationClipEvent>>,
) -> AnimationClip {
    let events = events.unwrap_or(vec![]);
    let mut out = allocate_entity();
    initialize_animation_clip(
        (out).clone(),
        channels,
        (duration).clone(),
        Some(((events).clone()).clone()),
    );
    return finish_entity((out).clone());
}

// Source: upstream/packages/animation/src/animationClip.ts:43 (sha256:5170fe66e557961eb386a6c5ab55b5019e9cbca5a0ad5a92857369ea225c4fa5)
pub fn create_animation_clip_event(
    time: f64,
    name: String,
    payload: Option<crate::FlightValue>,
) -> AnimationClipEvent {
    let payload = payload.unwrap_or(crate::FlightValue::Null);
    let mut out = allocate_entity();
    initialize_animation_clip_event(
        (out).clone(),
        time,
        (name).clone(),
        Some(((payload).clone()).clone()),
    );
    return finish_entity((out).clone());
}

// Source: upstream/packages/animation/src/animationClip.ts:50 (sha256:ad743756bb2c6c2a2abf30874a8da56139ce42be09c1bd42e2bc63bdcfa3d0f2)
pub fn get_animation_clip_duration(clip: &AnimationClip) -> f64 {
    return clip.duration;
}

// Source: upstream/packages/animation/src/animationClip.ts:55 (sha256:dc4758d158ed8061da3b13b92966066dcd0f93f4781b8bd3d6e7dd8f81a1b079)
pub fn initialize_animation_channel(
    out: EntityConstruction<AnimationChannel>,
    track: &AnimationTrack,
    target_ref: crate::FlightValue,
) -> () {
    crate::host_set("host.targetRef", target_ref);
    crate::host_set("host.track", track);
}

// Source: upstream/packages/animation/src/animationClip.ts:66 (sha256:16112ed5e699a553eae6dca45e25bf6c525c22600367c83b43f4a648303a74e9)
pub fn initialize_animation_clip(
    out: EntityConstruction<AnimationClip>,
    channels: &Vec<AnimationChannel>,
    duration: Option<f64>,
    mut events: Option<Vec<AnimationClipEvent>>,
) -> () {
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
    crate::host_set("host.channels", channels);
    crate::host_set("host.duration", (duration).unwrap_or(computed_duration));
    crate::host_set("host.events", copied_events);
}

// Source: upstream/packages/animation/src/animationClip.ts:87 (sha256:5e13dc6f9ba08a168a0e3cdb092808dbf93d970b2f3875ad97576cd5378aa3cc)
pub fn initialize_animation_clip_event(
    out: EntityConstruction<AnimationClipEvent>,
    time: f64,
    name: String,
    payload: Option<crate::FlightValue>,
) -> () {
    let payload = payload.unwrap_or(crate::FlightValue::Null);
    crate::host_set("host.name", name);
    crate::host_set("host.payload", payload);
    crate::host_set("host.time", time);
}

// Source: upstream/packages/animation/src/animationClip.ts:103 (sha256:1f0827db3b313ce1c970adcdedb0ab918edd3ef13cc272502d97479f217a7fa3)
pub fn sample_animation_clip(
    out: &mut crate::FlightUnion2<Vec<f64>, Vec<f32>>,
    clip: &AnimationClip,
    time: f64,
    visit: &mut impl FnMut(crate::FlightUnion2<Vec<f64>, Vec<f32>>, AnimationChannel, f64) -> (),
) -> () {
    {
        let mut i = 0.0_f64;
        while (i < (clip.channels.len() as f64)) {
            let channel = clip.channels[i as usize].clone();
            sample_animation_track(out, &channel.track, time);
            visit((*out).clone(), (channel).clone(), i);
            {
                i += 1.0;
                i
            };
        }
    }
}

// Source: upstream/packages/animation/src/animationClip.ts:117 (sha256:23b495cd45e3ca12fb6543dee0e21101ba8df07085564202305cf698f637265b)
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

// Source: upstream/packages/animation/src/animationClip.ts:127 (sha256:fc46d1a6a5b57c8a11eaa7799fc01e43255070499a5cb0b36a6076ea9ced5ecd)
fn compute_animation_clip_events_duration(events: &Vec<AnimationClipEvent>) -> f64 {
    return if ((events.len() as f64) > 0.0_f64) {
        events[((events.len() as f64) - 1.0_f64) as usize].time
    } else {
        0.0_f64
    };
}

// Source: upstream/packages/animation/src/animationClip.ts:131 (sha256:9df0ecf726061537d67f0cd4bd3c0947c0740c3f5537500a5a4a3298db0f979f)
fn validate_animation_clip_events(events: &Vec<AnimationClipEvent>) -> () {
    for event in (events).iter().cloned() {
        if (!(event.time).is_finite()) || (event.time < 0.0_f64) {
            panic!("{}", "generated Flight function threw");
        }
    }
}
