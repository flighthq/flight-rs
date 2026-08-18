// @generated from upstream/packages/types/src/OrthographicProjectionOptions.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/OrthographicProjectionOptions.ts:2 (sha256:10fca9c7533cef2be0a3c068ffafb9e851966a47e21568c1077b91aa7c8f7e81)
#[derive(Clone, Default)]
pub struct OrthographicProjectionOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub half_height: f64,
    pub half_width: f64,
}
impl PartialEq for OrthographicProjectionOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
