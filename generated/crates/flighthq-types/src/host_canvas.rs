// @generated from upstream/packages/types/src/HostCanvas.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{AppWindow, CanvasSurface, NativeSurfaceHandle, Surface};

// Source: upstream/packages/types/src/HostCanvas.ts:11 (sha256:5201a3169f3736a29aa381c0c5012520c8dc4cf1afbb604e8b7312a6b23a70e7)
#[derive(Clone)]
pub struct HostCanvasCapability {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub acquire: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(Surface, Option<crate::OpaqueHostValue>) -> Option<crate::OpaqueHostValue>
                    + Send
                    + 'static,
            >,
        >,
    >,
    pub create: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(
                        AppWindow,
                        f64,
                        f64,
                        Option<crate::OpaqueHostValue>,
                    ) -> Option<NativeSurfaceHandle>
                    + Send
                    + 'static,
            >,
        >,
    >,
    pub create_surface: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut(f64, f64) -> Option<CanvasSurface> + Send + 'static>>,
    >,
    pub destroy_surface:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(CanvasSurface) -> () + Send + 'static>>>,
    pub release: std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(Surface) -> () + Send + 'static>>>,
}
impl PartialEq for HostCanvasCapability {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
