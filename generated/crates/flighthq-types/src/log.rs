// @generated from upstream/packages/types/src/Log.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/Log.ts:6 (sha256:5e0faa717365b990bb5e57f1ad7e4dcfe9b7e6ae8992248fd57a4837411e2006)
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

// Source: upstream/packages/types/src/Log.ts:16 (sha256:313d487a0bb03997817e5978f5481e3fbd42d8ffdeddb816cc9b40996c07be93)
pub type LogData = crate::FlightUnion2<String, Vec<(String, crate::FlightValue)>>;

// Source: upstream/packages/types/src/Log.ts:20 (sha256:d983c478cbedc28ac64db9a78d5b83adb8b0750a9224e97fa3ddfcfb7c7f2eaa)
#[derive(Clone, Default)]
pub struct LogContext {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub channel: Option<String>,
    pub fields: Vec<(String, crate::FlightValue)>,
}
impl PartialEq for LogContext {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Log.ts:27 (sha256:c0822a51fd605fc96b3a579f6942990c7b4f839d5590c31eaae5746a8b7300e4)
pub type LogDataProvider =
    std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> LogData + Send + 'static>>>;

// Source: upstream/packages/types/src/Log.ts:31 (sha256:8a98f5b8ea6d3797e3084f4b84d53498742e1d9b2a70b004941f6cb74e9b73ec)
pub type LogFormatter =
    std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(LogEntry) -> String + Send + 'static>>>;

// Source: upstream/packages/types/src/Log.ts:35 (sha256:03d5fbfd5d9c17508b12ce413efa7dd22bcba10df869e54bf449316a542abe71)
#[derive(Clone, Default)]
pub struct LogSpan {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub name: String,
    pub fields: Vec<(String, crate::FlightValue)>,
    pub channel: Option<String>,
}
impl PartialEq for LogSpan {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Log.ts:43 (sha256:a56853b5dcd8fd8a420ab21e31ab308857ff83b921b6b51ba6356b5c35f534da)
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

// Source: upstream/packages/types/src/Log.ts:52 (sha256:2e7c04b41d35f784f0070e5e08a87a9254138a1facfe0acdce267fc430d50a2f)
#[derive(Clone)]
pub struct LogTransportBackend {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub write: std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(String) -> () + Send + 'static>>>,
    pub flush: Option<std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>>,
    pub dispose: Option<std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>>,
}
impl PartialEq for LogTransportBackend {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Log.ts:60 (sha256:386ab33911ac9d6cfda1c53f076da7a8c79abd8f042508f5e4210185d9eacd08)
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

// Source: upstream/packages/types/src/Log.ts:70 (sha256:96ab8898f3b002b3707c2bbb15c6ba2a32d03c846562c9324d2be5f6e0815231)
pub type LogSink =
    std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(LogEntry) -> () + Send + 'static>>>;
