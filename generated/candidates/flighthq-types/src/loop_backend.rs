// @generated from upstream/packages/types/src/LoopBackend.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/LoopBackend.ts:3 (sha256:2553303d0a627764378cbf08fb7a0df92efd3414472bc0605d09377d05cc640c)
#[derive(Clone)]
pub struct LoopBackend {
    pub request_frame: crate::OpaqueHostValue,
    pub cancel_frame: crate::OpaqueHostValue,
    pub now: crate::OpaqueHostValue,
}
