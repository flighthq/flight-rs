// @generated from upstream/packages/types/src/WgpuShapeRendererData.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{Image, WgpuShapeMesh, WgpuShapeMeshBuffers};

// Source: upstream/packages/types/src/WgpuShapeRendererData.ts:9 (sha256:aa2460bc8817e1063225259a5afe70eeef50b8dce6723d3f72a52fe6632c2cc8)
#[derive(Clone, Default)]
pub struct WgpuShapeRasterSurface {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub canvas: crate::OpaqueHostValue,
    pub ctx: crate::OpaqueHostValue,
    pub image: Image,
}
impl PartialEq for WgpuShapeRasterSurface {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/WgpuShapeRendererData.ts:23 (sha256:8bf31aa0b755de712e58851fc670470e5a3027dcca83854e402fb31490ea8a01)
#[derive(Clone, Default)]
pub struct WgpuShapeRendererData {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub surface: Option<WgpuShapeRasterSurface>,
    pub last_content_id: f64,
    pub last_pixel_ratio: f64,
    pub last_w: f64,
    pub last_h: f64,
    pub mesh_version: f64,
    pub meshes: Option<Vec<WgpuShapeMesh>>,
    pub mesh_buffers: WgpuShapeMeshBuffers,
}
impl PartialEq for WgpuShapeRendererData {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
