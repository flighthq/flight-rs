// @generated from upstream/packages/types/src/ColorLutAdjustment.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{AdjustmentKind, ColorTransformFunction};

// Source: upstream/packages/types/src/ColorLutAdjustment.ts:9 (sha256:dcc71782e1b7f84958ad199337f7e9d218cec3e95608aa661f08ae03030bcb45)
#[derive(Clone)]
pub struct ColorLutAdjustment {
    pub kind: AdjustmentKind,
    pub transform: ColorTransformFunction,
}
