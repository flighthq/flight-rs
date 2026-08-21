// @generated from upstream/packages/skeleton2d/src/skeleton2dDrawOrderTarget.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{
    find_skeleton2_d_step_keyframe, register_skeleton2_d_animation_target_binder,
    report_skeleton2_d_coerced_interpolation, unregister_skeleton2_d_animation_target_binder,
};
use flighthq_animation::{create_animation_channel, create_animation_track};
use flighthq_node::{add_node_order_list_entry, clear_node_order_list};
use flighthq_types::{
    AnimationChannel, AnimationInterpolation, EasingFunction, Node, NodeOrderList,
    SKELETON2_D_ANIMATION_TARGET_KIND as target_kind_constant, Skeleton2D,
    Skeleton2DDrawOrderAnimationTarget, Skeleton2DDrawOrderTimeline,
};

// Source: upstream/packages/skeleton2d/src/skeleton2dDrawOrderTarget.ts:28 (sha256:838ac6003b63c6baca5b5498f8d99449ce8cc9fa1ab27c2e775978e1702b3530)
pub fn create_skeleton2_d_draw_order_animation_target(
    nodes: &Vec<Option<Node>>,
    order_list: &NodeOrderList,
) -> Skeleton2DDrawOrderAnimationTarget {
    return Skeleton2DDrawOrderAnimationTarget {
        __flight_identity: std::sync::Arc::new(()),
        kind: (target_kind_constant.draw_order).clone(),
        nodes: (*nodes).clone(),
        order_list: (*order_list).clone(),
    };
}

// Source: upstream/packages/skeleton2d/src/skeleton2dDrawOrderTarget.ts:52 (sha256:9fb2d4c8a1fd3407dd794b119b77c70a2eede6ca91ddd52464deb62c7e34444d)
#[derive(Clone, Default)]
struct OptsContextRecord6 {
    __flight_identity: std::sync::Arc<()>,
    times: Vec<f64>,
    values: Vec<f64>,
    components: Option<f64>,
    interpolation: Option<AnimationInterpolation>,
    quaternion: Option<bool>,
    easing: Option<EasingFunction>,
    segment_easings: Option<Vec<Option<EasingFunction>>>,
}
impl PartialEq for OptsContextRecord6 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn create_skeleton2_d_draw_order_channel(
    timeline: &Skeleton2DDrawOrderTimeline,
    nodes: &Vec<Option<Node>>,
    order_list: &NodeOrderList,
) -> Option<AnimationChannel> {
    let keyframes = (timeline.times.len() as f64);
    if (keyframes == 0.0_f64) {
        return None;
    }
    let slot_count = ((timeline.orderings.len() as f64) / keyframes);
    if (!(slot_count).is_finite() && (slot_count).fract() == 0.0_f64) || (slot_count == 0.0_f64) {
        return None;
    }
    return Some(create_animation_channel(
        &create_animation_track(&OptsContextRecord6 {
            __flight_identity: std::sync::Arc::new(()),
            components: Some(slot_count),
            interpolation: Some((STEP_INTERPOLATION).to_owned()),
            times: (timeline.times).clone(),
            values: (timeline.orderings).clone(),
            quaternion: None,
            easing: None,
            segment_easings: None,
        }),
        {
            let __flight_portable_source =
                create_skeleton2_d_draw_order_animation_target(nodes, order_list);
            crate::FlightValue::Record({
                let mut __flight_record = Vec::new();
                __flight_record.push((
                    "kind".to_owned(),
                    crate::FlightValue::String((&((&__flight_portable_source).kind)).clone()),
                ));
                __flight_record.push((
                    "nodes".to_owned(),
                    crate::FlightValue::Array(
                        (&((&__flight_portable_source).nodes))
                            .iter()
                            .map(|value| match (value).as_ref() {
                                Some(value) => crate::FlightValue::Record({
                                    let mut __flight_record = Vec::new();
                                    __flight_record.push((
                                        "data".to_owned(),
                                        match (&((value).data)).as_ref() {
                                            Some(value) => (value).clone(),
                                            None => crate::FlightValue::Null,
                                        },
                                    ));
                                    __flight_record.push((
                                        "enabled".to_owned(),
                                        crate::FlightValue::Bool(*(&((value).enabled))),
                                    ));
                                    __flight_record.push((
                                        "kind".to_owned(),
                                        crate::FlightValue::String((&((value).kind)).clone()),
                                    ));
                                    __flight_record.push((
                                        "name".to_owned(),
                                        match (&((value).name)).as_ref() {
                                            Some(value) => {
                                                crate::FlightValue::String((value).clone())
                                            }
                                            None => crate::FlightValue::Null,
                                        },
                                    ));
                                    __flight_record
                                }),
                                None => crate::FlightValue::Null,
                            })
                            .collect(),
                    ),
                ));
                __flight_record.push((
                    "orderList".to_owned(),
                    crate::FlightValue::Record({
                        let mut __flight_record = Vec::new();
                        __flight_record.push((
                            "entryCount".to_owned(),
                            crate::FlightValue::Number(
                                *(&((&((&__flight_portable_source).order_list)).entry_count))
                                    as f64,
                            ),
                        ));
                        __flight_record.push((
                            "nodes".to_owned(),
                            crate::FlightValue::Array(
                                (&((&((&__flight_portable_source).order_list)).nodes))
                                    .iter()
                                    .map(|value| {
                                        crate::FlightValue::Record({
                                            let mut __flight_record = Vec::new();
                                            __flight_record.push((
                                                "data".to_owned(),
                                                match (&((value).data)).as_ref() {
                                                    Some(value) => (value).clone(),
                                                    None => crate::FlightValue::Null,
                                                },
                                            ));
                                            __flight_record.push((
                                                "enabled".to_owned(),
                                                crate::FlightValue::Bool(*(&((value).enabled))),
                                            ));
                                            __flight_record.push((
                                                "kind".to_owned(),
                                                crate::FlightValue::String(
                                                    (&((value).kind)).clone(),
                                                ),
                                            ));
                                            __flight_record.push((
                                                "name".to_owned(),
                                                match (&((value).name)).as_ref() {
                                                    Some(value) => {
                                                        crate::FlightValue::String((value).clone())
                                                    }
                                                    None => crate::FlightValue::Null,
                                                },
                                            ));
                                            __flight_record
                                        })
                                    })
                                    .collect(),
                            ),
                        ));
                        __flight_record.push((
                            "sortKeys".to_owned(),
                            crate::FlightValue::Array(
                                (&((&((&__flight_portable_source).order_list)).sort_keys))
                                    .iter()
                                    .map(|value| crate::FlightValue::Number(*(value) as f64))
                                    .collect(),
                            ),
                        ));
                        __flight_record
                    }),
                ));
                __flight_record
            })
        },
    ));
}

