// @generated from upstream/packages/types/src/HitTestFunction.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::NodeAny;

// Source: upstream/packages/types/src/HitTestFunction.ts:7 (sha256:d390784b46eb5991c010fafe5999442a2def08711412f2eb30bcaa67d9d2f934)
pub type HitTestFunction =
    std::sync::Arc<dyn Fn(NodeAny, f64, f64) -> bool + Send + Sync + 'static>;

// Source: upstream/packages/types/src/HitTestFunction.ts:15 (sha256:0b3e703677958e18c7549a5309c81c86696716b7f7bd1f91a8e5f63d030bcefa)
pub type HitTestPreciseFunction =
    std::sync::Arc<dyn Fn(NodeAny, f64, f64) -> f64 + Send + Sync + 'static>;
