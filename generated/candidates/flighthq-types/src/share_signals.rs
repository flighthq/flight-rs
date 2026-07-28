// @generated from upstream/packages/types/src/ShareSignals.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::Signal;

// Source: upstream/packages/types/src/ShareSignals.ts:6 (sha256:7522c9fddb92d36d1848e1facdffa80f784d41eb67a492b0b83c738bc4dadc61)
#[derive(Clone)]
pub struct ShareSignals {
    pub on_share_result: Signal,
}
