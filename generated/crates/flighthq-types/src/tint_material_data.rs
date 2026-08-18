// @generated from upstream/packages/types/src/TintMaterialData.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/TintMaterialData.ts:5 (sha256:85e5c19833edd9c741cb580573307ee617acb2647d59e7544e757f4d1b5da3bb)
#[derive(Clone, Default)]
pub struct TintMaterialData {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub tint: f64,
}
impl PartialEq for TintMaterialData {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
