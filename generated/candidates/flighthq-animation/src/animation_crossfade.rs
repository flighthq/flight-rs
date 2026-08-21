// @generated from upstream/packages/animation/src/animationCrossfade.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{advance_animation_player, blend_animation_samples, sample_animation_track};
use flighthq_entity::create_entity;
use flighthq_types::{
    AnimationChannel, AnimationCrossfade, AnimationCrossfadeChannel, AnimationCrossfadeOptions,
    AnimationPlayer,
};

// Source: upstream/packages/animation/src/animationCrossfade.ts:16 (sha256:5f5b265f65fb2ef4d948d2d15945403823ad37bf76dfcc30936e0075653c927d)
pub fn advance_animation_crossfade(state: &mut AnimationCrossfade, dt: f64) -> () {
    advance_animation_player(&mut state.from, dt);
    advance_animation_player(&mut state.to, dt);
    state.elapsed += dt;
    state.weight = {
        let __flight_callback = (state.curve).clone();
        let __flight_result = __flight_callback.lock().unwrap()(
            get_linear_animation_crossfade_weight(state.elapsed, state.duration),
        );
        __flight_result
    };
}

// Source: upstream/packages/animation/src/animationCrossfade.ts:26 (sha256:0090cffe255bc65a325c8ee03fba93aa6efac2a86d326a70fd05c2f0e7aadbf0)
pub fn create_animation_crossfade(
    from: &AnimationPlayer,
    to: &AnimationPlayer,
    duration: f64,
    opts: Option<AnimationCrossfadeOptions>,
) -> AnimationCrossfade {
    let resolved_duration = (0.0_f64).max(duration);
    let curve = (opts.as_ref().and_then(|value| (value.curve).clone()))
        .clone()
        .unwrap_or(std::sync::Arc::new(std::sync::Mutex::new(Box::new(
            move |__flight_argument_0: f64| -> f64 {
                linear_animation_crossfade_curve(__flight_argument_0)
            },
        )
            as Box<dyn FnMut(f64) -> f64 + Send + 'static>)));
    let channels = create_animation_crossfade_channels(from, to);
    let mut sample_width = 0.0_f64;
    for entry in (channels).iter().cloned() {
        sample_width = (sample_width).max(entry.channel.track.components);
    }
    return create_entity(Some(AnimationCrossfade {
        __flight_identity: std::sync::Arc::new(()),
        __flight_entity_runtime: Default::default(),
        channels: (channels).clone(),
        curve: (curve).clone(),
        duration: resolved_duration,
        elapsed: 0.0_f64,
        from: (*from).clone(),
        from_sample: vec![0.0_f32; (sample_width) as usize],
        to: (*to).clone(),
        to_sample: vec![0.0_f32; (sample_width) as usize],
        weight: {
            let __flight_callback = (curve).clone();
            let __flight_result = __flight_callback.lock().unwrap()(
                get_linear_animation_crossfade_weight(0.0_f64, resolved_duration),
            );
            __flight_result
        },
    }));
}

// Source: upstream/packages/animation/src/animationCrossfade.ts:53 (sha256:1cc6c3a8e7af7eb03fb2c50bc073dba3cb7caef4f59066034d45ba59983a63ae)
pub fn is_animation_crossfade_complete(state: &AnimationCrossfade) -> bool {
    return (state.duration <= 0.0_f64) || (state.elapsed >= state.duration);
}

// Source: upstream/packages/animation/src/animationCrossfade.ts:61 (sha256:b80e57755dbcd922f3c0b01373b050909b75eb013bc4b2090181075d545adde4)
pub fn sample_animation_crossfade(
    out: &mut crate::FlightUnion2<Vec<f64>, Vec<f32>>,
    state: &mut AnimationCrossfade,
    visit: &mut impl FnMut(crate::FlightUnion2<Vec<f64>, Vec<f32>>, AnimationChannel, f64) -> (),
) -> () {
    {
        let mut index = 0.0_f64;
        while (index < (state.channels.len() as f64)) {
            let entry = state.channels[index as usize].clone();
            if (entry.from_index).is_none() {
                sample_animation_track(
                    &((*out).clone()),
                    &mut state.to.clip.channels[entry.to_index as usize].track,
                    state.to.time,
                );
            } else {
                if (entry.to_index).is_none() {
                    sample_animation_track(
                        &((*out).clone()),
                        &mut state.from.clip.channels[entry.from_index as usize].track,
                        state.from.time,
                    );
                } else {
                    {
                        let __flight_argument_2 = state.from.time;
                        let __flight_result = sample_animation_track(
                            &(crate::FlightUnion2::<Vec<f64>, Vec<f32>>::B(
                                (state.from_sample).clone(),
                            )),
                            &mut state.from.clip.channels[entry.from_index as usize].track,
                            __flight_argument_2,
                        );
                        __flight_result
                    };
                    {
                        let __flight_argument_2 = state.to.time;
                        let __flight_result = sample_animation_track(
                            &(crate::FlightUnion2::<Vec<f64>, Vec<f32>>::B(
                                (state.to_sample).clone(),
                            )),
                            &mut state.to.clip.channels[entry.to_index as usize].track,
                            __flight_argument_2,
                        );
                        __flight_result
                    };
                    {
                        let __flight_argument_1 = (state.from_sample).clone();
                        let __flight_argument_3 = state.weight;
                        let __flight_result = blend_animation_samples(
                            &((*out).clone()),
                            &__flight_argument_1,
                            &mut state.to_sample,
                            __flight_argument_3,
                            Some(
                                state.from.clip.channels[entry.from_index as usize]
                                    .track
                                    .quaternion,
                            ),
                        );
                        __flight_result
                    };
                }
            }
            visit((*out).clone(), (entry.channel).clone(), index);
            {
                index += 1.0;
                index
            };
        }
    }
}

// Source: upstream/packages/animation/src/animationCrossfade.ts:85 (sha256:09d6eda0048c2584bcddb4211e8d2528df171bd885b87918b6ddd38bbe6af0e3)
fn create_animation_crossfade_channels(
    from: &AnimationPlayer,
    to: &AnimationPlayer,
) -> Vec<AnimationCrossfadeChannel> {
    assert_unique_animation_crossfade_targets(&from.clip.channels, "source".to_owned());
    assert_unique_animation_crossfade_targets(&to.clip.channels, "destination".to_owned());
    let mut to_by_target: Vec<(crate::FlightValue, f64)> = Vec::new();
    {
        let mut index = 0.0_f64;
        while (index < (to.clip.channels.len() as f64)) {
            if (!to_by_target.iter().any(|(entry_key, _)| {
                entry_key == &(to.clip.channels[index as usize].target_ref).clone()
            })) {
                {
                    let __flight_key = (to.clip.channels[index as usize].target_ref).clone();
                    let __flight_value = index;
                    if let Some((_, value)) = to_by_target
                        .iter_mut()
                        .find(|(key, _)| key == &__flight_key)
                    {
                        *value = __flight_value;
                    } else {
                        to_by_target.push((__flight_key, __flight_value));
                    }
                };
            }
            {
                index += 1.0;
                index
            };
        }
    }
    let mut channels: Vec<AnimationCrossfadeChannel> = vec![];
    let mut matched_to: Vec<f64> = Vec::new();
    {
        let mut from_index = 0.0_f64;
        while (from_index < (from.clip.channels.len() as f64)) {
            let from_channel = from.clip.channels[from_index as usize].clone();
            let mut to_index = to_by_target
                .iter()
                .find(|(entry_key, _)| entry_key == &(from_channel.target_ref).clone())
                .map(|(_, value)| value.clone());
            if (to_index).is_none() {
                channels.push(AnimationCrossfadeChannel {
                    __flight_identity: std::sync::Arc::new(()),
                    channel: (from_channel).clone(),
                    from_index: Some(from_index),
                    to_index: None,
                });
                {
                    from_index += 1.0;
                    from_index
                };
                continue;
            }
            let to_channel = to.clip.channels[to_index as usize].clone();
            if (from_channel.track.components != to_channel.track.components) {
                panic!("{}", "generated Flight function threw");
            }
            if (from_channel.track.quaternion != to_channel.track.quaternion) {
                panic!("{}", "generated Flight function threw");
            }
            channels.push(AnimationCrossfadeChannel {
                __flight_identity: std::sync::Arc::new(()),
                channel: (to_channel).clone(),
                from_index: Some(from_index),
                to_index: to_index,
            });
            {
                let __flight_value = (to_index).clone().unwrap();
                if !matched_to.contains(&__flight_value) {
                    matched_to.push(__flight_value);
                }
            };
            {
                from_index += 1.0;
                from_index
            };
        }
    }
    {
        let mut to_index = 0.0_f64;
        while (to_index < (to.clip.channels.len() as f64)) {
            if (!matched_to.iter().any(|item| item == &to_index)) {
                channels.push(AnimationCrossfadeChannel {
                    __flight_identity: std::sync::Arc::new(()),
                    channel: to.clip.channels[to_index as usize].clone(),
                    from_index: None,
                    to_index: Some(to_index),
                });
            }
            {
                to_index += 1.0;
                to_index
            };
        }
    }
    return channels;
}

// Source: upstream/packages/animation/src/animationCrossfade.ts:127 (sha256:8a85dc44f5ef218e53c39773f4859c524e4583a272c8ea3cf9dd947b27203b80)
fn assert_unique_animation_crossfade_targets(
    channels: &Vec<AnimationChannel>,
    clip_label: String,
) -> () {
    let mut targets: Vec<crate::FlightValue> = Vec::new();
    for channel in (channels).iter().cloned() {
        if targets
            .iter()
            .any(|item| item == &(channel.target_ref).clone())
        {
            panic!("{}", "generated Flight function threw");
        }
        {
            let __flight_value = (channel.target_ref).clone();
            if !targets.contains(&__flight_value) {
                targets.push(__flight_value);
            }
        };
    }
}

// Source: upstream/packages/animation/src/animationCrossfade.ts:140 (sha256:cc21114d1daf99a9141591bcfbf2bd545e76599e4260fb7c34e0f72c48132810)
fn get_linear_animation_crossfade_weight(elapsed: f64, duration: f64) -> f64 {
    if (duration <= 0.0_f64) {
        return 1.0_f64;
    }
    let normalized = (elapsed / duration);
    return if (normalized < 0.0_f64) {
        0.0_f64
    } else {
        if (normalized > 1.0_f64) {
            1.0_f64
        } else {
            normalized
        }
    };
}

// Source: upstream/packages/animation/src/animationCrossfade.ts:146 (sha256:b95a87f91beebcc0f89d0cb4fcc969cb7474c8f60c9a3d3306a9efe7a79a6de0)
fn linear_animation_crossfade_curve(t: f64) -> f64 {
    return t;
}