// Source: upstream/packages/skeleton2d/src/skeleton2dDrawOrderTarget.ts:80 (sha256:b03815978b472b7b11f3b324ae1a68bfcd39d85ecc0ae7c9d65ce71632beef47)
pub fn register_skeleton2_d_draw_order_animation_binder() -> () {
    register_skeleton2_d_animation_target_binder(
        (target_kind_constant.draw_order).clone(),
        std::sync::Arc::new(std::sync::Mutex::new(Box::new(
            move |__flight_argument_0: AnimationChannel,
                  __flight_argument_1: Skeleton2D,
                  __flight_argument_2: Skeleton2D,
                  __flight_argument_3: crate::FlightValue,
                  __flight_argument_4: f64|
                  -> () {
                bind_skeleton2_d_draw_order_channel(
                    &__flight_argument_0,
                    (__flight_argument_1).clone(),
                    (__flight_argument_2).clone(),
                    (__flight_argument_3).clone(),
                    __flight_argument_4,
                )
            },
        )
            as Box<
                dyn FnMut(AnimationChannel, Skeleton2D, Skeleton2D, crate::FlightValue, f64) -> ()
                    + Send
                    + 'static,
            >)),
    );
}

// Source: upstream/packages/skeleton2d/src/skeleton2dDrawOrderTarget.ts:84 (sha256:2fdab0eda1d943cfa63db8f393ec075bbbbb81ca80a4f73bfe5781db330e5ee3)
pub fn unregister_skeleton2_d_draw_order_animation_binder() -> () {
    unregister_skeleton2_d_animation_target_binder((target_kind_constant.draw_order).clone());
}

// Source: upstream/packages/skeleton2d/src/skeleton2dDrawOrderTarget.ts:104 (sha256:08069ed460017b76fce00b728f0a60b406df17c4d49187187924ffc13bdf8df0)
fn bind_skeleton2_d_draw_order_channel(
    channel: &AnimationChannel,
    _setup: crate::FlightValue,
    _pose: crate::FlightValue,
    target: crate::FlightValue,
    time: f64,
) -> () {
    let mut draw_target = target;
    if ((((draw_target.nodes).is_none()) || ((draw_target.nodes).is_none()))
        || ((draw_target.order_list).is_none()))
        || ((draw_target.order_list).is_none())
    {
        return;
    }
    let components = channel.track.components;
    if (components == 0.0_f64) {
        return;
    }
    let keyframe = find_skeleton2_d_step_keyframe(&channel.track.times, time);
    if (keyframe < 0.0_f64) {
        return;
    }
    if ((channel.track.interpolation).clone() != STEP_INTERPOLATION) {
        report_skeleton2_d_coerced_interpolation(
            (DRAW_ORDER_SUBJECT).clone(),
            (channel.track.interpolation).clone(),
            (STEP_INTERPOLATION).clone(),
        );
    }
    clear_node_order_list(&mut draw_target.order_list);
    let base = (keyframe * components);
    let count = if (components < (draw_target.nodes.len() as f64)) {
        components
    } else {
        (draw_target.nodes.len() as f64)
    };
    {
        let mut slot = 0.0_f64;
        while (slot < count) {
            let node: Option<Node> = draw_target.nodes.get(slot as usize).cloned().flatten();
            if (((node).clone()).is_none()) || (((node).clone()).is_none()) {
                {
                    slot += 1.0;
                    slot
                };
                continue;
            }
            add_node_order_list_entry(
                &mut draw_target.order_list,
                node.as_ref().unwrap(),
                channel.track.values[(base + slot) as usize].clone(),
            );
            {
                slot += 1.0;
                slot
            };
        }
    }
}

// Source: upstream/packages/skeleton2d/src/skeleton2dDrawOrderTarget.ts:137 (sha256:8dcbc82a9e1decc3eec900c68b7df1d0dbab08ae687892fca147b49ff8e098b1)
const DRAW_ORDER_SUBJECT: &'static str = "DrawOrder";

// Source: upstream/packages/skeleton2d/src/skeleton2dDrawOrderTarget.ts:138 (sha256:2a65dd44fe1e532dafbbac24f32c30ec16768129d77b37d30e80ed27ad4ce3de)
const STEP_INTERPOLATION: &'static str = "Step";
