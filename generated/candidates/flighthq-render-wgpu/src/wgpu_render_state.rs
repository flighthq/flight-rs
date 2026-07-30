// @generated from upstream/packages/render-wgpu/src/wgpuRenderState.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_render::create_render_state_runtime;
use flighthq_types::{
    BlendMode, ColorTransform, DisplayObjectClipHooks, Matrix, SceneGraphSyncPolicy, TextureWrap,
    WgpuRenderOptions, WgpuRenderState, WgpuRenderStateRuntime,
};

#[derive(Clone, Default)]
pub struct FlightPartialRecord1 {
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
impl PartialEq for FlightPartialRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct FlightPartialRecord2 {
    pub __flight_identity: std::sync::Arc<()>,
    pub color_transform: Option<ColorTransform>,
}
impl PartialEq for FlightPartialRecord2 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/render-wgpu/src/wgpuRenderState.ts:14 (sha256:fb7e7b9f8ef3d4213984ee2b46906f6382920b88af63f0a58526d45589931a6a)
const RING_SLOT_COUNT: f64 = 4096.0_f64;

// Source: upstream/packages/render-wgpu/src/wgpuRenderState.ts:16 (sha256:f307b9234dd0ff4f3ad7ab31be9b51f27d12ccdca81862275abe1da0f7c54223)
#[derive(Clone, Default)]
struct CreateWgpuRenderStateRecord3 {
    __flight_identity: std::sync::Arc<()>,
}
impl PartialEq for CreateWgpuRenderStateRecord3 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn create_wgpu_render_state(
    canvas: crate::OpaqueHostValue,
    options: Option<WgpuRenderOptions>,
) -> crate::Promise<WgpuRenderState> {
    Default::default()
}

// Source: upstream/packages/render-wgpu/src/wgpuRenderState.ts:170 (sha256:fae6b37511b78ca8790e6629ee47bfb55973e3500adaf6ca63a028edf0b9b5d0)
pub fn create_wgpu_render_state_runtime() -> WgpuRenderStateRuntime {
    return create_render_state_runtime();
}

// Source: upstream/packages/render-wgpu/src/wgpuRenderState.ts:183 (sha256:c59627f2ecd963c6aea0e99c465947a3fc11f04a61492ebae689bbc12887adbb)
pub fn destroy_wgpu_render_state(state: &WgpuRenderState) -> () {
    let runtime = get_wgpu_render_state_runtime(state);
    crate::host_value::<()>("host.destroy");
    crate::host_value::<()>("host.destroy");
    crate::host_value::<()>("host.destroy");
    for slot in ((runtime.inner.lock().unwrap().sprite_batch_buffer_pool).clone())
        .iter()
        .cloned()
    {
        crate::host_value::<()>("host.destroy");
        crate::host_value::<()>("host.destroy");
    }
}

// Source: upstream/packages/render-wgpu/src/wgpuRenderState.ts:196 (sha256:62d211b7dc372546661bdf1fcfff473682e1fa9d530abeaf3946cd2f9d123137)
pub fn get_wgpu_render_state_runtime(state: &WgpuRenderState) -> WgpuRenderStateRuntime {
    return ({
        let __flight_runtime = flighthq_types::FlightEntity::__flight_entity_runtime(state)
            .lock()
            .unwrap()
            .clone()
            .expect("entity runtime was read before initialization");
        __flight_runtime
    })
    .clone();
}

// Source: upstream/packages/render-wgpu/src/wgpuRenderState.ts:208 (sha256:f9bd9db88a90564a71007b26b0baae7b8d389733300789eae15fae1b59e59892)
#[derive(Clone, Default)]
struct GetWgpuSamplerSynthesizedRecord2686443658 {
    __flight_identity: std::sync::Arc<()>,
    address_mode_u: TextureWrap,
    address_mode_v: TextureWrap,
    mag_filter: String,
    min_filter: String,
}
impl PartialEq for GetWgpuSamplerSynthesizedRecord2686443658 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn get_wgpu_sampler(
    state: &WgpuRenderState,
    filter: crate::OpaqueHostValue,
    wrap_u: TextureWrap,
    wrap_v: TextureWrap,
    mipmap_filter: Option<crate::OpaqueHostValue>,
    max_anisotropy: Option<f64>,
) -> crate::OpaqueHostValue {
    let max_anisotropy = max_anisotropy.unwrap_or(1.0_f64);
    let mut runtime = get_wgpu_render_state_runtime(state);
    let anisotropy = (1.0_f64).max((max_anisotropy).floor());
    let effective_filter = if (anisotropy > 1.0_f64) {
        "linear".to_owned()
    } else {
        filter
    };
    let effective_mipmap_filter = if (anisotropy > 1.0_f64) {
        "linear".to_owned()
    } else {
        (mipmap_filter).clone().unwrap()
    };
    let key = format!(
        "{}|{}|{}|{}|{}",
        effective_filter, wrap_u, wrap_v, effective_mipmap_filter, anisotropy
    );
    let mut sampler = runtime
        .inner
        .lock()
        .unwrap()
        .sampler_cache
        .iter()
        .find(|(key, _)| key == &(key).clone())
        .map(|(_, value)| value.clone());
    if (sampler).is_none() {
        let mut descriptor = GetWgpuSamplerSynthesizedRecord2686443658 {
            __flight_identity: std::sync::Arc::new(()),
            min_filter: (effective_filter).clone(),
            mag_filter: (effective_filter).clone(),
            address_mode_u: (wrap_u).clone(),
            address_mode_v: (wrap_v).clone(),
        };
        if (effective_mipmap_filter).is_some() {
            descriptor.mipmap_filter = effective_mipmap_filter;
        }
        if (anisotropy > 1.0_f64) {
            descriptor.max_anisotropy = anisotropy;
        }
        sampler = Some(crate::host_value::<crate::OpaqueHostValue>(
            "host.createSampler",
        ));
        {
            let __flight_key = (key).clone();
            let __flight_value = (sampler).clone().unwrap();
            if let Some((_, value)) = runtime
                .inner
                .lock()
                .unwrap()
                .sampler_cache
                .iter_mut()
                .find(|(key, _)| key == &__flight_key)
            {
                *value = __flight_value;
            } else {
                runtime
                    .inner
                    .lock()
                    .unwrap()
                    .sampler_cache
                    .push((__flight_key, __flight_value));
            }
        };
    }
    return ((sampler).clone().unwrap()).clone();
}

// Source: upstream/packages/render-wgpu/src/wgpuRenderState.ts:237 (sha256:e11140e0ea3a22839dd9529ee9327c7c4b71c4d0c18fe34eb9661b2192539888)
pub fn is_wgpu_supported() -> bool {
    return (("undefined" != "undefined") && (false))
        && ((crate::host_value::<Option<crate::OpaqueHostValue>>("host.gpu")).is_some());
}
