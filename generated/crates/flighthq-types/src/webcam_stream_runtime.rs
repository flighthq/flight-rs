// @generated from upstream/packages/types/src/WebcamStreamRuntime.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::EntityRuntime;

// Source: upstream/packages/types/src/WebcamStreamRuntime.ts:3 (sha256:0401787d3444684f1f80ff0d874b6cfb0bf8486ff91097f9e2641c10994d9f5b)
#[doc(hidden)]
pub struct WebcamStreamRuntimeStorage {
    pub binding: crate::OpaqueHostValue,
}
impl Default for WebcamStreamRuntimeStorage {
    fn default() -> Self {
        Self {
            binding: Default::default(),
        }
    }
}
pub type WebcamStreamRuntime = crate::EntityRuntime;
