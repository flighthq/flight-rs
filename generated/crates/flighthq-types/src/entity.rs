// @generated from upstream/packages/types/src/Entity.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/Entity.ts:2 (sha256:f85f87cc89d4e93a438fb81e5b911bfb1ecfdec7d52c81fa9ae258bec22021a6)
#[derive(Clone, Default)]
pub struct Entity {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<EntityRuntime>>>,
}
impl PartialEq for Entity {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
#[doc(hidden)]
pub trait FlightEntity {
    fn __flight_entity_runtime(&self) -> &std::sync::Arc<std::sync::Mutex<Option<EntityRuntime>>>;
    fn __flight_fresh_clone(&self) -> Self
    where
        Self: Sized;
}
impl FlightEntity for Entity {
    fn __flight_entity_runtime(&self) -> &std::sync::Arc<std::sync::Mutex<Option<EntityRuntime>>> {
        &self.__flight_entity_runtime
    }
    fn __flight_fresh_clone(&self) -> Self {
        let mut cloned = self.clone();
        cloned.__flight_identity = std::sync::Arc::new(());
        cloned.__flight_entity_runtime = std::sync::Arc::new(std::sync::Mutex::new(
            self.__flight_entity_runtime.lock().unwrap().clone(),
        ));
        cloned
    }
}

// Source: upstream/packages/types/src/Entity.ts:6 (sha256:2442a2b2f11e739d0ec1d2f573d38c0fe55fef3a64723ef98ff3c0f7b4981bdb)
#[derive(Clone, Default)]
pub struct EntityRuntime {
    #[doc(hidden)]
    pub inner: std::sync::Arc<std::sync::Mutex<EntityRuntimeStorage>>,
}
#[doc(hidden)]
#[derive(Default)]
pub struct EntityRuntimeStorage {
    pub binding: Option<crate::OpaqueHostValue>,
}
impl PartialEq for EntityRuntime {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.inner, &other.inner)
    }
}
#[doc(hidden)]
pub trait FlightEntityRuntimeMarker {
    type Runtime;
}
impl<Marker> FlightEntityRuntimeMarker for std::marker::PhantomData<Marker> {
    type Runtime = EntityRuntime;
}
