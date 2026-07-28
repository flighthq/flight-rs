// @generated from upstream/packages/types/src/FocusEventData.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::NodeAny;

// Source: upstream/packages/types/src/FocusEventData.ts:13 (sha256:4023679f50e6d11b765f0721c4d05d8ce680fcc4c4ae59be897eed87facd55e0)
#[derive(Clone)]
pub struct FocusEventData {
    pub current_target: Option<NodeAny>,
    pub related_target: Option<NodeAny>,
    pub target: Option<NodeAny>,
}
