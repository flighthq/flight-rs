// @generated from upstream/packages/types/src/SpritesheetAnimation.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::SpritesheetAnimationDirection;

// Source: upstream/packages/types/src/SpritesheetAnimation.ts:4 (sha256:8ac0c9d0f7a4d1503d6f2ba9bdb34706ca82ef0ab0b9c75b5d02cacf528d77e2)
#[derive(Clone)]
pub struct SpritesheetAnimation {
    pub frames: Vec<f64>,
    pub frame_duration: f64,
    pub frame_durations: Option<Vec<f64>>,
    pub direction: SpritesheetAnimationDirection,
    pub loop_: bool,
    pub origin_x: f64,
    pub origin_y: f64,
}
