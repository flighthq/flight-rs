// @generated from upstream/packages/types/src/Load.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::Signal;

// Source: upstream/packages/types/src/Load.ts:2 (sha256:b749f3b8fd4d2461b6c7f76cebe522ade45c82f46ce9e25c78b9145ace2e712e)
#[derive(Clone, Default)]
pub struct LoadProgress {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub url: String,
    pub loaded: f64,
    pub total: f64,
    pub phase: String,
}
impl PartialEq for LoadProgress {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Load.ts:8 (sha256:5464517e41f0fef623a1a74349663b1351690fccac32921cda0e1e14354c3de2)
#[derive(Clone, Default)]
pub struct LoadOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub signal: Option<crate::OpaqueHostValue>,
    pub progress: Option<
        Signal<
            std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(LoadProgress) -> () + Send + 'static>>>,
        >,
    >,
}
impl PartialEq for LoadOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
