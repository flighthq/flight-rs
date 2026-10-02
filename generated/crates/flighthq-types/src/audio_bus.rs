// @generated from upstream/packages/types/src/AudioBus.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::EntityRuntime;

// Source: upstream/packages/types/src/AudioBus.ts:3 (sha256:8933daecfebfb0b88cf4a3a510c64af8b7d3bde2b51ee8883b33e5a69399117d)
#[derive(Clone, Default)]
pub struct AudioBus {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    #[doc(hidden)]
    pub __flight_entity_snapshot: Option<std::sync::Arc<dyn std::any::Any + Send + Sync>>,
    pub gain: f64,
    pub muted: bool,
    pub name: String,
    pub pan: f64,
}
impl PartialEq for AudioBus {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for AudioBus {
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

// Source: upstream/packages/types/src/AudioBus.ts:9 (sha256:9210d61ab3b5c83f9cee9723c9836095f16b8e0b3ce41a73704d227c93b6dfe7)
#[derive(Clone, Default)]
pub struct AudioBusOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub gain: Option<f64>,
    pub muted: Option<bool>,
    pub name: Option<String>,
    pub pan: Option<f64>,
}
impl PartialEq for AudioBusOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/AudioBus.ts:15 (sha256:3502df11befe88084b00ae4ebbc20670d229bd442a6062ea2411f2a05c87bf0e)
#[derive(Clone, Default)]
pub struct AudioMixer {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    #[doc(hidden)]
    pub __flight_entity_snapshot: Option<std::sync::Arc<dyn std::any::Any + Send + Sync>>,
    pub master_gain: f64,
    pub master_muted: bool,
}
impl PartialEq for AudioMixer {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for AudioMixer {
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

// Source: upstream/packages/types/src/AudioBus.ts:19 (sha256:35f7cc79617b727435ae8469587bc72f31b854a18d193482ab5ecc4bf433a64e)
#[derive(Clone, Default)]
pub struct AudioMixerOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub master_gain: Option<f64>,
    pub master_muted: Option<bool>,
}
impl PartialEq for AudioMixerOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/AudioBus.ts:25 (sha256:fc14fcea83c62f469d5047fcc81a43216dfaa7f96c0546d9b1cd9519a0f0c03d)
pub type AudioBusMixerOperation = String;

// Source: upstream/packages/types/src/AudioBus.ts:30 (sha256:b064a6dce85e4b719fa9f26686386f4becf4ffa93ea32aeb21f3b0715a000cbf)
pub type AudioBusMixerGuard = std::sync::Arc<
    std::sync::Mutex<Box<dyn FnMut(AudioBusMixerOperation, AudioBus) -> () + Send + 'static>>,
>;
