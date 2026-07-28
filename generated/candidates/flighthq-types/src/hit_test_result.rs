// @generated from upstream/packages/types/src/HitTestResult.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::NodeAny;

// Source: upstream/packages/types/src/HitTestResult.ts:7 (sha256:8c306f73c07c7e0446d168a5392ac66020e80454ef17cc57d870a516f58de455)
#[derive(Clone)]
pub struct HitTestResult {
    pub local_x: f64,
    pub local_y: f64,
    pub node: NodeAny,
    pub sub_index: f64,
}
