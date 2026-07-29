// @generated from upstream/packages/types/src/Debug.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{LogLevel, LogSink};

// Source: upstream/packages/types/src/Debug.ts:9 (sha256:8931b36e135f7d330a53565e34a555cbb24c18357d7664b9bfb0c5d64e8377b3)
pub type DebugSubsystemName = String;

// Source: upstream/packages/types/src/Debug.ts:25 (sha256:658414808105f35fb9c07e5fcb3d9b0b0931e482a9946c47a6aa585c56615a0f)
#[derive(Clone, Default)]
pub struct DebugSubsystemHooks {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub channels: Option<Vec<String>>,
    pub enable_guards:
        Option<std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>>,
    pub disable_guards:
        Option<std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>>,
}
impl PartialEq for DebugSubsystemHooks {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Debug.ts:36 (sha256:11018664ec44ec8b07795b670e356ddc7bda6e50c9d04a148943e0e0b76ec738)
#[derive(Clone, Default)]
pub struct DebugOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub subsystems: Option<Vec<DebugSubsystemName>>,
    pub level: Option<LogLevel>,
    pub channels: Option<Vec<String>>,
    pub sink: Option<LogSink>,
}
impl PartialEq for DebugOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
