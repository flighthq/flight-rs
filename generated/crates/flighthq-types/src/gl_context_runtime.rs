// @generated from upstream/packages/types/src/GlContextRuntime.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{
    EntityRuntime, GlContext, GlParticleResources, GlQuadBatchResources, ImageResource,
    TextureSource,
};

// Source: upstream/packages/types/src/GlContextRuntime.ts:21 (sha256:0bdd0024e8e55488626e046fff6f25343b406be6920136d319f88c67de7b5c4d)
#[derive(Clone, Default)]
pub struct GlContextRuntimeRecord1 {
    pub __flight_identity: std::sync::Arc<()>,
    pub texture: crate::OpaqueHostValue,
    pub uploaded_version: f64,
}
impl PartialEq for GlContextRuntimeRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct GlContextRuntimeRecord2 {
    pub __flight_identity: std::sync::Arc<()>,
    pub texture: crate::OpaqueHostValue,
    pub version: f64,
}
impl PartialEq for GlContextRuntimeRecord2 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[doc(hidden)]
pub struct GlContextRuntimeStorage {
    pub teardowns:
        Vec<std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(GlContext) -> () + Send + 'static>>>>,
    pub texture_cache: Vec<(crate::OpaqueHostValue, crate::OpaqueHostValue)>,
    pub texture_source_premultiplied_texture_cache: Vec<(TextureSource, GlContextRuntimeRecord2)>,
    pub texture_source_premultiplied_srgb_texture_cache:
        Vec<(TextureSource, GlContextRuntimeRecord2)>,
    pub texture_source_straight_texture_cache: Vec<(TextureSource, GlContextRuntimeRecord2)>,
    pub texture_source_straight_srgb_texture_cache: Vec<(TextureSource, GlContextRuntimeRecord2)>,
    pub video_texture_cache: Option<Vec<(ImageResource, GlContextRuntimeRecord1)>>,
    pub video_srgb_texture_cache: Option<Vec<(ImageResource, GlContextRuntimeRecord1)>>,
    pub particle_resources: Option<GlParticleResources>,
    pub quad_batch_resources: Option<GlQuadBatchResources>,
}
impl Default for GlContextRuntimeStorage {
    fn default() -> Self {
        Self {
            teardowns: Default::default(),
            texture_cache: Default::default(),
            texture_source_premultiplied_texture_cache: Default::default(),
            texture_source_premultiplied_srgb_texture_cache: Default::default(),
            texture_source_straight_texture_cache: Default::default(),
            texture_source_straight_srgb_texture_cache: Default::default(),
            video_texture_cache: Default::default(),
            video_srgb_texture_cache: Default::default(),
            particle_resources: Default::default(),
            quad_batch_resources: Default::default(),
        }
    }
}
pub type GlContextRuntime = crate::EntityRuntime;
