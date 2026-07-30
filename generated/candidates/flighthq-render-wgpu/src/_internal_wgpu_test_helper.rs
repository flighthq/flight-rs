// @generated from upstream/packages/render-wgpu/src/wgpuTestHelper.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::{
    BlendMode, ColorTransform, DisplayObjectClipHooks, Matrix, SceneGraphSyncPolicy,
    WgpuRenderState,
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

// Source: upstream/packages/render-wgpu/src/wgpuTestHelper.ts:6 (sha256:6129b1225ebb747d841b12b242bdde0b399d8554169b1f59e66bdf37b714ebca)
#[derive(Clone, Default)]
struct InstallWgpuConstantsRecord3 {
    __flight_identity: std::sync::Arc<()>,
    map_read: f64,
    map_write: f64,
    copy_src: f64,
    copy_dst: f64,
    index: f64,
    vertex: f64,
    uniform: f64,
    storage: f64,
    indirect: f64,
    query_resolve: f64,
}
impl PartialEq for InstallWgpuConstantsRecord3 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
struct InstallWgpuConstantsRecord4 {
    __flight_identity: std::sync::Arc<()>,
    copy_src: f64,
    copy_dst: f64,
    texture_binding: f64,
    storage_binding: f64,
    render_attachment: f64,
}
impl PartialEq for InstallWgpuConstantsRecord4 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
struct InstallWgpuConstantsRecord5 {
    __flight_identity: std::sync::Arc<()>,
    vertex: f64,
    fragment: f64,
    compute: f64,
}
impl PartialEq for InstallWgpuConstantsRecord5 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
struct InstallWgpuConstantsRecord6 {
    __flight_identity: std::sync::Arc<()>,
    red: f64,
    green: f64,
    blue: f64,
    alpha: f64,
    all: f64,
}
impl PartialEq for InstallWgpuConstantsRecord6 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
struct InstallWgpuConstantsRecord7 {
    __flight_identity: std::sync::Arc<()>,
    read: f64,
    write: f64,
}
impl PartialEq for InstallWgpuConstantsRecord7 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
struct InstallWgpuConstantsSynthesizedRecord210998270 {
    __flight_identity: std::sync::Arc<()>,
    copy_dst: f64,
    copy_src: f64,
    index: f64,
    indirect: f64,
    map_read: f64,
    map_write: f64,
    query_resolve: f64,
    storage: f64,
    uniform: f64,
    vertex: f64,
}
impl PartialEq for InstallWgpuConstantsSynthesizedRecord210998270 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
struct InstallWgpuConstantsSynthesizedRecord3840256937 {
    __flight_identity: std::sync::Arc<()>,
    copy_dst: f64,
    copy_src: f64,
    render_attachment: f64,
    storage_binding: f64,
    texture_binding: f64,
}
impl PartialEq for InstallWgpuConstantsSynthesizedRecord3840256937 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
struct InstallWgpuConstantsSynthesizedRecord3217851777 {
    __flight_identity: std::sync::Arc<()>,
    compute: f64,
    fragment: f64,
    vertex: f64,
}
impl PartialEq for InstallWgpuConstantsSynthesizedRecord3217851777 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
struct InstallWgpuConstantsSynthesizedRecord424485483 {
    __flight_identity: std::sync::Arc<()>,
    all: f64,
    alpha: f64,
    blue: f64,
    green: f64,
    red: f64,
}
impl PartialEq for InstallWgpuConstantsSynthesizedRecord424485483 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

