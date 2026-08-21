// @generated from upstream/packages/types/src/ApplicationRenderView.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{
    ApplicationWindow, EntityRuntime, GlRenderOptions, GlRenderState, GlRenderTarget,
    RenderTargetColorSpace, RenderTargetDepth, RenderTargetFormat, Viewport,
};

#[derive(Clone, Default)]
pub struct FlightOmitRecord617336933 {
    pub __flight_identity: std::sync::Arc<()>,
    pub format: Option<RenderTargetFormat>,
    pub color_attachments: Option<f64>,
    pub color_formats: Option<Vec<RenderTargetFormat>>,
    pub sample_count: Option<f64>,
    pub depth: Option<RenderTargetDepth>,
    pub color_space: Option<RenderTargetColorSpace>,
    pub clear_colors: Option<Vec<f64>>,
    pub clear_depth: Option<f64>,
}
impl PartialEq for FlightOmitRecord617336933 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/ApplicationRenderView.ts:13 (sha256:3bab8e786238eca3bad3e9f7195ace7d18843ee002a1401e420182ea974ede49)
#[derive(Clone)]
pub struct ApplicationRenderView<State, Target> {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    pub render_state: State,
    pub render_target: Target,
    pub viewport: Viewport,
    pub window: ApplicationWindow,
}
impl<State, Target> PartialEq for ApplicationRenderView<State, Target> {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl<State: Clone, Target: Clone> crate::FlightEntity for ApplicationRenderView<State, Target> {
    fn __flight_entity_runtime(
        &self,
    ) -> &std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>> {
        &self.__flight_entity_runtime
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

// Source: upstream/packages/types/src/ApplicationRenderView.ts:23 (sha256:06083a2fb200947e4a38216617cfba0fb524b2b3dbfdbe5d80ee8d44c301f8c7)
pub type ApplicationRenderViewResize<State, Target> = std::sync::Arc<
    std::sync::Mutex<Box<dyn FnMut(State, Target, f64, f64) -> () + Send + 'static>>,
>;

// Source: upstream/packages/types/src/ApplicationRenderView.ts:30 (sha256:eb8525a5d9222472fb5869c534132ba50a6ccdd3268184201c9fe95b1b5a2e3b)
pub type ApplicationRenderViewTargetOptions = FlightOmitRecord617336933;

// Source: upstream/packages/types/src/ApplicationRenderView.ts:32 (sha256:ecfc5fe6a47a84cd8f0e0d73b48b65b08ebc0545af89d726550bc678a4020b69)
#[derive(Clone, Default)]
pub struct GlApplicationRenderViewOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub render: Option<GlRenderOptions>,
    pub target: Option<ApplicationRenderViewTargetOptions>,
}
impl PartialEq for GlApplicationRenderViewOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/ApplicationRenderView.ts:37 (sha256:1c75a6fa841a7cb5faf6a826324717805b3fe2e81c3e682691276a2d2a1336d7)
pub type GlApplicationRenderView = ApplicationRenderView<GlRenderState, GlRenderTarget>;
