// @generated from upstream/packages/types/src/BloomEffect.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/BloomEffect.ts:3 (sha256:575d8222621603c027271487bb284ef02d543150099fb4a29c4abc85f608d4bb)
#[derive(Clone)]
pub struct BloomEffect {
    pub kind: String,
    pub threshold: Option<f64>,
    pub intensity: Option<f64>,
    pub radius: Option<f64>,
    pub passes: Option<f64>,
}
