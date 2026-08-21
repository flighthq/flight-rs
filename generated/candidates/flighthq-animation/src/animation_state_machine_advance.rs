// @generated from upstream/packages/animation/src/animationStateMachineAdvance.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::advance_animation_players;
use flighthq_types::{AnimationPlayer, AnimationStateMachine};

// Source: upstream/packages/animation/src/animationStateMachineAdvance.ts:6 (sha256:3ff7a62f02a7eb577d46cdb100440d53b7db168076a70c089c1551321e419123)
pub fn advance_animation_state_machine_with_scratch(
    machine: &mut AnimationStateMachine,
    dt: f64,
    advanced: &mut Vec<AnimationPlayer>,
) -> () {
    advance_animation_players(
        &machine.states[machine.current_state_index as usize]
            .blend_tree
            .players,
        dt,
        advanced,
    );
    let to_index = machine.transition_to_state_index;
    if (to_index).is_none() {
        return;
    }
    advance_animation_players(
        &machine.states[*(to_index.as_ref().unwrap()) as usize]
            .blend_tree
            .players,
        dt,
        advanced,
    );
    machine.transition_elapsed += dt;
    machine.transition_weight = {
        let __flight_callback = (machine.transition_curve).clone();
        let __flight_result = __flight_callback.lock().unwrap()(
            get_linear_animation_state_machine_transition_weight(
                machine.transition_elapsed,
                machine.transition_duration,
            ),
        );
        __flight_result
    };
    if (machine.transition_duration <= 0.0_f64)
        || (machine.transition_elapsed >= machine.transition_duration)
    {
        machine.current_state_index = *(to_index.as_ref().unwrap());
        machine.transition_from_state_index = None;
        machine.transition_to_state_index = None;
    }
}

// Source: upstream/packages/animation/src/animationStateMachineAdvance.ts:26 (sha256:b1a9aceaa22994c623d57f431df632361fb95b43e591a826f306b80c8b601bce)
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
