// @generated from upstream/packages/skeleton2d/src/skeleton2dAnimationTarget.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::report_skeleton2_d_coerced_interpolation;
use flighthq_animation::sample_animation_track;
use flighthq_registry::{
    create_keyed_table, get_registry_table_entry, get_registry_table_keys,
    with_registry_table_entry, without_registry_table_entry,
};
use flighthq_types::{
    AnimationChannel, Attachment2D, KeyedTable, RegistryTable,
    SKELETON2_D_ANIMATION_PATH as bone_path_constant,
    SKELETON2_D_ANIMATION_TARGET_KIND as target_kind_constant,
    SKELETON2_D_SLOT_ANIMATION_PATH as slot_path_constant, Skeleton2D, Skeleton2DAnimationPath,
    Skeleton2DAnimationTarget, Skeleton2DAnimationTargetBinder, Skeleton2DAnimationTargetKind,
    Skeleton2DSlotAnimationPath, Skeleton2DSlotAnimationTarget, Slot2D,
};

#[inline]
fn __flight_js_to_u32(value: f64) -> u32 {
    if !value.is_finite() || value == 0.0 {
        return 0;
    }
    value.trunc().rem_euclid(4294967296.0_f64) as u32
}

#[inline]
fn __flight_js_to_i32(value: f64) -> i32 {
    __flight_js_to_u32(value) as i32
}

// Source: upstream/packages/skeleton2d/src/skeleton2dAnimationTarget.ts:32 (sha256:0bca5f366159d93371f0b0fbb1dddf0e142a152940002056da332432bca6a073)
pub fn create_skeleton2_d_bone_animation_target(
    bone_index: f64,
    path: Skeleton2DAnimationPath,
) -> Skeleton2DAnimationTarget {
    return Skeleton2DAnimationTarget {
        __flight_identity: std::sync::Arc::new(()),
        bone_index: bone_index,
        kind: (target_kind_constant.bone).clone(),
        path: (path).clone(),
    };
}

// Source: upstream/packages/skeleton2d/src/skeleton2dAnimationTarget.ts:41 (sha256:be1c87d6a8eb8f42384360d3c48b4d776ffa1fc23ae9f85feacc32b14c9cace7)
pub fn create_skeleton2_d_slot_animation_target(
    slot_index: f64,
    path: Skeleton2DSlotAnimationPath,
    attachments: Option<Vec<Option<Attachment2D>>>,
) -> Skeleton2DSlotAnimationTarget {
    return Skeleton2DSlotAnimationTarget {
        __flight_identity: std::sync::Arc::new(()),
        attachments: (attachments).clone(),
        kind: (target_kind_constant.slot).clone(),
        path: (path).clone(),
        slot_index: slot_index,
    };
}

// Source: upstream/packages/skeleton2d/src/skeleton2dAnimationTarget.ts:56 (sha256:416655869a25f7d101a37d650df2d7144374fad347be39fe32e4fb6342e08d50)
pub fn find_skeleton2_d_step_keyframe(times: &Vec<f64>, time: f64) -> f64 {
    let count = (times.len() as f64);
    if (count == 0.0_f64) {
        return (-1.0_f64);
    }
    {
        let mut i = (count - 1.0_f64);
        while (i >= 0.0_f64) {
            if (times[i as usize].clone() <= time) {
                return i;
            }
            {
                i -= 1.0;
                i
            };
        }
    }
    return 0.0_f64;
}

// Source: upstream/packages/skeleton2d/src/skeleton2dAnimationTarget.ts:67 (sha256:57905e76d98b372cf63110866cbd4b0d24e42f79f20c9573f24d2edb7d91eff5)
pub fn get_skeleton2_d_animation_target_binder(
    kind: Skeleton2DAnimationTargetKind,
) -> Option<Skeleton2DAnimationTargetBinder> {
    return get_registry_table_entry(
        &flighthq_types::RegistryTable::<Skeleton2DAnimationTargetBinder>::A(
            get_skeleton2_d_animation_target_binder_registry(),
        ),
        (kind).clone(),
    );
}

// Source: upstream/packages/skeleton2d/src/skeleton2dAnimationTarget.ts:75 (sha256:bfdf68d2eeb58e06ab47ae45a4c0ba9714f8c44a0122d89ecf5a1d982dba3f10)
pub fn get_skeleton2_d_animation_target_binder_kinds() -> Vec<Skeleton2DAnimationTargetKind> {
    let mut kinds: Vec<Skeleton2DAnimationTargetKind> = vec![];
    get_registry_table_keys(
        &mut kinds,
        &get_skeleton2_d_animation_target_binder_registry(),
    );
    return kinds;
}

