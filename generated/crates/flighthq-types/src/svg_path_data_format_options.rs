// @generated from upstream/packages/types/src/SvgPathDataFormatOptions.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/SvgPathDataFormatOptions.ts:2 (sha256:2e93648a067876534510289f78aea171242ebe5c7f86f781b2d1a140e7530bd0)
#[derive(Clone, Default)]
pub struct SvgPathDataFormatOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub precision: Option<f64>,
}
impl PartialEq for SvgPathDataFormatOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
