// @generated from upstream/packages/types/src/WgpuRenderTarget.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/WgpuRenderTarget.ts:1 (sha256:4119446d9d5f93e81e22de7f2e8bc53ba65c91d006b051e88cca86d41991e07f)
#[derive(Clone)]
pub struct WgpuRenderTarget {
    pub bind_group: crate::OpaqueHostValue,
    pub depth_stencil_texture: crate::OpaqueHostValue,
    pub depth_stencil_view: crate::OpaqueHostValue,
    pub format: crate::OpaqueHostValue,
    pub height: f64,
    pub clear_colors: Vec<f64>,
    pub clear_depth: f64,
    pub texture: crate::OpaqueHostValue,
    pub view: crate::OpaqueHostValue,
    pub width: f64,
}

// Source: upstream/packages/types/src/WgpuRenderTarget.ts:21 (sha256:e26b8edd9106ce50ee4389b3903f4ce932dffc2f17e8369ef7e4f64bcb8b4b53)
#[derive(Clone)]
pub struct WgpuRenderTargetPool {
    pub free: Vec<WgpuRenderTarget>,
}
