// @generated from upstream/packages/types/src/WgpuRenderTarget.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::RenderTargetColorSpace;

// Source: upstream/packages/types/src/WgpuRenderTarget.ts:3 (sha256:d5e40ef824804481c0135b2b35a6745fc6d84140f5c43fb4644b3a8af5a12b45)
#[derive(Clone, Default)]
pub struct WgpuRenderTarget {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub height: f64,
    pub width: f64,
    pub bind_group: crate::OpaqueHostValue,
    pub color_space: RenderTargetColorSpace,
    pub depth_stencil_texture: crate::OpaqueHostValue,
    pub depth_stencil_view: crate::OpaqueHostValue,
    pub format: crate::OpaqueHostValue,
    pub clear_colors: Vec<f64>,
    pub clear_depth: f64,
    pub texture: crate::OpaqueHostValue,
    pub view: crate::OpaqueHostValue,
}
impl PartialEq for WgpuRenderTarget {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/WgpuRenderTarget.ts:26 (sha256:e26b8edd9106ce50ee4389b3903f4ce932dffc2f17e8369ef7e4f64bcb8b4b53)
#[derive(Clone, Default)]
pub struct WgpuRenderTargetPool {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub free: Vec<WgpuRenderTarget>,
}
impl PartialEq for WgpuRenderTargetPool {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
