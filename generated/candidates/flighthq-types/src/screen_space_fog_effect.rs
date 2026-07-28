// @generated from upstream/packages/types/src/ScreenSpaceFogEffect.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/ScreenSpaceFogEffect.ts:3 (sha256:f3d3b737c900dec1e1d29f430e0c19659a1e43f9e15647bd464aa846e46bc399)
#[derive(Clone)]
pub struct ScreenSpaceFogEffect {
    pub kind: String,
    pub color: Option<f64>,
    pub near: Option<f64>,
    pub far: Option<f64>,
    pub density: Option<f64>,
}
