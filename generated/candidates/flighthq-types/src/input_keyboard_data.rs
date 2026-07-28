// @generated from upstream/packages/types/src/InputKeyboardData.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/InputKeyboardData.ts:1 (sha256:771b0863ccf5de23a04937149a041c06baa00c7f1fdc857df31c9928a0953f0d)
#[derive(Clone)]
pub struct InputKeyboardData {
    pub alt_key: bool,
    pub caps_lock: bool,
    pub code: String,
    pub ctrl_key: bool,
    pub key: String,
    pub key_code: f64,
    pub location: f64,
    pub meta_key: bool,
    pub modifier: f64,
    pub num_lock: bool,
    pub repeat: bool,
    pub shift_key: bool,
    pub time_stamp: f64,
}
