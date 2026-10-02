// @generated from upstream/packages/types/src/Log.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::EntityRuntime;

#[derive(Clone, Default)]
pub struct SharedStructuralRecord1 {
    pub __flight_identity: std::sync::Arc<()>,
    pub reason: String,
}
impl PartialEq for SharedStructuralRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct SharedStructuralRecord2 {
    pub __flight_identity: std::sync::Arc<()>,
    pub reason: String,
    pub message: String,
}
impl PartialEq for SharedStructuralRecord2 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct LogTransportWriteOutcomeRecord3 {
    pub __flight_identity: std::sync::Arc<()>,
    pub reason: String,
    pub retry_after_ms: Option<f64>,
}
impl PartialEq for LogTransportWriteOutcomeRecord3 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct LogTransportWriteOutcomeRecord4 {
    pub __flight_identity: std::sync::Arc<()>,
    pub reason: String,
}
impl PartialEq for LogTransportWriteOutcomeRecord4 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct LogTransportFlushOutcomeRecord3 {
    pub __flight_identity: std::sync::Arc<()>,
    pub reason: String,
    pub delivery: LogTransportDeliveryBoundary,
}
impl PartialEq for LogTransportFlushOutcomeRecord3 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct LogTransportDestroyOutcomeRecord2 {
    pub __flight_identity: std::sync::Arc<()>,
    pub reason: String,
}
impl PartialEq for LogTransportDestroyOutcomeRecord2 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Log.ts:8 (sha256:5e0faa717365b990bb5e57f1ad7e4dcfe9b7e6ae8992248fd57a4837411e2006)
#[derive(Clone, Copy, Debug, Default, Eq, Ord, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct LogLevel(pub u32);

impl LogLevel {
    #[allow(non_upper_case_globals)]
    pub const None: Self = Self(0_u32);

    #[allow(non_upper_case_globals)]
    pub const Error: Self = Self(1_u32);

    #[allow(non_upper_case_globals)]
    pub const Warn: Self = Self(2_u32);

    #[allow(non_upper_case_globals)]
    pub const Info: Self = Self(3_u32);

    #[allow(non_upper_case_globals)]
    pub const Debug: Self = Self(4_u32);

    #[allow(non_upper_case_globals)]
    pub const Verbose: Self = Self(5_u32);
}

