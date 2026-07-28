// @generated from upstream/packages/types/src/TextureContainerLevel.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/TextureContainerLevel.ts:9 (sha256:22098c0143137cb3701785c749ec0e7c11469f7bd572f09d9e5d884bb1662a2a)
#[derive(Clone)]
pub struct TextureContainerLevel {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub byte_offset: f64,
    pub byte_length: f64,
    pub width: f64,
    pub height: f64,
}
impl PartialEq for TextureContainerLevel {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
