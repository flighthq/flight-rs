// @generated from upstream/packages/types/src/AudioResource.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::Signal;

// Source: upstream/packages/types/src/AudioResource.ts:3 (sha256:e96d9acaf895f660d102e2ae0946674047b60240c2321da84a2b267e037cba02)
pub type AudioChannelState = String;

// Source: upstream/packages/types/src/AudioResource.ts:5 (sha256:4f580b094a34dc3caa1dc3657d818287678067ff5ec33868dedd5be998ce6117)
#[derive(Clone)]
pub struct AudioChannel {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub current_time: f64,
    pub gain: f64,
    pub length: f64,
    pub loops: f64,
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

// Source: upstream/packages/types/src/AudioResource.ts:16 (sha256:07c7981f0fa5015691d9aecdffec83d3e8aa66cc930ba16af85e6de948189e8f)
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

// Source: upstream/packages/types/src/AudioResource.ts:23 (sha256:d17808fcafdb2857423b4feb55d2b94250690da4b885fc122c08c9a8944aad2c)
#[derive(Clone, Default)]
pub struct AudioResource {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub buffer: Option<crate::OpaqueHostValue>,
}
impl PartialEq for AudioResource {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/AudioResource.ts:27 (sha256:18ed341db225455421ca6bc6cfb29f8e58a6f370e9e516cf1c6eb32ccfdfb6a8)
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
