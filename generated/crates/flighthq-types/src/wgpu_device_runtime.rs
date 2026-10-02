// @generated from upstream/packages/types/src/WgpuDeviceRuntime.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{
    EntityRuntime, ImageResource, TextureSource, WgpuParticleResources, WgpuQuadBatchResources,
    WgpuTextureEntry, WgpuTextureSourceTextureEntry, WgpuVideoTextureEntry,
};

// Source: upstream/packages/types/src/WgpuDeviceRuntime.ts:13 (sha256:38e338e78480dfa1e4c0b157e8fd2bd086395694a49561c6ba8196abcf088f33)
#[derive(Clone, Default)]
pub struct WgpuDeviceRuntimeRecord1 {
    pub __flight_identity: std::sync::Arc<()>,
    pub bind_group_layout: crate::OpaqueHostValue,
    pub pipeline: crate::OpaqueHostValue,
}
impl PartialEq for WgpuDeviceRuntimeRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[doc(hidden)]
pub struct WgpuDeviceRuntimeStorage {
    pub teardowns: Vec<
        std::sync::Arc<
            std::sync::Mutex<Box<dyn FnMut(crate::OpaqueHostValue) -> () + Send + 'static>>,
        >,
    >,
    pub mipmap_pipeline_cache: Vec<(crate::OpaqueHostValue, WgpuDeviceRuntimeRecord1)>,
    pub texture_cache: Vec<(crate::OpaqueHostValue, WgpuTextureEntry)>,
    pub texture_source_premultiplied_texture_cache:
        Vec<(TextureSource, WgpuTextureSourceTextureEntry)>,
    pub texture_source_premultiplied_srgb_texture_cache:
        Vec<(TextureSource, WgpuTextureSourceTextureEntry)>,
    pub texture_source_straight_texture_cache: Vec<(TextureSource, WgpuTextureSourceTextureEntry)>,
    pub texture_source_straight_srgb_texture_cache:
        Vec<(TextureSource, WgpuTextureSourceTextureEntry)>,
    pub video_texture_cache: Option<Vec<(ImageResource, WgpuVideoTextureEntry)>>,
    pub video_srgb_texture_cache: Option<Vec<(ImageResource, WgpuVideoTextureEntry)>>,
    pub particle_resources: Option<WgpuParticleResources>,
    pub quad_batch_resources: Option<WgpuQuadBatchResources>,
}
impl Default for WgpuDeviceRuntimeStorage {
    fn default() -> Self {
        Self {
            teardowns: Default::default(),
            mipmap_pipeline_cache: Default::default(),
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
pub type WgpuDeviceRuntime = crate::EntityRuntime;

// Source: upstream/packages/types/src/WgpuDeviceRuntime.ts:50 (sha256:861239ac850cde9cc8c60f45366c8d28478995f24f17710fb63596cedea64b5e)
#[derive(Clone, Default)]
pub struct WgpuDeviceRuntimeResources {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub linear_sampler: crate::OpaqueHostValue,
    pub nearest_sampler: crate::OpaqueHostValue,
    pub texture_bind_group_layout: crate::OpaqueHostValue,
    pub uniform_bind_group_layout: crate::OpaqueHostValue,
}
impl PartialEq for WgpuDeviceRuntimeResources {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
