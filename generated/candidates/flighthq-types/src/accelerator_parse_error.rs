// @generated from upstream/packages/types/src/AcceleratorParseError.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/AcceleratorParseError.ts:3 (sha256:353b654b412c10670deff7c560b299be80853ca9514e424354aafae28f18ebd6)
#[derive(Clone, Default)]
pub struct AcceleratorParseError {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub token: String,
    pub reason: AcceleratorParseErrorReason,
}
impl PartialEq for AcceleratorParseError {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/AcceleratorParseError.ts:9 (sha256:0431cb5daaee2a19e5b641ec1b57075797f41b9708c8ecf9c08188aff7cf0b12)
pub type AcceleratorParseErrorReason = String;
