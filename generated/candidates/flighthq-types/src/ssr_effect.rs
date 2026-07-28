// @generated from upstream/packages/types/src/SsrEffect.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/SsrEffect.ts:3 (sha256:b8cd16061084ca5ebe1d0ff834eeec28042ac12c48f60b14478d08da81560b9a)
#[derive(Clone)]
pub struct SsrEffect {
    pub kind: String,
    pub max_distance: Option<f64>,
    pub resolution: Option<f64>,
    pub steps: Option<f64>,
}
