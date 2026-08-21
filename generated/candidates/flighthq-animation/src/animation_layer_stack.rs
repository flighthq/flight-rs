// @generated from upstream/packages/animation/src/animationLayerStack.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{
    add_animation_sample, advance_animation_players, advance_animation_state_machine_with_scratch,
    blend_animation_samples, sample_animation_blend_tree_channel,
    sample_animation_state_machine_channel,
};
use flighthq_entity::create_entity;
use flighthq_types::{
    AnimationBlendTree, AnimationChannel, AnimationLayer, AnimationLayerOptions,
    AnimationLayerStack, AnimationLayerStackChannel, AnimationLayerStackChannelSource,
    AnimationStateMachine,
};

#[derive(Clone, Default)]
pub struct SharedStructuralRecord1 {
    pub __flight_identity: std::sync::Arc<()>,
    pub channel: AnimationChannel,
}
impl PartialEq for SharedStructuralRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/animation/src/animationLayerStack.ts:20 (sha256:b37409d10efde744aadfe1477887f7bc3264a1f460ae9e786d81e31ca8034a39)
pub fn advance_animation_layer_stack(stack: &mut AnimationLayerStack, dt: f64) -> () {
    stack.advance_scratch.clear();
    for tree in ((stack.blend_trees).clone()).iter().cloned() {
        advance_animation_players(&tree.players, dt, &mut stack.advance_scratch);
    }
    for mut machine in ((stack.state_machines).clone()).iter().cloned() {
        advance_animation_state_machine_with_scratch(&mut machine, dt, &mut stack.advance_scratch);
    }
}

// Source: upstream/packages/animation/src/animationLayerStack.ts:28 (sha256:f8c3458499dfb49d6530f24bd9b5e0952ca57f0a50f95a404e7e1f65b430d12b)
pub fn create_animation_blend_tree_layer(
    blend_tree: &AnimationBlendTree,
    mut options: Option<AnimationLayerOptions>,
) -> AnimationLayer {
    return create_animation_layer(
        (blend_tree.channels.len() as f64),
        &(Some((blend_tree).clone())),
        &(None),
        ((options).clone()).clone(),
    );
}

