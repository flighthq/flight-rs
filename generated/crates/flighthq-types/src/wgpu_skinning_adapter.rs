// @generated from upstream/packages/types/src/WgpuSkinningAdapter.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{Mesh, MeshGeometry, WgpuRenderState};

// Source: upstream/packages/types/src/WgpuSkinningAdapter.ts:5 (sha256:f711d111fe4120bc28738dcad2473c25874582a202a2c7c62088d3438162d0ce)
#[derive(Clone)]
pub struct WgpuSkinningAdapter {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub extend_mesh_prelude:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(String) -> String + Send + 'static>>>,
    pub extend_shadow_depth_prelude:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(String) -> String + Send + 'static>>>,
    pub get_draw_bind_group: std::sync::Arc<
        std::sync::Mutex<
            Box<dyn FnMut(WgpuRenderState, Vec<f32>) -> crate::OpaqueHostValue + Send + 'static>,
        >,
    >,
    pub get_draw_layout: std::sync::Arc<
        std::sync::Mutex<
            Box<dyn FnMut(WgpuRenderState) -> crate::OpaqueHostValue + Send + 'static>,
        >,
    >,
    pub get_upload_vertices: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut(MeshGeometry) -> Option<Vec<f32>> + Send + 'static>>,
    >,
    pub has_bind_pose:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(MeshGeometry) -> bool + Send + 'static>>>,
    pub is_gpu_skinned:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(Mesh) -> bool + Send + 'static>>>,
    pub vertex_buffer_layouts: Vec<crate::OpaqueHostValue>,
}
impl PartialEq for WgpuSkinningAdapter {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
