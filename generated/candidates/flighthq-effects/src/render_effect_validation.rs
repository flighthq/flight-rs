// @generated from upstream/packages/effects/src/renderEffectValidation.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::get_render_effect_inputs;
use flighthq_types::{RenderEffect, RenderEffectInput};

// Source: upstream/packages/effects/src/renderEffectValidation.ts:11 (sha256:0656abe5ea229cefb0697b182c73425a9cb39023a8197dd386929240861a68ae)
pub fn validate_render_effect_list(
    effects: &Vec<RenderEffect>,
    available: &Vec<RenderEffectInput>,
) -> Option<RenderEffectInput> {
    for effect in (effects).iter().cloned() {
        let required = get_render_effect_inputs(&effect);
        for input in (required).iter().cloned() {
            if (!{
                let __flight_value = (input).clone();
                (available).iter().any(|item| item == &__flight_value)
            }) {
                return Some((input).clone());
            }
        }
    }
    return None;
}
