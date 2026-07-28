// @generated from upstream/packages/types/src/TextInputManager.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::RichText;

// Source: upstream/packages/types/src/TextInputManager.ts:4 (sha256:25f1c1cf0cc86e7ea162adc431abce3f681b4ece190fe96afef5380b3c890a1f)
#[derive(Clone)]
pub struct TextInputSource {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
}
impl PartialEq for TextInputSource {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/TextInputManager.ts:8 (sha256:8cc504b6ffd314ebe6b2ee4b1fbaffd18de06d27d0633a503699869b5d8a3656)
#[derive(Clone)]
pub struct TextInputManager {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub enabled: bool,
    pub focused: Option<RichText>,
}
impl PartialEq for TextInputManager {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
