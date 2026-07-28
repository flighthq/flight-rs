// @generated from upstream/packages/types/src/PanniniProjectionEffect.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/PanniniProjectionEffect.ts:2 (sha256:32de0892b5da6b236b2676f8c99f13992c0447241ac9d5fe70d4bcd91c352036)
#[derive(Clone)]
pub struct PanniniProjectionEffect {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub kind: String,
    pub compression: Option<f64>,
    pub crop: Option<f64>,
}
impl PartialEq for PanniniProjectionEffect {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
