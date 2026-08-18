// @generated from upstream/packages/types/src/CanvasTextureResolver.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{Bitmap, Kind, RenderRegistry, Texture, TextureSourceKind};

// Source: upstream/packages/types/src/CanvasTextureResolver.ts:7 (sha256:67692df08bcef88ba0edfb6d1ec9be31c5515ebde25f4e372ad757f39796cfb9)
pub type CanvasTextureResolver = std::sync::Arc<
    std::sync::Mutex<
        Box<
            dyn FnMut(CanvasTextureResolvers, Texture) -> Option<crate::OpaqueHostValue>
                + Send
                + 'static,
        >,
    >,
>;

// Source: upstream/packages/types/src/CanvasTextureResolver.ts:23 (sha256:f60a25b918ccf0e081d0b367d63c9ac47bcfeb8b5587c5bfa07de896892b07c8)
#[derive(Clone, Default)]
pub struct CanvasTextureResolversRecord1 {
    pub __flight_identity: std::sync::Arc<()>,
    pub element: crate::OpaqueHostValue,
    pub flip_x: bool,
    pub flip_y: bool,
    pub image_version: f64,
    pub source: crate::OpaqueHostValue,
    pub texture_version: f64,
    pub uv_offset_x: f64,
    pub uv_offset_y: f64,
    pub uv_rotation: f64,
    pub uv_scale_x: f64,
    pub uv_scale_y: f64,
}
impl PartialEq for CanvasTextureResolversRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct CanvasTextureResolversRecord2 {
    pub __flight_identity: std::sync::Arc<()>,
    pub element: crate::OpaqueHostValue,
    pub version: f64,
}
impl PartialEq for CanvasTextureResolversRecord2 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct CanvasTextureResolvers {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub registry: Option<Vec<(TextureSourceKind, CanvasTextureResolver)>>,
    pub bitmap_element_cache: Option<Vec<(Bitmap, CanvasTextureResolversRecord2)>>,
    pub texture_window_element_cache: Option<Vec<(Texture, CanvasTextureResolversRecord1)>>,
    pub registry_miss: Option<
        std::sync::Arc<
            std::sync::Mutex<Box<dyn FnMut(RenderRegistry, Kind) -> () + Send + 'static>>,
        >,
    >,
}
impl PartialEq for CanvasTextureResolvers {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
