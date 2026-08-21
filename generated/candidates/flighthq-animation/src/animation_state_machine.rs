// @generated from upstream/packages/animation/src/animationStateMachine.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{
    advance_animation_state_machine_with_scratch, blend_animation_samples,
    sample_animation_blend_tree_channel,
};
use flighthq_entity::create_entity;
use flighthq_types::{
    AnimationBlendTree, AnimationChannel, AnimationStateMachine, AnimationStateMachineChannel,
    AnimationStateMachineState, EasingFunction,
};

// Source: upstream/packages/animation/src/animationStateMachine.ts:18 (sha256:84251c436fde0aee9a986b6eb18556691b1f2da377d728687955a06249bfe046)
pub fn advance_animation_state_machine(machine: &mut AnimationStateMachine, dt: f64) -> () {
    machine.advance_scratch.clear();
    advance_animation_state_machine_with_scratch(machine, dt, &mut machine.advance_scratch);
}

// Source: upstream/packages/animation/src/animationStateMachine.ts:26 (sha256:01ee69c81534b0afd7feef68ff01e03034870411d950b4f9d55d774ab34a3eec)
pub fn create_animation_state_machine(
    states: &Vec<AnimationStateMachineState>,
    initial_state: Option<crate::FlightUnion2<String, f64>>,
) -> AnimationStateMachine {
    let initial_state = initial_state.unwrap_or(crate::FlightUnion2::<String, f64>::B(0.0_f64));
    if ((states.len() as f64) == 0.0_f64) {
        panic!("{}", "generated Flight function threw");
    }
    let copied_states = (states).clone();
    let mut state_by_name: Vec<(String, f64)> = Vec::new();
    {
        let mut index = 0.0_f64;
        while (index < (copied_states.len() as f64)) {
            let name = (copied_states[index as usize].name).clone();
            if state_by_name
                .iter()
                .any(|(entry_key, _)| entry_key == &(name).clone())
            {
                panic!("{}", "generated Flight function threw");
            }
            {
                let __flight_key = (name).clone();
                let __flight_value = index;
                if let Some((_, value)) = state_by_name
                    .iter_mut()
                    .find(|(key, _)| key == &__flight_key)
                {
                    *value = __flight_value;
                } else {
                    state_by_name.push((__flight_key, __flight_value));
                }
            };
            {
                index += 1.0;
                index
            };
        }
    }
    let initial_state_index = if ((match &((initial_state).clone()) {
        crate::FlightUnion2::A(_) => "string",
        crate::FlightUnion2::B(value) => "number",
    })
    .to_owned()
        == "number")
    {
        match (initial_state).clone() {
            crate::FlightUnion2::A(_) => panic!("TypeScript union narrowing failed"),
            crate::FlightUnion2::B(value) => value,
        }
    } else {
        (state_by_name
            .iter()
            .find(|(entry_key, _)| {
                entry_key
                    == &match (initial_state).clone() {
                        crate::FlightUnion2::A(value) => value,
                        crate::FlightUnion2::B(_) => panic!("TypeScript union narrowing failed"),
                    }
            })
            .map(|(_, value)| value.clone()))
        .clone()
        .unwrap_or((copied_states.len() as f64))
    };
    if ((!(initial_state_index).is_finite() && (initial_state_index).fract() == 0.0_f64)
        || (initial_state_index < 0.0_f64))
        || (initial_state_index >= (copied_states.len() as f64))
    {
        panic!("{}", "generated Flight function threw");
    }
    let channels = create_animation_state_machine_channels(&copied_states);
    let mut sample_width = 0.0_f64;
    for entry in (channels).iter().cloned() {
        sample_width = (sample_width).max(entry.channel.track.components);
    }
    return create_entity(Some(AnimationStateMachine {
        __flight_identity: std::sync::Arc::new(()),
        __flight_entity_snapshot: Default::default(),
        __flight_entity_runtime: Default::default(),
        advance_scratch: vec![],
        channels: (channels).clone(),
        current_state_index: initial_state_index,
        from_sample: vec![0.0_f32; (sample_width) as usize],
        states: (copied_states).clone(),
        to_sample: vec![0.0_f32; (sample_width) as usize],
        transition_curve: std::sync::Arc::new(std::sync::Mutex::new(Box::new(
            move |__flight_argument_0: f64| -> f64 {
                linear_animation_state_machine_curve(__flight_argument_0)
            },
        )
            as Box<dyn FnMut(f64) -> f64 + Send + 'static>)),
        transition_duration: 0.0_f64,
        transition_elapsed: 0.0_f64,
        transition_from_state_index: None,
        transition_to_state_index: None,
        transition_weight: 0.0_f64,
    }));
}

// Source: upstream/packages/animation/src/animationStateMachine.ts:64 (sha256:b9918e8c52bdf40ff213b90c1e582ba02e08f9db922cdf69092aa6964a10f437)
pub fn create_animation_state_machine_state(
    name: String,
    blend_tree: &AnimationBlendTree,
) -> AnimationStateMachineState {
    return create_entity(Some(AnimationStateMachineState {
        __flight_identity: std::sync::Arc::new(()),
        __flight_entity_snapshot: Default::default(),
        __flight_entity_runtime: Default::default(),
        blend_tree: (*blend_tree).clone(),
        name: (name).clone(),
    }));
}

// Source: upstream/packages/animation/src/animationStateMachine.ts:72 (sha256:dbc286556ce5e96a4d70a9c18addb8ff0ec00eb27ffcf5e3a54f6af57ab7ee18)
pub fn get_animation_state_machine_current_state(
    machine: &AnimationStateMachine,
) -> AnimationStateMachineState {
    return machine.states[machine.current_state_index as usize].clone();
}

// Source: upstream/packages/animation/src/animationStateMachine.ts:79 (sha256:934923e4fada469f931f9b3be90fa4ddb20b12e7acf9599071f07e053de4009d)
pub fn is_animation_state_machine_transitioning(machine: &AnimationStateMachine) -> bool {
    return (machine.transition_to_state_index).is_some();
}

// Source: upstream/packages/animation/src/animationStateMachine.ts:85 (sha256:23b3f7192b54924e7720e6b401570e8c0e7d5bc81b6179db95e5717faa785a2e)
pub fn sample_animation_state_machine(
    out: &mut crate::FlightUnion2<Vec<f64>, Vec<f32>>,
    machine: &mut AnimationStateMachine,
    visit: &mut impl FnMut(crate::FlightUnion2<Vec<f64>, Vec<f32>>, AnimationChannel, f64) -> (),
) -> () {
    {
        let mut index = 0.0_f64;
        while (index < (machine.channels.len() as f64)) {
            if sample_animation_state_machine_channel(out, machine, index) {
                visit(
                    (*out).clone(),
                    (machine.channels[index as usize].channel).clone(),
                    index,
                );
            }
            {
                index += 1.0;
                index
            };
        }
    }
}

// Source: upstream/packages/animation/src/animationStateMachine.ts:98 (sha256:e6abecb60b719dbc044a6508b24692da8be3652d387fd3fd5dcd88a4e08cefcf)
pub fn sample_animation_state_machine_channel(
    out: &mut crate::FlightUnion2<Vec<f64>, Vec<f32>>,
    machine: &mut AnimationStateMachine,
    channel_index: f64,
) -> bool {
    let entry: Option<AnimationStateMachineChannel> =
        machine.channels.get(channel_index as usize).cloned();
    if (entry).is_none() {
        return false;
    }
    let to_state_index = machine.transition_to_state_index;
    if (to_state_index).is_none() {
        let current_channel_index: Option<f64> = entry
            .as_ref()
            .unwrap()
            .state_channel_indices
            .get(machine.current_state_index as usize)
            .cloned()
            .flatten();
        return ((current_channel_index).is_some())
            && (sample_animation_blend_tree_channel(
                out,
                &mut machine.states[machine.current_state_index as usize].blend_tree,
                *(current_channel_index.as_ref().unwrap()),
            ));
    }
    let from_state_index = machine.transition_from_state_index;
    let from_channel_index: Option<f64> = entry
        .as_ref()
        .unwrap()
        .state_channel_indices
        .get((from_state_index).clone().unwrap() as usize)
        .cloned()
        .flatten();
    let to_channel_index: Option<f64> = entry
        .as_ref()
        .unwrap()
        .state_channel_indices
        .get(*(to_state_index.as_ref().unwrap()) as usize)
        .cloned()
        .flatten();
    let has_from = ((from_channel_index).is_some())
        && ({
            let mut __flight_argument_0 = crate::FlightUnion2::<Vec<f64>, Vec<f32>>::B(
                std::mem::take(&mut (machine.from_sample)),
            );
            let __flight_result = sample_animation_blend_tree_channel(
                &mut __flight_argument_0,
                &mut machine.states[(from_state_index).clone().unwrap() as usize].blend_tree,
                *(from_channel_index.as_ref().unwrap()),
            );
            machine.from_sample = match __flight_argument_0 {
                crate::FlightUnion2::A(_) => panic!("TypeScript union narrowing failed"),
                crate::FlightUnion2::B(value) => value,
            };
            __flight_result
        });
    let has_to = ((to_channel_index).is_some())
        && ({
            let mut __flight_argument_0 = crate::FlightUnion2::<Vec<f64>, Vec<f32>>::B(
                std::mem::take(&mut (machine.to_sample)),
            );
            let __flight_result = sample_animation_blend_tree_channel(
                &mut __flight_argument_0,
                &mut machine.states[*(to_state_index.as_ref().unwrap()) as usize].blend_tree,
                *(to_channel_index.as_ref().unwrap()),
            );
            machine.to_sample = match __flight_argument_0 {
                crate::FlightUnion2::A(_) => panic!("TypeScript union narrowing failed"),
                crate::FlightUnion2::B(value) => value,
            };
            __flight_result
        });
    if (!has_from) && (!has_to) {
        return false;
    }
    if (has_from) && (has_to) {
        blend_animation_samples(
            out,
            &(((machine.from_sample).clone())
                .iter()
                .map(|__flight_value| (*__flight_value) as f64)
                .collect::<Vec<_>>()),
            &(((machine.to_sample).clone())
                .iter()
                .map(|__flight_value| (*__flight_value) as f64)
                .collect::<Vec<_>>()),
            machine.transition_weight,
            Some(entry.as_ref().unwrap().channel.track.quaternion),
        );
    } else {
        let source = if has_from {
            (machine.from_sample).clone()
        } else {
            (machine.to_sample).clone()
        };
        let width = ((match &*(out) {
            crate::FlightUnion2::A(values) => (values.len() as f64),
            crate::FlightUnion2::B(values) => (values.len() as f64),
        })
        .min(entry.as_ref().unwrap().channel.track.components))
        .min((source.len() as f64));
        {
            let mut component = 0.0_f64;
            while (component < width) {
                {
                    let __flight_index = (component) as usize;
                    let __flight_value = (source[component as usize] as f64);
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
    return true;
}

// Source: upstream/packages/animation/src/animationStateMachine.ts:141 (sha256:cf18427cfa53b87c26277212d5482e11a20160aad438a2371a954c85ef2a97ae)
pub fn transition_animation_state_machine(
    machine: &mut AnimationStateMachine,
    to_state: &crate::FlightUnion2<String, f64>,
    duration: f64,
    curve: Option<EasingFunction>,
) -> bool {
    let curve = curve.unwrap_or(std::sync::Arc::new(std::sync::Mutex::new(Box::new(
        move |__flight_argument_0: f64| -> f64 {
            linear_animation_state_machine_curve(__flight_argument_0)
        },
    )
        as Box<dyn FnMut(f64) -> f64 + Send + 'static>)));
    if (machine.transition_to_state_index).is_some() {
        return false;
    }
    let to_state_index = find_animation_state_machine_state_index(&machine.states, to_state);
    if (to_state_index < 0.0_f64) || (to_state_index == machine.current_state_index) {
        return false;
    }
    machine.transition_curve = (curve).clone();
    machine.transition_duration = (0.0_f64).max(duration);
    machine.transition_elapsed = 0.0_f64;
    machine.transition_from_state_index = Some(machine.current_state_index);
    machine.transition_to_state_index = Some(to_state_index);
    machine.transition_weight = {
        let __flight_callback = (curve).clone();
        let __flight_result = __flight_callback.lock().unwrap()(
            get_linear_animation_state_machine_transition_weight(
                0.0_f64,
                machine.transition_duration,
            ),
        );
        __flight_result
    };
    if (machine.transition_duration == 0.0_f64) {
        machine.current_state_index = to_state_index;
        machine.transition_from_state_index = None;
        machine.transition_to_state_index = None;
    }
    return true;
}

// Source: upstream/packages/animation/src/animationStateMachine.ts:164 (sha256:08f2910203454be9fc49da3bea3ff136a28824948d749af1e3b6a41b95ffad2b)
fn assert_compatible_animation_state_machine_channels(
    existing: &AnimationChannel,
    channel: &AnimationChannel,
) -> () {
    if (existing.track.components != channel.track.components)
        || (existing.track.quaternion != channel.track.quaternion)
    {
        panic!("{}", "generated Flight function threw");
    }
}

// Source: upstream/packages/animation/src/animationStateMachine.ts:176 (sha256:3556dfa5557d192ef9e59cd381666e5b16ad1666ba171262bf1c722a84be4433)
fn create_animation_state_machine_channels(
    states: &Vec<AnimationStateMachineState>,
) -> Vec<AnimationStateMachineChannel> {
    let mut channels: Vec<AnimationStateMachineChannel> = vec![];
    let mut channel_by_target: Vec<(crate::FlightValue, f64)> = Vec::new();
    {
        let mut state_index = 0.0_f64;
        while (state_index < (states.len() as f64)) {
            {
                let mut state_channel_index = 0.0_f64;
                while (state_channel_index
                    < (states[state_index as usize].blend_tree.channels.len() as f64))
                {
                    let existing_index = channel_by_target
                        .iter()
                        .find(|(entry_key, _)| {
                            entry_key
                                == &(states[state_index as usize].blend_tree.channels
                                    [state_channel_index as usize]
                                    .channel
                                    .target_ref)
                                    .clone()
                        })
                        .map(|(_, value)| value.clone());
                    if (existing_index).is_none() {
                        let mut state_channel_indices = vec![None; (states.len() as f64) as usize];
                        {
                            let __flight_index = (state_index) as usize;
                            let __flight_value = {
                                let __flight_portable_source = state_channel_index;
                                crate::FlightValue::Number(*(&__flight_portable_source) as f64)
                            };
                            if __flight_index == state_channel_indices.len() {
                                state_channel_indices.push(__flight_value);
                            } else {
                                state_channel_indices[__flight_index] = __flight_value;
                            }
                        };
                        {
                            let __flight_key = (states[state_index as usize].blend_tree.channels
                                [state_channel_index as usize]
                                .channel
                                .target_ref)
                                .clone();
                            let __flight_value = (channels.len() as f64);
                            if let Some((_, value)) = channel_by_target
                                .iter_mut()
                                .find(|(key, _)| key == &__flight_key)
                            {
                                *value = __flight_value;
                            } else {
                                channel_by_target.push((__flight_key, __flight_value));
                            }
                        };
                        channels.push(AnimationStateMachineChannel {
                            __flight_identity: std::sync::Arc::new(()),
                            channel: (states[state_index as usize].blend_tree.channels
                                [state_channel_index as usize]
                                .channel)
                                .clone(),
                            state_channel_indices: (state_channel_indices).clone(),
                        });
                        {
                            state_channel_index += 1.0;
                            state_channel_index
                        };
                        continue;
                    }
                    let mut existing = channels[(existing_index).clone().unwrap() as usize].clone();
                    assert_compatible_animation_state_machine_channels(
                        &existing.channel,
                        &states[state_index as usize].blend_tree.channels
                            [state_channel_index as usize]
                            .channel,
                    );
                    {
                        let __flight_index = (state_index) as usize;
                        let __flight_value = Some(state_channel_index);
                        if __flight_index == (existing.state_channel_indices).clone().len() {
                            (existing.state_channel_indices)
                                .clone()
                                .push(__flight_value);
                        } else {
                            (existing.state_channel_indices).clone()[__flight_index] =
                                __flight_value;
                        }
                    };
                    {
                        state_channel_index += 1.0;
                        state_channel_index
                    };
                }
            }
            {
                state_index += 1.0;
                state_index
            };
        }
    }
    return channels;
}

// Source: upstream/packages/animation/src/animationStateMachine.ts:201 (sha256:31dc00b9b705579f6bd8ff097185c0c933fed00c4d3d15bd8c91d16ea83afa45)
fn find_animation_state_machine_state_index(
    states: &Vec<AnimationStateMachineState>,
    state: &crate::FlightUnion2<String, f64>,
) -> f64 {
    if ((match &(state) {
        crate::FlightUnion2::A(_) => "string",
        crate::FlightUnion2::B(value) => "number",
    })
    .to_owned()
        == "number")
    {
        return if (((match (*state).clone() {
            crate::FlightUnion2::A(_) => panic!("TypeScript union narrowing failed"),
            crate::FlightUnion2::B(value) => value,
        })
        .is_finite()
            && (match (*state).clone() {
                crate::FlightUnion2::A(_) => panic!("TypeScript union narrowing failed"),
                crate::FlightUnion2::B(value) => value,
            })
            .fract()
                == 0.0_f64)
            && (match (*state).clone() {
                crate::FlightUnion2::A(_) => panic!("TypeScript union narrowing failed"),
                crate::FlightUnion2::B(value) => value,
            } >= 0.0_f64))
            && (match (*state).clone() {
                crate::FlightUnion2::A(_) => panic!("TypeScript union narrowing failed"),
                crate::FlightUnion2::B(value) => value,
            } < (states.len() as f64))
        {
            match (*state).clone() {
                crate::FlightUnion2::A(_) => panic!("TypeScript union narrowing failed"),
                crate::FlightUnion2::B(value) => value,
            }
        } else {
            (-1.0_f64)
        };
    }
    {
        let mut index = 0.0_f64;
        while (index < (states.len() as f64)) {
            if ((states[index as usize].name).clone()
                == match (*state).clone() {
                    crate::FlightUnion2::A(value) => value,
                    crate::FlightUnion2::B(_) => panic!("TypeScript union narrowing failed"),
                })
            {
                return index;
            }
            {
                index += 1.0;
                index
            };
        }
    }
    return (-1.0_f64);
}

// Source: upstream/packages/animation/src/animationStateMachine.ts:212 (sha256:b1a9aceaa22994c623d57f431df632361fb95b43e591a826f306b80c8b601bce)
fn get_linear_animation_state_machine_transition_weight(elapsed: f64, duration: f64) -> f64 {
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

// Source: upstream/packages/animation/src/animationStateMachine.ts:218 (sha256:593388a69166c6ee7fc4bbcfe31dda37f8c107308c925575a99575a7b0e5b4b7)
fn linear_animation_state_machine_curve(t: f64) -> f64 {
    return t;
}
