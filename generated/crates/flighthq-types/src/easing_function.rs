// @generated from upstream/packages/types/src/EasingFunction.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/EasingFunction.ts:1 (sha256:0f259146624a35494c78ba50a67e20625a139feaeb9659f38abbb76a6c3f16ab)
pub type EasingFunction =
    std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(f64) -> f64 + Send + 'static>>>;
