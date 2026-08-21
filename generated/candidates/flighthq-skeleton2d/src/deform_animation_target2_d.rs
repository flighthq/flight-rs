// @generated from upstream/packages/skeleton2d/src/deformAnimationTarget2D.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{register_skeleton2_d_animation_target_binder, set_skeleton2_d_slot_deform};
use flighthq_animation::sample_animation_track;
use flighthq_types::{
    AnimationChannel,
    SKELETON2_D_ANIMATION_TARGET_KIND as skeleton2_d_animation_target_kind_constant, Skeleton2D,
};

// Source: upstream/packages/skeleton2d/src/deformAnimationTarget2D.ts:11 (sha256:d923f70c5795b0b6cfcf44b492c11849be7fe1b7cfc1d40b162ef178305a142d)
pub fn register_skeleton2_d_deform_animation_target() -> () {
    register_skeleton2_d_animation_target_binder(
        (skeleton2_d_animation_target_kind_constant.deform).clone(),
        std::sync::Arc::new(std::sync::Mutex::new(Box::new(
            move |mut __flight_argument_0: AnimationChannel,
                  __flight_argument_1: Skeleton2D,
                  mut __flight_argument_2: Skeleton2D,
                  __flight_argument_3: crate::FlightValue,
                  __flight_argument_4: f64|
                  -> () {
                bind_skeleton2_d_deform_channel(
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
    );
}

// Source: upstream/packages/skeleton2d/src/deformAnimationTarget2D.ts:25 (sha256:f8e9ca5c4ffc0c695d992ab16daec2ba3dcf36acb40d08b79dd2fbab2faa10cd)
fn bind_skeleton2_d_deform_channel(
    channel: &mut AnimationChannel,
    _setup: &Skeleton2D,
    pose: &mut Skeleton2D,
    target: crate::FlightValue,
    time: f64,
) -> () {
    let deform_target = target;
    let mut slots = (pose.slots).clone();
    if ((slots).is_none()) || ((slots).is_none()) {
        return;
    }
    let slot_index = deform_target.slot_index;
    if (("number".to_owned() != "number") || (slot_index < 0.0_f64))
        || (slot_index >= (slots.as_mut().unwrap().len() as f64))
    {
        return;
    }
    let components = channel.track.components;
    if (components <= 0.0_f64) {
        return;
    }
    if (((*_SCRATCH.lock().unwrap()).len() as f64) < components) {
        (*_SCRATCH.lock().unwrap()) = vec![0.0_f32; (components) as usize];
    }
    sample_animation_track(
        &(crate::FlightUnion2::<Vec<f64>, Vec<f32>>::B((*_SCRATCH.lock().unwrap()).clone())),
        &mut channel.track,
        time,
    );
    set_skeleton2_d_slot_deform(
        &mut slots.as_mut().unwrap()[slot_index as usize],
        &((deform_target.attachment).clone()),
        &(Some(
            (*_SCRATCH.lock().unwrap()).clone()[(0.0_f64) as usize..(components) as usize].to_vec(),
        )),
    );
}

// Source: upstream/packages/skeleton2d/src/deformAnimationTarget2D.ts:47 (sha256:58830405e6da08c10f3ad5a9b814bba22dcb001813e253b593c1b78a26c02ca0)
static _SCRATCH: std::sync::LazyLock<std::sync::Mutex<Vec<f32>>> =
    std::sync::LazyLock::new(|| std::sync::Mutex::new(vec![0.0_f32; (0.0_f64) as usize]));