fn install_wgpu_constants() -> () {
    let mut g = crate::OpaqueHostValue::Object;
    if !(g
        .iter()
        .find(|(key, _)| key == &"GPUBufferUsage".to_owned())
        .map(|(_, value)| value)
        .expect("TypeScript Record key was absent")
        .clone())
    {
        g.iter()
            .find(|(key, _)| key == &"GPUBufferUsage".to_owned())
            .map(|(_, value)| value)
            .expect("TypeScript Record key was absent") =
            InstallWgpuConstantsSynthesizedRecord210998270 {
                __flight_identity: std::sync::Arc::new(()),
                map_read: 1.0_f64,
                map_write: 2.0_f64,
                copy_src: 4.0_f64,
                copy_dst: 8.0_f64,
                index: 16.0_f64,
                vertex: 32.0_f64,
                uniform: 64.0_f64,
                storage: 128.0_f64,
                indirect: 256.0_f64,
                query_resolve: 512.0_f64,
            };
    }
    if !(g
        .iter()
        .find(|(key, _)| key == &"GPUTextureUsage".to_owned())
        .map(|(_, value)| value)
        .expect("TypeScript Record key was absent")
        .clone())
    {
        g.iter()
            .find(|(key, _)| key == &"GPUTextureUsage".to_owned())
            .map(|(_, value)| value)
            .expect("TypeScript Record key was absent") =
            InstallWgpuConstantsSynthesizedRecord3840256937 {
                __flight_identity: std::sync::Arc::new(()),
                copy_src: 1.0_f64,
                copy_dst: 2.0_f64,
                texture_binding: 4.0_f64,
                storage_binding: 8.0_f64,
                render_attachment: 16.0_f64,
            };
    }
    if !(g
        .iter()
        .find(|(key, _)| key == &"GPUShaderStage".to_owned())
        .map(|(_, value)| value)
        .expect("TypeScript Record key was absent")
        .clone())
    {
        g.iter()
            .find(|(key, _)| key == &"GPUShaderStage".to_owned())
            .map(|(_, value)| value)
            .expect("TypeScript Record key was absent") =
            InstallWgpuConstantsSynthesizedRecord3217851777 {
                __flight_identity: std::sync::Arc::new(()),
                vertex: 1.0_f64,
                fragment: 2.0_f64,
                compute: 4.0_f64,
            };
    }
    if !(g
        .iter()
        .find(|(key, _)| key == &"GPUColorWrite".to_owned())
        .map(|(_, value)| value)
        .expect("TypeScript Record key was absent")
        .clone())
    {
        g.iter()
            .find(|(key, _)| key == &"GPUColorWrite".to_owned())
            .map(|(_, value)| value)
            .expect("TypeScript Record key was absent") =
            InstallWgpuConstantsSynthesizedRecord424485483 {
                __flight_identity: std::sync::Arc::new(()),
                red: 1.0_f64,
                green: 2.0_f64,
                blue: 4.0_f64,
                alpha: 8.0_f64,
                all: 15.0_f64,
            };
    }
    if !(g
        .iter()
        .find(|(key, _)| key == &"GPUMapMode".to_owned())
        .map(|(_, value)| value)
        .expect("TypeScript Record key was absent")
        .clone())
    {
        g.iter()
            .find(|(key, _)| key == &"GPUMapMode".to_owned())
            .map(|(_, value)| value)
            .expect("TypeScript Record key was absent") = InstallWgpuConstantsRecord7 {
            __flight_identity: std::sync::Arc::new(()),
            read: 1.0_f64,
            write: 2.0_f64,
        };
    }
}

// Source: upstream/packages/render-wgpu/src/wgpuTestHelper.ts:40 (sha256:55cf9c89fd0cd9485c43860c2a04b4dabb3f66ad6355921d33d95e5ebbd3b138)
#[derive(Clone, Default)]
struct MakeBindGroupRecord3 {
    __flight_identity: std::sync::Arc<()>,
}
impl PartialEq for MakeBindGroupRecord3 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

fn make_bind_group() -> crate::OpaqueHostValue {
    return MakeBindGroupRecord3 {
        __flight_identity: std::sync::Arc::new(()),
    };
}

// Source: upstream/packages/render-wgpu/src/wgpuTestHelper.ts:44 (sha256:0efe9d29a87f2e7ba72d40d26f35f339357c7d449844fabed81cddaa84f0bbb5)
#[derive(Clone, Default)]
struct MakeBindGroupLayoutRecord3 {
    __flight_identity: std::sync::Arc<()>,
}
impl PartialEq for MakeBindGroupLayoutRecord3 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

fn make_bind_group_layout() -> crate::OpaqueHostValue {
    return MakeBindGroupLayoutRecord3 {
        __flight_identity: std::sync::Arc::new(()),
    };
}

