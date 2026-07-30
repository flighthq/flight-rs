// @generated from upstream/packages/types/src/AccessibilityDescriptor.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/AccessibilityDescriptor.ts:1 (sha256:496abee7f03f4315f5dc12ff7b8351e29ff07cfb093d16334f0925fce9b88b93)
#[derive(Clone, Default)]
pub struct AccessibilityDescriptor {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub label: Option<String>,
    pub role: Option<String>,
    pub tab_focusable: Option<bool>,
}
impl PartialEq for AccessibilityDescriptor {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
