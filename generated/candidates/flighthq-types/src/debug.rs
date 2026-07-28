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
#[derive(Clone)]
pub struct DebugSubsystemHooks {
    pub channels: Option<Vec<String>>,
    pub enable_guards: Option<std::sync::Arc<dyn Fn() -> () + Send + Sync + 'static>>,
    pub disable_guards: Option<std::sync::Arc<dyn Fn() -> () + Send + Sync + 'static>>,
}

// Source: upstream/packages/types/src/Debug.ts:36 (sha256:11018664ec44ec8b07795b670e356ddc7bda6e50c9d04a148943e0e0b76ec738)
#[derive(Clone)]
pub struct DebugOptions {
    pub subsystems: Option<Vec<DebugSubsystemName>>,
    pub level: Option<LogLevel>,
    pub channels: Option<Vec<String>>,
    pub sink: Option<LogSink>,
}