// Source: upstream/packages/skeleton2d/src/skeleton2dAnimationTarget.ts:85 (sha256:59e578126e7365d10ac99d7afe35b613d63f0a8c1d0ad96c7634e90c7e723fb2)
pub fn register_skeleton2_d_animation_target_binder(
    kind: Skeleton2DAnimationTargetKind,
    bind: Skeleton2DAnimationTargetBinder,
) -> () {
    (*_BINDERS.lock().unwrap()) = Some(with_registry_table_entry(
        &get_skeleton2_d_animation_target_binder_registry(),
        (kind).clone(),
        (bind).clone(),
    ));
}

// Source: upstream/packages/skeleton2d/src/skeleton2dAnimationTarget.ts:92 (sha256:e420be3cc6834dfce1e64341178d7a0fda2a02bb228093b1896f13689a32c2af)
pub fn unregister_skeleton2_d_animation_target_binder(kind: Skeleton2DAnimationTargetKind) -> () {
    (*_BINDERS.lock().unwrap()) = Some(without_registry_table_entry(
        &get_skeleton2_d_animation_target_binder_registry(),
        (kind).clone(),
    ));
}

// Source: upstream/packages/skeleton2d/src/skeleton2dAnimationTarget.ts:99 (sha256:a543bd5dd92b76001b63a41e3fd0d0880a2aeb5e8ea146bf2cbfb2b697dc586f)
fn bind_skeleton2_d_bone_channel(
    channel: &mut AnimationChannel,
    setup: &Skeleton2D,
    pose: &mut Skeleton2D,
    target: crate::FlightValue,
    time: f64,
) -> () {
    let bone_target = target;
    let bone_index = bone_target.bone_index;
    if ((bone_index < 0.0_f64) || (bone_index >= (pose.bones.len() as f64)))
        || (bone_index >= (setup.bones.len() as f64))
    {
        return;
    }
    sample_animation_track(
        &(crate::FlightUnion2::<Vec<f64>, Vec<f32>>::A((*_SCRATCH.lock().unwrap()).clone())),
        &mut channel.track,
        time,
    );
    let setup_bone = setup.bones[bone_index as usize].clone();
    let mut pose_bone = pose.bones[bone_index as usize].clone();
    {
        let __switch_value = (bone_target.path).clone();
        let __flight_case = if __switch_value == bone_path_constant.translation {
            0_usize
        } else if __switch_value == bone_path_constant.rotation {
            1_usize
        } else if __switch_value == bone_path_constant.scale {
            2_usize
        } else if __switch_value == bone_path_constant.shear {
            3_usize
        } else if __switch_value == bone_path_constant.translation_x {
            4_usize
        } else if __switch_value == bone_path_constant.translation_y {
            5_usize
        } else if __switch_value == bone_path_constant.scale_x {
            6_usize
        } else if __switch_value == bone_path_constant.scale_y {
            7_usize
        } else if __switch_value == bone_path_constant.shear_x {
            8_usize
        } else if __switch_value == bone_path_constant.shear_y {
            9_usize
        } else {
            10_usize
        };
        '__flight_switch: {
            if __flight_case <= 0_usize {
                pose_bone.x =
                    (setup_bone.x + (*_SCRATCH.lock().unwrap())[0.0_f64 as usize].clone());
                pose_bone.y =
                    (setup_bone.y + (*_SCRATCH.lock().unwrap())[1.0_f64 as usize].clone());
                break '__flight_switch;
            }
            if __flight_case <= 1_usize {
                pose_bone.rotation =
                    (setup_bone.rotation + (*_SCRATCH.lock().unwrap())[0.0_f64 as usize].clone());
                break '__flight_switch;
            }
            if __flight_case <= 2_usize {
                pose_bone.scale_x =
                    (setup_bone.scale_x * (*_SCRATCH.lock().unwrap())[0.0_f64 as usize].clone());
                pose_bone.scale_y =
                    (setup_bone.scale_y * (*_SCRATCH.lock().unwrap())[1.0_f64 as usize].clone());
                break '__flight_switch;
            }
            if __flight_case <= 3_usize {
                pose_bone.shear_x =
                    (setup_bone.shear_x + (*_SCRATCH.lock().unwrap())[0.0_f64 as usize].clone());
                pose_bone.shear_y =
                    (setup_bone.shear_y + (*_SCRATCH.lock().unwrap())[1.0_f64 as usize].clone());
                break '__flight_switch;
            }
            if __flight_case <= 4_usize {
                pose_bone.x =
                    (setup_bone.x + (*_SCRATCH.lock().unwrap())[0.0_f64 as usize].clone());
                break '__flight_switch;
            }
            if __flight_case <= 5_usize {
                pose_bone.y =
                    (setup_bone.y + (*_SCRATCH.lock().unwrap())[0.0_f64 as usize].clone());
                break '__flight_switch;
            }
            if __flight_case <= 6_usize {
                pose_bone.scale_x =
                    (setup_bone.scale_x * (*_SCRATCH.lock().unwrap())[0.0_f64 as usize].clone());
                break '__flight_switch;
            }
            if __flight_case <= 7_usize {
                pose_bone.scale_y =
                    (setup_bone.scale_y * (*_SCRATCH.lock().unwrap())[0.0_f64 as usize].clone());
                break '__flight_switch;
            }
            if __flight_case <= 8_usize {
                pose_bone.shear_x =
                    (setup_bone.shear_x + (*_SCRATCH.lock().unwrap())[0.0_f64 as usize].clone());
                break '__flight_switch;
            }
            if __flight_case <= 9_usize {
                pose_bone.shear_y =
                    (setup_bone.shear_y + (*_SCRATCH.lock().unwrap())[0.0_f64 as usize].clone());
                break '__flight_switch;
            }
            if __flight_case <= 10_usize {
                break '__flight_switch;
            }
            unreachable!("exhaustive TypeScript switch completed without exiting");
        }
    }
}

