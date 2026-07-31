// @generated from upstream/packages/scene-wgpu/src/wgpuSceneTestHelper.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_render::create_render_state;
use flighthq_render_wgpu::create_wgpu_render_state_runtime;
use flighthq_types::{
    BlendMode, ColorTransform, DisplayObjectClipHooks, Matrix, SceneGraphSyncPolicy,
    WgpuRenderState,
};

#[derive(Clone, Default)]
pub struct SharedStructuralRecord1 {
    pub __flight_identity: std::sync::Arc<()>,
    pub name: String,
    pub args: Vec<crate::OpaqueHostValue>,
}
impl PartialEq for SharedStructuralRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct SharedStructuralRecord2 {
    pub __flight_identity: std::sync::Arc<()>,
    pub fake: FakeWgpu,
    pub state: WgpuRenderState,
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

// Source: upstream/packages/scene-wgpu/src/wgpuSceneTestHelper.ts:12 (sha256:00f175c411bbf69213473cee49f64ece95f3be57fc400fbee9a4bfc554ec9310)
#[derive(Clone, Default)]
pub struct FakeWgpu {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub calls: Vec<SharedStructuralRecord1>,
}
impl PartialEq for FakeWgpu {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/scene-wgpu/src/wgpuSceneTestHelper.ts:18 (sha256:26b451012dcac97b5528d9cd892f68d5e95beef415c438d644a9cfb6922796bc)
#[derive(Clone, Default)]
struct InstallWgpuConstantsRecord5 {
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
impl PartialEq for InstallWgpuConstantsRecord5 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
struct InstallWgpuConstantsRecord6 {
    __flight_identity: std::sync::Arc<()>,
    copy_src: f64,
    copy_dst: f64,
    texture_binding: f64,
    storage_binding: f64,
    render_attachment: f64,
}
impl PartialEq for InstallWgpuConstantsRecord6 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
struct InstallWgpuConstantsRecord7 {
    __flight_identity: std::sync::Arc<()>,
    vertex: f64,
    fragment: f64,
    compute: f64,
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
}

// Source: upstream/packages/scene-wgpu/src/wgpuSceneTestHelper.ts:46 (sha256:82566436b3f65cb60f34652d6afea94f0528594d3348a80249407ab173c22762)
#[derive(Clone, Default)]
struct MakeWgpuSceneStateRecord5 {
    __flight_identity: std::sync::Arc<()>,
}
impl PartialEq for MakeWgpuSceneStateRecord5 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
struct MakeWgpuSceneStateRecord6 {
    __flight_identity: std::sync::Arc<()>,
    min_uniform_buffer_offset_alignment: f64,
}
impl PartialEq for MakeWgpuSceneStateRecord6 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
struct MakeWgpuSceneStateRecord7 {
    __flight_identity: std::sync::Arc<()>,
    width: f64,
    height: f64,
}
impl PartialEq for MakeWgpuSceneStateRecord7 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
struct MakeWgpuSceneStateRecord8 {
    __flight_identity: std::sync::Arc<()>,
    allow_smoothing: bool,
    background_color_rgba: Vec<f64>,
}
impl PartialEq for MakeWgpuSceneStateRecord8 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone)]
struct MakeWgpuSceneStateSynthesizedRecord1771211978 {
    __flight_identity: std::sync::Arc<()>,
    draw: std::sync::Arc<
        std::sync::Mutex<
            Box<dyn FnMut(Vec<crate::OpaqueHostValue>) -> crate::OpaqueHostValue + Send + 'static>,
        >,
    >,
    draw_indexed: std::sync::Arc<
        std::sync::Mutex<
            Box<dyn FnMut(Vec<crate::OpaqueHostValue>) -> crate::OpaqueHostValue + Send + 'static>,
        >,
    >,
    end: std::sync::Arc<
        std::sync::Mutex<
            Box<dyn FnMut(Vec<crate::OpaqueHostValue>) -> crate::OpaqueHostValue + Send + 'static>,
        >,
    >,
    set_bind_group: std::sync::Arc<
        std::sync::Mutex<
            Box<dyn FnMut(Vec<crate::OpaqueHostValue>) -> crate::OpaqueHostValue + Send + 'static>,
        >,
    >,
    set_index_buffer: std::sync::Arc<
        std::sync::Mutex<
            Box<dyn FnMut(Vec<crate::OpaqueHostValue>) -> crate::OpaqueHostValue + Send + 'static>,
        >,
    >,
    set_pipeline: std::sync::Arc<
        std::sync::Mutex<
            Box<dyn FnMut(Vec<crate::OpaqueHostValue>) -> crate::OpaqueHostValue + Send + 'static>,
        >,
    >,
    set_vertex_buffer: std::sync::Arc<
        std::sync::Mutex<
            Box<dyn FnMut(Vec<crate::OpaqueHostValue>) -> crate::OpaqueHostValue + Send + 'static>,
        >,
    >,
    set_viewport: std::sync::Arc<
        std::sync::Mutex<
            Box<dyn FnMut(Vec<crate::OpaqueHostValue>) -> crate::OpaqueHostValue + Send + 'static>,
        >,
    >,
}
impl PartialEq for MakeWgpuSceneStateSynthesizedRecord1771211978 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone)]
struct MakeWgpuSceneStateSynthesizedRecord2439109277 {
    __flight_identity: std::sync::Arc<()>,
    begin_render_pass: std::sync::Arc<
        std::sync::Mutex<
            Box<dyn FnMut(Vec<crate::OpaqueHostValue>) -> crate::OpaqueHostValue + Send + 'static>,
        >,
    >,
    finish: std::sync::Arc<
        std::sync::Mutex<
            Box<dyn FnMut(Vec<crate::OpaqueHostValue>) -> crate::OpaqueHostValue + Send + 'static>,
        >,
    >,
}
impl PartialEq for MakeWgpuSceneStateSynthesizedRecord2439109277 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone)]
struct MakeWgpuSceneStateSynthesizedRecord1545306918 {
    __flight_identity: std::sync::Arc<()>,
    create_bind_group: std::sync::Arc<
        std::sync::Mutex<
            Box<dyn FnMut(Vec<crate::OpaqueHostValue>) -> crate::OpaqueHostValue + Send + 'static>,
        >,
    >,
    create_bind_group_layout: std::sync::Arc<
        std::sync::Mutex<
            Box<dyn FnMut(Vec<crate::OpaqueHostValue>) -> crate::OpaqueHostValue + Send + 'static>,
        >,
    >,
    create_buffer: std::sync::Arc<
        std::sync::Mutex<
            Box<dyn FnMut(crate::OpaqueHostValue) -> crate::OpaqueHostValue + Send + 'static>,
        >,
    >,
    create_command_encoder:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>,
    create_pipeline_layout: std::sync::Arc<
        std::sync::Mutex<
            Box<dyn FnMut(Vec<crate::OpaqueHostValue>) -> crate::OpaqueHostValue + Send + 'static>,
        >,
    >,
    create_render_pipeline: std::sync::Arc<
        std::sync::Mutex<
            Box<dyn FnMut(Vec<crate::OpaqueHostValue>) -> crate::OpaqueHostValue + Send + 'static>,
        >,
    >,
    create_sampler: std::sync::Arc<
        std::sync::Mutex<
            Box<dyn FnMut(Vec<crate::OpaqueHostValue>) -> crate::OpaqueHostValue + Send + 'static>,
        >,
    >,
    create_shader_module: std::sync::Arc<
        std::sync::Mutex<
            Box<dyn FnMut(crate::OpaqueHostValue) -> crate::OpaqueHostValue + Send + 'static>,
        >,
    >,
    create_texture: std::sync::Arc<
        std::sync::Mutex<
            Box<dyn FnMut(crate::OpaqueHostValue) -> crate::OpaqueHostValue + Send + 'static>,
        >,
    >,
    limits: MakeWgpuSceneStateRecord6,
    queue: MakeWgpuSceneStateSynthesizedRecord308913228,
}
impl PartialEq for MakeWgpuSceneStateSynthesizedRecord1545306918 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone)]
struct MakeWgpuSceneStateSynthesizedRecord308913228 {
    __flight_identity: std::sync::Arc<()>,
    copy_external_image_to_texture: std::sync::Arc<
        std::sync::Mutex<
            Box<dyn FnMut(Vec<crate::OpaqueHostValue>) -> crate::OpaqueHostValue + Send + 'static>,
        >,
    >,
    submit: std::sync::Arc<
        std::sync::Mutex<
            Box<dyn FnMut(Vec<crate::OpaqueHostValue>) -> crate::OpaqueHostValue + Send + 'static>,
        >,
    >,
    write_buffer: std::sync::Arc<
        std::sync::Mutex<
            Box<dyn FnMut(Vec<crate::OpaqueHostValue>) -> crate::OpaqueHostValue + Send + 'static>,
        >,
    >,
    write_texture: std::sync::Arc<
        std::sync::Mutex<
            Box<dyn FnMut(Vec<crate::OpaqueHostValue>) -> crate::OpaqueHostValue + Send + 'static>,
        >,
    >,
}
impl PartialEq for MakeWgpuSceneStateSynthesizedRecord308913228 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
struct MakeWgpuSceneStateSynthesizedRecord1726436091 {
    __flight_identity: std::sync::Arc<()>,
    height: f64,
    width: f64,
}
impl PartialEq for MakeWgpuSceneStateSynthesizedRecord1726436091 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn make_wgpu_scene_state() -> SharedStructuralRecord2 {
    install_wgpu_constants();
    let calls: std::sync::Arc<std::sync::Mutex<Vec<SharedStructuralRecord1>>> =
        std::sync::Arc::new(std::sync::Mutex::new(vec![]));
    let mut record: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(
                        String,
                        crate::OpaqueHostValue,
                    ) -> std::sync::Arc<
                        std::sync::Mutex<
                            Box<
                                dyn FnMut(Vec<crate::OpaqueHostValue>) -> crate::OpaqueHostValue
                                    + Send
                                    + 'static,
                            >,
                        >,
                    > + Send
                    + 'static,
            >,
        >,
    > = std::sync::Arc::new(std::sync::Mutex::new(Box::new({
        let mut calls = calls.clone();
        move |name: String,
              result: Option<crate::OpaqueHostValue>|
              -> std::sync::Arc<
            std::sync::Mutex<
                Box<
                    dyn FnMut(Vec<crate::OpaqueHostValue>) -> crate::OpaqueHostValue
                        + Send
                        + 'static,
                >,
            >,
        > {
            std::sync::Arc::new(std::sync::Mutex::new(Box::new({
                let mut calls = calls.clone();
                move |args: Vec<crate::OpaqueHostValue>| -> crate::OpaqueHostValue {
                    (*calls.lock().unwrap()).push(SharedStructuralRecord1 {
                        __flight_identity: std::sync::Arc::new(()),
                        name: (name).clone(),
                        args: (args).clone(),
                    });
                    return ((result).clone().unwrap()).clone();
                }
            })
                as Box<
                    dyn FnMut(Vec<crate::OpaqueHostValue>) -> crate::OpaqueHostValue
                        + Send
                        + 'static,
                >))
        }
    })
        as Box<
            dyn FnMut(
                    String,
                    crate::OpaqueHostValue,
                ) -> std::sync::Arc<
                    std::sync::Mutex<
                        Box<
                            dyn FnMut(Vec<crate::OpaqueHostValue>) -> crate::OpaqueHostValue
                                + Send
                                + 'static,
                        >,
                    >,
                > + Send
                + 'static,
        >));
    let render_pass = MakeWgpuSceneStateSynthesizedRecord1771211978 {
        __flight_identity: std::sync::Arc::new(()),
        draw: {
            let __flight_callback = (record).clone();
            let __flight_result = __flight_callback.lock().unwrap()("draw".to_owned());
            __flight_result
        },
        draw_indexed: {
            let __flight_callback = (record).clone();
            let __flight_result = __flight_callback.lock().unwrap()("drawIndexed".to_owned());
            __flight_result
        },
        end: {
            let __flight_callback = (record).clone();
            let __flight_result = __flight_callback.lock().unwrap()("end".to_owned());
            __flight_result
        },
        set_bind_group: {
            let __flight_callback = (record).clone();
            let __flight_result = __flight_callback.lock().unwrap()("setBindGroup".to_owned());
            __flight_result
        },
        set_index_buffer: {
            let __flight_callback = (record).clone();
            let __flight_result = __flight_callback.lock().unwrap()("setIndexBuffer".to_owned());
            __flight_result
        },
        set_pipeline: {
            let __flight_callback = (record).clone();
            let __flight_result = __flight_callback.lock().unwrap()("setPipeline".to_owned());
            __flight_result
        },
        set_vertex_buffer: {
            let __flight_callback = (record).clone();
            let __flight_result = __flight_callback.lock().unwrap()("setVertexBuffer".to_owned());
            __flight_result
        },
        set_viewport: {
            let __flight_callback = (record).clone();
            let __flight_result = __flight_callback.lock().unwrap()("setViewport".to_owned());
            __flight_result
        },
    };
    let command_encoder = MakeWgpuSceneStateSynthesizedRecord2439109277 {
        __flight_identity: std::sync::Arc::new(()),
        begin_render_pass: {
            let __flight_callback = (record).clone();
            let __flight_result =
                __flight_callback.lock().unwrap()("beginRenderPass".to_owned(), render_pass);
            __flight_result
        },
        finish: {
            let __flight_callback = (record).clone();
            let __flight_result = __flight_callback.lock().unwrap()(
                "finish".to_owned(),
                MakeWgpuSceneStateRecord5 {
                    __flight_identity: std::sync::Arc::new(()),
                },
            );
            __flight_result
        },
    };
    let device = MakeWgpuSceneStateSynthesizedRecord1545306918 {
        __flight_identity: std::sync::Arc::new(()),
        limits: MakeWgpuSceneStateRecord6 {
            __flight_identity: std::sync::Arc::new(()),
            min_uniform_buffer_offset_alignment: 256.0_f64,
        },
        queue: MakeWgpuSceneStateSynthesizedRecord308913228 {
            __flight_identity: std::sync::Arc::new(()),
            copy_external_image_to_texture: {
                let __flight_callback = (record).clone();
                let __flight_result =
                    __flight_callback.lock().unwrap()("copyExternalImageToTexture".to_owned());
                __flight_result
            },
            submit: {
                let __flight_callback = (record).clone();
                let __flight_result = __flight_callback.lock().unwrap()("submit".to_owned());
                __flight_result
            },
            write_buffer: {
                let __flight_callback = (record).clone();
                let __flight_result = __flight_callback.lock().unwrap()("writeBuffer".to_owned());
                __flight_result
            },
            write_texture: {
                let __flight_callback = (record).clone();
                let __flight_result = __flight_callback.lock().unwrap()("writeTexture".to_owned());
                __flight_result
            },
        },
        create_bind_group: {
            let __flight_callback = (record).clone();
            let __flight_result = __flight_callback.lock().unwrap()(
                "createBindGroup".to_owned(),
                MakeWgpuSceneStateRecord5 {
                    __flight_identity: std::sync::Arc::new(()),
                },
            );
            __flight_result
        },
        create_bind_group_layout: {
            let __flight_callback = (record).clone();
            let __flight_result = __flight_callback.lock().unwrap()(
                "createBindGroupLayout".to_owned(),
                MakeWgpuSceneStateRecord5 {
                    __flight_identity: std::sync::Arc::new(()),
                },
            );
            __flight_result
        },
        create_command_encoder: std::sync::Arc::new(std::sync::Mutex::new(Box::new({
            let mut calls = calls.clone();
            let command_encoder = command_encoder.clone();
            move || -> () {
                (*calls.lock().unwrap()).push(SharedStructuralRecord1 {
                    __flight_identity: std::sync::Arc::new(()),
                    name: "createCommandEncoder".to_owned(),
                    args: vec![],
                });
                return command_encoder;
            }
        })
            as Box<dyn FnMut() -> () + Send + 'static>)),
        create_buffer: std::sync::Arc::new(std::sync::Mutex::new(Box::new({
            let mut calls = calls.clone();
            move |descriptor: crate::OpaqueHostValue| -> crate::OpaqueHostValue {
                (*calls.lock().unwrap()).push(SharedStructuralRecord1 {
                    __flight_identity: std::sync::Arc::new(()),
                    name: "createBuffer".to_owned(),
                    args: vec![(descriptor).clone()],
                });
                return ClosureSynthesizedRecord667054565 {
                    __flight_identity: std::sync::Arc::new(()),
                    destroy: std::sync::Arc::new(std::sync::Mutex::new(
                        Box::new(move || -> () {}) as Box<dyn FnMut() -> () + Send + 'static>
                    )),
                };
            }
        })
            as Box<dyn FnMut(crate::OpaqueHostValue) -> crate::OpaqueHostValue + Send + 'static>)),
        create_pipeline_layout: {
            let __flight_callback = (record).clone();
            let __flight_result = __flight_callback.lock().unwrap()(
                "createPipelineLayout".to_owned(),
                MakeWgpuSceneStateRecord5 {
                    __flight_identity: std::sync::Arc::new(()),
                },
            );
            __flight_result
        },
        create_render_pipeline: {
            let __flight_callback = (record).clone();
            let __flight_result = __flight_callback.lock().unwrap()(
                "createRenderPipeline".to_owned(),
                MakeWgpuSceneStateRecord5 {
                    __flight_identity: std::sync::Arc::new(()),
                },
            );
            __flight_result
        },
        create_sampler: {
            let __flight_callback = (record).clone();
            let __flight_result = __flight_callback.lock().unwrap()(
                "createSampler".to_owned(),
                MakeWgpuSceneStateRecord5 {
                    __flight_identity: std::sync::Arc::new(()),
                },
            );
            __flight_result
        },
        create_shader_module: std::sync::Arc::new(std::sync::Mutex::new(Box::new({
            let mut calls = calls.clone();
            move |descriptor: crate::OpaqueHostValue| -> crate::OpaqueHostValue {
                (*calls.lock().unwrap()).push(SharedStructuralRecord1 {
                    __flight_identity: std::sync::Arc::new(()),
                    name: "createShaderModule".to_owned(),
                    args: vec![(descriptor).clone()],
                });
                return MakeWgpuSceneStateRecord5 {
                    __flight_identity: std::sync::Arc::new(()),
                };
            }
        })
            as Box<dyn FnMut(crate::OpaqueHostValue) -> crate::OpaqueHostValue + Send + 'static>)),
        create_texture: std::sync::Arc::new(std::sync::Mutex::new(Box::new({
            let mut calls = calls.clone();
            move |descriptor: crate::OpaqueHostValue| -> crate::OpaqueHostValue {
                (*calls.lock().unwrap()).push(SharedStructuralRecord1 {
                    __flight_identity: std::sync::Arc::new(()),
                    name: "createTexture".to_owned(),
                    args: vec![(descriptor).clone()],
                });
                return ClosureSynthesizedRecord694067184 {
                    __flight_identity: std::sync::Arc::new(()),
                    create_view: std::sync::Arc::new(std::sync::Mutex::new(Box::new(
                        move || -> crate::OpaqueHostValue {
                            MakeWgpuSceneStateRecord5 {
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
        })
            as Box<dyn FnMut(crate::OpaqueHostValue) -> crate::OpaqueHostValue + Send + 'static>)),
    };
    let canvas = MakeWgpuSceneStateSynthesizedRecord1726436091 {
        __flight_identity: std::sync::Arc::new(()),
        width: 256.0_f64,
        height: 256.0_f64,
    };
    let mut state = create_render_state(Some(FlightPartialRecord3 {
        __flight_identity: std::sync::Arc::new(()),
        allow_smoothing: Some(true),
        background_color_rgba: Some(vec![0.0_f64, 0.0_f64, 0.0_f64, 0.0_f64]),
        background_color: None,
        background_color_string: None,
        current_clip_depth: None,
        display_object_clip_hooks: None,
        pixel_ratio: None,
        render_alpha: None,
        render_blend_mode: None,
        render_transform2_d: None,
        scene_graph_sync_policy: None,
        round_pixels: None,
    }));
    crate::host_value::<()>("host.assign");
    let runtime = create_wgpu_render_state_runtime();
    crate::host_value::<()>("host.assign");
    *flighthq_types::FlightEntity::__flight_entity_runtime(&(state))
        .lock()
        .unwrap() = Some((runtime).clone());
    return SharedStructuralRecord2 {
        __flight_identity: std::sync::Arc::new(()),
        fake: FakeWgpu {
            __flight_identity: std::sync::Arc::new(()),
            calls: (*calls.lock().unwrap()).clone(),
        },
        state: (state).clone(),
    };
}
