// @generated from upstream/packages/types/src/AppRenderView.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{
    AppWindow, EntityRuntime, GlContextOptions, GlRenderState, GlRenderStateOptions,
    GlTextureRenderTarget, RenderState, RenderTargetColorSpace, RenderTargetDepth,
    RenderTargetDimensions, RenderTargetFormat, Viewport,
};

#[derive(Clone, Default)]
pub struct FlightOmitRecord2663357065 {
    pub __flight_identity: std::sync::Arc<()>,
    pub format: Option<RenderTargetFormat>,
    pub color_attachments: Option<f64>,
    pub color_formats: Option<Vec<RenderTargetFormat>>,
    pub sample_count: Option<f64>,
    pub depth: Option<RenderTargetDepth>,
    pub color_space: Option<RenderTargetColorSpace>,
}
impl PartialEq for FlightOmitRecord2663357065 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/AppRenderView.ts:14 (sha256:787d6cbe264fe601ee77692f04ef5d5befb985df55e7d514d9e7fbd703bd80ec)
#[derive(Clone)]
pub struct AppRenderView<State = RenderState, Target = RenderTargetDimensions> {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    #[doc(hidden)]
    pub __flight_entity_snapshot: Option<std::sync::Arc<dyn std::any::Any + Send + Sync>>,
    pub render_state: State,
    pub render_target: Target,
    pub viewport: Viewport,
    pub window: AppWindow,
}
impl<State, Target> PartialEq for AppRenderView<State, Target> {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl<State: Clone + Send + Sync + 'static, Target: Clone + Send + Sync + 'static>
    crate::FlightEntity for AppRenderView<State, Target>
{
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

// Source: upstream/packages/types/src/AppRenderView.ts:24 (sha256:994290ed5e0179a847533b9d9157f2d2e84d23acdbb120ec2f173974061decc2)
pub type AppRenderViewResize<State = RenderState, Target = RenderTargetDimensions> = std::sync::Arc<
    std::sync::Mutex<Box<dyn FnMut(State, Target, f64, f64) -> () + Send + 'static>>,
>;

// Source: upstream/packages/types/src/AppRenderView.ts:31 (sha256:28926c3fb9d36159d4d97d3646b933e025c32dfacef10ccd710befa606a3a9ac)
pub type AppRenderViewTargetOptions = FlightOmitRecord2663357065;

// Source: upstream/packages/types/src/AppRenderView.ts:33 (sha256:13029546d437bb3ee25cc3891573e02dbf115a92fc2a6c3102692845fb96d4d6)
#[derive(Clone, Default)]
pub struct GlAppRenderViewOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub context: Option<GlContextOptions>,
    pub render: Option<GlRenderStateOptions>,
    pub target: Option<AppRenderViewTargetOptions>,
}
impl PartialEq for GlAppRenderViewOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/AppRenderView.ts:39 (sha256:2344dfd512654d96f004c468bbd63d6849947289cef33c22822955e4598c233b)
pub type GlAppRenderView = AppRenderView<GlRenderState, GlTextureRenderTarget>;

// Source: upstream/packages/types/src/AppRenderView.ts:45 (sha256:ec7b14405c05b1b871a0b48de77b7006dde5e3e9330f1547090e47b90c09c060)
#[derive(Clone, Default)]
pub struct GlRenderViewResources {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    #[doc(hidden)]
    pub __flight_entity_snapshot: Option<std::sync::Arc<dyn std::any::Any + Send + Sync>>,
    pub render_state: GlRenderState,
    pub render_target: GlTextureRenderTarget,
    pub viewport: Viewport,
}
impl PartialEq for GlRenderViewResources {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for GlRenderViewResources {
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