// Source: upstream/packages/animation/src/animationLayerStack.ts:36 (sha256:5415e5393ea35d790524a47211e7679dee06f897009fab380f6e85f9b08c9549)
pub fn create_animation_layer_stack(layers: &Vec<AnimationLayer>) -> AnimationLayerStack {
    let copied_layers = (layers).clone();
    let mut blend_trees: Vec<AnimationBlendTree> = vec![];
    let mut channels: Vec<AnimationLayerStackChannel> = vec![];
    let mut channel_by_target: Vec<(crate::FlightValue, f64)> = Vec::new();
    let mut state_machines: Vec<AnimationStateMachine> = vec![];
    let mut sample_width = 0.0_f64;
    {
        let mut layer_index = 0.0_f64;
        while (layer_index < (copied_layers.len() as f64)) {
            let layer = copied_layers[layer_index as usize].clone();
            if ((layer.blend_tree).clone()).is_some() {
                if (!{
                    let __flight_value = ((layer.blend_tree).clone()).unwrap();
                    (blend_trees).iter().any(|item| item == &__flight_value)
                }) {
                    blend_trees.push(((layer.blend_tree).clone()).unwrap());
                }
            } else {
                if (!{
                    let __flight_value = ((layer.state_machine).clone()).unwrap();
                    (state_machines).iter().any(|item| item == &__flight_value)
                }) {
                    state_machines.push(((layer.state_machine).clone()).unwrap());
                }
            }
            let source_channels = get_animation_layer_channels(&layer);
            let channel_indices = ((layer.channel_indices).clone()).clone().unwrap_or(
                ((source_channels)
                    .iter()
                    .cloned()
                    .map(
                        |_: SharedStructuralRecord1, index: crate::OpaqueHostValue| -> f64 {
                            index
                        },
                    )
                    .collect::<Vec<_>>())
                .iter()
                .map(|__flight_value| *__flight_value)
                .collect::<Vec<_>>(),
            );
            for channel_index in (channel_indices).iter().cloned() {
                sample_width = (sample_width).max(
                    source_channels[channel_index as usize]
                        .channel
                        .track
                        .components,
                );
                let existing_index = channel_by_target
                    .iter()
                    .find(|(entry_key, _)| {
                        entry_key
                            == &(source_channels[channel_index as usize].channel.target_ref).clone()
                    })
                    .map(|(_, value)| value.clone());
                if (existing_index).is_none() {
                    {
                        let __flight_key =
                            (source_channels[channel_index as usize].channel.target_ref).clone();
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
                    channels.push(AnimationLayerStackChannel {
                        __flight_identity: std::sync::Arc::new(()),
                        channel: (source_channels[channel_index as usize].channel).clone(),
                        sources: vec![AnimationLayerStackChannelSource {
                            __flight_identity: std::sync::Arc::new(()),
                            channel_index: channel_index,
                            layer_index: layer_index,
                        }],
                    });
                    continue;
                }
                let mut existing = channels[(existing_index).clone().unwrap() as usize].clone();
                assert_compatible_animation_layer_channels(
                    &existing.channel,
                    &source_channels[channel_index as usize].channel,
                );
                (existing.sources)
                    .clone()
                    .push(AnimationLayerStackChannelSource {
                        __flight_identity: std::sync::Arc::new(()),
                        channel_index: channel_index,
                        layer_index: layer_index,
                    });
            }
            {
                layer_index += 1.0;
                layer_index
            };
        }
    }
    return create_entity(Some(AnimationLayerStack {
        __flight_identity: std::sync::Arc::new(()),
        __flight_entity_snapshot: Default::default(),
        __flight_entity_runtime: Default::default(),
        advance_scratch: vec![],
        blend_trees: (blend_trees).clone(),
        channels: (channels).clone(),
        layers: (copied_layers).clone(),
        sample_scratch: vec![0.0_f32; (sample_width) as usize],
        state_machines: (state_machines).clone(),
    }));
}

// Source: upstream/packages/animation/src/animationLayerStack.ts:80 (sha256:9ec26eb060258212879de0dbbc80c9a1ba12f7f7d82001a181767efecfe29da2)
pub fn create_animation_state_machine_layer(
    state_machine: &AnimationStateMachine,
    mut options: Option<AnimationLayerOptions>,
) -> AnimationLayer {
    return create_animation_layer(
        (state_machine.channels.len() as f64),
        &(None),
        &(Some((state_machine).clone())),
        ((options).clone()).clone(),
    );
}

// Source: upstream/packages/animation/src/animationLayerStack.ts:89 (sha256:5f17af696e2377eee2b6e55d3d6f0cb07c5418dff3bec2f6366078f9cd908aa9)
pub fn sample_animation_layer_stack(
    out: &mut crate::FlightUnion2<Vec<f64>, Vec<f32>>,
    stack: &mut AnimationLayerStack,
    visit: &mut impl FnMut(crate::FlightUnion2<Vec<f64>, Vec<f32>>, AnimationChannel, f64) -> (),
) -> () {
    {
        let mut index = 0.0_f64;
        while (index < (stack.channels.len() as f64)) {
            if sample_animation_layer_stack_channel(out, stack, index) {
                visit(
                    (*out).clone(),
                    (stack.channels[index as usize].channel).clone(),
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

// Source: upstream/packages/animation/src/animationLayerStack.ts:101 (sha256:92014540a71e9428ff9eb0a6e5c34940a96e719e20778cd76fed21367167a422)
pub fn sample_animation_layer_stack_channel(
    out: &mut crate::FlightUnion2<Vec<f64>, Vec<f32>>,
    stack: &mut AnimationLayerStack,
    channel_index: f64,
) -> bool {
    let entry: Option<AnimationLayerStackChannel> =
        stack.channels.get(channel_index as usize).cloned();
    if (entry).is_none() {
        return false;
    }
    let mut has_pose = false;
    for source in ((entry.as_ref().unwrap().sources).clone()).iter().cloned() {
        let mut layer = stack.layers[source.layer_index as usize].clone();
        if (!(layer.weight > 0.0_f64))
            || (!{
                let mut __flight_argument_0 = crate::FlightUnion2::<Vec<f64>, Vec<f32>>::B(
                    std::mem::take(&mut (stack.sample_scratch)),
                );
                let __flight_result = sample_animation_layer(
                    &mut __flight_argument_0,
                    &mut layer,
                    source.channel_index,
                );
                stack.sample_scratch = match __flight_argument_0 {
                    crate::FlightUnion2::A(_) => panic!("TypeScript union narrowing failed"),
                    crate::FlightUnion2::B(value) => value,
                };
                __flight_result
            })
        {
            continue;
        }
        if layer.additive {
            if (!has_pose) {
                write_animation_layer_identity(
                    out,
                    entry.as_ref().unwrap().channel.track.components,
                    entry.as_ref().unwrap().channel.track.quaternion,
                );
                has_pose = true;
            }
            {
                let __flight_argument_1 = match &((*out).clone()) {
                    crate::FlightUnion2::A(values) => values
                        .iter()
                        .map(|__flight_value| *__flight_value)
                        .collect::<Vec<_>>(),
                    crate::FlightUnion2::B(values) => values
                        .iter()
                        .map(|__flight_value| (*__flight_value) as f64)
                        .collect::<Vec<_>>(),
                };
                let __flight_result = add_animation_sample(
                    out,
                    &__flight_argument_1,
                    &(((stack.sample_scratch).clone())
                        .iter()
                        .map(|__flight_value| (*__flight_value) as f64)
                        .collect::<Vec<_>>()),
                    layer.weight,
                    Some(entry.as_ref().unwrap().channel.track.quaternion),
                );
                __flight_result
            };
        } else {
            if has_pose {
                {
                    let __flight_argument_1 = match &((*out).clone()) {
                        crate::FlightUnion2::A(values) => values
                            .iter()
                            .map(|__flight_value| *__flight_value)
                            .collect::<Vec<_>>(),
                        crate::FlightUnion2::B(values) => values
                            .iter()
                            .map(|__flight_value| (*__flight_value) as f64)
                            .collect::<Vec<_>>(),
                    };
                    let __flight_result = blend_animation_samples(
                        out,
                        &__flight_argument_1,
                        &(((stack.sample_scratch).clone())
                            .iter()
                            .map(|__flight_value| (*__flight_value) as f64)
                            .collect::<Vec<_>>()),
                        layer.weight,
                        Some(entry.as_ref().unwrap().channel.track.quaternion),
                    );
                    __flight_result
                };
            } else {
                copy_animation_layer_sample(
                    out,
                    &(((stack.sample_scratch).clone())
                        .iter()
                        .map(|__flight_value| (*__flight_value) as f64)
                        .collect::<Vec<_>>()),
                    entry.as_ref().unwrap().channel.track.components,
                );
                has_pose = true;
            }
        }
    }
    return has_pose;
}

// Source: upstream/packages/animation/src/animationLayerStack.ts:130 (sha256:27e047874fe8fcd9a7e70ca7e17d77d383ed5fe7cbde562b00e94433d7f8f70e)
pub fn set_animation_layer_weight(
    stack: &mut AnimationLayerStack,
    layer_index: f64,
    weight: f64,
) -> bool {
    let mut layer: Option<AnimationLayer> = stack.layers.get(layer_index as usize).cloned();
    if (layer).is_none() {
        return false;
    }
    layer.as_mut().unwrap().weight = weight;
    return true;
}

// Source: upstream/packages/animation/src/animationLayerStack.ts:137 (sha256:1cfe49a7775e25d87d44a59843802a9b2919e7c94e1584a4a940e8cefd0142d6)
fn assert_compatible_animation_layer_channels(
    existing: &AnimationChannel,
    channel: &AnimationChannel,
) -> () {
    if (existing.track.components != channel.track.components)
        || (existing.track.quaternion != channel.track.quaternion)
    {
        panic!("{}", "generated Flight function threw");
    }
}

// Source: upstream/packages/animation/src/animationLayerStack.ts:149 (sha256:57e0e435a0a2849b654a6a7c6d270e38ec116211894245bd13bc10c583158fc4)
fn copy_animation_layer_sample(
    out: &mut crate::FlightUnion2<Vec<f64>, Vec<f32>>,
    sample: &Vec<f64>,
    components: f64,
) -> () {
    let width = ((match &*(out) {
        crate::FlightUnion2::A(values) => (values.len() as f64),
        crate::FlightUnion2::B(values) => (values.len() as f64),
    })
    .min((sample.len() as f64)))
    .min(components);
    {
        let mut component = 0.0_f64;
        while (component < width) {
            {
                let __flight_index = (component) as usize;
                let __flight_value = sample[component as usize].clone();
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

// Source: upstream/packages/animation/src/animationLayerStack.ts:154 (sha256:cbec1f935ad56da2263d8443130c9e6881a264c14340bd9e2ea24b0ccd19f354)
fn create_animation_layer(
    channel_count: f64,
    blend_tree: &Option<AnimationBlendTree>,
    state_machine: &Option<AnimationStateMachine>,
    mut options: Option<AnimationLayerOptions>,
) -> AnimationLayer {
    return create_entity(Some(AnimationLayer {
        __flight_identity: std::sync::Arc::new(()),
        __flight_entity_snapshot: Default::default(),
        __flight_entity_runtime: Default::default(),
        additive: (options.as_ref().and_then(|value| value.additive))
            .clone()
            .unwrap_or(false),
        blend_tree: (*blend_tree).clone(),
        channel_indices: copy_animation_layer_channel_indices(
            &mut (options.as_mut().unwrap().channel_indices),
            channel_count,
        ),
        state_machine: (*state_machine).clone(),
        weight: (options.as_ref().and_then(|value| value.weight))
            .clone()
            .unwrap_or(1.0_f64),
    }));
}

// Source: upstream/packages/animation/src/animationLayerStack.ts:169 (sha256:829473a30a8df8061a7d116529f4dc104e426b4cf7e6e01c9a146755424c3a79)
fn copy_animation_layer_channel_indices(
    channel_indices: &mut Option<Vec<f64>>,
    channel_count: f64,
) -> Option<Vec<f64>> {
    if (channel_indices).is_none() {
        return None;
    }
    let copied = {
        let mut __flight_values = (channel_indices.as_mut().unwrap()).clone();
        __flight_values.sort_by(|left, right| {
            let __flight_order = (|a: f64, b: f64| -> f64 { (a - b) })(left.clone(), right.clone());
            __flight_order
                .partial_cmp(&0.0_f64)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        __flight_values
    };
    {
        let mut index = 0.0_f64;
        while (index < (copied.len() as f64)) {
            let channel_index = copied[index as usize].clone();
            if ((!(channel_index).is_finite() && (channel_index).fract() == 0.0_f64)
                || (channel_index < 0.0_f64))
                || (channel_index >= channel_count)
            {
                panic!("{}", "generated Flight function threw");
            }
            if (index > 0.0_f64) && (copied[(index - 1.0_f64) as usize].clone() == channel_index) {
                panic!("{}", "generated Flight function threw");
            }
            {
                index += 1.0;
                index
            };
        }
    }
    return Some((copied).clone());
}

// Source: upstream/packages/animation/src/animationLayerStack.ts:187 (sha256:3e3d1f4d6d559aa955c6d8ed488b29aa2a08f392b87765defcfbf3c982dc85e3)
fn get_animation_layer_channels(layer: &AnimationLayer) -> Vec<SharedStructuralRecord1> {
    return (layer
        .blend_tree
        .as_ref()
        .map(|value| (value.channels).clone()))
    .clone()
    .unwrap_or((layer.state_machine.as_ref().unwrap().channels).clone());
}

// Source: upstream/packages/animation/src/animationLayerStack.ts:193 (sha256:04696b2e850da43f7f04a3fefcd60a466116ba4c1eb695c6b86655afd6ea34ea)
fn sample_animation_layer(
    out: &mut crate::FlightUnion2<Vec<f64>, Vec<f32>>,
    layer: &mut AnimationLayer,
    channel_index: f64,
) -> bool {
    if ((layer.blend_tree).clone()).is_some() {
        return sample_animation_blend_tree_channel(
            out,
            layer.blend_tree.as_mut().unwrap(),
            channel_index,
        );
    }
    return sample_animation_state_machine_channel(
        out,
        layer.state_machine.as_mut().unwrap(),
        channel_index,
    );
}

// Source: upstream/packages/animation/src/animationLayerStack.ts:202 (sha256:bed28be4f103ff045979887ac3fcf64052585d3e1df0fdcd7fd86b4b0fbfaaf0)
fn write_animation_layer_identity(
    out: &mut crate::FlightUnion2<Vec<f64>, Vec<f32>>,
    components: f64,
    quaternion: bool,
) -> () {
    let width = (match &*(out) {
        crate::FlightUnion2::A(values) => (values.len() as f64),
        crate::FlightUnion2::B(values) => (values.len() as f64),
    })
    .min(components);
    {
        let mut component = 0.0_f64;
        while (component < width) {
            {
                let __flight_index = (component) as usize;
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
                component += 1.0;
                component
            };
        }
    }
    if (quaternion) && (width >= 4.0_f64) {
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
    }
}
