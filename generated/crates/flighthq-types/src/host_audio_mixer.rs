// @generated from upstream/packages/types/src/HostAudioMixer.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{AudioDeviceHandle, AudioSourceHandle};

// Source: upstream/packages/types/src/HostAudioMixer.ts:3 (sha256:96e054cd42b653c10005cf36f17ddbf3876d93e545126834a301237a5df90aef)
#[derive(Clone, Default)]
pub struct AudioBusNodeHandle {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub __brand: String,
}
impl PartialEq for AudioBusNodeHandle {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/HostAudioMixer.ts:5 (sha256:0a312762f2ad1058e232af729a3d82192cd46df25d3930b6b4819242b86a2b4c)
#[derive(Clone, Default)]
pub struct AudioMixerGraphHandle {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub __brand: String,
}
impl PartialEq for AudioMixerGraphHandle {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/HostAudioMixer.ts:7 (sha256:eb9ea2311869914e034cb63ceb2e7df477dcb8e566830997eee1f6861c889371)
#[derive(Clone)]
pub struct HostAudioMixerCapability {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub create_mixer_graph: std::sync::Arc<
        std::sync::Mutex<
            Box<dyn FnMut(AudioDeviceHandle, f64) -> AudioMixerGraphHandle + Send + 'static>,
        >,
    >,
    pub destroy_mixer_graph: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut(AudioMixerGraphHandle) -> () + Send + 'static>>,
    >,
    pub create_bus_node: std::sync::Arc<
        std::sync::Mutex<
            Box<dyn FnMut(AudioMixerGraphHandle, f64, f64) -> AudioBusNodeHandle + Send + 'static>,
        >,
    >,
    pub destroy_bus_node: std::sync::Arc<
        std::sync::Mutex<
            Box<dyn FnMut(AudioMixerGraphHandle, AudioBusNodeHandle) -> () + Send + 'static>,
        >,
    >,
    pub set_bus_node_gain: std::sync::Arc<
        std::sync::Mutex<
            Box<dyn FnMut(AudioMixerGraphHandle, AudioBusNodeHandle, f64) -> () + Send + 'static>,
        >,
    >,
    pub set_bus_node_pan: std::sync::Arc<
        std::sync::Mutex<
            Box<dyn FnMut(AudioMixerGraphHandle, AudioBusNodeHandle, f64) -> () + Send + 'static>,
        >,
    >,
    pub fade_bus_node_gain: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(AudioMixerGraphHandle, AudioBusNodeHandle, f64, f64) -> ()
                    + Send
                    + 'static,
            >,
        >,
    >,
    pub set_master_gain: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut(AudioMixerGraphHandle, f64) -> () + Send + 'static>>,
    >,
    pub route_source_to_bus: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(AudioMixerGraphHandle, AudioSourceHandle, AudioBusNodeHandle) -> ()
                    + Send
                    + 'static,
            >,
        >,
    >,
    pub unroute_source: std::sync::Arc<
        std::sync::Mutex<
            Box<dyn FnMut(AudioMixerGraphHandle, AudioSourceHandle) -> () + Send + 'static>,
        >,
    >,
    pub route_source_to_default: std::sync::Arc<
        std::sync::Mutex<
            Box<dyn FnMut(AudioMixerGraphHandle, AudioSourceHandle) -> () + Send + 'static>,
        >,
    >,
}
impl PartialEq for HostAudioMixerCapability {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
