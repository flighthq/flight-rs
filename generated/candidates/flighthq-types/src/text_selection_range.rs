// @generated from upstream/packages/types/src/TextSelectionRange.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/TextSelectionRange.ts:1 (sha256:96078cda6e89736707ddc7b1e7a04db8aa77d347428394e0b6fe493a1891e353)
#[derive(Clone)]
pub struct TextSelectionRange {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub length: f64,
    pub start: f64,
    pub text: String,
}
impl PartialEq for TextSelectionRange {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
