// @generated from upstream/packages/types/src/ParticleSerializeResult.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/ParticleSerializeResult.ts:8 (sha256:3106ffbeeb982c4e55f8014c33ae2336824afe6d8278d87a6769f55b1db838ed)
#[derive(Clone, Default)]
pub struct ParticleSerializeResult {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub text: String,
    pub warnings: Vec<String>,
}
impl PartialEq for ParticleSerializeResult {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
