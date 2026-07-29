// @generated from upstream/packages/render-wgpu/src/wgpuDraw.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{
    generate_wgpu_mipmaps, get_active_wgpu_pipeline, get_wgpu_mip_level_count, get_wgpu_pipeline,
    get_wgpu_render_state_runtime, write_wgpu_quad_uniforms,
};
use flighthq_types::{
    BLEND_MODE as blend_mode_constant, ColorTransform, DisplayObjectClipHooks, ImageResource,
    Matrix, RenderProxy, RenderProxy2D, SceneGraphSyncPolicy, WgpuImageResourceTextureEntry,
    WgpuRenderState, WgpuTextureEntry,
};

#[derive(Clone, Default)]
pub struct SharedStructuralRecord1 {
    pub __flight_identity: std::sync::Arc<()>,
    pub a: f64,
    pub b: f64,
    pub c: f64,
    pub d: f64,
    pub tx: f64,
    pub ty: f64,
}
impl PartialEq for SharedStructuralRecord1 {
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

#[derive(Clone, Default)]
pub struct FlightPartialRecord3 {
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
impl PartialEq for FlightPartialRecord3 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/render-wgpu/src/wgpuDraw.ts:17 (sha256:caed0e0fe9177b377165c6756217c0ab3aa4717abcaa886f4a6407643ee432e1)
pub fn apply_wgpu_blend_mode(state: &WgpuRenderState, blend_mode: Option<BlendMode>) -> () {
    {
        let __flight_runtime = get_wgpu_render_state_runtime(state);
        let __flight_value = (blend_mode).clone();
        let mut __flight_storage = __flight_runtime.inner.lock().unwrap();
        __flight_storage.current_blend_mode = __flight_value;
    };
}

// Source: upstream/packages/render-wgpu/src/wgpuDraw.ts:28 (sha256:b014e6760cfb1d9dc4d500959f07306bc0371b5c711fddbea9da64d0b88802c6)
pub fn bind_wgpu_image_resource_texture(
    state: &WgpuRenderState,
    image: &ImageResource,
    generate_mips: Option<bool>,
) -> WgpuTextureEntry {
    let generate_mips = generate_mips.unwrap_or(false);
    let mut cached = get_wgpu_render_state_runtime(state)
        .inner
        .lock()
        .unwrap()
        .wgpu_render_state_runtime
        .image_resource_texture_cache
        .iter()
        .find(|(key, _)| key == &(*image).clone())
        .map(|(_, value)| value.clone());
    if ((cached).is_some()) && (cached.as_mut().unwrap().version == image.version) {
        return (cached).clone().unwrap();
    }
    let built = upload_wgpu_image_resource_entry(state, image, generate_mips);
    if (cached).is_some() {
        crate::host_value::<()>("host.destroy");
        cached.as_mut().unwrap().texture = (built.texture).clone();
        cached.as_mut().unwrap().view = (built.view).clone();
        cached.as_mut().unwrap().bind_group = (built.bind_group).clone();
        cached.as_mut().unwrap().version = image.version;
        return {
            let __flight_source = &(cached.as_mut().unwrap());
            WgpuTextureEntry {
                __flight_identity: std::sync::Arc::clone(&__flight_source.__flight_identity),
                bind_group: (__flight_source.bind_group).clone(),
                texture: (__flight_source.texture).clone(),
                view: (__flight_source.view).clone(),
            }
        };
    }
    let entry: WgpuImageResourceTextureEntry = {
        let __flight_spread_0 = built;
        WgpuImageResourceTextureEntry {
            __flight_identity: std::sync::Arc::new(()),
            bind_group: (__flight_spread_0.bind_group).clone(),
            texture: (__flight_spread_0.texture).clone(),
            view: (__flight_spread_0.view).clone(),
            version: image.version,
        }
    };
    {
        let __flight_key = (*image).clone();
        let __flight_value = (entry).clone();
        if let Some((_, value)) = get_wgpu_render_state_runtime(state)
            .inner
            .lock()
            .unwrap()
            .wgpu_render_state_runtime
            .image_resource_texture_cache
            .iter_mut()
            .find(|(key, _)| key == &__flight_key)
        {
            *value = __flight_value;
        } else {
            get_wgpu_render_state_runtime(state)
                .inner
                .lock()
                .unwrap()
                .wgpu_render_state_runtime
                .image_resource_texture_cache
                .push((__flight_key, __flight_value));
        }
    };
    return {
        let __flight_source = &(entry);
        WgpuTextureEntry {
            __flight_identity: std::sync::Arc::clone(&__flight_source.__flight_identity),
            bind_group: (__flight_source.bind_group).clone(),
            texture: (__flight_source.texture).clone(),
            view: (__flight_source.view).clone(),
        }
    };
}

// Source: upstream/packages/render-wgpu/src/wgpuDraw.ts:57 (sha256:b1417cdbab303aa15d3dec7ccbeaac4012847a400e3db783ac3d3d2cb2d7b77c)
#[derive(Clone, Default)]
struct BindWgpuTextureRecord4 {
    __flight_identity: std::sync::Arc<()>,
    source: crate::OpaqueHostValue,
    flip_y: bool,
}
impl PartialEq for BindWgpuTextureRecord4 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn bind_wgpu_texture(
    state: &WgpuRenderState,
    image_source: crate::OpaqueHostValue,
    generate_mips: Option<bool>,
) -> WgpuTextureEntry {
    let generate_mips = generate_mips.unwrap_or(false);
    let mut runtime = get_wgpu_render_state_runtime(state);
    let cached = runtime
        .inner
        .lock()
        .unwrap()
        .wgpu_render_state_runtime
        .texture_cache
        .iter()
        .find(|(key, _)| key == &(image_source).clone())
        .map(|(_, value)| value.clone());
    if (cached).is_some() {
        return ((cached.as_ref().unwrap()).clone()).clone();
    }
    let device = (state.device).clone();
    let texture_bind_group_layout =
        (runtime.inner.lock().unwrap().texture_bind_group_layout).clone();
    let mut width = 1.0_f64;
    let mut height = 1.0_f64;
    if false {
        width = (crate::host_value::<crate::OpaqueHostValue>("host.width") || 1.0_f64);
        height = (crate::host_value::<crate::OpaqueHostValue>("host.height") || 1.0_f64);
    } else {
        if false {
            width = (crate::host_value::<crate::OpaqueHostValue>("host.naturalWidth") || 1.0_f64);
            height = (crate::host_value::<crate::OpaqueHostValue>("host.naturalHeight") || 1.0_f64);
        } else {
            if false {
                width = (crate::host_value::<crate::OpaqueHostValue>("host.videoWidth") || 1.0_f64);
                height =
                    (crate::host_value::<crate::OpaqueHostValue>("host.videoHeight") || 1.0_f64);
            } else {
                if false {
                    width = (crate::host_value::<crate::OpaqueHostValue>("host.width") || 1.0_f64);
                    height =
                        (crate::host_value::<crate::OpaqueHostValue>("host.height") || 1.0_f64);
                } else {
                }
            }
        }
    }
    let mip_level_count = if generate_mips {
        get_wgpu_mip_level_count(width, height)
    } else {
        1.0_f64
    };
    let texture = crate::host_value::<()>("host.createTexture");
    crate::host_value::<()>("host.copyExternalImageToTexture");
    if (mip_level_count > 1.0_f64) {
        generate_wgpu_mipmaps(
            state,
            (texture).clone(),
            width,
            height,
            crate::OpaqueHostValue::String("rgba8unorm".to_owned()),
        );
    }
    let view = crate::host_value::<()>("host.createView");
    let sampler = if state.allow_smoothing {
        (runtime.inner.lock().unwrap().linear_sampler).clone()
    } else {
        (runtime.inner.lock().unwrap().nearest_sampler).clone()
    };
    let bind_group = crate::host_value::<()>("host.createBindGroup");
    let entry: WgpuTextureEntry = WgpuTextureEntry {
        __flight_identity: std::sync::Arc::new(()),
        texture: (texture).clone(),
        view: (view).clone(),
        bind_group: (bind_group).clone(),
    };
    {
        let __flight_key = (image_source).clone();
        let __flight_value = (entry).clone();
        if let Some((_, value)) = runtime
            .inner
            .lock()
            .unwrap()
            .wgpu_render_state_runtime
            .texture_cache
            .iter_mut()
            .find(|(key, _)| key == &__flight_key)
        {
            *value = __flight_value;
        } else {
            runtime
                .inner
                .lock()
                .unwrap()
                .wgpu_render_state_runtime
                .texture_cache
                .push((__flight_key, __flight_value));
        }
    };
    return entry;
}

// Source: upstream/packages/render-wgpu/src/wgpuDraw.ts:129 (sha256:f888118c2be5e0b5c356ee0de6feaa3dd4cee12b4e071fd85ed9a1fa4644dd3b)
pub fn build_wgpu_render_target_bind_group(
    state: &WgpuRenderState,
    view: crate::OpaqueHostValue,
) -> crate::OpaqueHostValue {
    let runtime = get_wgpu_render_state_runtime(state);
    let sampler = if state.allow_smoothing {
        (runtime.inner.lock().unwrap().linear_sampler).clone()
    } else {
        (runtime.inner.lock().unwrap().nearest_sampler).clone()
    };
    return crate::host_value::<crate::OpaqueHostValue>("host.createBindGroup");
}

// Source: upstream/packages/render-wgpu/src/wgpuDraw.ts:141 (sha256:2170e0011854d85d132a7459d70649a6a8d67d85d0ec221e19eea9d46a0774f2)
#[derive(Clone, Default)]
struct CreateWgpuTextureEntryRecord4 {
    __flight_identity: std::sync::Arc<()>,
    source: crate::OpaqueHostValue,
    flip_y: bool,
}
impl PartialEq for CreateWgpuTextureEntryRecord4 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn create_wgpu_texture_entry(
    state: &WgpuRenderState,
    width: f64,
    height: f64,
    canvas: crate::OpaqueHostValue,
) -> WgpuTextureEntry {
    let runtime = get_wgpu_render_state_runtime(state);
    let device = (state.device).clone();
    let texture_bind_group_layout =
        (runtime.inner.lock().unwrap().texture_bind_group_layout).clone();
    let w = (1.0_f64).max(width);
    let h = (1.0_f64).max(height);
    let texture = crate::host_value::<()>("host.createTexture");
    crate::host_value::<()>("host.copyExternalImageToTexture");
    let view = crate::host_value::<()>("host.createView");
    let sampler = if state.allow_smoothing {
        (runtime.inner.lock().unwrap().linear_sampler).clone()
    } else {
        (runtime.inner.lock().unwrap().nearest_sampler).clone()
    };
    let bind_group = crate::host_value::<()>("host.createBindGroup");
    return WgpuTextureEntry {
        __flight_identity: std::sync::Arc::new(()),
        texture: (texture).clone(),
        view: (view).clone(),
        bind_group: (bind_group).clone(),
    };
}

// Source: upstream/packages/render-wgpu/src/wgpuDraw.ts:179 (sha256:de9feb211c9ed8dc32347536c51630218824519e0e7194f1990933555fef5270)
#[derive(Clone, Default)]
struct RenderProxyContextRecord4 {
    __flight_identity: std::sync::Arc<()>,
    alpha: f64,
    transform2_d: SharedStructuralRecord1,
}
impl PartialEq for RenderProxyContextRecord4 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn draw_wgpu_quad(
    state: &WgpuRenderState,
    render_proxy: &RenderProxy2D,
    texture_entry: &WgpuTextureEntry,
    x0: f64,
    y0: f64,
    x1: f64,
    y1: f64,
    u0: f64,
    v0: f64,
    u1: f64,
    v1: f64,
) -> () {
    let runtime = get_wgpu_render_state_runtime(state);
    let pass = (runtime.inner.lock().unwrap().render_pass).clone();
    if (pass).is_none() {
        return;
    }
    let uniform_offset = write_wgpu_quad_uniforms(
        state,
        &{
            let __flight_source = &(render_proxy);
            RenderProxyContextRecord4 {
                __flight_identity: std::sync::Arc::clone(&__flight_source.__flight_identity),
                alpha: __flight_source.alpha,
                transform2_d: (__flight_source.transform2_d).clone(),
            }
        },
        (get_wgpu_render_proxy_color_transform(&{
            let __flight_source = &(render_proxy);
            RenderProxy {
                __flight_identity: std::sync::Arc::clone(&__flight_source.__flight_identity),
                __flight_entity_runtime: std::sync::Arc::clone(
                    &__flight_source.__flight_entity_runtime,
                ),
                source: (__flight_source.source).clone(),
                kind: (__flight_source.kind).clone(),
                next: (__flight_source.next).clone(),
                alpha: __flight_source.alpha,
                appearance_frame_id: __flight_source.appearance_frame_id,
                blend_mode: (__flight_source.blend_mode).clone(),
                color_transform: (__flight_source.color_transform).clone(),
                material: (__flight_source.material).clone(),
                material_data: (__flight_source.material_data).clone(),
                last_appearance_id: __flight_source.last_appearance_id,
                last_local_content_id: __flight_source.last_local_content_id,
                last_local_transform_id: __flight_source.last_local_transform_id,
                name: (__flight_source.name).clone(),
                renderer: (__flight_source.renderer).clone(),
                renderer_data: (__flight_source.renderer_data).clone(),
                renderer_data_source: (__flight_source.renderer_data_source).clone(),
                renderer_map_id: __flight_source.renderer_map_id,
                transform_frame_id: __flight_source.transform_frame_id,
                visible: __flight_source.visible,
            }
        }))
        .clone(),
        x0,
        y0,
        x1,
        y1,
        u0,
        v0,
        u1,
        v1,
    );
    submit_wgpu_quad_draw(state, uniform_offset, (texture_entry.bind_group).clone());
}

// Source: upstream/packages/render-wgpu/src/wgpuDraw.ts:212 (sha256:e91b15ec9fefcde67a7658a1b0439a9a0fd456b749afc28db7c06508012e6c49)
#[derive(Clone, Default)]
struct RenderProxyContextRecord4 {
    __flight_identity: std::sync::Arc<()>,
    alpha: f64,
    transform2_d: SharedStructuralRecord1,
}
impl PartialEq for RenderProxyContextRecord4 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn draw_wgpu_quad_with_transform(
    state: &WgpuRenderState,
    render_proxy: &RenderProxy,
    transform: &SharedStructuralRecord1,
    texture_entry: &WgpuTextureEntry,
    x0: f64,
    y0: f64,
    x1: f64,
    y1: f64,
    u0: f64,
    v0: f64,
    u1: f64,
    v1: f64,
) -> () {
    let runtime = get_wgpu_render_state_runtime(state);
    if ((runtime.inner.lock().unwrap().render_pass).clone()).is_none() {
        return;
    }
    let uniform_offset = write_wgpu_quad_uniforms(
        state,
        &RenderProxyContextRecord4 {
            __flight_identity: std::sync::Arc::new(()),
            alpha: render_proxy.alpha,
            transform2_d: (*transform).clone(),
        },
        (get_wgpu_render_proxy_color_transform(render_proxy)).clone(),
        x0,
        y0,
        x1,
        y1,
        u0,
        v0,
        u1,
        v1,
    );
    submit_wgpu_quad_draw(state, uniform_offset, (texture_entry.bind_group).clone());
}

// Source: upstream/packages/render-wgpu/src/wgpuDraw.ts:245 (sha256:76eeeb74a1fac9d26d8b12c6de68a1f0e642edff9554ec0ca093909878011fde)
pub fn enable_wgpu_blend_mode_support(state: &mut WgpuRenderState) -> () {
    state.apply_blend_mode = Some(std::sync::Arc::new(std::sync::Mutex::new(Box::new(
        move |__flight_argument_0: WgpuRenderState, __flight_argument_1: Option<BlendMode>| -> () {
            apply_wgpu_blend_mode(
                &__flight_argument_0,
                ((__flight_argument_1).clone()).clone(),
            )
        },
    )
        as Box<dyn FnMut(WgpuRenderState, Option<BlendMode>) -> () + Send + 'static>)));
}

// Source: upstream/packages/render-wgpu/src/wgpuDraw.ts:251 (sha256:165d326d4807ba09fefd41ccfb796da0349cfc946dd949752c664d068de3b2f5)
pub fn get_wgpu_render_proxy_color_transform(render_proxy: &RenderProxy) -> Option<ColorTransform> {
    return Some((render_proxy.color_transform).clone());
}

// Source: upstream/packages/render-wgpu/src/wgpuDraw.ts:255 (sha256:6f45d01b4e91f6c292b2363032eb9c770b821b38728342e466b4bac365b56a3c)
pub fn submit_wgpu_quad_draw(
    state: &WgpuRenderState,
    uniform_offset: f64,
    texture_bind_group: crate::OpaqueHostValue,
) -> () {
    let runtime = get_wgpu_render_state_runtime(state);
    let pass = (runtime.inner.lock().unwrap().render_pass).clone();
    if (pass).is_none() {
        return;
    }
    let pipeline = get_active_wgpu_pipeline(state);
    crate::host_value::<()>("host.setPipeline");
    crate::host_value::<()>("host.setBindGroup");
    crate::host_value::<()>("host.setBindGroup");
    if (runtime.inner.lock().unwrap().current_mask_depth > 0.0_f64) {
        crate::host_value::<()>("host.setStencilReference");
    }
    crate::host_value::<()>("host.draw");
}

// Source: upstream/packages/render-wgpu/src/wgpuDraw.ts:271 (sha256:23ed194e1b16fc4a7905230ff821da1695bb59468832a79103ee5fb3bc2150ec)
pub fn update_wgpu_texture_entry(
    state: &WgpuRenderState,
    entry: &WgpuTextureEntry,
    canvas: crate::OpaqueHostValue,
) -> () {
    let device = (state.device).clone();
    let w = (1.0_f64).max(crate::host_value::<crate::OpaqueHostValue>("host.width"));
    let h = (1.0_f64).max(crate::host_value::<crate::OpaqueHostValue>("host.height"));
    crate::host_value::<()>("host.copyExternalImageToTexture");
}

// Source: upstream/packages/render-wgpu/src/wgpuDraw.ts:288 (sha256:7c1ebb94dc0e895708da1bffbdd95ab1b5d4bf7c0edd31559e5d6236c48ba06e)
pub fn warm_wgpu_pipelines(state: &WgpuRenderState) -> () {
    get_wgpu_pipeline(
        state,
        Some((blend_mode_constant.normal).clone()),
        "normal".to_owned(),
    );
    get_wgpu_pipeline(
        state,
        Some((blend_mode_constant.add).clone()),
        "normal".to_owned(),
    );
}

// Source: upstream/packages/render-wgpu/src/wgpuDraw.ts:295 (sha256:fbbefbf7b0fa9df681fdaa726878b9894066dc559e57cc37fcda190d3c7e34d9)
fn premultiply_straight_rgba8(data: &Vec<u8>) -> Vec<u8> {
    let mut out: Vec<u8> = vec![0_u8; (data.len() as f64) as usize];
    {
        let mut i = 0.0_f64;
        while (i < (data.len() as f64)) {
            let a = (data[(i + 3.0_f64) as usize] as f64);
            out[i as usize] = (((data[i as usize] as f64) * a) / 255.0_f64) as u8;
            out[(i + 1.0_f64) as usize] =
                (((data[(i + 1.0_f64) as usize] as f64) * a) / 255.0_f64) as u8;
            out[(i + 2.0_f64) as usize] =
                (((data[(i + 2.0_f64) as usize] as f64) * a) / 255.0_f64) as u8;
            out[(i + 3.0_f64) as usize] = ((a).clone()) as u8;
            {
                i += 4.0_f64;
                i
            };
        }
    }
    return out;
}

// Source: upstream/packages/render-wgpu/src/wgpuDraw.ts:311 (sha256:5e1613feb144567ce2126c4e9e2a6bc3b7e894ae8ddc0b3b325b3caccdc952b5)
#[derive(Clone, Default)]
struct UploadWgpuImageResourceEntryRecord4 {
    __flight_identity: std::sync::Arc<()>,
    source: crate::OpaqueHostValue,
    flip_y: bool,
}
impl PartialEq for UploadWgpuImageResourceEntryRecord4 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

fn upload_wgpu_image_resource_entry(
    state: &WgpuRenderState,
    image: &ImageResource,
    generate_mips: bool,
) -> WgpuTextureEntry {
    let runtime = get_wgpu_render_state_runtime(state);
    let device = (state.device).clone();
    let width = if (image.width) != 0.0_f64 {
        image.width
    } else {
        1.0_f64
    };
    let height = if (image.height) != 0.0_f64 {
        image.height
    } else {
        1.0_f64
    };
    let mip_level_count = if generate_mips {
        get_wgpu_mip_level_count(width, height)
    } else {
        1.0_f64
    };
    let texture = crate::host_value::<()>("host.createTexture");
    if ((image.source).clone()).is_some() {
        crate::host_value::<()>("host.copyExternalImageToTexture");
    } else {
        let data = if ((image.alpha_type).clone() == "straight") {
            premultiply_straight_rgba8(image.data.as_ref().unwrap())
        } else {
            ((image.data).clone()).unwrap()
        };
        crate::host_value::<()>("host.writeTexture");
    }
    if (mip_level_count > 1.0_f64) {
        generate_wgpu_mipmaps(
            state,
            (texture).clone(),
            width,
            height,
            crate::OpaqueHostValue::String("rgba8unorm".to_owned()),
        );
    }
    let view = crate::host_value::<()>("host.createView");
    let sampler = if state.allow_smoothing {
        (runtime.inner.lock().unwrap().linear_sampler).clone()
    } else {
        (runtime.inner.lock().unwrap().nearest_sampler).clone()
    };
    let bind_group = crate::host_value::<()>("host.createBindGroup");
    return WgpuTextureEntry {
        __flight_identity: std::sync::Arc::new(()),
        texture: (texture).clone(),
        view: (view).clone(),
        bind_group: (bind_group).clone(),
    };
}
