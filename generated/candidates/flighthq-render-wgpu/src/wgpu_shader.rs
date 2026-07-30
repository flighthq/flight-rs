// @generated from upstream/packages/render-wgpu/src/wgpuShader.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::get_wgpu_render_state_runtime;
pub use flighthq_types::WgpuBitmapShader;
use flighthq_types::{
    BLEND_MODE as blend_mode_constant, ColorTransform, DisplayObjectClipHooks, Matrix, RenderProxy,
    SceneGraphSyncPolicy, WgpuRenderState,
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
    pub uniform_bind_group_layout: crate::OpaqueHostValue,
    pub texture_bind_group_layout: crate::OpaqueHostValue,
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
pub struct SharedStructuralRecord3 {
    pub __flight_identity: std::sync::Arc<()>,
    pub width: f64,
    pub height: f64,
}
impl PartialEq for SharedStructuralRecord3 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct SharedStructuralRecord4 {
    pub __flight_identity: std::sync::Arc<()>,
    pub alpha: f64,
    pub transform2_d: SharedStructuralRecord2,
}
impl PartialEq for SharedStructuralRecord4 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct FlightPartialRecord5 {
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
impl PartialEq for FlightPartialRecord5 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct FlightPartialRecord6 {
    pub __flight_identity: std::sync::Arc<()>,
    pub color_transform: Option<ColorTransform>,
}
impl PartialEq for FlightPartialRecord6 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/render-wgpu/src/wgpuShader.ts:20 (sha256:e089bffd5739353af7f518ec8b794e91ade63a222f5cce92e46525b388c80307)
pub const UNIFORM_BYTE_SIZE: f64 = 128.0_f64;

// Source: upstream/packages/render-wgpu/src/wgpuShader.ts:22 (sha256:79b6ad4d3fa253acaf2f538033587b4bc25913f3122f341c776aae2623aa48e4)
const BITMAP_SHADER_SRC: &'static str = "\nstruct Uniforms {\n  matrix : mat3x3f,\n  alpha : f32,\n  hasColorTransform : u32,\n  _pad0 : f32,\n  _pad1 : f32,\n  colorMultiplier : vec4f,\n  colorOffset : vec4f,\n  x0 : f32, y0 : f32, x1 : f32, y1 : f32,\n  u0 : f32, v0 : f32, u1 : f32, v1 : f32,\n}\n\n@group(0) @binding(0) var<uniform> uni : Uniforms;\n@group(1) @binding(0) var tex : texture_2d<f32>;\n@group(1) @binding(1) var smp : sampler;\n\nstruct VertexOut {\n  @builtin(position) position : vec4f,\n  @location(0) uv : vec2f,\n}\n\n// Quad corner order matching index pattern [0,1,2, 0,2,3]:\n//   vi 0 → corner (x0,y0,u0,v0)\n//   vi 1 → corner (x1,y0,u1,v0)\n//   vi 2 → corner (x1,y1,u1,v1)\n//   vi 3 → corner (x0,y0,u0,v0)  [repeated]\n//   vi 4 → corner (x1,y1,u1,v1)  [repeated]\n//   vi 5 → corner (x0,y1,u0,v1)\n\n@vertex\nfn vs_main(@builtin(vertex_index) vi : u32) -> VertexOut {\n  let xi = (vi == 1u || vi == 2u || vi == 4u);\n  let yi = (vi == 2u || vi == 4u || vi == 5u);\n  let x = select(uni.x0, uni.x1, xi);\n  let y = select(uni.y0, uni.y1, yi);\n  let u = select(uni.u0, uni.u1, xi);\n  let v = select(uni.v0, uni.v1, yi);\n  let p = uni.matrix * vec3f(x, y, 1.0);\n  var out : VertexOut;\n  out.position = vec4f(p.x, p.y, 0.0, 1.0);\n  out.uv = vec2f(u, v);\n  return out;\n}\n\n@fragment\nfn fs_main(in : VertexOut) -> @location(0) vec4f {\n  var color = textureSample(tex, smp, in.uv);\n  if (color.a <= 0.0) { discard; }\n  if (uni.hasColorTransform != 0u && color.a > 0.0) {\n    // Unpremultiply, apply transform, repremultiply\n    color = vec4f(color.rgb / color.a, color.a);\n    color = clamp(color * uni.colorMultiplier + uni.colorOffset, vec4f(0.0), vec4f(1.0));\n    color = vec4f(color.rgb * color.a, color.a);\n  }\n  return color * clamp(uni.alpha, 0.0, 1.0);\n}\n";

// Source: upstream/packages/render-wgpu/src/wgpuShader.ts:83 (sha256:ffb8f9aabe098cec580377c5bee67541fd9ee6c66a800b0670c8b4aa16c13973)
const MASK_FRAGMENT_SRC: &'static str = "\nstruct Uniforms {\n  matrix : mat3x3f,\n  alpha : f32,\n  hasColorTransform : u32,\n  _pad0 : f32,\n  _pad1 : f32,\n  colorMultiplier : vec4f,\n  colorOffset : vec4f,\n  x0 : f32, y0 : f32, x1 : f32, y1 : f32,\n  u0 : f32, v0 : f32, u1 : f32, v1 : f32,\n}\n\n@group(0) @binding(0) var<uniform> uni : Uniforms;\n@group(1) @binding(0) var tex : texture_2d<f32>;\n@group(1) @binding(1) var smp : sampler;\n\nstruct VertexOut {\n  @builtin(position) position : vec4f,\n  @location(0) uv : vec2f,\n}\n\n@vertex\nfn vs_main(@builtin(vertex_index) vi : u32) -> VertexOut {\n  let xi = (vi == 1u || vi == 2u || vi == 4u);\n  let yi = (vi == 2u || vi == 4u || vi == 5u);\n  let x = select(uni.x0, uni.x1, xi);\n  let y = select(uni.y0, uni.y1, yi);\n  let u = select(uni.u0, uni.u1, xi);\n  let v = select(uni.v0, uni.v1, yi);\n  let p = uni.matrix * vec3f(x, y, 1.0);\n  var out : VertexOut;\n  out.position = vec4f(p.x, p.y, 0.0, 1.0);\n  out.uv = vec2f(u, v);\n  return out;\n}\n\n@fragment\nfn fs_main(in : VertexOut) -> @location(0) vec4f {\n  let s = textureSample(tex, smp, in.uv);\n  if (s.a <= 0.0) { discard; }\n  return vec4f(0.0);\n}\n";

// Source: upstream/packages/render-wgpu/src/wgpuShader.ts:130 (sha256:4b80950e1d65fd9298bd084e6db49f35b810ad3c11171ef6862a247474c9ac38)
type StencilMode = String;

// Source: upstream/packages/render-wgpu/src/wgpuShader.ts:132 (sha256:a82fa016310018b2b9046206d7451474b5dd97d319308f9330f2f8d5bfd78d15)
static NORMAL_BLEND: std::sync::LazyLock<crate::OpaqueHostValue> = std::sync::LazyLock::new(|| {
    create_wgpu_blend_state(
        crate::OpaqueHostValue::String("one".to_owned()),
        crate::OpaqueHostValue::String("one-minus-src-alpha".to_owned()),
        None,
    )
});

// Source: upstream/packages/render-wgpu/src/wgpuShader.ts:140 (sha256:bb60cbee9b94100ae5b6fe57023b09d34513e6864a004bc27f1cf8154f37b554)
static BLEND_MODES: std::sync::LazyLock<Vec<(BlendMode, Option<crate::OpaqueHostValue>)>> =
    std::sync::LazyLock::new(|| {
        let mut __flight_record = Vec::new();
        __flight_record.push((
            (blend_mode_constant.add).clone(),
            create_wgpu_blend_state(
                crate::OpaqueHostValue::String("one".to_owned()),
                crate::OpaqueHostValue::String("one".to_owned()),
                None,
            ),
        ));
        __flight_record.push((
            (blend_mode_constant.darken).clone(),
            create_wgpu_blend_state(
                crate::OpaqueHostValue::String("one".to_owned()),
                crate::OpaqueHostValue::String("one".to_owned()),
                Some((crate::OpaqueHostValue::String("min".to_owned())).clone()),
            ),
        ));
        __flight_record.push((
            (blend_mode_constant.lighten).clone(),
            create_wgpu_blend_state(
                crate::OpaqueHostValue::String("one".to_owned()),
                crate::OpaqueHostValue::String("one".to_owned()),
                Some((crate::OpaqueHostValue::String("max".to_owned())).clone()),
            ),
        ));
        __flight_record.push((
            (blend_mode_constant.multiply).clone(),
            create_wgpu_blend_state(
                crate::OpaqueHostValue::String("dst".to_owned()),
                crate::OpaqueHostValue::String("one-minus-src-alpha".to_owned()),
                None,
            ),
        ));
        __flight_record.push(((blend_mode_constant.normal).clone(), NORMAL_BLEND));
        __flight_record.push((
            (blend_mode_constant.screen).clone(),
            create_wgpu_blend_state(
                crate::OpaqueHostValue::String("one".to_owned()),
                crate::OpaqueHostValue::String("one-minus-src".to_owned()),
                None,
            ),
        ));
        __flight_record
    });

// Source: upstream/packages/render-wgpu/src/wgpuShader.ts:153 (sha256:7681b93f47c0dcef918268191727fd45bad77a33cd95a05d4a6983ad5b432ffb)
#[derive(Clone, Default)]
struct CreateWgpuBindGroupLayoutsRecord7 {
    __flight_identity: std::sync::Arc<()>,
    sample_type: String,
}
impl PartialEq for CreateWgpuBindGroupLayoutsRecord7 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
struct CreateWgpuBindGroupLayoutsRecord8 {
    __flight_identity: std::sync::Arc<()>,
    type_: String,
}
impl PartialEq for CreateWgpuBindGroupLayoutsRecord8 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn create_wgpu_bind_group_layouts(device: crate::OpaqueHostValue) -> SharedStructuralRecord1 {
    let uniform_bind_group_layout = crate::host_value::<()>("host.createBindGroupLayout");
    let texture_bind_group_layout = crate::host_value::<()>("host.createBindGroupLayout");
    return SharedStructuralRecord1 {
        __flight_identity: std::sync::Arc::new(()),
        uniform_bind_group_layout: (uniform_bind_group_layout).clone(),
        texture_bind_group_layout: (texture_bind_group_layout).clone(),
    };
}

// Source: upstream/packages/render-wgpu/src/wgpuShader.ts:177 (sha256:9cb125574251eb6f56c612d68bb7422299217f1962dfeeb643378aeed10d397a)
pub fn create_wgpu_pipeline_layout(
    device: crate::OpaqueHostValue,
    uniform_bind_group_layout: crate::OpaqueHostValue,
    texture_bind_group_layout: crate::OpaqueHostValue,
) -> crate::OpaqueHostValue {
    return crate::host_value::<crate::OpaqueHostValue>("host.createPipelineLayout");
}

// Source: upstream/packages/render-wgpu/src/wgpuShader.ts:187 (sha256:1d49befda836d362bbf190e8f94d42c735801d13deb640c9daca41ca0127a421)
#[derive(Clone, Default)]
struct BuildStencilFaceStateRecord7 {
    __flight_identity: std::sync::Arc<()>,
    compare: String,
    pass_op: String,
    fail_op: String,
    depth_fail_op: String,
}
impl PartialEq for BuildStencilFaceStateRecord7 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
struct BuildStencilFaceStateSynthesizedRecord1306782738 {
    __flight_identity: std::sync::Arc<()>,
    compare: String,
    depth_fail_op: String,
    fail_op: String,
    pass_op: String,
}
impl PartialEq for BuildStencilFaceStateSynthesizedRecord1306782738 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

fn build_stencil_face_state(stencil_mode: StencilMode) -> crate::OpaqueHostValue {
    if (stencil_mode == "maskwrite") {
        return BuildStencilFaceStateSynthesizedRecord1306782738 {
            __flight_identity: std::sync::Arc::new(()),
            compare: "always".to_owned(),
            pass_op: "replace".to_owned(),
            fail_op: "keep".to_owned(),
            depth_fail_op: "keep".to_owned(),
        };
    }
    if (stencil_mode == "masked") {
        return BuildStencilFaceStateSynthesizedRecord1306782738 {
            __flight_identity: std::sync::Arc::new(()),
            compare: "equal".to_owned(),
            pass_op: "keep".to_owned(),
            fail_op: "keep".to_owned(),
            depth_fail_op: "keep".to_owned(),
        };
    }
    return BuildStencilFaceStateSynthesizedRecord1306782738 {
        __flight_identity: std::sync::Arc::new(()),
        compare: "always".to_owned(),
        pass_op: "keep".to_owned(),
        fail_op: "keep".to_owned(),
        depth_fail_op: "keep".to_owned(),
    };
}

// Source: upstream/packages/render-wgpu/src/wgpuShader.ts:197 (sha256:077a6e69a8664cf116da0bd018693a759f30be7fda089aa74f5eb211c7044f36)
pub fn get_active_wgpu_pipeline(state: &WgpuRenderState) -> crate::OpaqueHostValue {
    let runtime = get_wgpu_render_state_runtime(state);
    let stencil_mode: StencilMode = if runtime.inner.lock().unwrap().mask_write_mode {
        "maskwrite".to_owned()
    } else {
        if (runtime.inner.lock().unwrap().current_mask_depth > 0.0_f64) {
            "masked".to_owned()
        } else {
            "normal".to_owned()
        }
    };
    return get_wgpu_pipeline(
        state,
        ((runtime.inner.lock().unwrap().current_blend_mode).clone()).clone(),
        (stencil_mode).clone(),
    );
}

// Source: upstream/packages/render-wgpu/src/wgpuShader.ts:207 (sha256:4f2986811a7730c41b19c7761170ae262f42e2be34e2d837f9ada90745b0b75f)
#[derive(Clone, Default)]
struct GetWgpuPipelineRecord7 {
    __flight_identity: std::sync::Arc<()>,
    topology: String,
}
impl PartialEq for GetWgpuPipelineRecord7 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn get_wgpu_pipeline(
    state: &WgpuRenderState,
    blend_mode: Option<BlendMode>,
    stencil_mode: StencilMode,
) -> crate::OpaqueHostValue {
    let mut runtime = get_wgpu_render_state_runtime(state);
    let format = ((runtime.inner.lock().unwrap().current_color_format).clone())
        .unwrap_or((state.format).clone());
    let key = format!(
        "{}-{}-{}",
        (blend_mode).unwrap_or("null".to_owned()),
        stencil_mode,
        format
    );
    let cached = runtime
        .inner
        .lock()
        .unwrap()
        .pipeline_cache
        .iter()
        .find(|(key, _)| key == &(key).clone())
        .map(|(_, value)| value.clone());
    if (cached).is_some() {
        return ((cached.as_ref().unwrap()).clone()).clone();
    }
    let blend = (if (blend_mode).is_some() {
        BLEND_MODES
            .iter()
            .find(|(key, _)| key == &(blend_mode.as_ref().unwrap()).clone())
            .map(|(_, value)| value)
            .expect("TypeScript Record key was absent")
            .clone()
    } else {
        None
    })
    .unwrap_or((NORMAL_BLEND).clone());
    let is_mask_write = (stencil_mode == "maskwrite");
    let stencil_face = build_stencil_face_state((stencil_mode).clone());
    let device = (state.device).clone();
    let shader_src = if is_mask_write {
        ((MASK_FRAGMENT_SRC).clone()).to_owned()
    } else {
        ((BITMAP_SHADER_SRC).clone()).to_owned()
    };
    let module = crate::host_value::<()>("host.createShaderModule");
    let layout = create_wgpu_pipeline_layout(
        (device).clone(),
        (runtime.inner.lock().unwrap().uniform_bind_group_layout).clone(),
        (runtime.inner.lock().unwrap().texture_bind_group_layout).clone(),
    );
    let pipeline = crate::host_value::<()>("host.createRenderPipeline");
    {
        let __flight_key = (key).clone();
        let __flight_value = (pipeline).clone();
        if let Some((_, value)) = runtime
            .inner
            .lock()
            .unwrap()
            .pipeline_cache
            .iter_mut()
            .find(|(key, _)| key == &__flight_key)
        {
            *value = __flight_value;
        } else {
            runtime
                .inner
                .lock()
                .unwrap()
                .pipeline_cache
                .push((__flight_key, __flight_value));
        }
    };
    return pipeline;
}

// Source: upstream/packages/render-wgpu/src/wgpuShader.ts:261 (sha256:254088737f4456fbbb4c14e4c4cf9ca58a2ac468ca55aa5648ffda73fbbc6837)
pub fn set_wgpu_matrix_from_transform(
    matrix_array: &mut Vec<f32>,
    t: &SharedStructuralRecord2,
    viewport: &SharedStructuralRecord3,
) -> () {
    let iw = (2.0_f64 / viewport.width);
    let ih = (2.0_f64 / viewport.height);
    matrix_array[0.0_f64 as usize] = (t.a * iw) as f32;
    matrix_array[1.0_f64 as usize] = ((-t.b) * ih) as f32;
    matrix_array[2.0_f64 as usize] = (0.0_f64) as f32;
    matrix_array[3.0_f64 as usize] = (t.c * iw) as f32;
    matrix_array[4.0_f64 as usize] = ((-t.d) * ih) as f32;
    matrix_array[5.0_f64 as usize] = (0.0_f64) as f32;
    matrix_array[6.0_f64 as usize] = ((t.tx * iw) - 1.0_f64) as f32;
    matrix_array[7.0_f64 as usize] = (((-t.ty) * ih) + 1.0_f64) as f32;
    matrix_array[8.0_f64 as usize] = (1.0_f64) as f32;
}

// Source: upstream/packages/render-wgpu/src/wgpuShader.ts:285 (sha256:357acccfdceab1c336b3cfd4cff518d62c79e12481a6a9f5ded5c5ff25d00387)
pub fn write_wgpu_matrix_only_uniforms(
    state: &WgpuRenderState,
    render_proxy: &RenderProxy,
    transform: &SharedStructuralRecord2,
    x0: f64,
    y0: f64,
    x1: f64,
    y1: f64,
    u0: f64,
    v0: f64,
    u1: f64,
    v1: f64,
) -> f64 {
    return write_wgpu_quad_uniforms(
        state,
        &SharedStructuralRecord4 {
            __flight_identity: std::sync::Arc::new(()),
            alpha: render_proxy.alpha,
            transform2_d: (*transform).clone(),
        },
        None,
        x0,
        y0,
        x1,
        y1,
        u0,
        v0,
        u1,
        v1,
    );
}

// Source: upstream/packages/render-wgpu/src/wgpuShader.ts:316 (sha256:7fbd558086800c525d11037630f41e02e3e04b0a7805c08616e389efc0741aca)
pub fn write_wgpu_quad_uniforms(
    state: &WgpuRenderState,
    render_proxy: &SharedStructuralRecord4,
    color_transform: Option<ColorTransform>,
    x0: f64,
    y0: f64,
    x1: f64,
    y1: f64,
    u0: f64,
    v0: f64,
    u1: f64,
    v1: f64,
) -> f64 {
    let mut runtime = get_wgpu_render_state_runtime(state);
    let byte_offset = runtime.inner.lock().unwrap().uniform_offset;
    let float_base = (__flight_js_to_i32(byte_offset) >> (__flight_js_to_u32(2.0_f64) & 31)) as f64;
    let viewport = ((runtime
        .inner
        .lock()
        .unwrap()
        .wgpu_render_state_runtime
        .render_target_viewport)
        .clone())
    .unwrap_or((state.canvas).clone());
    set_wgpu_matrix_from_transform(
        &mut runtime.inner.lock().unwrap().matrix_array,
        &render_proxy.transform2_d,
        &{
            let __flight_source = &(viewport);
            SharedStructuralRecord3 {
                __flight_identity: std::sync::Arc::clone(&__flight_source.__flight_identity),
                width: __flight_source.width,
                height: __flight_source.height,
            }
        },
    );
    runtime.inner.lock().unwrap().uniform_data[(float_base + 0.0_f64) as usize] =
        (runtime.inner.lock().unwrap().matrix_array[0.0_f64 as usize] as f64) as f32;
    runtime.inner.lock().unwrap().uniform_data[(float_base + 1.0_f64) as usize] =
        (runtime.inner.lock().unwrap().matrix_array[1.0_f64 as usize] as f64) as f32;
    runtime.inner.lock().unwrap().uniform_data[(float_base + 2.0_f64) as usize] =
        (runtime.inner.lock().unwrap().matrix_array[2.0_f64 as usize] as f64) as f32;
    runtime.inner.lock().unwrap().uniform_data[(float_base + 3.0_f64) as usize] = (0.0_f64) as f32;
    runtime.inner.lock().unwrap().uniform_data[(float_base + 4.0_f64) as usize] =
        (runtime.inner.lock().unwrap().matrix_array[3.0_f64 as usize] as f64) as f32;
    runtime.inner.lock().unwrap().uniform_data[(float_base + 5.0_f64) as usize] =
        (runtime.inner.lock().unwrap().matrix_array[4.0_f64 as usize] as f64) as f32;
    runtime.inner.lock().unwrap().uniform_data[(float_base + 6.0_f64) as usize] =
        (runtime.inner.lock().unwrap().matrix_array[5.0_f64 as usize] as f64) as f32;
    runtime.inner.lock().unwrap().uniform_data[(float_base + 7.0_f64) as usize] = (0.0_f64) as f32;
    runtime.inner.lock().unwrap().uniform_data[(float_base + 8.0_f64) as usize] =
        (runtime.inner.lock().unwrap().matrix_array[6.0_f64 as usize] as f64) as f32;
    runtime.inner.lock().unwrap().uniform_data[(float_base + 9.0_f64) as usize] =
        (runtime.inner.lock().unwrap().matrix_array[7.0_f64 as usize] as f64) as f32;
    runtime.inner.lock().unwrap().uniform_data[(float_base + 10.0_f64) as usize] =
        (runtime.inner.lock().unwrap().matrix_array[8.0_f64 as usize] as f64) as f32;
    runtime.inner.lock().unwrap().uniform_data[(float_base + 11.0_f64) as usize] = (0.0_f64) as f32;
    runtime.inner.lock().unwrap().uniform_data[(float_base + 12.0_f64) as usize] =
        (render_proxy.alpha) as f32;
    runtime.inner.lock().unwrap().uniform_data_u32[(float_base + 13.0_f64) as usize] =
        if (color_transform).is_some() {
            (1.0_f64) as u32
        } else {
            (0.0_f64) as u32
        };
    runtime.inner.lock().unwrap().uniform_data[(float_base + 14.0_f64) as usize] = (0.0_f64) as f32;
    runtime.inner.lock().unwrap().uniform_data[(float_base + 15.0_f64) as usize] = (0.0_f64) as f32;
    runtime.inner.lock().unwrap().uniform_data[(float_base + 16.0_f64) as usize] =
        ((color_transform.as_ref().map(|value| value.red_multiplier)).unwrap_or(1.0_f64)) as f32;
    runtime.inner.lock().unwrap().uniform_data[(float_base + 17.0_f64) as usize] =
        ((color_transform.as_ref().map(|value| value.green_multiplier)).unwrap_or(1.0_f64)) as f32;
    runtime.inner.lock().unwrap().uniform_data[(float_base + 18.0_f64) as usize] =
        ((color_transform.as_ref().map(|value| value.blue_multiplier)).unwrap_or(1.0_f64)) as f32;
    runtime.inner.lock().unwrap().uniform_data[(float_base + 19.0_f64) as usize] =
        ((color_transform.as_ref().map(|value| value.alpha_multiplier)).unwrap_or(1.0_f64)) as f32;
    runtime.inner.lock().unwrap().uniform_data[(float_base + 20.0_f64) as usize] =
        ((color_transform.as_ref().map(|value| value.red_offset)).unwrap_or(0.0_f64) / 255.0_f64)
            as f32;
    runtime.inner.lock().unwrap().uniform_data[(float_base + 21.0_f64) as usize] =
        ((color_transform.as_ref().map(|value| value.green_offset)).unwrap_or(0.0_f64) / 255.0_f64)
            as f32;
    runtime.inner.lock().unwrap().uniform_data[(float_base + 22.0_f64) as usize] =
        ((color_transform.as_ref().map(|value| value.blue_offset)).unwrap_or(0.0_f64) / 255.0_f64)
            as f32;
    runtime.inner.lock().unwrap().uniform_data[(float_base + 23.0_f64) as usize] =
        ((color_transform.as_ref().map(|value| value.alpha_offset)).unwrap_or(0.0_f64) / 255.0_f64)
            as f32;
    runtime.inner.lock().unwrap().uniform_data[(float_base + 24.0_f64) as usize] = (x0) as f32;
    runtime.inner.lock().unwrap().uniform_data[(float_base + 25.0_f64) as usize] = (y0) as f32;
    runtime.inner.lock().unwrap().uniform_data[(float_base + 26.0_f64) as usize] = (x1) as f32;
    runtime.inner.lock().unwrap().uniform_data[(float_base + 27.0_f64) as usize] = (y1) as f32;
    runtime.inner.lock().unwrap().uniform_data[(float_base + 28.0_f64) as usize] = (u0) as f32;
    runtime.inner.lock().unwrap().uniform_data[(float_base + 29.0_f64) as usize] = (v0) as f32;
    runtime.inner.lock().unwrap().uniform_data[(float_base + 30.0_f64) as usize] = (u1) as f32;
    runtime.inner.lock().unwrap().uniform_data[(float_base + 31.0_f64) as usize] = (v1) as f32;
    {
        let __flight_runtime = runtime;
        let __flight_value = runtime.inner.lock().unwrap().uniform_stride;
        let mut __flight_storage = __flight_runtime.inner.lock().unwrap();
        __flight_storage.uniform_offset += __flight_value;
    };
    return byte_offset;
}

// Source: upstream/packages/render-wgpu/src/wgpuShader.ts:388 (sha256:42ce2a5963a95dc0103dc99939ffba846e305623c06edb3f9b497df38fa376ef)
#[derive(Clone, Default)]
struct CreateWgpuBlendStateSynthesizedRecord1545448619 {
    __flight_identity: std::sync::Arc<()>,
    dst_factor: crate::OpaqueHostValue,
    operation: crate::OpaqueHostValue,
    src_factor: crate::OpaqueHostValue,
}
impl PartialEq for CreateWgpuBlendStateSynthesizedRecord1545448619 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
struct CreateWgpuBlendStateSynthesizedRecord3556186453 {
    __flight_identity: std::sync::Arc<()>,
    alpha: CreateWgpuBlendStateSynthesizedRecord1545448619,
    color: CreateWgpuBlendStateSynthesizedRecord1545448619,
}
impl PartialEq for CreateWgpuBlendStateSynthesizedRecord3556186453 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

fn create_wgpu_blend_state(
    src_factor: crate::OpaqueHostValue,
    dst_factor: crate::OpaqueHostValue,
    operation: Option<crate::OpaqueHostValue>,
) -> crate::OpaqueHostValue {
    let operation = operation.unwrap_or(crate::OpaqueHostValue::String("add".to_owned()));
    let component = CreateWgpuBlendStateSynthesizedRecord1545448619 {
        __flight_identity: std::sync::Arc::new(()),
        src_factor: (src_factor).clone(),
        dst_factor: (dst_factor).clone(),
        operation: (operation).clone(),
    };
    return CreateWgpuBlendStateSynthesizedRecord3556186453 {
        __flight_identity: std::sync::Arc::new(()),
        color: (component).clone(),
        alpha: (component).clone(),
    };
}