// Source: upstream/packages/render-wgpu/src/wgpuTestHelper.ts:48 (sha256:3f845084735976eb96a37e4c154e336016140d66735a9bac5cfb4e920867408d)
#[derive(Clone)]
struct MakeBufferSynthesizedRecord667054565 {
    __flight_identity: std::sync::Arc<()>,
    destroy: std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>,
}
impl PartialEq for MakeBufferSynthesizedRecord667054565 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

fn make_buffer() -> crate::OpaqueHostValue {
    return MakeBufferSynthesizedRecord667054565 {
        __flight_identity: std::sync::Arc::new(()),
        destroy: std::sync::Arc::new(std::sync::Mutex::new(
            Box::new(move || -> () {}) as Box<dyn FnMut() -> () + Send + 'static>
        )),
    };
}

// Source: upstream/packages/render-wgpu/src/wgpuTestHelper.ts:52 (sha256:8218b36842f3e64ee6669a09bf67514e536c754daee1e18d24da5b03c2b8f881)
#[derive(Clone, Default)]
struct MakeTextureRecord3 {
    __flight_identity: std::sync::Arc<()>,
}
impl PartialEq for MakeTextureRecord3 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone)]
struct MakeTextureSynthesizedRecord694067184 {
    __flight_identity: std::sync::Arc<()>,
    create_view: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut() -> crate::OpaqueHostValue + Send + 'static>>,
    >,
    destroy: std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>,
}
impl PartialEq for MakeTextureSynthesizedRecord694067184 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

fn make_texture() -> crate::OpaqueHostValue {
    return MakeTextureSynthesizedRecord694067184 {
        __flight_identity: std::sync::Arc::new(()),
        create_view: std::sync::Arc::new(std::sync::Mutex::new(Box::new(
            move || -> crate::OpaqueHostValue {
                MakeTextureRecord3 {
                    __flight_identity: std::sync::Arc::new(()),
                }
            },
        )
            as Box<dyn FnMut() -> crate::OpaqueHostValue + Send + 'static>)),
        destroy: std::sync::Arc::new(std::sync::Mutex::new(
            Box::new(move || -> () {}) as Box<dyn FnMut() -> () + Send + 'static>
        )),
    };
}

// Source: upstream/packages/render-wgpu/src/wgpuTestHelper.ts:59 (sha256:9f4ba3050c92ccd9c353b3c806876588f9fd194a597604e85b1f58611e2cab67)
#[derive(Clone)]
struct MakeRenderPassEncoderSynthesizedRecord3734475925 {
    __flight_identity: std::sync::Arc<()>,
    draw: std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>,
    end: std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>,
    set_bind_group: std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>,
    set_pipeline: std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>,
    set_scissor_rect: std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>,
    set_stencil_reference:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>,
    set_viewport: std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>,
}
impl PartialEq for MakeRenderPassEncoderSynthesizedRecord3734475925 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

fn make_render_pass_encoder() -> crate::OpaqueHostValue {
    return MakeRenderPassEncoderSynthesizedRecord3734475925 {
        __flight_identity: std::sync::Arc::new(()),
        draw: std::sync::Arc::new(std::sync::Mutex::new(
            Box::new(move || -> () {}) as Box<dyn FnMut() -> () + Send + 'static>
        )),
        end: std::sync::Arc::new(std::sync::Mutex::new(
            Box::new(move || -> () {}) as Box<dyn FnMut() -> () + Send + 'static>
        )),
        set_bind_group: std::sync::Arc::new(std::sync::Mutex::new(
            Box::new(move || -> () {}) as Box<dyn FnMut() -> () + Send + 'static>
        )),
        set_pipeline: std::sync::Arc::new(std::sync::Mutex::new(
            Box::new(move || -> () {}) as Box<dyn FnMut() -> () + Send + 'static>
        )),
        set_scissor_rect: std::sync::Arc::new(std::sync::Mutex::new(
            Box::new(move || -> () {}) as Box<dyn FnMut() -> () + Send + 'static>
        )),
        set_stencil_reference: std::sync::Arc::new(std::sync::Mutex::new(
            Box::new(move || -> () {}) as Box<dyn FnMut() -> () + Send + 'static>,
        )),
        set_viewport: std::sync::Arc::new(std::sync::Mutex::new(
            Box::new(move || -> () {}) as Box<dyn FnMut() -> () + Send + 'static>
        )),
    };
}

// Source: upstream/packages/render-wgpu/src/wgpuTestHelper.ts:71 (sha256:5f07dd9745a541db7d878d84c802b19e3bfa0ecb92f1476cbc7dca534b2ca5a4)
#[derive(Clone, Default)]
struct MakeCommandEncoderRecord3 {
    __flight_identity: std::sync::Arc<()>,
}
impl PartialEq for MakeCommandEncoderRecord3 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone)]
struct MakeCommandEncoderSynthesizedRecord1369794951 {
    __flight_identity: std::sync::Arc<()>,
    begin_render_pass: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut() -> crate::OpaqueHostValue + Send + 'static>>,
    >,
    finish: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut() -> crate::OpaqueHostValue + Send + 'static>>,
    >,
}
impl PartialEq for MakeCommandEncoderSynthesizedRecord1369794951 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

