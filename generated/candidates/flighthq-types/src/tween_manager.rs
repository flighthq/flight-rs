// @generated from upstream/packages/types/src/TweenManager.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::EasingFunction;

// Source: upstream/packages/types/src/TweenManager.ts:6 (sha256:74cf1a5bc21f3d60a6b0df85b4243e9b29ee0f31fd6cbd94a3071afe1c7f3911)
#[derive(Clone)]
pub struct TweenManager {
    pub __brand: String,
    pub default_ease: EasingFunction,
    pub tweens: crate::OpaqueHostValue,
}
