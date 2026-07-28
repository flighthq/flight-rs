// @generated from upstream/packages/types/src/ColorTransformFunction.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/ColorTransformFunction.ts:7 (sha256:db27c64b1beadcf3dc013ae11a38d8e0345bd5fbf0b95177e4707696013f4ed8)
pub type ColorTransformFunction =
    std::sync::Arc<dyn Fn(Vec<f64>, f64, f64, f64) -> () + Send + Sync + 'static>;
