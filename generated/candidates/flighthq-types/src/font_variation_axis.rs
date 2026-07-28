// @generated from upstream/packages/types/src/FontVariationAxis.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/FontVariationAxis.ts:1 (sha256:a1735be007d708fbe1d10bd68b92b3b1964fa11c1037702659b831cd18b3321a)
#[derive(Clone, Default)]
pub struct FontVariationAxis {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub default_value: f64,
    pub max: f64,
    pub min: f64,
    pub name: String,
    pub tag: String,
}
impl PartialEq for FontVariationAxis {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