// Source: upstream/packages/skeleton2d/src/skeleton2dAnimationTarget.ts:166 (sha256:efe03c051ab4cad03e6a02b6741516473a6f911b946fcba67c3ace66246df48e)
fn bind_skeleton2_d_slot_channel(
    channel: &mut AnimationChannel,
    _setup: &Skeleton2D,
    pose: &mut Skeleton2D,
    target: crate::FlightValue,
    time: f64,
) -> () {
    let slot_target = target;
    let mut slots = (pose.slots).clone();
    if ((slots).is_none()) || ((slots).is_none()) {
        return;
    }
    let slot_index = slot_target.slot_index;
    if (("number".to_owned() != "number") || (slot_index < 0.0_f64))
        || (slot_index >= (slots.as_mut().unwrap().len() as f64))
    {
        return;
    }
    if ((slot_target.path).clone() == slot_path_constant.attachment) {
        bind_skeleton2_d_slot_attachment(
            channel,
            &mut slots.as_mut().unwrap()[slot_index as usize],
            &slot_target,
            time,
        );
        return;
    }
    if ((slot_target.path).clone() != slot_path_constant.color) {
        return;
    }
    sample_animation_track(
        &(crate::FlightUnion2::<Vec<f64>, Vec<f32>>::A((*_SCRATCH.lock().unwrap()).clone())),
        &mut channel.track,
        time,
    );
    slots.as_mut().unwrap()[slot_index as usize].color = Some(
        (__flight_js_to_u32(
            (__flight_js_to_i32(
                (__flight_js_to_i32(
                    (__flight_js_to_i32(
                        __flight_js_to_i32(clamp_color_channel(
                            (*_SCRATCH.lock().unwrap())[0.0_f64 as usize].clone(),
                        ))
                        .wrapping_shl((__flight_js_to_u32(24.0_f64) & 31))
                            as f64,
                    ) | __flight_js_to_i32(
                        __flight_js_to_i32(clamp_color_channel(
                            (*_SCRATCH.lock().unwrap())[1.0_f64 as usize].clone(),
                        ))
                        .wrapping_shl((__flight_js_to_u32(16.0_f64) & 31))
                            as f64,
                    )) as f64,
                ) | __flight_js_to_i32(
                    __flight_js_to_i32(clamp_color_channel(
                        (*_SCRATCH.lock().unwrap())[2.0_f64 as usize].clone(),
                    ))
                    .wrapping_shl((__flight_js_to_u32(8.0_f64) & 31)) as f64,
                )) as f64,
            ) | __flight_js_to_i32(clamp_color_channel(
                (*_SCRATCH.lock().unwrap())[3.0_f64 as usize].clone(),
            ))) as f64,
        ) >> (__flight_js_to_u32(0.0_f64) & 31)) as f64,
    );
}