fn make_command_encoder() -> crate::OpaqueHostValue {
    return MakeCommandEncoderSynthesizedRecord1369794951 {
        __flight_identity: std::sync::Arc::new(()),
        begin_render_pass: std::sync::Arc::new(std::sync::Mutex::new(Box::new(
            move || -> crate::OpaqueHostValue { make_render_pass_encoder() },
        )
            as Box<dyn FnMut() -> crate::OpaqueHostValue + Send + 'static>)),
        finish: std::sync::Arc::new(std::sync::Mutex::new(Box::new(
            move || -> crate::OpaqueHostValue {
                MakeCommandEncoderRecord3 {
                    __flight_identity: std::sync::Arc::new(()),
                }
            },
        )
            as Box<dyn FnMut() -> crate::OpaqueHostValue + Send + 'static>)),
    };
}

// Source: upstream/packages/render-wgpu/src/wgpuTestHelper.ts:80 (sha256:154b4576333009d36e671b52c597b3b6633e13ff57fb72c1d7ce26a338fc38ae)
#[derive(Clone, Default)]
struct MakePipelineSynthesizedRecord1967302679 {
    __flight_identity: std::sync::Arc<()>,
    __descriptor: Option<crate::OpaqueHostValue>,
}
impl PartialEq for MakePipelineSynthesizedRecord1967302679 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

fn make_pipeline(descriptor: Option<crate::OpaqueHostValue>) -> crate::OpaqueHostValue {
    return MakePipelineSynthesizedRecord1967302679 {
        __flight_identity: std::sync::Arc::new(()),
        __descriptor: (descriptor).clone(),
    };
}

// Source: upstream/packages/render-wgpu/src/wgpuTestHelper.ts:84 (sha256:fe234171c8d681fdee6bc651ffcd750eefa2a7435337a7a53debb7e8175405a0)
#[derive(Clone, Default)]
struct MakeShaderModuleRecord3 {
    __flight_identity: std::sync::Arc<()>,
}
impl PartialEq for MakeShaderModuleRecord3 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

fn make_shader_module() -> crate::OpaqueHostValue {
    return MakeShaderModuleRecord3 {
        __flight_identity: std::sync::Arc::new(()),
    };
}

// Source: upstream/packages/render-wgpu/src/wgpuTestHelper.ts:88 (sha256:4c8fa8a753df75abdca48e1505c6e55debd20b845b5c2227b2e192c3c77a7a87)
#[derive(Clone, Default)]
struct MakeSamplerRecord3 {
    __flight_identity: std::sync::Arc<()>,
}
impl PartialEq for MakeSamplerRecord3 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

fn make_sampler() -> crate::OpaqueHostValue {
    return MakeSamplerRecord3 {
        __flight_identity: std::sync::Arc::new(()),
    };
}

// Source: upstream/packages/render-wgpu/src/wgpuTestHelper.ts:92 (sha256:d74a118c853bef5ce45427826a8584906f90a675da69a69b0d15f0769a9ac94e)
#[derive(Clone, Default)]
struct MakePipelineLayoutRecord3 {
    __flight_identity: std::sync::Arc<()>,
}
impl PartialEq for MakePipelineLayoutRecord3 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

fn make_pipeline_layout() -> crate::OpaqueHostValue {
    return MakePipelineLayoutRecord3 {
        __flight_identity: std::sync::Arc::new(()),
    };
}

// Source: upstream/packages/render-wgpu/src/wgpuTestHelper.ts:96 (sha256:f40df9a14444eb4daea153e6535b50b2f0c9a7698189ba59b2aaaff8b5f76724)
#[derive(Clone, Default)]
struct MakeDeviceRecord3 {
    __flight_identity: std::sync::Arc<()>,
    min_uniform_buffer_offset_alignment: f64,
}
impl PartialEq for MakeDeviceRecord3 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone)]
struct MakeDeviceSynthesizedRecord446889381 {
    __flight_identity: std::sync::Arc<()>,
    create_bind_group: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut() -> crate::OpaqueHostValue + Send + 'static>>,
    >,
    create_bind_group_layout: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut() -> crate::OpaqueHostValue + Send + 'static>>,
    >,
    create_buffer: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut() -> crate::OpaqueHostValue + Send + 'static>>,
    >,
    create_command_encoder: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut() -> crate::OpaqueHostValue + Send + 'static>>,
    >,
    create_pipeline_layout: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut() -> crate::OpaqueHostValue + Send + 'static>>,
    >,
    create_render_pipeline: std::sync::Arc<
        std::sync::Mutex<
            Box<dyn FnMut(crate::OpaqueHostValue) -> crate::OpaqueHostValue + Send + 'static>,
        >,
    >,
    create_sampler: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut() -> crate::OpaqueHostValue + Send + 'static>>,
    >,
    create_shader_module: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut() -> crate::OpaqueHostValue + Send + 'static>>,
    >,
    create_texture: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut() -> crate::OpaqueHostValue + Send + 'static>>,
    >,
    limits: MakeDeviceRecord3,
    queue: MakeDeviceSynthesizedRecord1500877648,
}
impl PartialEq for MakeDeviceSynthesizedRecord446889381 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone)]
struct MakeDeviceSynthesizedRecord1500877648 {
    __flight_identity: std::sync::Arc<()>,
    copy_external_image_to_texture:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>,
    submit: std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>,
    write_buffer: std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>,
    write_texture: std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>,
}
impl PartialEq for MakeDeviceSynthesizedRecord1500877648 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

