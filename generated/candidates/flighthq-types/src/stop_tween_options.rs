// @generated from upstream/packages/types/src/StopTweenOptions.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/StopTweenOptions.ts:1 (sha256:f98aeac39ecac327c8680435b2a6ba09725750f980cd6e002b44199ac35e559e)
#[derive(Clone, Default)]
pub struct StopTweenOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub complete: Option<bool>,
    pub send_event: Option<bool>,
}
impl PartialEq for StopTweenOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
