// @generated from upstream/packages/render-wgpu/src/wgpuRenderTarget.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{
    build_wgpu_render_target_bind_group, draw_wgpu_quad_with_transform,
    get_wgpu_render_state_runtime,
};
use flighthq_geometry::{copy_matrix, create_matrix};
use flighthq_types::{
    BlendMode, ColorTransform, DisplayObjectClipHooks, Material, Matrix, MatrixLike,
    RenderPassPreserve, SceneGraphSyncPolicy, WgpuRenderState, WgpuRenderTarget,
    WgpuSavedPassState, WgpuTextureEntry,
};

#[inline]
fn __flight_js_to_u32(value: f64) -> u32 {
    if !value.is_finite() || value == 0.0 {
        return 0;
    }
    value.trunc().rem_euclid(4294967296.0_f64) as u32
}

#[inline]
fn __flight_js_to_i32(value: f64) -> i32 {
    __flight_js_to_u32(value) as i32
}

#[derive(Clone, Default)]
pub struct SharedStructuralRecord1 {
    pub __flight_identity: std::sync::Arc<()>,
    pub alpha: f64,
    pub material: Option<Material>,
    pub transform2_d: SharedStructuralRecord2,
}
impl PartialEq for SharedStructuralRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct SharedStructuralRecord2 {
    pub __flight_identity: std::sync::Arc<()>,
    pub a: f64,
    pub b: f64,
    pub c: f64,
    pub d: f64,
    pub tx: f64,
    pub ty: f64,
}
impl PartialEq for SharedStructuralRecord2 {
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

#[derive(Clone, Default)]
pub struct FlightPartialRecord4 {
    pub __flight_identity: std::sync::Arc<()>,
    pub color_transform: Option<ColorTransform>,
}
impl PartialEq for FlightPartialRecord4 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/render-wgpu/src/wgpuRenderTarget.ts:7 (sha256:a9d8cd336ab909392e0888773698339420eee445f7159d84edbd90e7558d566d)
#[derive(Clone, Default)]
struct BeginWgpuRenderPassEncoderRecord5 {
    __flight_identity: std::sync::Arc<()>,
    r: f64,
    g: f64,
    b: f64,
    a: f64,
}
impl PartialEq for BeginWgpuRenderPassEncoderRecord5 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
struct BeginWgpuRenderPassEncoderSynthesizedRecord1596336968 {
    __flight_identity: std::sync::Arc<()>,
    a: f64,
    b: f64,
    g: f64,
    r: f64,
}
impl PartialEq for BeginWgpuRenderPassEncoderSynthesizedRecord1596336968 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

fn begin_wgpu_render_pass_encoder(
    state: &WgpuRenderState,
    color_view: crate::OpaqueHostValue,
    depth_stencil_view: crate::OpaqueHostValue,
    width: f64,
    height: f64,
    load_op: crate::OpaqueHostValue,
    clear_color: Option<crate::OpaqueHostValue>,
    depth_load_op: Option<crate::OpaqueHostValue>,
    depth_clear_value: Option<f64>,
) -> crate::OpaqueHostValue {
    let clear_color =
        clear_color.unwrap_or(BeginWgpuRenderPassEncoderSynthesizedRecord1596336968 {
            __flight_identity: std::sync::Arc::new(()),
            r: 0.0_f64,
            g: 0.0_f64,
            b: 0.0_f64,
            a: 0.0_f64,
        });
    let depth_load_op = depth_load_op.unwrap_or(crate::OpaqueHostValue::String("clear".to_owned()));
    let depth_clear_value = depth_clear_value.unwrap_or(1.0_f64);
    let runtime = get_wgpu_render_state_runtime(state);
    let pass = crate::host_value::<()>("host.beginRenderPass");
    (pass.set_viewport)(0.0_f64, 0.0_f64, width, height, 0.0_f64, 1.0_f64);
    return pass;
}

// Source: upstream/packages/render-wgpu/src/wgpuRenderTarget.ts:41 (sha256:1d37fd12b00590fe50ded971aecf2596226741018d6397a28d176d136653e57e)
pub fn begin_wgpu_render_pass(
    state: &WgpuRenderState,
    target: &WgpuRenderTarget,
    preserve: Option<RenderPassPreserve>,
) -> () {
    let mut runtime = get_wgpu_render_state_runtime(state);
    if ((runtime.inner.lock().unwrap().render_pass).clone()).is_some() {
        crate::host_value::<()>("host.end");
        {
            let __flight_runtime = runtime;
            let __flight_value = None;
            let mut __flight_storage = __flight_runtime.inner.lock().unwrap();
            __flight_storage.render_pass = __flight_value;
        };
    }
    runtime
        .inner
        .lock()
        .unwrap()
        .render_target_stack
        .push(WgpuSavedPassState {
            __flight_identity: std::sync::Arc::new(()),
            canvas_texture_view: (runtime.inner.lock().unwrap().canvas_texture_view).clone(),
            canvas_view_cleared: runtime.inner.lock().unwrap().canvas_view_cleared,
            depth_stencil_view: (runtime.inner.lock().unwrap().depth_stencil_view).clone(),
            render_target_viewport: (runtime
                .inner
                .lock()
                .unwrap()
                .wgpu_render_state_runtime
                .render_target_viewport)
                .clone(),
            render_transform2_d: (state.render_transform2_d).clone(),
            color_format: (runtime.inner.lock().unwrap().current_color_format).clone(),
        });
    {
        let __flight_runtime = runtime;
        let __flight_value = Some(flighthq_types::WgpuRenderStateRuntimeRecord1 {
            __flight_identity: std::sync::Arc::new(()),
            width: target.width,
            height: target.height,
        });
        let mut __flight_storage = __flight_runtime.inner.lock().unwrap();
        __flight_storage
            .wgpu_render_state_runtime
            .render_target_viewport = __flight_value;
    };
    {
        let __flight_runtime = runtime;
        let __flight_value = Some((target.format).clone());
        let mut __flight_storage = __flight_runtime.inner.lock().unwrap();
        __flight_storage.current_color_format = __flight_value;
    };
    {
        let __flight_runtime = runtime;
        let __flight_value = 0.0_f64;
        let mut __flight_storage = __flight_runtime.inner.lock().unwrap();
        __flight_storage.current_mask_depth = __flight_value;
    };
    {
        let __flight_runtime = runtime;
        let __flight_value = false;
        let mut __flight_storage = __flight_runtime.inner.lock().unwrap();
        __flight_storage.mask_write_mode = __flight_value;
    };
    {
        let __flight_runtime = runtime;
        let __flight_value = None;
        let mut __flight_storage = __flight_runtime.inner.lock().unwrap();
        __flight_storage
            .wgpu_render_state_runtime
            .current_scissor_rect = __flight_value;
    };
    {
        let __flight_runtime = runtime;
        let __flight_value = vec![];
        let mut __flight_storage = __flight_runtime.inner.lock().unwrap();
        __flight_storage.wgpu_render_state_runtime.scissor_stack = __flight_value;
    };
    let color_load_op = if is_wgpu_color_preserved(
        &((preserve
            .as_ref()
            .and_then(|value| (value.preserve_color).clone()))
        .unwrap_or(crate::FlightUnion2::<bool, Vec<bool>>::A(false))),
        0.0_f64,
    ) {
        "load".to_owned()
    } else {
        "clear".to_owned()
    };
    let depth_load_op = if (preserve.as_ref().and_then(|value| value.preserve_depth)) == Some(true)
    {
        "load".to_owned()
    } else {
        "clear".to_owned()
    };
    {
        let __flight_runtime = runtime;
        let __flight_value = Some(begin_wgpu_render_pass_encoder(
            state,
            (target.view).clone(),
            (target.depth_stencil_view).clone(),
            target.width,
            target.height,
            (color_load_op).clone(),
            Some((resolve_wgpu_clear_color(target)).clone()),
            Some((depth_load_op).clone()),
            Some(target.clear_depth),
        ));
        let mut __flight_storage = __flight_runtime.inner.lock().unwrap();
        __flight_storage.render_pass = __flight_value;
    };
}

// Source: upstream/packages/render-wgpu/src/wgpuRenderTarget.ts:90 (sha256:76c45c75b1f28c6a0ceb68bc04580e5fd5fd1d0d3e258b47703267b23366a7f1)
pub fn create_wgpu_render_target(
    state: &WgpuRenderState,
    width: f64,
    height: f64,
    format: Option<crate::OpaqueHostValue>,
) -> WgpuRenderTarget {
    let format = format.unwrap_or((state.format).clone());
    let device = (state.device).clone();
    let w = (1.0_f64).max((width).ceil());
    let h = (1.0_f64).max((height).ceil());
    let texture = crate::host_value::<()>("host.createTexture");
    let view = crate::host_value::<()>("host.createView");
    let bind_group = build_wgpu_render_target_bind_group(state, (view).clone());
    let depth_stencil_texture = crate::host_value::<()>("host.createTexture");
    let depth_stencil_view = crate::host_value::<()>("host.createView");
    return WgpuRenderTarget {
        __flight_identity: std::sync::Arc::new(()),
        bind_group: (bind_group).clone(),
        texture: (texture).clone(),
        view: (view).clone(),
        depth_stencil_texture: (depth_stencil_texture).clone(),
        depth_stencil_view: (depth_stencil_view).clone(),
        format: (format).clone(),
        clear_colors: vec![],
        clear_depth: 1.0_f64,
        width: w,
        height: h,
    };
}

// Source: upstream/packages/render-wgpu/src/wgpuRenderTarget.ts:129 (sha256:e731f1823a7cbf90d4f580ffe2a8665412d9c1fd9b0f349998cefc697c439915)
pub fn destroy_wgpu_render_target(_state: &WgpuRenderState, target: &WgpuRenderTarget) -> () {
    crate::host_value::<()>("host.destroy");
    crate::host_value::<()>("host.destroy");
}

// Source: upstream/packages/render-wgpu/src/wgpuRenderTarget.ts:134 (sha256:dc4a5dd0308fa20ec0186d191f8d8fe202911cbaa447191201075f4df34cab97)
pub fn draw_wgpu_render_target_result(
    state: &WgpuRenderState,
    render_proxy: &SharedStructuralRecord1,
    target: &WgpuRenderTarget,
    transform: &Matrix,
) -> () {
    if (target.width <= 0.0_f64) || (target.height <= 0.0_f64) {
        return;
    }
    let runtime = get_wgpu_render_state_runtime(state);
    if ((runtime.inner.lock().unwrap().render_pass).clone()).is_none() {
        return;
    }
    let a = render_proxy.transform2_d.a;
    let b = render_proxy.transform2_d.b;
    let c = render_proxy.transform2_d.c;
    let d = render_proxy.transform2_d.d;
    let tx = render_proxy.transform2_d.tx;
    let ty = render_proxy.transform2_d.ty;
    let ta = transform.a;
    let tb = transform.b;
    let tc = transform.c;
    let td = transform.d;
    let ttx = transform.tx;
    let tty = transform.ty;
    let composed_transform = Matrix {
        __flight_identity: std::sync::Arc::new(()),
        __flight_entity_runtime: Default::default(),
        a: ((a * ta) + (c * tb)),
        b: ((b * ta) + (d * tb)),
        c: ((a * tc) + (c * td)),
        d: ((b * tc) + (d * td)),
        tx: (((a * ttx) + (c * tty)) + tx),
        ty: (((b * ttx) + (d * tty)) + ty),
    };
    draw_wgpu_quad_with_transform(
        state,
        &render_proxy,
        &{
            let __flight_source = &(composed_transform);
            SharedStructuralRecord2 {
                __flight_identity: std::sync::Arc::clone(&__flight_source.__flight_identity),
                a: __flight_source.a,
                b: __flight_source.b,
                c: __flight_source.c,
                d: __flight_source.d,
                tx: __flight_source.tx,
                ty: __flight_source.ty,
            }
        },
        &WgpuTextureEntry {
            __flight_identity: std::sync::Arc::new(()),
            texture: (target.texture).clone(),
            view: (target.view).clone(),
            bind_group: (target.bind_group).clone(),
        },
        0.0_f64,
        0.0_f64,
        target.width,
        target.height,
        0.0_f64,
        1.0_f64,
        1.0_f64,
        0.0_f64,
    );
}

// Source: upstream/packages/render-wgpu/src/wgpuRenderTarget.ts:182 (sha256:6ee04f7ec13d581b06508d6465f0e21afe769e6b866094cb89ddbd0082cc300a)
pub fn end_wgpu_render_pass(state: &mut WgpuRenderState) -> () {
    let mut runtime = get_wgpu_render_state_runtime(state);
    if ((runtime.inner.lock().unwrap().render_pass).clone()).is_some() {
        crate::host_value::<()>("host.end");
        {
            let __flight_runtime = runtime;
            let __flight_value = None;
            let mut __flight_storage = __flight_runtime.inner.lock().unwrap();
            __flight_storage.render_pass = __flight_value;
        };
    }
    let saved = runtime
        .inner
        .lock()
        .unwrap()
        .render_target_stack
        .pop()
        .expect("TypeScript Array.pop returned undefined");
    if (saved).is_none() {
        return;
    }
    {
        let __flight_runtime = runtime;
        let __flight_value = (saved.as_ref().unwrap().canvas_texture_view).clone();
        let mut __flight_storage = __flight_runtime.inner.lock().unwrap();
        __flight_storage.canvas_texture_view = __flight_value;
    };
    {
        let __flight_runtime = runtime;
        let __flight_value = saved.as_ref().unwrap().canvas_view_cleared;
        let mut __flight_storage = __flight_runtime.inner.lock().unwrap();
        __flight_storage.canvas_view_cleared = __flight_value;
    };
    {
        let __flight_runtime = runtime;
        let __flight_value = (saved.as_ref().unwrap().render_target_viewport).clone();
        let mut __flight_storage = __flight_runtime.inner.lock().unwrap();
        __flight_storage
            .wgpu_render_state_runtime
            .render_target_viewport = __flight_value;
    };
    {
        let __flight_runtime = runtime;
        let __flight_value = (saved.as_ref().unwrap().color_format).clone();
        let mut __flight_storage = __flight_runtime.inner.lock().unwrap();
        __flight_storage.current_color_format = __flight_value;
    };
    state.render_transform2_d = (saved.as_ref().unwrap().render_transform2_d).clone();
    {
        let __flight_runtime = runtime;
        let __flight_value = 0.0_f64;
        let mut __flight_storage = __flight_runtime.inner.lock().unwrap();
        __flight_storage.current_mask_depth = __flight_value;
    };
    {
        let __flight_runtime = runtime;
        let __flight_value = false;
        let mut __flight_storage = __flight_runtime.inner.lock().unwrap();
        __flight_storage.mask_write_mode = __flight_value;
    };
    {
        let __flight_runtime = runtime;
        let __flight_value = None;
        let mut __flight_storage = __flight_runtime.inner.lock().unwrap();
        __flight_storage
            .wgpu_render_state_runtime
            .current_scissor_rect = __flight_value;
    };
    {
        let __flight_runtime = runtime;
        let __flight_value = vec![];
        let mut __flight_storage = __flight_runtime.inner.lock().unwrap();
        __flight_storage.wgpu_render_state_runtime.scissor_stack = __flight_value;
    };
    if ((saved.as_ref().unwrap().canvas_texture_view).clone()).is_some() {
        {
            let __flight_runtime = runtime;
            let __flight_value = Some(begin_wgpu_render_pass_encoder(
                state,
                ((saved.as_ref().unwrap().canvas_texture_view).clone()).unwrap(),
                ((saved.as_ref().unwrap().depth_stencil_view).clone()).unwrap_or(
                    ((runtime.inner.lock().unwrap().depth_stencil_view).clone()).unwrap(),
                ),
                (runtime
                    .inner
                    .lock()
                    .unwrap()
                    .wgpu_render_state_runtime
                    .render_target_viewport
                    .as_ref()
                    .map(|value| value.width))
                .unwrap_or(crate::host_value::<f64>("host.width")),
                (runtime
                    .inner
                    .lock()
                    .unwrap()
                    .wgpu_render_state_runtime
                    .render_target_viewport
                    .as_ref()
                    .map(|value| value.height))
                .unwrap_or(crate::host_value::<f64>("host.height")),
                crate::OpaqueHostValue::String("load".to_owned()),
                None,
                None,
                None,
            ));
            let mut __flight_storage = __flight_runtime.inner.lock().unwrap();
            __flight_storage.render_pass = __flight_value;
        };
    }
}

// Source: upstream/packages/render-wgpu/src/wgpuRenderTarget.ts:217 (sha256:c223ee4e6b183c7a39f2c8a001c90f3da54e3d80a76f49989809bd201aeff45a)
pub fn resize_wgpu_render_target(
    state: &WgpuRenderState,
    target: &mut WgpuRenderTarget,
    width: f64,
    height: f64,
) -> () {
    let device = (state.device).clone();
    let format = (target.format).clone();
    let w = (1.0_f64).max((width).ceil());
    let h = (1.0_f64).max((height).ceil());
    target.width = w;
    target.height = h;
    crate::host_value::<()>("host.destroy");
    crate::host_value::<()>("host.destroy");
    let new_texture = crate::host_value::<()>("host.createTexture");
    target.texture = (new_texture).clone();
    target.view = crate::host_value::<crate::OpaqueHostValue>("host.createView");
    target.bind_group = build_wgpu_render_target_bind_group(state, (target.view).clone());
    let new_depth = crate::host_value::<()>("host.createTexture");
    target.depth_stencil_texture = (new_depth).clone();
    target.depth_stencil_view = crate::host_value::<crate::OpaqueHostValue>("host.createView");
}

// Source: upstream/packages/render-wgpu/src/wgpuRenderTarget.ts:254 (sha256:292c34351b381c9eae19e60421542712e5ee0898d1310aa70720861e3954359a)
fn is_wgpu_color_preserved(preserve: &crate::FlightUnion2<bool, Vec<bool>>, index: f64) -> bool {
    if (match &(preserve) {
        crate::FlightUnion2::A(_) => "boolean",
        crate::FlightUnion2::B(value) => "object",
    } == "boolean")
    {
        return match (*preserve).clone() {
            crate::FlightUnion2::A(value) => value,
            crate::FlightUnion2::B(_) => panic!("TypeScript union narrowing failed"),
        };
    }
    return (preserve[index as usize].clone() == true);
}

// Source: upstream/packages/render-wgpu/src/wgpuRenderTarget.ts:261 (sha256:c4d175def9148e2fa313874e289eaf170ff6fefa508f99f9ed095d71bc0999d6)
#[derive(Clone, Default)]
struct ResolveWgpuClearColorRecord5 {
    __flight_identity: std::sync::Arc<()>,
    r: f64,
    g: f64,
    b: f64,
    a: f64,
}
impl PartialEq for ResolveWgpuClearColorRecord5 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
struct ResolveWgpuClearColorSynthesizedRecord1596336968 {
    __flight_identity: std::sync::Arc<()>,
    a: f64,
    b: f64,
    g: f64,
    r: f64,
}
impl PartialEq for ResolveWgpuClearColorSynthesizedRecord1596336968 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

fn resolve_wgpu_clear_color(target: &WgpuRenderTarget) -> crate::OpaqueHostValue {
    let packed = target.clear_colors[0.0_f64 as usize].clone();
    if (packed).is_none() {
        return ResolveWgpuClearColorSynthesizedRecord1596336968 {
            __flight_identity: std::sync::Arc::new(()),
            r: 0.0_f64,
            g: 0.0_f64,
            b: 0.0_f64,
            a: 0.0_f64,
        };
    }
    return ResolveWgpuClearColorSynthesizedRecord1596336968 {
        __flight_identity: std::sync::Arc::new(()),
        r: ((__flight_js_to_i32(
            (__flight_js_to_u32(packed) >> (__flight_js_to_u32(24.0_f64) & 31)) as f64,
        ) & __flight_js_to_i32(255.0_f64)) as f64
            / 255.0_f64),
        g: ((__flight_js_to_i32(
            (__flight_js_to_u32(packed) >> (__flight_js_to_u32(16.0_f64) & 31)) as f64,
        ) & __flight_js_to_i32(255.0_f64)) as f64
            / 255.0_f64),
        b: ((__flight_js_to_i32(
            (__flight_js_to_u32(packed) >> (__flight_js_to_u32(8.0_f64) & 31)) as f64,
        ) & __flight_js_to_i32(255.0_f64)) as f64
            / 255.0_f64),
        a: ((__flight_js_to_i32(packed) & __flight_js_to_i32(255.0_f64)) as f64 / 255.0_f64),
    };
}

// Source: upstream/packages/render-wgpu/src/wgpuRenderTarget.ts:276 (sha256:7e01c090fb79de4c1c18f73fdd9eb98ed666c3363c28b88ec7a29eae9f25b1f2)
pub fn set_wgpu_render_transform2_d(state: &mut WgpuRenderState, transform: &Matrix) -> () {
    let mut next = create_matrix(None, None, None, None, None, None);
    copy_matrix(&mut next, &{
        let __flight_source = &(transform);
        MatrixLike {
            __flight_identity: std::sync::Arc::clone(&__flight_source.__flight_identity),
            __flight_entity_runtime: std::sync::Arc::clone(
                &__flight_source.__flight_entity_runtime,
            ),
            a: __flight_source.a,
            b: __flight_source.b,
            c: __flight_source.c,
            d: __flight_source.d,
            tx: __flight_source.tx,
            ty: __flight_source.ty,
        }
    });
    state.render_transform2_d = Some((next).clone());
}
