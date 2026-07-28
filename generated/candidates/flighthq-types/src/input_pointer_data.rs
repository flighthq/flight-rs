// @generated from upstream/packages/types/src/InputPointerData.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{MouseWheelMode, PointerType};

// Source: upstream/packages/types/src/InputPointerData.ts:4 (sha256:68dfff739dbd1da432c2948738490cd16465a4b8165214a711165ef6c7f52acc)
#[derive(Clone)]
pub struct InputPointerData {
    pub alt_key: bool,
    pub button: f64,
    pub buttons: f64,
    pub ctrl_key: bool,
    pub delta_x: f64,
    pub delta_y: f64,
    pub height: f64,
    pub is_primary: bool,
    pub meta_key: bool,
    pub pointer_id: f64,
    pub pointer_type: PointerType,
    pub pressure: f64,
    pub shift_key: bool,
    pub tilt_x: f64,
    pub tilt_y: f64,
    pub time_stamp: f64,
    pub twist: f64,
    pub wheel_mode: MouseWheelMode,
    pub width: f64,
    pub x: f64,
    pub y: f64,
}
