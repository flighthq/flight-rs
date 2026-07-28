// @generated from upstream/packages/types/src/RandomSource.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/RandomSource.ts:1 (sha256:30844842d1fa49c534b6c19476c00f1c8b1d91c288f6ed265034b47b8b98b7b6)
pub type RandomSource = std::sync::Arc<dyn Fn() -> f64 + Send + Sync + 'static>;
