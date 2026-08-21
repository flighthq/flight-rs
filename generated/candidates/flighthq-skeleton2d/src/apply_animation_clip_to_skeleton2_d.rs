// @generated from upstream/packages/skeleton2d/src/applyAnimationClipToSkeleton2D.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::get_skeleton2_d_animation_target_binder;
use flighthq_types::{AnimationClip, Skeleton2D};

// Source: upstream/packages/skeleton2d/src/applyAnimationClipToSkeleton2D.ts:28 (sha256:2b18980e1453800e2065422cce7748d76718f199edbd66cccfd5f600b8fd0613)
pub fn apply_animation_clip_to_skeleton2_d(
    clip: &AnimationClip,
    setup: &Skeleton2D,
    pose: &Skeleton2D,
    time: f64,
) -> () {
    if (setup == pose) {
        panic!(
            "{}",
            "applyAnimationClipToSkeleton2D: setup and pose must be distinct skeletons — pass a cloneSkeleton2D(setup) as pose"
        );
    }
    {
        let mut i = 0.0_f64;
        while (i < (clip.channels.len() as f64)) {
            let channel = clip.channels[i as usize].clone();
            let target = (channel.target_ref).clone();
            if (((target).clone()).is_none())
                || ((((target).clone())
                    .as_ref()
                    .map_or("undefined", |_| "object"))
                .to_owned()
                    != "object")
            {
                {
                    i += 1.0;
                    i
                };
                continue;
            }
            let bind =
                get_skeleton2_d_animation_target_binder((target.as_ref().unwrap().kind).clone());
            if (bind).is_none() {
                {
                    i += 1.0;
                    i
                };
                continue;
            }
            bind.as_ref().unwrap().lock().unwrap()(
                (channel).clone(),
                (*setup).clone(),
                (*pose).clone(),
                {
                    let __flight_portable_source = (target).clone();
                    match (&__flight_portable_source).as_ref() {
                        Some(value) => crate::FlightValue::Record({
                            let mut __flight_record = Vec::new();
                            __flight_record.push((
                                "boneIndex".to_owned(),
                                crate::FlightValue::Number(*(&((value).bone_index)) as f64),
                            ));
                            __flight_record.push((
                                "kind".to_owned(),
                                crate::FlightValue::String((&((value).kind)).clone()),
                            ));
                            __flight_record.push((
                                "path".to_owned(),
                                crate::FlightValue::String((&((value).path)).clone()),
                            ));
                            __flight_record
                        }),
                        None => crate::FlightValue::Null,
                    }
                },
                time,
            );
            {
                i += 1.0;
                i
            };
        }
    }
}
