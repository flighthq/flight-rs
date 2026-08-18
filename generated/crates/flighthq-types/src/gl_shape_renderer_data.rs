// @generated from upstream/packages/types/src/GlShapeRendererData.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{GlShapeMesh, Image};

// Source: upstream/packages/types/src/GlShapeRendererData.ts:7 (sha256:106de80ca0ba81c8d7d820fd5e6287b21cea576240f6a61c12c7844d0cf9feb8)
#[derive(Clone, Default)]
pub struct GlShapeRasterSurface {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub canvas: crate::OpaqueHostValue,
    pub ctx: crate::OpaqueHostValue,
    pub image: Image,
}
impl PartialEq for GlShapeRasterSurface {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/GlShapeRendererData.ts:20 (sha256:ae1793eea0c5323a3989c3b263cf25e9d862080401783b4209e04677c86e3f65)
#[derive(Clone, Default)]
pub struct GlShapeRendererData {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub surface: Option<GlShapeRasterSurface>,
    pub last_content_id: f64,
    pub last_pixel_ratio: f64,
    pub last_w: f64,
    pub last_h: f64,
    pub mesh_version: f64,
    pub meshes: Option<Vec<GlShapeMesh>>,
}
impl PartialEq for GlShapeRendererData {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
