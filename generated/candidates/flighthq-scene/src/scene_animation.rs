// @generated from upstream/packages/scene/src/sceneAnimation.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_animation::sample_animation_track;
use flighthq_geometry::{set_quaternion, set_vector3};
use flighthq_node::invalidate_node_local_transform;
use flighthq_types::{
    Adjustment, AnimationClip, ColorTransform, InteractionSignals, Node, NodeInteractionState,
    NodeSignals, NodeTraitsKey,
};

#[derive(Clone)]
pub struct FlightPartialRecord1 {
    pub __flight_identity: std::sync::Arc<()>,
    pub binding: Option<crate::OpaqueHostValue>,
    pub appearance_id: Option<f64>,
    pub bounds_using_local_bounds_id: Option<f64>,
    pub bounds_using_local_transform_id: Option<f64>,
    pub can_add_child: Option<
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(Node, Node) -> bool + Send + 'static>>>,
    >,
    pub children: Option<Vec<Node>>,
    pub color_adjustments: Option<Vec<Adjustment>>,
    pub resolved_color_transform: Option<ColorTransform>,
    pub color_adjustments_channel_mixing: Option<bool>,
    pub traits: Option<NodeTraitsKey>,
    pub interaction_signals: Option<InteractionSignals>,
    pub local_bounds_id: Option<f64>,
    pub local_bounds_using_local_bounds_id: Option<f64>,
    pub local_content_id: Option<f64>,
    pub local_transform_id: Option<f64>,
    pub local_transform_using_local_transform_id: Option<f64>,
    pub node_signals: Option<NodeSignals>,
    pub interaction_state: Option<NodeInteractionState>,
    pub parent: Option<Node>,
    pub world_bounds_using_local_bounds_id: Option<f64>,
    pub world_bounds_using_world_transform_id: Option<f64>,
    pub world_transform_id: Option<f64>,
    pub world_transform_using_local_transform_id: Option<f64>,
    pub world_transform_using_parent_transform_id: Option<f64>,
}
impl PartialEq for FlightPartialRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/scene/src/sceneAnimation.ts:15 (sha256:22c0199d511e86ff772e7fc587faf9b43383181bdebac30fd77b3a100caeb9e4)
pub fn apply_animation_clip_to_scene(clip: &mut AnimationClip, time: f64) -> () {
    {
        let mut i = 0.0_f64;
        while (i < (clip.channels.len() as f64)) {
            let mut channel = clip.channels[i as usize].clone();
            let mut target = (channel.target_ref).clone();
            if (((target).is_none()) || ("object" != "object"))
                || (((target.as_mut().unwrap().node).clone()).is_none())
            {
                {
                    i += 1.0;
                    i
                };
                continue;
            }
            if ((target.as_mut().unwrap().path).clone() == "Weights") {
                let mut morph = ((target.as_mut().unwrap().node).clone().morph).clone();
                if (morph).is_none() {
                    {
                        i += 1.0;
                        i
                    };
                    continue;
                }
                sample_animation_track(
                    &(crate::FlightUnion2::<Vec<f64>, Vec<f32>>::B(
                        (morph.as_mut().unwrap().weights).clone(),
                    )),
                    &mut channel.track,
                    time,
                );
                {
                    i += 1.0;
                    i
                };
                continue;
            }
            sample_animation_track(
                &(crate::FlightUnion2::<Vec<f64>, Vec<f32>>::A((_SCRATCH).clone())),
                &mut channel.track,
                time,
            );
            if ((target.as_mut().unwrap().path).clone() == "Translation") {
                set_vector3(
                    &mut target.as_mut().unwrap().node.position,
                    _SCRATCH[0.0_f64 as usize].clone(),
                    _SCRATCH[1.0_f64 as usize].clone(),
                    _SCRATCH[2.0_f64 as usize].clone(),
                );
            } else {
                if ((target.as_mut().unwrap().path).clone() == "Scale") {
                    set_vector3(
                        &mut target.as_mut().unwrap().node.scale,
                        _SCRATCH[0.0_f64 as usize].clone(),
                        _SCRATCH[1.0_f64 as usize].clone(),
                        _SCRATCH[2.0_f64 as usize].clone(),
                    );
                } else {
                    set_quaternion(
                        &mut target.as_mut().unwrap().node.rotation,
                        _SCRATCH[0.0_f64 as usize].clone(),
                        _SCRATCH[1.0_f64 as usize].clone(),
                        _SCRATCH[2.0_f64 as usize].clone(),
                        _SCRATCH[3.0_f64 as usize].clone(),
                    );
                }
            }
            invalidate_node_local_transform(&Node {
                __flight_identity: std::sync::Arc::clone(
                    &(target.as_mut().unwrap().node).__flight_identity,
                ),
                data: ((target.as_mut().unwrap().node).data).clone(),
                enabled: (target.as_mut().unwrap().node).enabled,
                kind: ((target.as_mut().unwrap().node).kind).clone(),
                name: ((target.as_mut().unwrap().node).name).clone(),
            });
            {
                i += 1.0;
                i
            };
        }
    }
}

// Source: upstream/packages/scene/src/sceneAnimation.ts:41 (sha256:626320efcb0ab0ce28580ab847003b0ec637c0aee3a1a86494a0407d93144a80)
static _SCRATCH: std::sync::LazyLock<std::sync::Mutex<Vec<f64>>> =
    std::sync::LazyLock::new(|| std::sync::Mutex::new(vec![0.0_f64, 0.0_f64, 0.0_f64, 0.0_f64]));
