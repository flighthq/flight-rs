// @generated from upstream/packages/types/src/AudioDeviceHandle.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/AudioDeviceHandle.ts:1 (sha256:8ef02135b12cd3282d6d6e03c5e3f426982e5b23f9b564bb639f1b52d8fb1476)
#[derive(Clone, Default)]
pub struct AudioBufferHandle {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub __brand: String,
}
impl PartialEq for AudioBufferHandle {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/AudioDeviceHandle.ts:3 (sha256:b5f59dc457a1347649840a8920de75f372c6d1d94004c13e18bbf8e15eee9975)
#[derive(Clone, Default)]
pub struct AudioDeviceHandle {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub __brand: String,
}
impl PartialEq for AudioDeviceHandle {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/AudioDeviceHandle.ts:5 (sha256:3f3c421be81b9332c0d88d9bafb09bd73bd89d544f0e1721b7e0bc2cdf2f40ae)
#[derive(Clone, Default)]
pub struct AudioSourceHandle {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub __brand: String,
}
impl PartialEq for AudioSourceHandle {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
