// @generated from upstream/packages/types/src/HostAudioDevice.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{AudioBufferHandle, AudioDeviceHandle, AudioSourceHandle};

// Source: upstream/packages/types/src/HostAudioDevice.ts:4 (sha256:87e6ddba47f81c34e149a6a9b631e5165ff459de09814a416e62de7e8537d90b)
#[derive(Clone)]
pub struct HostAudioDeviceCapability {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub create_buffer: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(AudioDeviceHandle, f64, f64, f64, Vec<Vec<f32>>) -> AudioBufferHandle
                    + Send
                    + 'static,
            >,
        >,
    >,
    pub create_device:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(f64) -> AudioDeviceHandle + Send + 'static>>>,
    pub create_source: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(AudioDeviceHandle, AudioBufferHandle) -> AudioSourceHandle
                    + Send
                    + 'static,
            >,
        >,
    >,
    pub destroy_buffer:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(AudioBufferHandle) -> () + Send + 'static>>>,
    pub destroy_device:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(AudioDeviceHandle) -> () + Send + 'static>>>,
    pub destroy_source:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(AudioSourceHandle) -> () + Send + 'static>>>,
    pub fade_source_gain: Option<
        std::sync::Arc<
            std::sync::Mutex<Box<dyn FnMut(AudioSourceHandle, f64, f64) -> () + Send + 'static>>,
        >,
    >,
    pub get_device_time:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(AudioDeviceHandle) -> f64 + Send + 'static>>>,
    pub on_source_ended: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(
                        AudioSourceHandle,
                        Option<
                            std::sync::Arc<
                                std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>,
                            >,
                        >,
                    ) -> ()
                    + Send
                    + 'static,
            >,
        >,
    >,
    pub resume_device:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(AudioDeviceHandle) -> () + Send + 'static>>>,
    pub set_source_gain: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut(AudioSourceHandle, f64) -> () + Send + 'static>>,
    >,
    pub set_source_pan: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut(AudioSourceHandle, f64) -> () + Send + 'static>>,
    >,
    pub set_source_playback_rate: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut(AudioSourceHandle, f64) -> () + Send + 'static>>,
    >,
    pub start_source: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut(AudioSourceHandle, f64, f64) -> () + Send + 'static>>,
    >,
    pub stop_source:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(AudioSourceHandle) -> () + Send + 'static>>>,
}
impl PartialEq for HostAudioDeviceCapability {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/HostAudioDevice.ts:45 (sha256:bdc5867c5568f67265f3f426b7097adb2b34def6c28f81adadbafcb200727edc)
pub type AudioDeviceOperation = HostAudioDeviceCapability;
