// @generated from upstream/packages/types/src/FlightDocumentToken.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{EntityRuntime, FlightDocumentValue, Kind};

// Source: upstream/packages/types/src/FlightDocumentToken.ts:8 (sha256:d456426dd24a8b0c7070890724d29945440f45136b98f74a8528056347e8ca80)
#[derive(Clone, Default)]
pub struct FlightDocumentToken {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub key: String,
    pub kind: Kind,
    pub values: FlightDocumentTokenValues,
}
impl PartialEq for FlightDocumentToken {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/FlightDocumentToken.ts:18 (sha256:d39c6211ce82ad870be1fffff7a71b9fe005a67665da1ec44d89ad2af081e393)
#[derive(Clone, Default)]
pub struct FlightDocumentTokenValues {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
}
impl PartialEq for FlightDocumentTokenValues {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/FlightDocumentToken.ts:24 (sha256:8d95bad51dd2db1773a8531c09eaeac46f13c770a7cc01b282d24ead7e7e2732)
#[derive(Clone, Default)]
pub struct FlightDocumentTokenResolution {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub mode: String,
    pub values: Vec<(String, FlightDocumentValue)>,
}
impl PartialEq for FlightDocumentTokenResolution {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/FlightDocumentToken.ts:31 (sha256:ac781f381275e425869891f0180c8007ed91d76a1842f08ce34727476c323675)
pub type FlightDocumentTokenResolver = std::sync::Arc<
    std::sync::Mutex<
        Box<
            dyn FnMut(FlightDocumentValue, FlightDocumentToken) -> Option<FlightDocumentValue>
                + Send
                + 'static,
        >,
    >,
>;

// Source: upstream/packages/types/src/FlightDocumentToken.ts:39 (sha256:b33ec42c52fe6b7ce386092cefebc1bbfa7e3cbc8af19bfdd63ae3a25933b5df)
#[derive(Clone, Default)]
pub struct FlightDocumentTokenResolverRegistry {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    #[doc(hidden)]
    pub __flight_entity_snapshot: Option<std::sync::Arc<dyn std::any::Any + Send + Sync>>,
    pub resolvers: Vec<(Kind, FlightDocumentTokenResolver)>,
}
impl PartialEq for FlightDocumentTokenResolverRegistry {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for FlightDocumentTokenResolverRegistry {
    fn __flight_entity_runtime(
        &self,
    ) -> &std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>> {
        &self.__flight_entity_runtime
    }
    fn __flight_entity_snapshot(&self) -> &Option<std::sync::Arc<dyn std::any::Any + Send + Sync>> {
        &self.__flight_entity_snapshot
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
