// @generated from upstream/packages/types/src/AudioResource.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{EntityRuntime, Signal};

// Source: upstream/packages/types/src/AudioResource.ts:4 (sha256:e96d9acaf895f660d102e2ae0946674047b60240c2321da84a2b267e037cba02)
pub type AudioChannelState = String;

// Source: upstream/packages/types/src/AudioResource.ts:6 (sha256:30f231e643c42be4ab6ec674b75ff9fcfa3f0f4275545de2dd685dd0502a2d28)
#[derive(Clone)]
pub struct AudioChannel {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub current_time: f64,
    pub gain: f64,
    pub length: f64,
    pub loop_end: f64,
    pub loops: f64,
    pub loop_start: f64,
    pub muted: bool,
    pub pan: f64,
    pub playback_rate: f64,
    pub source: AudioResource,
    pub state: AudioChannelState,
    pub on_complete:
        Signal<std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>>,
}
impl PartialEq for AudioChannel {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/AudioResource.ts:27 (sha256:07c7981f0fa5015691d9aecdffec83d3e8aa66cc930ba16af85e6de948189e8f)
#[derive(Clone, Default)]
pub struct AudioPlayOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub current_time: Option<f64>,
    pub gain: Option<f64>,
    pub loops: Option<f64>,
    pub playback_rate: Option<f64>,
}
impl PartialEq for AudioPlayOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/AudioResource.ts:34 (sha256:ef0abf16e044699e78c805ff0201c4a8b45cd4c385fa38a202fc8d6d62da3353)
#[derive(Clone, Default)]
pub struct AudioResource {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    #[doc(hidden)]
    pub __flight_entity_snapshot: Option<std::sync::Arc<dyn std::any::Any + Send + Sync>>,
    pub buffer: Option<crate::OpaqueHostValue>,
}
impl PartialEq for AudioResource {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for AudioResource {
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

// Source: upstream/packages/types/src/AudioResource.ts:38 (sha256:18ed341db225455421ca6bc6cfb29f8e58a6f370e9e516cf1c6eb32ccfdfb6a8)
#[derive(Clone, Default)]
pub struct AudioResourceUrl {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub url: String,
    pub type_: Option<String>,
}
impl PartialEq for AudioResourceUrl {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