impl std::ops::BitAnd for LogLevel {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl std::ops::BitOr for LogLevel {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl std::ops::BitXor for LogLevel {
    type Output = Self;
    fn bitxor(self, rhs: Self) -> Self {
        Self(self.0 ^ rhs.0)
    }
}
impl std::ops::Not for LogLevel {
    type Output = Self;
    fn not(self) -> Self {
        Self(!self.0)
    }
}
impl PartialEq<f64> for LogLevel {
    fn eq(&self, rhs: &f64) -> bool {
        self.0 as f64 == *rhs
    }
}

// Source: upstream/packages/types/src/Log.ts:18 (sha256:313d487a0bb03997817e5978f5481e3fbd42d8ffdeddb816cc9b40996c07be93)
pub type LogData = crate::FlightUnion2<String, Vec<(String, crate::FlightValue)>>;

// Source: upstream/packages/types/src/Log.ts:22 (sha256:d60cca70f541b83b70e870bacd1dddf4969f744d6350f38e7cede5f237fada40)
#[derive(Clone, Default)]
pub struct LogContext {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    #[doc(hidden)]
    pub __flight_entity_snapshot: Option<std::sync::Arc<dyn std::any::Any + Send + Sync>>,
    pub channel: Option<String>,
    pub fields: Vec<(String, crate::FlightValue)>,
}
impl PartialEq for LogContext {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for LogContext {
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

// Source: upstream/packages/types/src/Log.ts:29 (sha256:c0822a51fd605fc96b3a579f6942990c7b4f839d5590c31eaae5746a8b7300e4)
pub type LogDataProvider =
    std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> LogData + Send + 'static>>>;

// Source: upstream/packages/types/src/Log.ts:33 (sha256:8a98f5b8ea6d3797e3084f4b84d53498742e1d9b2a70b004941f6cb74e9b73ec)
pub type LogFormatter =
    std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(LogEntry) -> String + Send + 'static>>>;

// Source: upstream/packages/types/src/Log.ts:37 (sha256:cbc3029e80c43eb79798a14dfd1a6fc4f75cebadb5049104002e40020ba0d783)
#[derive(Clone, Default)]
pub struct LogSpan {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    #[doc(hidden)]
    pub __flight_entity_snapshot: Option<std::sync::Arc<dyn std::any::Any + Send + Sync>>,
    pub name: String,
    pub fields: Vec<(String, crate::FlightValue)>,
    pub channel: Option<String>,
}
impl PartialEq for LogSpan {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for LogSpan {
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

// Source: upstream/packages/types/src/Log.ts:45 (sha256:a56853b5dcd8fd8a420ab21e31ab308857ff83b921b6b51ba6356b5c35f534da)
#[derive(Clone, Default)]
pub struct LogTimer {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub label: String,
    pub channel: Option<String>,
    pub started_at: f64,
}
impl PartialEq for LogTimer {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Log.ts:52 (sha256:f963abc5e3bab99596b850885a461ef5a72cf5972292af223e91e0a159911ab6)
pub type LogTransportWriteOutcome = crate::FlightUnion2<
    LogTransportWriteOutcomeRecord4,
    crate::FlightUnion2<
        LogTransportWriteOutcomeRecord3,
        crate::FlightUnion2<SharedStructuralRecord1, SharedStructuralRecord2>,
    >,
>;

// Source: upstream/packages/types/src/Log.ts:59 (sha256:0f2250de27b3ff78c8564915df835e402e151a33c79dd09a2308b7ba063c257b)
pub type LogTransportDeliveryBoundary = String;

// Source: upstream/packages/types/src/Log.ts:66 (sha256:6b1593a94aac6eed3a641e80166bf723a719e0509c78181ba29d208c8ca4f401)
pub type LogTransportFlushOutcome = crate::FlightUnion2<
    LogTransportFlushOutcomeRecord3,
    crate::FlightUnion2<SharedStructuralRecord1, SharedStructuralRecord2>,
>;

// Source: upstream/packages/types/src/Log.ts:72 (sha256:0a88aa3c4a281a3a0a2f666f738051842a52cc944394ddb736c5193c2f150e99)
pub type LogTransportDestroyOutcome = crate::FlightUnion2<
    SharedStructuralRecord1,
    crate::FlightUnion2<LogTransportDestroyOutcomeRecord2, SharedStructuralRecord2>,
>;

// Source: upstream/packages/types/src/Log.ts:86 (sha256:4389a58eda880517242df363198e9c0ab778934a709ef3e3d2eab57d0a19404d)
#[derive(Clone)]
pub struct LogTransport {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    #[doc(hidden)]
    pub __flight_entity_snapshot: Option<std::sync::Arc<dyn std::any::Any + Send + Sync>>,
    pub write: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut(String) -> LogTransportWriteOutcome + Send + 'static>>,
    >,
    pub flush: std::sync::Arc<
        std::sync::Mutex<
            Box<dyn FnMut() -> crate::FlightTask<LogTransportFlushOutcome> + Send + 'static>,
        >,
    >,
    pub destroy: std::sync::Arc<
        std::sync::Mutex<
            Box<dyn FnMut() -> crate::FlightTask<LogTransportDestroyOutcome> + Send + 'static>,
        >,
    >,
}
impl PartialEq for LogTransport {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for LogTransport {
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

// Source: upstream/packages/types/src/Log.ts:94 (sha256:386ab33911ac9d6cfda1c53f076da7a8c79abd8f042508f5e4210185d9eacd08)
#[derive(Clone)]
pub struct LogEntry {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub level: LogLevel,
    pub channel: Option<String>,
    pub data: LogData,
}
impl PartialEq for LogEntry {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Log.ts:104 (sha256:96ab8898f3b002b3707c2bbb15c6ba2a32d03c846562c9324d2be5f6e0815231)
pub type LogSink =
    std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(LogEntry) -> () + Send + 'static>>>;
