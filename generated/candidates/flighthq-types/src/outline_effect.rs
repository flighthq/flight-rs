// @generated from upstream/packages/types/src/OutlineEffect.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/OutlineEffect.ts:3 (sha256:e972992e03ced9fb69d2b4ba2fce5bf11410e5c020156d747ef06554c1c3cb36)
#[derive(Clone)]
pub struct OutlineEffect {
    pub kind: String,
    pub threshold: Option<f64>,
    pub thickness: Option<f64>,
    pub color: Option<f64>,
}
