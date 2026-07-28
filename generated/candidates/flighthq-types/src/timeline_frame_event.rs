// @generated from upstream/packages/types/src/TimelineFrameEvent.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/TimelineFrameEvent.ts:4 (sha256:63a379e9ad0bff11be97df12047c205eae1ec9a90f2894a482e347ec956df8cf)
#[derive(Clone)]
pub struct TimelineFrameEvent {
    pub frame: f64,
    pub previous_frame: f64,
}
