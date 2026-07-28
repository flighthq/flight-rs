// @generated from upstream/packages/types/src/Entity.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/Entity.ts:1 (sha256:cdb46c2fe96dc3464760172db6afea575505426c2d0a8207914ee42315a65204)
pub type Kind = String;

// Source: upstream/packages/types/src/Entity.ts:2 (sha256:f85f87cc89d4e93a438fb81e5b911bfb1ecfdec7d52c81fa9ae258bec22021a6)
#[derive(Clone)]
pub struct Entity {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
}
impl PartialEq for Entity {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Entity.ts:5 (sha256:e8922dec976bcfcb17943d6646ad2d8a649cf0c12bc2a77930283dacd421e57a)
pub type EntityWithoutRuntime<Type> = Type;

// Source: upstream/packages/types/src/Entity.ts:6 (sha256:2442a2b2f11e739d0ec1d2f573d38c0fe55fef3a64723ef98ff3c0f7b4981bdb)
#[derive(Clone)]
pub struct EntityRuntime {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub binding: Option<crate::OpaqueHostValue>,
}
impl PartialEq for EntityRuntime {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Entity.ts:9 (sha256:9f3e8b58b7216dc7038f2b87e275302645d5d4dce805b89b90c83f9094f1f048)
pub static ENTITY_RUNTIME_KEY: std::sync::LazyLock<crate::FlightSymbol> =
    std::sync::LazyLock::new(|| crate::FlightSymbol::for_name(&("EntityRuntime".to_owned())));
