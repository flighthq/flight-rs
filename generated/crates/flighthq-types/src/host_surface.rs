// @generated from upstream/packages/types/src/HostSurface.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::Surface;

// Source: upstream/packages/types/src/HostSurface.ts:9 (sha256:b579cdf881be4c86aa7884f26ad731b6e56110779cf40ae5b1dfbe1a76253e23)
#[derive(Clone)]
pub struct HostSurfaceDisplayCapability {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub set_display_size:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(Surface, f64, f64) -> () + Send + 'static>>>,
}
impl PartialEq for HostSurfaceDisplayCapability {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/HostSurface.ts:14 (sha256:1d745d61b82829b5e12fbe88e26152c8241224fd2ff29ab9279e33f9a8093d3c)
#[derive(Clone)]
pub struct HostSurfaceResizeCapability {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub resize:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(Surface, f64, f64) -> () + Send + 'static>>>,
}
impl PartialEq for HostSurfaceResizeCapability {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