// Source: upstream/packages/skeleton2d/src/skeleton2dAnimationTarget.ts:201 (sha256:825f2b582a6cb78cac6f5dae1fe0d0d0fcd2acfe926bf8d537c1e997237e81cd)
fn bind_skeleton2_d_slot_attachment(
    channel: &AnimationChannel,
    slot: &mut Slot2D,
    target: &Skeleton2DSlotAnimationTarget,
    time: f64,
) -> () {
    let table = (target.attachments).clone();
    if ((table).is_none()) || ((table).is_none()) {
        return;
    }
    let keyframe = find_skeleton2_d_step_keyframe(&channel.track.times, time);
    if (keyframe < 0.0_f64) {
        return;
    }
    if ((channel.track.interpolation).clone() != STEP_INTERPOLATION) {
        report_skeleton2_d_coerced_interpolation(
            "Attachment".to_owned(),
            (channel.track.interpolation).clone(),
            (STEP_INTERPOLATION).clone(),
        );
    }
    let index =
        (channel.track.values[(keyframe * channel.track.components) as usize].clone()).round();
    slot.attachment = if (index >= 0.0_f64) && (index < (table.as_ref().unwrap().len() as f64)) {
        table.as_ref().unwrap()[index as usize].clone()
    } else {
        None
    };
}

// Source: upstream/packages/skeleton2d/src/skeleton2dAnimationTarget.ts:220 (sha256:52e1796dbefc06bf13970e9481435b35904f5f0b6298dd0fcc6bd3591f103675)
fn clamp_color_channel(value: f64) -> f64 {
    return if (value <= 0.0_f64) {
        0.0_f64
    } else {
        if (value >= 1.0_f64) {
            255.0_f64
        } else {
            (value * 255.0_f64).round()
        }
    };
}

// Source: upstream/packages/skeleton2d/src/skeleton2dAnimationTarget.ts:229 (sha256:70ded29e6735c07595a1ea7169bd490b438d70b2dca86872996cfb4b27cd7317)
fn get_skeleton2_d_animation_target_binder_registry() -> KeyedTable<Skeleton2DAnimationTargetBinder>
{
    if ((*_BINDERS.lock().unwrap()).clone()).is_some() {
        return (((*_BINDERS.lock().unwrap()).as_mut().unwrap()).clone()).clone();
    }
    (*_BINDERS.lock().unwrap()) = Some(create_keyed_table(
        "Skeleton2DAnimationTargetBinder".to_owned(),
        "Unclaimed".to_owned(),
    ));
    (*_BINDERS.lock().unwrap()) = Some(with_registry_table_entry(
        (*_BINDERS.lock().unwrap()).as_ref().unwrap(),
        (target_kind_constant.bone).clone(),
        std::sync::Arc::new(std::sync::Mutex::new(Box::new(
            move |mut __flight_argument_0: AnimationChannel,
                  __flight_argument_1: Skeleton2D,
                  mut __flight_argument_2: Skeleton2D,
                  __flight_argument_3: crate::FlightValue,
                  __flight_argument_4: f64|
                  -> () {
                bind_skeleton2_d_bone_channel(
                    &mut __flight_argument_0,
                    &__flight_argument_1,
                    &mut __flight_argument_2,
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
    ));
    (*_BINDERS.lock().unwrap()) = Some(with_registry_table_entry(
        (*_BINDERS.lock().unwrap()).as_ref().unwrap(),
        (target_kind_constant.slot).clone(),
        std::sync::Arc::new(std::sync::Mutex::new(Box::new(
            move |mut __flight_argument_0: AnimationChannel,
                  __flight_argument_1: Skeleton2D,
                  mut __flight_argument_2: Skeleton2D,
                  __flight_argument_3: crate::FlightValue,
                  __flight_argument_4: f64|
                  -> () {
                bind_skeleton2_d_slot_channel(
                    &mut __flight_argument_0,
                    &__flight_argument_1,
                    &mut __flight_argument_2,
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
    ));
    return (((*_BINDERS.lock().unwrap()).clone()).clone().unwrap()).clone();
}

// Source: upstream/packages/skeleton2d/src/skeleton2dAnimationTarget.ts:237 (sha256:6c785c01525401a93b07e2eca458835fb5e53597c4d9135d4ef819b17495d04a)
static _BINDERS: std::sync::LazyLock<
    std::sync::Mutex<Option<KeyedTable<Skeleton2DAnimationTargetBinder>>>,
> = std::sync::LazyLock::new(|| std::sync::Mutex::new(None));

// Source: upstream/packages/skeleton2d/src/skeleton2dAnimationTarget.ts:238 (sha256:626320efcb0ab0ce28580ab847003b0ec637c0aee3a1a86494a0407d93144a80)
static _SCRATCH: std::sync::LazyLock<std::sync::Mutex<Vec<f64>>> =
    std::sync::LazyLock::new(|| std::sync::Mutex::new(vec![0.0_f64, 0.0_f64, 0.0_f64, 0.0_f64]));

// Source: upstream/packages/skeleton2d/src/skeleton2dAnimationTarget.ts:241 (sha256:2a65dd44fe1e532dafbbac24f32c30ec16768129d77b37d30e80ed27ad4ce3de)
const STEP_INTERPOLATION: &'static str = "Step";
