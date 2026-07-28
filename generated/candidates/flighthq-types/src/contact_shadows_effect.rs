// @generated from upstream/packages/types/src/ContactShadowsEffect.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/ContactShadowsEffect.ts:2 (sha256:18141af08abdac885e63bf9981ac65735c9d041498d778ad59ee7191837ee6f7)
#[derive(Clone)]
pub struct ContactShadowsEffect {
    pub kind: String,
    pub distance: Option<f64>,
    pub opacity: Option<f64>,
    pub samples: Option<f64>,
    pub smoothness: Option<f64>,
}
