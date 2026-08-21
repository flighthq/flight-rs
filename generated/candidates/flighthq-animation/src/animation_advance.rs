// @generated from upstream/packages/animation/src/animationAdvance.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::advance_animation_player;
use flighthq_types::AnimationPlayer;

// Source: upstream/packages/animation/src/animationAdvance.ts:7 (sha256:96f43d850de3e7322ce39ae39e4f98a284eb3d3fae7149c65f0d5e1f5341b209)
pub fn advance_animation_players(
    players: &Vec<AnimationPlayer>,
    dt: f64,
    advanced: &mut Vec<AnimationPlayer>,
) -> () {
    for mut player in (players).iter().cloned() {
        if {
            let __flight_value = (player).clone();
            (advanced).iter().any(|item| item == &__flight_value)
        } {
            continue;
        }
        advanced.push(((player).clone()).clone());
        advance_animation_player(&mut player, dt);
    }
}
