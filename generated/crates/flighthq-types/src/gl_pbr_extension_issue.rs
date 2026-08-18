// @generated from upstream/packages/types/src/GlPbrExtensionIssue.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::Kind;

// Source: upstream/packages/types/src/GlPbrExtensionIssue.ts:3 (sha256:ce096ddc188be6058ecb328af7f2d11b6c995711c35e27d39e079f11ac7aa946)
pub type GlPbrExtensionIssueCode = String;

// Source: upstream/packages/types/src/GlPbrExtensionIssue.ts:11 (sha256:b08094a0064e3ffcb3e407556f0e6fa471eba41d1e05fccadac8f5c45ba0e909)
#[derive(Clone, Default)]
pub struct GlPbrExtensionIssue {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub code: GlPbrExtensionIssueCode,
    pub kind: Kind,
}
impl PartialEq for GlPbrExtensionIssue {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
