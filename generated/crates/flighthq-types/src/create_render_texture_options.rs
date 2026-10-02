// @generated from upstream/packages/types/src/CreateRenderTextureOptions.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{RenderTargetDepth, RenderTargetFormat, SamplerLike, TextureColorSpace, Vector2Like};

// Source: upstream/packages/types/src/CreateRenderTextureOptions.ts:7 (sha256:252b6cb5fae179e8469a2e7344ea40a6da88eee33f7fb113769c03220f0e8d31)
#[derive(Clone, Default)]
pub struct CreateRenderTextureOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub color_space: Option<TextureColorSpace>,
    pub color_attachments: Option<f64>,
    pub color_formats: Option<Vec<RenderTargetFormat>>,
    pub depth: Option<RenderTargetDepth>,
    pub flip_x: Option<bool>,
    pub flip_y: Option<bool>,
    pub format: Option<RenderTargetFormat>,
    pub height: f64,
    pub sample_count: Option<f64>,
    pub sampler: Option<SamplerLike>,
    pub uv_offset: Option<Vector2Like>,
    pub uv_rotation: Option<f64>,
    pub uv_scale: Option<Vector2Like>,
    pub width: f64,
}
impl PartialEq for CreateRenderTextureOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
