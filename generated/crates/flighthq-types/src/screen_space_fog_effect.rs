// @generated from upstream/packages/types/src/ScreenSpaceFogEffect.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::Kind;

// Source: upstream/packages/types/src/ScreenSpaceFogEffect.ts:3 (sha256:f3d3b737c900dec1e1d29f430e0c19659a1e43f9e15647bd464aa846e46bc399)
#[derive(Clone, Default)]
pub struct ScreenSpaceFogEffect {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub kind: Kind,
    pub color: Option<f64>,
    pub near: Option<f64>,
    pub far: Option<f64>,
    pub density: Option<f64>,
}
impl PartialEq for ScreenSpaceFogEffect {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