fn make_device() -> crate::OpaqueHostValue {
    return MakeDeviceSynthesizedRecord446889381 {
        __flight_identity: std::sync::Arc::new(()),
        limits: MakeDeviceRecord3 {
            __flight_identity: std::sync::Arc::new(()),
            min_uniform_buffer_offset_alignment: 256.0_f64,
        },
        create_bind_group: std::sync::Arc::new(std::sync::Mutex::new(Box::new(
            move || -> crate::OpaqueHostValue { make_bind_group() },
        )
            as Box<dyn FnMut() -> crate::OpaqueHostValue + Send + 'static>)),
        create_bind_group_layout: std::sync::Arc::new(std::sync::Mutex::new(Box::new(
            move || -> crate::OpaqueHostValue { make_bind_group_layout() },
        )
            as Box<dyn FnMut() -> crate::OpaqueHostValue + Send + 'static>)),
        create_buffer: std::sync::Arc::new(std::sync::Mutex::new(Box::new(
            move || -> crate::OpaqueHostValue { make_buffer() },
        )
            as Box<dyn FnMut() -> crate::OpaqueHostValue + Send + 'static>)),
        create_command_encoder: std::sync::Arc::new(std::sync::Mutex::new(Box::new(
            move || -> crate::OpaqueHostValue { make_command_encoder() },
        )
            as Box<dyn FnMut() -> crate::OpaqueHostValue + Send + 'static>)),
        create_pipeline_layout: std::sync::Arc::new(std::sync::Mutex::new(Box::new(
            move || -> crate::OpaqueHostValue { make_pipeline_layout() },
        )
            as Box<dyn FnMut() -> crate::OpaqueHostValue + Send + 'static>)),
        create_render_pipeline: std::sync::Arc::new(std::sync::Mutex::new(Box::new(
            move |descriptor: crate::OpaqueHostValue| -> crate::OpaqueHostValue {
                make_pipeline(Some(((descriptor).clone()).clone()))
            },
        )
            as Box<dyn FnMut(crate::OpaqueHostValue) -> crate::OpaqueHostValue + Send + 'static>)),
        create_sampler: std::sync::Arc::new(std::sync::Mutex::new(Box::new(
            move || -> crate::OpaqueHostValue { make_sampler() },
        )
            as Box<dyn FnMut() -> crate::OpaqueHostValue + Send + 'static>)),
        create_shader_module: std::sync::Arc::new(std::sync::Mutex::new(Box::new(
            move || -> crate::OpaqueHostValue { make_shader_module() },
        )
            as Box<dyn FnMut() -> crate::OpaqueHostValue + Send + 'static>)),
        create_texture: std::sync::Arc::new(std::sync::Mutex::new(Box::new(
            move || -> crate::OpaqueHostValue { make_texture() },
        )
            as Box<dyn FnMut() -> crate::OpaqueHostValue + Send + 'static>)),
        queue: MakeDeviceSynthesizedRecord1500877648 {
            __flight_identity: std::sync::Arc::new(()),
            copy_external_image_to_texture: std::sync::Arc::new(std::sync::Mutex::new(Box::new(
                move || -> () {},
            )
                as Box<dyn FnMut() -> () + Send + 'static>)),
            submit: std::sync::Arc::new(std::sync::Mutex::new(
                Box::new(move || -> () {}) as Box<dyn FnMut() -> () + Send + 'static>
            )),
            write_buffer: std::sync::Arc::new(std::sync::Mutex::new(
                Box::new(move || -> () {}) as Box<dyn FnMut() -> () + Send + 'static>
            )),
            write_texture: std::sync::Arc::new(std::sync::Mutex::new(
                Box::new(move || -> () {}) as Box<dyn FnMut() -> () + Send + 'static>
            )),
        },
    };
}

// Source: upstream/packages/render-wgpu/src/wgpuTestHelper.ts:117 (sha256:141e3416352a7c484b1752f2e6c102054eb40b8362ddc9eb09588900c90878cd)
#[derive(Clone, Default)]
struct MakeAdapterRecord3 {
    __flight_identity: std::sync::Arc<()>,
    max_bind_groups: f64,
    min_uniform_buffer_offset_alignment: f64,
}
impl PartialEq for MakeAdapterRecord3 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone)]
struct MakeAdapterSynthesizedRecord3546639382 {
    __flight_identity: std::sync::Arc<()>,
    limits: MakeAdapterRecord3,
    request_device: std::sync::Arc<
        std::sync::Mutex<
            Box<dyn FnMut() -> crate::Promise<crate::OpaqueHostValue> + Send + 'static>,
        >,
    >,
}
impl PartialEq for MakeAdapterSynthesizedRecord3546639382 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

fn make_adapter() -> crate::OpaqueHostValue {
    return MakeAdapterSynthesizedRecord3546639382 {
        __flight_identity: std::sync::Arc::new(()),
        limits: MakeAdapterRecord3 {
            __flight_identity: std::sync::Arc::new(()),
            max_bind_groups: 8.0_f64,
            min_uniform_buffer_offset_alignment: 256.0_f64,
        },
        request_device: std::sync::Arc::new(std::sync::Mutex::new(Box::new(
            move || -> crate::Promise<crate::OpaqueHostValue> {
                {
                    let __flight_value = make_device();
                    let _ = &__flight_value;
                    crate::Promise::<crate::OpaqueHostValue>::default()
                }
            },
        )
            as Box<dyn FnMut() -> crate::Promise<crate::OpaqueHostValue> + Send + 'static>)),
    };
}

// Source: upstream/packages/render-wgpu/src/wgpuTestHelper.ts:126 (sha256:f9405bf588a520e76d9533461a24c2d89bdfa42ddb1e5a5481805c3e94c03d5a)
pub fn create_wgpu_render_state_for_test() -> crate::Promise<WgpuRenderState> {
    Default::default()
}

// Source: upstream/packages/render-wgpu/src/wgpuTestHelper.ts:133 (sha256:1c971f98ef94c776fe45f42aca76624a7e045a7e9bae0f66d408712187f6d117)
#[derive(Clone, Default)]
struct InstallWgpuMockRecord3 {
    __flight_identity: std::sync::Arc<()>,
    get_context: crate::OpaqueHostValue,
}
impl PartialEq for InstallWgpuMockRecord3 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
struct InstallWgpuMockRecord4 {
    __flight_identity: std::sync::Arc<()>,
    value: InstallWgpuMockRecord5,
    configurable: bool,
    writable: bool,
}
impl PartialEq for InstallWgpuMockRecord4 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
struct InstallWgpuMockRecord5 {
    __flight_identity: std::sync::Arc<()>,
}
impl PartialEq for InstallWgpuMockRecord5 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone)]
struct InstallWgpuMockSynthesizedRecord1664981611 {
    __flight_identity: std::sync::Arc<()>,
    get_preferred_canvas_format: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut() -> crate::OpaqueHostValue + Send + 'static>>,
    >,
    request_adapter: std::sync::Arc<
        std::sync::Mutex<
            Box<dyn FnMut() -> crate::Promise<crate::OpaqueHostValue> + Send + 'static>,
        >,
    >,
}
impl PartialEq for InstallWgpuMockSynthesizedRecord1664981611 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn install_wgpu_mock() -> () {
    install_wgpu_constants();
    let gpu = InstallWgpuMockSynthesizedRecord1664981611 {
        __flight_identity: std::sync::Arc::new(()),
        get_preferred_canvas_format: std::sync::Arc::new(std::sync::Mutex::new(Box::new(
            move || -> crate::OpaqueHostValue {
                crate::OpaqueHostValue::String("bgra8unorm".to_owned())
            },
        )
            as Box<dyn FnMut() -> crate::OpaqueHostValue + Send + 'static>)),
        request_adapter: std::sync::Arc::new(std::sync::Mutex::new(Box::new(
            move || -> crate::Promise<crate::OpaqueHostValue> {
                {
                    let __flight_value = make_adapter();
                    let _ = &__flight_value;
                    crate::Promise::<crate::OpaqueHostValue>::default()
                }
            },
        )
            as Box<dyn FnMut() -> crate::Promise<crate::OpaqueHostValue> + Send + 'static>)),
    };
    if (crate::host_value::<Option<crate::OpaqueHostValue>>("host.navigator")).is_none() {
        crate::host_value::<()>("host.defineProperty");
    }
    crate::host_value::<()>("host.defineProperty");
    let orig_get_context = crate::host_value::<crate::OpaqueHostValue>("host.getContext");
    crate::host_value::<InstallWgpuMockRecord3>("host.prototype").get_context =
        std::sync::Arc::new(std::sync::Mutex::new(Box::new(
            move |context_id: String, options: Option<crate::OpaqueHostValue>| -> () {
                if (context_id == "webgpu") {
                    return ClosureSynthesizedRecord4276982621 {
                        __flight_identity: std::sync::Arc::new(()),
                        configure: std::sync::Arc::new(std::sync::Mutex::new(Box::new(
                            move || -> () {},
                        )
                            as Box<dyn FnMut() -> () + Send + 'static>)),
                        get_current_texture: std::sync::Arc::new(std::sync::Mutex::new(Box::new(
                            move || -> crate::OpaqueHostValue { make_texture() },
                        )
                            as Box<dyn FnMut() -> crate::OpaqueHostValue + Send + 'static>)),
                    };
                }
                return crate::host_value::<()>("host.call");
            },
        )
            as Box<dyn FnMut(String, crate::OpaqueHostValue) -> () + Send + 'static>));
}
