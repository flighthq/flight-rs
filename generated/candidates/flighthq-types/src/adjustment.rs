// @generated from upstream/packages/types/src/Adjustment.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::AdjustmentKind;

// Source: upstream/packages/types/src/Adjustment.ts:14 (sha256:1673fb67d4c310dc61a8d835bd1142cba40c71b2747eb67b708747860ba8430d)
#[derive(Clone)]
pub struct Adjustment {
    pub kind: AdjustmentKind,
}
