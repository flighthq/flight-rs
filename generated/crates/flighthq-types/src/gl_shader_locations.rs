// @generated from upstream/packages/types/src/GlShaderLocations.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{EntityRuntime, GlContext, GlRenderState, RenderProxy2D};

// Source: upstream/packages/types/src/GlShaderLocations.ts:4 (sha256:966bcc46298e490b6b57b9ed4eba613df279e74d504470ba591c2ac8163e5f94)
#[derive(Clone, Default)]
pub struct GlShaderLocations {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub program: crate::OpaqueHostValue,
    pub loc_position: f64,
    pub loc_tex_coord: f64,
    pub loc_matrix: crate::OpaqueHostValue,
    pub loc_alpha: crate::OpaqueHostValue,
    pub loc_color_scale: Option<crate::OpaqueHostValue>,
    pub loc_color_bias: Option<crate::OpaqueHostValue>,
    pub loc_has_color_scale_bias: Option<crate::OpaqueHostValue>,
    pub loc_texture: crate::OpaqueHostValue,
}
impl PartialEq for GlShaderLocations {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/GlShaderLocations.ts:16 (sha256:476aa745c70ffd77d6badfbc91c8ca1a4b8094ac8ab0fb8d294276c257c9f98f)
#[derive(Clone)]
pub struct GlBitmapShader {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    #[doc(hidden)]
    pub __flight_entity_snapshot: Option<std::sync::Arc<dyn std::any::Any + Send + Sync>>,
    pub program: crate::OpaqueHostValue,
    pub bind: std::sync::Arc<
        std::sync::Mutex<
            Box<dyn FnMut(GlContext, GlRenderState, RenderProxy2D) -> () + Send + 'static>,
        >,
    >,
    pub locations: GlShaderLocations,
}
impl PartialEq for GlBitmapShader {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for GlBitmapShader {
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
