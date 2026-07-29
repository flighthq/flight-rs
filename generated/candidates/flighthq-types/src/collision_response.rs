// @generated from upstream/packages/types/src/CollisionResponse.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/CollisionResponse.ts:3 (sha256:801757bc83a57b31e8922ff6be2250042dcde8f1a022db4cd9bc030a1aa20c6b)
#[derive(Clone, Default)]
pub struct CollisionResponse {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub restitution: Option<f64>,
    pub friction: Option<f64>,
}
impl PartialEq for CollisionResponse {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
