// @generated from upstream/packages/types/src/Haptics.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/Haptics.ts:4 (sha256:657eafd12e02526dbf22fd415c3a46ae863808492273f739783467e6bdf9dd5a)
pub type HapticImpactStyle = String;

// Source: upstream/packages/types/src/Haptics.ts:5 (sha256:576a412ea6080a8e1a6817d714b944af20ea35af738d84855a4b91f4914dcf7f)
pub type HapticNotificationType = String;

// Source: upstream/packages/types/src/Haptics.ts:9 (sha256:493ca4196d2dc3374693378a6d9043ca983c1eaafd0ffb8973b1edd62f841d28)
#[derive(Clone, Default)]
pub struct HapticsCapabilities {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub amplitude_control: bool,
    pub custom_events: bool,
    pub intensity: bool,
    pub patterns: bool,
    pub supported: bool,
}
impl PartialEq for HapticsCapabilities {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Haptics.ts:17 (sha256:7451aefe9a23805b4a47234fa02bd11c635b0188891dbcb413899544b2e44573)
#[derive(Clone)]
pub struct HapticsBackend {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub cancel: std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> bool + Send + 'static>>>,
    pub capabilities: std::sync::Arc<
        std::sync::Mutex<
            Box<dyn FnMut(HapticsCapabilities) -> HapticsCapabilities + Send + 'static>,
        >,
    >,
    pub impact: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut(HapticImpactStyle, Option<f64>) -> bool + Send + 'static>>,
    >,
    pub is_supported: std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> bool + Send + 'static>>>,
    pub notification: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut(HapticNotificationType) -> bool + Send + 'static>>,
    >,
    pub prepare: Option<std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>>,
    pub selection: std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> bool + Send + 'static>>>,
    pub vibrate: std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(f64) -> bool + Send + 'static>>>,
    pub vibrate_pattern:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(Vec<f64>) -> bool + Send + 'static>>>,
    pub vibrate_waveform: Option<
        std::sync::Arc<
            std::sync::Mutex<
                Box<dyn FnMut(Vec<f64>, Vec<f64>, Option<f64>) -> bool + Send + 'static>,
            >,
        >,
    >,
}
impl PartialEq for HapticsBackend {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
