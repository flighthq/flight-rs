// @generated from upstream/packages/types/src/HostGl.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{AppWindow, GlContext, GlContextOptions, NativeSurfaceHandle, Surface};

// Source: upstream/packages/types/src/HostGl.ts:12 (sha256:e435b0443dfadc741eb0bbe3b01f53c82913a9eef2c5a634e25d170529fb3b56)
#[derive(Clone)]
pub struct HostGlCapability {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub acquire: std::sync::Arc<
        std::sync::Mutex<
            Box<dyn FnMut(Surface, Option<GlContextOptions>) -> Option<GlContext> + Send + 'static>,
        >,
    >,
    pub create: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(
                        AppWindow,
                        f64,
                        f64,
                        Option<GlContextOptions>,
                    ) -> Option<NativeSurfaceHandle>
                    + Send
                    + 'static,
            >,
        >,
    >,
    pub release: std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(Surface) -> () + Send + 'static>>>,
    pub subscribe: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(
                        Surface,
                        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>,
                        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>,
                    ) -> std::sync::Arc<
                        std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>,
                    > + Send
                    + 'static,
            >,
        >,
    >,
}
impl PartialEq for HostGlCapability {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
