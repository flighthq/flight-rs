// @generated from upstream/packages/render-wgpu/src/wgpuRenderTargetPool.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{create_wgpu_render_target, destroy_wgpu_render_target};
use flighthq_types::{
    BlendMode, ColorTransform, DisplayObjectClipHooks, Matrix, SceneGraphSyncPolicy,
    WgpuRenderState, WgpuRenderTarget, WgpuRenderTargetPool,
};

#[derive(Clone, Default)]
pub struct SharedStructuralRecord1 {
    pub __flight_identity: std::sync::Arc<()>,
    pub width: f64,
    pub height: f64,
    pub format: Option<crate::OpaqueHostValue>,
}
impl PartialEq for SharedStructuralRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct FlightPartialRecord2 {
    pub __flight_identity: std::sync::Arc<()>,
    pub allow_smoothing: Option<bool>,
    pub background_color: Option<f64>,
    pub background_color_rgba: Option<Vec<f64>>,
    pub background_color_string: Option<String>,
    pub current_clip_depth: Option<f64>,
    pub display_object_clip_hooks: Option<DisplayObjectClipHooks>,
    pub pixel_ratio: Option<f64>,
    pub render_alpha: Option<f64>,
    pub render_blend_mode: Option<BlendMode>,
    pub render_transform2_d: Option<Matrix>,
    pub scene_graph_sync_policy: Option<SceneGraphSyncPolicy>,
    pub round_pixels: Option<bool>,
}
impl PartialEq for FlightPartialRecord2 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct FlightPartialRecord3 {
    pub __flight_identity: std::sync::Arc<()>,
    pub color_transform: Option<ColorTransform>,
}
impl PartialEq for FlightPartialRecord3 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/render-wgpu/src/wgpuRenderTargetPool.ts:12 (sha256:3db02aae2c72e8fd1767c2bc9de9be82c61f42b78427bcc9f298d2b100f78c13)
pub fn acquire_wgpu_render_target(
    state: &WgpuRenderState,
    pool: &mut WgpuRenderTargetPool,
    descriptor: &SharedStructuralRecord1,
) -> WgpuRenderTarget {
    let w = (1.0_f64).max((descriptor.width).ceil());
    let h = (1.0_f64).max((descriptor.height).ceil());
    let format = ((descriptor.format).clone()).unwrap_or((state.format).clone());
    {
        let mut i = 0.0_f64;
        while (i < (pool.free.len() as f64)) {
            let candidate = pool.free[i as usize].clone();
            if ((candidate.width == w) && (candidate.height == h))
                && ((candidate.format).clone() == format)
            {
                pool.free
                    .splice((i) as usize..((i) + (1.0_f64)) as usize, vec![]);
                return candidate;
            }
            {
                i += 1.0;
                i
            };
        }
    }
    return create_wgpu_render_target(state, w, h, Some(((format).clone()).clone()));
}

// Source: upstream/packages/render-wgpu/src/wgpuRenderTargetPool.ts:31 (sha256:2f3a976ed213c48797aafbc410d8d12c43aac1ef6130b3d1ea8082bc24c66255)
pub fn create_wgpu_render_target_pool() -> WgpuRenderTargetPool {
    return WgpuRenderTargetPool {
        __flight_identity: std::sync::Arc::new(()),
        free: vec![],
    };
}

// Source: upstream/packages/render-wgpu/src/wgpuRenderTargetPool.ts:35 (sha256:e3d784c955da9196c5f680abaffd47f39ca7ab2e554694f107dd39d448f8dbbd)
pub fn destroy_wgpu_render_target_pool(
    state: &WgpuRenderState,
    pool: &mut WgpuRenderTargetPool,
) -> () {
    for target in ((pool.free).clone()).iter().cloned() {
        destroy_wgpu_render_target(state, &target);
    }
    pool.free.clear();
}

// Source: upstream/packages/render-wgpu/src/wgpuRenderTargetPool.ts:40 (sha256:0dc5470bf9894c239fa8cba2ec691450e1747391a3db44a63bb6322899daa346)
pub fn release_wgpu_render_target(
    pool: &mut WgpuRenderTargetPool,
    target: &WgpuRenderTarget,
) -> () {
    pool.free.push(((*target).clone()).clone());
}
