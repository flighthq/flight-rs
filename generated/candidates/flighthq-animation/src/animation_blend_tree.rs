// @generated from upstream/packages/animation/src/animationBlendTree.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{
    accumulate_animation_sample, add_animation_sample, advance_animation_player,
    create_animation_sample_accumulator, finish_animation_sample,
    reset_animation_sample_accumulator, sample_animation_track,
};
use flighthq_entity::create_entity;
use flighthq_types::{
    AnimationBlendTree, AnimationBlendTreeChannel, AnimationBlendTreeChannelSource,
    AnimationBlendTreeInput, AnimationChannel, AnimationPlayer,
};

// Source: upstream/packages/animation/src/animationBlendTree.ts:23 (sha256:36b06a982b16caec7d393011dbccec6678b144688b2bb4d911b197e8bec9f143)
pub fn advance_animation_blend_tree(tree: &AnimationBlendTree, dt: f64) -> () {
    for mut player in ((tree.players).clone()).iter().cloned() {
        advance_animation_player(&mut player, dt);
    }
}

// Source: upstream/packages/animation/src/animationBlendTree.ts:29 (sha256:c31fae485ace9736391f49d8c03bdd7f047dfc0c9a4c3158c64da5f8f2a0835b)
pub fn create_animation_blend_tree(inputs: &Vec<AnimationBlendTreeInput>) -> AnimationBlendTree {
    let copied_inputs = (inputs).clone();
    let mut channels: Vec<AnimationBlendTreeChannel> = vec![];
    let mut channel_by_target: Vec<(crate::FlightValue, f64)> = Vec::new();
    let mut players: Vec<AnimationPlayer> = vec![];
    let mut sample_width = 0.0_f64;
    {
        let mut input_index = 0.0_f64;
        while (input_index < (copied_inputs.len() as f64)) {
            if (!{
                let __flight_value = (copied_inputs[input_index as usize].player).clone();
                (players).iter().any(|item| item == &__flight_value)
            }) {
                players.push(((copied_inputs[input_index as usize].player).clone()).clone());
            }
            assert_unique_animation_blend_tree_targets(
                &copied_inputs[input_index as usize].player.clip.channels,
                input_index,
            );
            {
                let mut channel_index = 0.0_f64;
                while (channel_index
                    < (copied_inputs[input_index as usize]
                        .player
                        .clip
                        .channels
                        .len() as f64))
                {
                    let channel = copied_inputs[input_index as usize].player.clip.channels
                        [channel_index as usize]
                        .clone();
                    sample_width = (sample_width).max(channel.track.components);
                    let existing_index = channel_by_target
                        .iter()
                        .find(|(entry_key, _)| entry_key == &(channel.target_ref).clone())
                        .map(|(_, value)| value.clone());
                    if (existing_index).is_none() {
                        {
                            let __flight_key = (channel.target_ref).clone();
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
                        channels.push(AnimationBlendTreeChannel {
                            __flight_identity: std::sync::Arc::new(()),
                            accumulator: create_animation_sample_accumulator(
                                channel.track.components,
                                Some(channel.track.quaternion),
                            ),
                            channel: (channel).clone(),
                            sources: vec![AnimationBlendTreeChannelSource {
                                __flight_identity: std::sync::Arc::new(()),
                                channel_index: channel_index,
                                input_index: input_index,
                            }],
                        });
                        {
                            channel_index += 1.0;
                            channel_index
                        };
                        continue;
                    }
                    let mut existing = channels[existing_index as usize].clone();
                    assert_compatible_animation_blend_tree_channels(&existing.channel, &channel);
                    (existing.sources)
                        .clone()
                        .push(AnimationBlendTreeChannelSource {
                            __flight_identity: std::sync::Arc::new(()),
                            channel_index: channel_index,
                            input_index: input_index,
                        });
                    {
                        channel_index += 1.0;
                        channel_index
                    };
                }
            }
            {
                input_index += 1.0;
                input_index
            };
        }
    }
    return create_entity(Some(AnimationBlendTree {
        __flight_identity: std::sync::Arc::new(()),
        __flight_entity_runtime: Default::default(),
        channels: (channels).clone(),
        inputs: (copied_inputs).clone(),
        players: (players).clone(),
        sample_scratch: vec![0.0_f32; (sample_width) as usize],
    }));
}

// Source: upstream/packages/animation/src/animationBlendTree.ts:65 (sha256:07c65d503f288d22f81fdb74894f5b73f7175ee1e43004908882ed0cf7366840)
pub fn create_animation_blend_tree_input(
    player: &AnimationPlayer,
    weight: Option<f64>,
    additive: Option<bool>,
) -> AnimationBlendTreeInput {
    let weight = weight.unwrap_or(1.0_f64);
    let additive = additive.unwrap_or(false);
    return create_entity(Some(AnimationBlendTreeInput {
        __flight_identity: std::sync::Arc::new(()),
        __flight_entity_runtime: Default::default(),
        additive: additive,
        player: (*player).clone(),
        weight: weight,
    }));
}

// Source: upstream/packages/animation/src/animationBlendTree.ts:75 (sha256:5c3b5e356b14ec60f047d65a0a62009b2789e124fce5a8f65d04ad9a84890aaa)
pub fn sample_animation_blend_tree(
    out: &mut crate::FlightUnion2<Vec<f64>, Vec<f32>>,
    tree: &mut AnimationBlendTree,
    visit: &mut impl FnMut(crate::FlightUnion2<Vec<f64>, Vec<f32>>, AnimationChannel, f64) -> (),
) -> () {
    {
        let mut index = 0.0_f64;
        while (index < (tree.channels.len() as f64)) {
            if sample_animation_blend_tree_channel(&((*out).clone()), tree, index) {
                visit(
                    (*out).clone(),
                    (tree.channels[index as usize].channel).clone(),
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

// Source: upstream/packages/animation/src/animationBlendTree.ts:87 (sha256:f59e75b3b8605f6d90e5ed76ca4e7ec7f25a1231237c493ec936439820e5f5a1)
pub fn sample_animation_blend_tree_channel(
    out: &mut crate::FlightUnion2<Vec<f64>, Vec<f32>>,
    tree: &mut AnimationBlendTree,
    channel_index: f64,
) -> bool {
    let mut entry = tree.channels[channel_index as usize].clone();
    if (entry).is_none() {
        return false;
    }
    reset_animation_sample_accumulator(&mut entry.accumulator);
    let mut has_additive = false;
    for source in ((entry.sources).clone()).iter().cloned() {
        let mut input = tree.inputs[source.input_index as usize].clone();
        if (input.additive) || (!(input.weight > 0.0_f64)) {
            if (input.additive) && (input.weight > 0.0_f64) {
                has_additive = true;
            }
            continue;
        }
        let mut channel = input.player.clip.channels[source.channel_index as usize].clone();
        sample_animation_track(
            &(crate::FlightUnion2::<Vec<f64>, Vec<f32>>::B((tree.sample_scratch).clone())),
            &mut channel.track,
            input.player.time,
        );
        accumulate_animation_sample(&mut entry.accumulator, &tree.sample_scratch, input.weight);
    }
    let has_override = finish_animation_sample(&((*out).clone()), &entry.accumulator);
    if (!has_override) && (!has_additive) {
        return false;
    }
    if (!has_override) {
        write_animation_blend_tree_identity(
            &((*out).clone()),
            entry.channel.track.components,
            entry.channel.track.quaternion,
        );
    }
    for source in ((entry.sources).clone()).iter().cloned() {
        let mut input = tree.inputs[source.input_index as usize].clone();
        if (!input.additive) || (!(input.weight > 0.0_f64)) {
            continue;
        }
        let mut channel = input.player.clip.channels[source.channel_index as usize].clone();
        sample_animation_track(
            &(crate::FlightUnion2::<Vec<f64>, Vec<f32>>::B((tree.sample_scratch).clone())),
            &mut channel.track,
            input.player.time,
        );
        {
            let __flight_argument_1 = (out).clone();
            let __flight_result = add_animation_sample(
                &((*out).clone()),
                &__flight_argument_1,
                &mut tree.sample_scratch,
                input.weight,
                Some(channel.track.quaternion),
            );
            __flight_result
        };
    }
    return true;
}

// Source: upstream/packages/animation/src/animationBlendTree.ts:125 (sha256:3e5fa2dc9ce58f623bb981d0f5c74086e2f61dd866e9e84582ef36fe9ff0a303)
pub fn set_animation_blend_tree_input_weight(
    tree: &mut AnimationBlendTree,
    input_index: f64,
    weight: f64,
) -> bool {
    let mut input = tree.inputs[input_index as usize].clone();
    if (input).is_none() {
        return false;
    }
    input.weight = weight;
    return true;
}

// Source: upstream/packages/animation/src/animationBlendTree.ts:136 (sha256:15d61c31cb877f7d82e1ad02bfa0d9a4696330123cc52b11fb547e2b4091720c)
fn assert_compatible_animation_blend_tree_channels(
    existing: &AnimationChannel,
    channel: &AnimationChannel,
) -> () {
    if (existing.track.components != channel.track.components) {
        panic!("{}", "generated Flight function threw");
    }
    if (existing.track.quaternion != channel.track.quaternion) {
        panic!("{}", "generated Flight function threw");
    }
}

// Source: upstream/packages/animation/src/animationBlendTree.ts:150 (sha256:8227372c9115a7212bc42036b9f17127538dcbc0725b00447572fbbb646f71e2)
fn assert_unique_animation_blend_tree_targets(
    channels: &Vec<AnimationChannel>,
    input_index: f64,
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

// Source: upstream/packages/animation/src/animationBlendTree.ts:163 (sha256:c97fd559d113a157fc0670b64cf2af43fc3a6ecb0beac8ebb5850e3318ca905e)
fn write_animation_blend_tree_identity(
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
