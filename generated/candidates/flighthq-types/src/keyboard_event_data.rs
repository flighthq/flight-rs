// @generated from upstream/packages/types/src/KeyboardEventData.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/KeyboardEventData.ts:1 (sha256:31ee934c70dc671de1fcf994c61ced46730f1b001bc666d65bcd71240f0101a3)
#[derive(Clone)]
pub struct KeyboardEventData {
    pub alt_key: bool,
    pub ctrl_key: bool,
    pub key: String,
    pub key_code: f64,
    pub meta_key: bool,
    pub shift_key: bool,
}
