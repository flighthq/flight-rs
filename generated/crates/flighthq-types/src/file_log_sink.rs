// @generated from upstream/packages/types/src/FileLogSink.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{EntityRuntime, LogSink, LogTransportDestroyOutcome, LogTransportFlushOutcome};

// Source: upstream/packages/types/src/FileLogSink.ts:8 (sha256:1b4d1eb4c743e35f3c6951d2e21893b6c37b180aa997c0bed98d6fa2afe0d3cb)
#[derive(Clone)]
pub struct FileLogSink {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    #[doc(hidden)]
    pub __flight_entity_snapshot: Option<std::sync::Arc<dyn std::any::Any + Send + Sync>>,
    pub sink: LogSink,
}
impl PartialEq for FileLogSink {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for FileLogSink {
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

// Source: upstream/packages/types/src/FileLogSink.ts:13 (sha256:4b1d5e76df8c7c3bed8a49630c86cf010989d313ecef608519e4cdbcb82a01cc)
#[derive(Clone)]
pub struct FileLogSinkDestroyOutcome {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub reason: String,
    pub flush: LogTransportFlushOutcome,
    pub destroy: LogTransportDestroyOutcome,
}
impl PartialEq for FileLogSinkDestroyOutcome {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
