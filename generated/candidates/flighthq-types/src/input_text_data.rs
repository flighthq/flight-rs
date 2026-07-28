// @generated from upstream/packages/types/src/InputTextData.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/InputTextData.ts:1 (sha256:ccd247bbdcefcf45f723d0a33eeb2d3cc4c690b9b1802a4d99adf76644e9d73f)
#[derive(Clone)]
pub struct InputTextData {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub is_composing: bool,
    pub text: String,
}
impl PartialEq for InputTextData {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
