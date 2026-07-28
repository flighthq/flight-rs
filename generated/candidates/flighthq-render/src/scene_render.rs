// @generated from upstream/packages/render/src/sceneRender.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_color::{LinearColor, unpack_color_to_linear};
use flighthq_geometry::{
    create_aabb, create_frustum, create_matrix4, is_frustum_intersecting_aabb, multiply_matrix4,
    set_frustum_from_matrix4, set_orthographic_matrix4, set_perspective_matrix4,
    transform_aabb_by_matrix4,
};
use flighthq_node::{get_node_runtime, get_node_world_matrix4};
use flighthq_skeleton3d::compute_skeleton3_d_joint_matrices;
use flighthq_types::{
    Aabb, AabbLike, Adjustment, AmbientLight, Camera, ColorTransform, DirectionalLight, Frustum,
    HemisphereLight, InteractionSignals, LinearColor,
    MAX_FORWARD_LIGHTS as max_forward_lights_constant, Matrix4, Mesh, Node, NodeAny,
    NodeInteractionState, NodeSignals, NodeTraitsKey, PointLight, Projection, RenderState,
    SCENE_LIGHT_AMBIENT_RADIANCE_OFFSET as scene_light_ambient_radiance_offset_constant,
    SCENE_LIGHT_BLOCK_FLOATS as scene_light_block_floats_constant,
    SCENE_LIGHT_DIRECTIONAL_DIRECTION_OFFSET as scene_light_directional_direction_offset_constant,
    SCENE_LIGHT_DIRECTIONAL_RADIANCE_OFFSET as scene_light_directional_radiance_offset_constant,
    SCENE_LIGHT_HEMISPHERE_OFFSET as scene_light_hemisphere_offset_constant,
    SCENE_LIGHT_HEMISPHERE_STRIDE as scene_light_hemisphere_stride_constant,
    SCENE_LIGHT_POINT_OFFSET as scene_light_point_offset_constant,
    SCENE_LIGHT_POINT_STRIDE as scene_light_point_stride_constant,
    SCENE_LIGHT_SPOT_OFFSET as scene_light_spot_offset_constant,
    SCENE_LIGHT_SPOT_STRIDE as scene_light_spot_stride_constant, SceneLightBlock, SceneLights,
    SceneNode, SceneRenderList, SpotLight, Transform3DNode,
};

#[derive(Clone)]
pub struct FlightPartialRecord1 {
    pub __flight_identity: std::sync::Arc<()>,
    pub binding: Option<crate::OpaqueHostValue>,
    pub appearance_id: Option<f64>,
    pub bounds_using_local_bounds_id: Option<f64>,
    pub bounds_using_local_transform_id: Option<f64>,
    pub can_add_child: Option<
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(Node, Node) -> bool + Send + 'static>>>,
    >,
    pub children: Option<Vec<Node>>,
    pub color_adjustments: Option<Vec<Adjustment>>,
    pub resolved_color_transform: Option<ColorTransform>,
    pub color_adjustments_channel_mixing: Option<bool>,
    pub traits: Option<NodeTraitsKey>,
    pub interaction_signals: Option<InteractionSignals>,
    pub local_bounds_id: Option<f64>,
    pub local_bounds_using_local_bounds_id: Option<f64>,
    pub local_content_id: Option<f64>,
    pub local_transform_id: Option<f64>,
    pub local_transform_using_local_transform_id: Option<f64>,
    pub node_signals: Option<NodeSignals>,
    pub interaction_state: Option<NodeInteractionState>,
    pub parent: Option<Node>,
    pub world_bounds_using_local_bounds_id: Option<f64>,
    pub world_bounds_using_world_transform_id: Option<f64>,
    pub world_transform_id: Option<f64>,
    pub world_transform_using_local_transform_id: Option<f64>,
    pub world_transform_using_parent_transform_id: Option<f64>,
}
impl PartialEq for FlightPartialRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/render/src/sceneRender.ts:61 (sha256:97f9c697f3929d9639144650dc1e47f071396fbf8324ef34acf934af39dc1a5d)
pub fn pack_scene_light_block(out: &mut SceneLightBlock, lights: &SceneLights) -> () {
    (*SCRATCH_LIGHT_DATA.lock().unwrap()).fill((0.0_f64) as f32);
    let mut directional_count = 0.0_f64;
    let directional = (lights.directional).clone();
    if (directional).is_some() {
        pack_directional_light(
            &mut (*SCRATCH_LIGHT_DATA.lock().unwrap()),
            &directional.as_ref().unwrap(),
        );
        directional_count = 1.0_f64;
    }
    let mut ambient_count = 0.0_f64;
    let ambient = (lights.ambient).clone();
    if (ambient).is_some() {
        pack_ambient_light(
            &mut (*SCRATCH_LIGHT_DATA.lock().unwrap()),
            &ambient.as_ref().unwrap(),
        );
        ambient_count = 1.0_f64;
    }
    let mut point_count = 0.0_f64;
    let point = (lights.point).clone();
    if (point).is_some() {
        point_count = (point.as_ref().unwrap().len() as f64).min(max_forward_lights_constant);
        {
            let mut i = 0.0_f64;
            while (i < point_count) {
                pack_point_light(
                    &mut (*SCRATCH_LIGHT_DATA.lock().unwrap()),
                    (scene_light_point_offset_constant + (i * scene_light_point_stride_constant)),
                    &point.as_ref().unwrap()[i as usize],
                );
                {
                    i += 1.0;
                    i
                };
            }
        }
    }
    let mut spot_count = 0.0_f64;
    let spot = (lights.spot).clone();
    if (spot).is_some() {
        spot_count = (spot.as_ref().unwrap().len() as f64).min(max_forward_lights_constant);
        {
            let mut i = 0.0_f64;
            while (i < spot_count) {
                pack_spot_light(
                    &mut (*SCRATCH_LIGHT_DATA.lock().unwrap()),
                    (scene_light_spot_offset_constant + (i * scene_light_spot_stride_constant)),
                    &spot.as_ref().unwrap()[i as usize],
                );
                {
                    i += 1.0;
                    i
                };
            }
        }
    }
    let mut hemisphere_count = 0.0_f64;
    let hemisphere = (lights.hemisphere).clone();
    if (hemisphere).is_some() {
        hemisphere_count =
            (hemisphere.as_ref().unwrap().len() as f64).min(max_forward_lights_constant);
        {
            let mut i = 0.0_f64;
            while (i < hemisphere_count) {
                pack_hemisphere_light(
                    &mut (*SCRATCH_LIGHT_DATA.lock().unwrap()),
                    (scene_light_hemisphere_offset_constant
                        + (i * scene_light_hemisphere_stride_constant)),
                    &hemisphere.as_ref().unwrap()[i as usize],
                );
                {
                    i += 1.0;
                    i
                };
            }
        }
    }
    if (((((out.directional_count == directional_count) && (out.ambient_count == ambient_count))
        && (out.point_count == point_count))
        && (out.spot_count == spot_count))
        && (out.hemisphere_count == hemisphere_count))
        && (is_float32_array_equal(&out.data, &(*SCRATCH_LIGHT_DATA.lock().unwrap())))
    {
        return;
    }
    {
        let __flight_offset = (0.0_f64) as usize;
        let __flight_values: Vec<f32> = ((*SCRATCH_LIGHT_DATA.lock().unwrap()).clone())
            .iter()
            .map(|value| (*value) as f32)
            .collect();
        out.data[__flight_offset..__flight_offset + __flight_values.len()]
            .copy_from_slice(&__flight_values);
    };
    out.directional_count = directional_count;
    out.ambient_count = ambient_count;
    out.point_count = point_count;
    out.spot_count = spot_count;
    out.hemisphere_count = hemisphere_count;
    {
        out.version += 1.0;
        out.version
    };
}

// Source: upstream/packages/render/src/sceneRender.ts:141 (sha256:15cb7e9d9c65dd3ed1e67243af14dd68ce8a16e65fc319d3343e34f06810b523)
pub fn prepare_scene_render(
    state: &RenderState,
    scene: &SceneNode,
    camera: &Camera,
    lights: &SceneLights,
) -> SceneRenderList {
    let mut prepared = ensure_prepared_scene(state);
    set_scene_view_projection_matrix4(
        &mut prepared.view_projection,
        camera,
        DEFAULT_VIEWPORT_ASPECT,
    );
    {
        let __flight_argument_1 = (prepared.view_projection).clone();
        set_frustum_from_matrix4(&mut prepared.frustum, &__flight_argument_1)
    };
    pack_scene_light_block(&mut prepared.list.lights, lights);
    prepared.meshes.clear();
    {
        let __flight_argument_1 = (prepared.frustum).clone();
        collect_visible_meshes(
            &NodeAny {
                __flight_identity: std::sync::Arc::clone(&(scene).__flight_identity),
                data: ((scene).data).clone(),
                enabled: (scene).enabled,
                kind: ((scene).kind).clone(),
                name: ((scene).name).clone(),
            },
            &__flight_argument_1,
            &mut prepared.world_bounds,
            &mut prepared.meshes,
        )
    };
    prepared.list.mesh_count = (prepared.meshes.len() as f64);
    {
        let mut m = 0.0_f64;
        while (m < (prepared.meshes.len() as f64)) {
            let mut skin = (prepared.meshes[m as usize].skin).clone();
            if (skin).is_some() {
                compute_skeleton3_d_joint_matrices(&mut skin.as_mut().unwrap().skeleton);
            }
            {
                m += 1.0;
                m
            };
        }
    }
    return prepared.list;
}

// Source: upstream/packages/render/src/sceneRender.ts:177 (sha256:689f4451932aba0abb62e8e99552350da08f40e378876dc60d75dc9c8185cd2a)
fn collect_visible_meshes(
    node: &NodeAny,
    frustum: &Frustum,
    world_bounds: &mut Aabb,
    out: &mut Vec<Mesh>,
) -> () {
    if (!node.enabled) || (!node.visible) {
        return;
    }
    let mesh = node;
    if (((mesh.geometry).clone()).is_some()) && (is_mesh_visible(&mesh, frustum, world_bounds)) {
        out.push(((mesh).clone()).clone());
    }
    let children = (get_node_runtime(node).children).clone();
    if (children).is_some() {
        {
            let mut i = 0.0_f64;
            while (i < (children.as_ref().unwrap().len() as f64)) {
                collect_visible_meshes(
                    &children.as_ref().unwrap()[i as usize],
                    frustum,
                    world_bounds,
                    out,
                );
                {
                    i += 1.0;
                    i
                };
            }
        }
    }
}

// Source: upstream/packages/render/src/sceneRender.ts:208 (sha256:62be9ffcf4bf19579e363737fce56082223122cb50302ffd165efac10efe6c67)
fn ensure_prepared_scene(state: &RenderState) -> PreparedScene {
    let mut prepared = (*PREPARED_SCENES.lock().unwrap())
        .iter()
        .find(|(key, _)| key == &(*state).clone())
        .map(|(_, value)| value.clone());
    if (prepared).is_none() {
        let view_projection = create_matrix4(
            None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            None, None,
        );
        let meshes: Vec<Mesh> = vec![];
        let list: SceneRenderList = SceneRenderList {
            __flight_identity: std::sync::Arc::new(()),
            lights: SceneLightBlock {
                __flight_identity: std::sync::Arc::new(()),
                ambient_count: 0.0_f64,
                data: vec![0.0_f32; (scene_light_block_floats_constant) as usize],
                directional_count: 0.0_f64,
                hemisphere_count: 0.0_f64,
                point_count: 0.0_f64,
                spot_count: 0.0_f64,
                version: 0.0_f64,
            },
            mesh_count: 0.0_f64,
            view_projection: (view_projection).clone(),
            visible_meshes: (meshes).clone(),
        };
        prepared = Some(PreparedScene {
            __flight_identity: std::sync::Arc::new(()),
            frustum: create_frustum(),
            list: (list).clone(),
            meshes: (meshes).clone(),
            view_projection: (view_projection).clone(),
            world_bounds: create_aabb(None, None, None, None, None, None),
        });
        {
            let __flight_key = (*state).clone();
            let __flight_value = (prepared).clone().unwrap();
            if let Some((_, value)) = (*PREPARED_SCENES.lock().unwrap())
                .iter_mut()
                .find(|(key, _)| key == &__flight_key)
            {
                *value = __flight_value;
            } else {
                (*PREPARED_SCENES.lock().unwrap()).push((__flight_key, __flight_value));
            }
        };
    }
    return (prepared).clone().unwrap();
}

// Source: upstream/packages/render/src/sceneRender.ts:239 (sha256:c0643880e674ac80bacd203c21f9f946cdb007d741b645acc8eb0ac92782b952)
fn is_float32_array_equal(a: &Vec<f32>, b: &Vec<f32>) -> bool {
    if ((a.len() as f64) != (b.len() as f64)) {
        return false;
    }
    {
        let mut i = 0.0_f64;
        while (i < (a.len() as f64)) {
            if ((a[i as usize] as f64) != (b[i as usize] as f64)) {
                return false;
            }
            {
                i += 1.0;
                i
            };
        }
    }
    return true;
}

// Source: upstream/packages/render/src/sceneRender.ts:247 (sha256:d7f5126930d4615d23b770831440415418b886c55dc009317502db81789f11ba)
fn is_mesh_visible(mesh: &Mesh, frustum: &Frustum, world_bounds: &mut Aabb) -> bool {
    let bounds = (mesh.geometry.bounds).clone();
    if (bounds).is_none() {
        return true;
    }
    transform_aabb_by_matrix4(
        world_bounds,
        &AabbLike {
            __flight_identity: std::sync::Arc::clone(&(bounds.as_ref().unwrap()).__flight_identity),
            max: ((bounds.as_ref().unwrap()).max).clone(),
            min: ((bounds.as_ref().unwrap()).min).clone(),
        },
        &get_node_world_matrix4(&Transform3DNode {
            __flight_identity: std::sync::Arc::clone(&(mesh).__flight_identity),
            data: ((mesh).data).clone(),
            enabled: (mesh).enabled,
            kind: ((mesh).kind).clone(),
            name: ((mesh).name).clone(),
            position: ((mesh).position).clone(),
            rotation: ((mesh).rotation).clone(),
            scale: ((mesh).scale).clone(),
        }),
    );
    return is_frustum_intersecting_aabb(
        frustum,
        &AabbLike {
            __flight_identity: std::sync::Arc::clone(&(world_bounds).__flight_identity),
            max: ((world_bounds).max).clone(),
            min: ((world_bounds).min).clone(),
        },
    );
}

// Source: upstream/packages/render/src/sceneRender.ts:257 (sha256:3ca6f79c93fdbec70532c0d385b0d6ccdac52980c6c9ae12578a3cf74e991a3a)
fn pack_ambient_light(data: &mut Vec<f32>, ambient: &AmbientLight) -> () {
    unpack_color_to_linear(&mut SCRATCH_COLOR, ambient.color);
    let intensity = ambient.intensity;
    data[(scene_light_ambient_radiance_offset_constant + 0.0_f64) as usize] =
        (SCRATCH_COLOR[0.0_f64 as usize].clone() * intensity) as f32;
    data[(scene_light_ambient_radiance_offset_constant + 1.0_f64) as usize] =
        (SCRATCH_COLOR[1.0_f64 as usize].clone() * intensity) as f32;
    data[(scene_light_ambient_radiance_offset_constant + 2.0_f64) as usize] =
        (SCRATCH_COLOR[2.0_f64 as usize].clone() * intensity) as f32;
}

// Source: upstream/packages/render/src/sceneRender.ts:265 (sha256:56e94f412dc4e6387b49cfaafb331481c84ef48be9a995da946a6504cb1dbd21)
fn pack_directional_light(data: &mut Vec<f32>, directional: &DirectionalLight) -> () {
    data[(scene_light_directional_direction_offset_constant + 0.0_f64) as usize] =
        (directional.direction.x) as f32;
    data[(scene_light_directional_direction_offset_constant + 1.0_f64) as usize] =
        (directional.direction.y) as f32;
    data[(scene_light_directional_direction_offset_constant + 2.0_f64) as usize] =
        (directional.direction.z) as f32;
    unpack_color_to_linear(&mut SCRATCH_COLOR, directional.color);
    let intensity = directional.intensity;
    data[(scene_light_directional_radiance_offset_constant + 0.0_f64) as usize] =
        (SCRATCH_COLOR[0.0_f64 as usize].clone() * intensity) as f32;
    data[(scene_light_directional_radiance_offset_constant + 1.0_f64) as usize] =
        (SCRATCH_COLOR[1.0_f64 as usize].clone() * intensity) as f32;
    data[(scene_light_directional_radiance_offset_constant + 2.0_f64) as usize] =
        (SCRATCH_COLOR[2.0_f64 as usize].clone() * intensity) as f32;
}

// Source: upstream/packages/render/src/sceneRender.ts:279 (sha256:3fb1f11a840cc564167f58eebfb0c933943a2a9f9dd7d7b539035867985a66c4)
fn pack_hemisphere_light(data: &mut Vec<f32>, offset: f64, hemisphere: &HemisphereLight) -> () {
    let intensity = hemisphere.intensity;
    unpack_color_to_linear(&mut SCRATCH_COLOR, hemisphere.sky_color);
    data[(offset + 0.0_f64) as usize] =
        (SCRATCH_COLOR[0.0_f64 as usize].clone() * intensity) as f32;
    data[(offset + 1.0_f64) as usize] =
        (SCRATCH_COLOR[1.0_f64 as usize].clone() * intensity) as f32;
    data[(offset + 2.0_f64) as usize] =
        (SCRATCH_COLOR[2.0_f64 as usize].clone() * intensity) as f32;
    unpack_color_to_linear(&mut SCRATCH_COLOR, hemisphere.ground_color);
    data[(offset + 4.0_f64) as usize] =
        (SCRATCH_COLOR[0.0_f64 as usize].clone() * intensity) as f32;
    data[(offset + 5.0_f64) as usize] =
        (SCRATCH_COLOR[1.0_f64 as usize].clone() * intensity) as f32;
    data[(offset + 6.0_f64) as usize] =
        (SCRATCH_COLOR[2.0_f64 as usize].clone() * intensity) as f32;
    data[(offset + 8.0_f64) as usize] = (0.0_f64) as f32;
    data[(offset + 9.0_f64) as usize] = (1.0_f64) as f32;
    data[(offset + 10.0_f64) as usize] = (0.0_f64) as f32;
}

// Source: upstream/packages/render/src/sceneRender.ts:296 (sha256:f62f05016667dbbb00d5561daad59d526b71b5ffe6ee9ee842a3565079f625ca)
fn pack_point_light(data: &mut Vec<f32>, offset: f64, point: &PointLight) -> () {
    let range = point.range;
    data[(offset + 0.0_f64) as usize] = (point.position.x) as f32;
    data[(offset + 1.0_f64) as usize] = (point.position.y) as f32;
    data[(offset + 2.0_f64) as usize] = (point.position.z) as f32;
    data[(offset + 3.0_f64) as usize] = (range) as f32;
    unpack_color_to_linear(&mut SCRATCH_COLOR, point.color);
    let intensity = point.intensity;
    data[(offset + 4.0_f64) as usize] =
        (SCRATCH_COLOR[0.0_f64 as usize].clone() * intensity) as f32;
    data[(offset + 5.0_f64) as usize] =
        (SCRATCH_COLOR[1.0_f64 as usize].clone() * intensity) as f32;
    data[(offset + 6.0_f64) as usize] =
        (SCRATCH_COLOR[2.0_f64 as usize].clone() * intensity) as f32;
    data[(offset + 7.0_f64) as usize] = if (range > 0.0_f64) {
        (1.0_f64 / (range * range)) as f32
    } else {
        (0.0_f64) as f32
    };
}

// Source: upstream/packages/render/src/sceneRender.ts:312 (sha256:b696592fb83e0467f3194775cc221b740a8cab05a823cb49def47258eb571152)
fn pack_spot_light(data: &mut Vec<f32>, offset: f64, spot: &SpotLight) -> () {
    let range = spot.range;
    data[(offset + 0.0_f64) as usize] = (spot.position.x) as f32;
    data[(offset + 1.0_f64) as usize] = (spot.position.y) as f32;
    data[(offset + 2.0_f64) as usize] = (spot.position.z) as f32;
    data[(offset + 3.0_f64) as usize] = (range) as f32;
    unpack_color_to_linear(&mut SCRATCH_COLOR, spot.color);
    let intensity = spot.intensity;
    data[(offset + 4.0_f64) as usize] =
        (SCRATCH_COLOR[0.0_f64 as usize].clone() * intensity) as f32;
    data[(offset + 5.0_f64) as usize] =
        (SCRATCH_COLOR[1.0_f64 as usize].clone() * intensity) as f32;
    data[(offset + 6.0_f64) as usize] =
        (SCRATCH_COLOR[2.0_f64 as usize].clone() * intensity) as f32;
    data[(offset + 7.0_f64) as usize] = if (range > 0.0_f64) {
        (1.0_f64 / (range * range)) as f32
    } else {
        (0.0_f64) as f32
    };
    data[(offset + 8.0_f64) as usize] = (spot.direction.x) as f32;
    data[(offset + 9.0_f64) as usize] = (spot.direction.y) as f32;
    data[(offset + 10.0_f64) as usize] = (spot.direction.z) as f32;
    data[(offset + 12.0_f64) as usize] = (spot.inner_cone_cos) as f32;
    data[(offset + 13.0_f64) as usize] = (spot.outer_cone_cos) as f32;
}

// Source: upstream/packages/render/src/sceneRender.ts:335 (sha256:6ae943fe437ece4fd9d4775454579270526f230091730ee52eb6d2c3bb9c610e)
fn set_scene_view_projection_matrix4(out: &mut Matrix4, camera: &Camera, aspect: f64) -> () {
    if matches!(&(camera.projection), flighthq_types::Projection::B(_)) {
        set_perspective_matrix4(
            &mut (*SCRATCH_PROJECTION.lock().unwrap()),
            ((match (camera.projection).clone() {
                flighthq_types::Projection::A(_) => panic!("TypeScript union narrowing failed"),
                flighthq_types::Projection::B(value) => value,
            })
            .fov_y
                * 0.5_f64)
                .tan(),
            if ((match (camera.projection).clone() {
                flighthq_types::Projection::A(_) => panic!("TypeScript union narrowing failed"),
                flighthq_types::Projection::B(value) => value,
            })
            .aspect
                != 0.0_f64)
            {
                (match (camera.projection).clone() {
                    flighthq_types::Projection::A(_) => panic!("TypeScript union narrowing failed"),
                    flighthq_types::Projection::B(value) => value,
                })
                .aspect
            } else {
                aspect
            },
            camera.near,
            camera.far,
        );
    } else {
        set_orthographic_matrix4(
            &mut (*SCRATCH_PROJECTION.lock().unwrap()),
            (-(match (camera.projection).clone() {
                flighthq_types::Projection::A(value) => value,
                flighthq_types::Projection::B(_) => panic!("TypeScript union narrowing failed"),
            })
            .half_width),
            (match (camera.projection).clone() {
                flighthq_types::Projection::A(value) => value,
                flighthq_types::Projection::B(_) => panic!("TypeScript union narrowing failed"),
            })
            .half_width,
            (-(match (camera.projection).clone() {
                flighthq_types::Projection::A(value) => value,
                flighthq_types::Projection::B(_) => panic!("TypeScript union narrowing failed"),
            })
            .half_height),
            (match (camera.projection).clone() {
                flighthq_types::Projection::A(value) => value,
                flighthq_types::Projection::B(_) => panic!("TypeScript union narrowing failed"),
            })
            .half_height,
            camera.near,
            camera.far,
        );
    }
    multiply_matrix4(out, &(*SCRATCH_PROJECTION.lock().unwrap()), &camera.view);
}

// Source: upstream/packages/render/src/sceneRender.ts:362 (sha256:dae722491fd94bb008c3e5f116635e5a16ace87866b5ee982c3b83751821366f)
#[derive(Clone)]
struct PreparedScene {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub frustum: Frustum,
    pub list: SceneRenderList,
    pub meshes: Vec<Mesh>,
    pub view_projection: Matrix4,
    pub world_bounds: Aabb,
}
impl PartialEq for PreparedScene {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/render/src/sceneRender.ts:371 (sha256:2c207485463f39c17904eb0af26e8fc3b6d8fb32051db45df5d4d6df8d4f1c47)
const DEFAULT_VIEWPORT_ASPECT: f64 = 1.0_f64;

// Source: upstream/packages/render/src/sceneRender.ts:375 (sha256:459e35164042636f1bf20d52b7ec8c15ab7d9352d14892e01fac8255219672d4)
static PREPARED_SCENES: std::sync::LazyLock<std::sync::Mutex<Vec<(RenderState, PreparedScene)>>> =
    std::sync::LazyLock::new(|| std::sync::Mutex::new(Vec::new()));

// Source: upstream/packages/render/src/sceneRender.ts:377 (sha256:2291cb70f7ead18931601464c4e2eee467fb94b63cf812082325c1054c687612)
static SCRATCH_COLOR: std::sync::LazyLock<std::sync::Mutex<LinearColor>> =
    std::sync::LazyLock::new(|| std::sync::Mutex::new(vec![0.0_f64, 0.0_f64, 0.0_f64, 0.0_f64]));

// Source: upstream/packages/render/src/sceneRender.ts:378 (sha256:1864a8aa2e7023d06e77587b824d77c7a3fa9c7e9c778c8befe1ceb1c69d9249)
static SCRATCH_PROJECTION: std::sync::LazyLock<std::sync::Mutex<Matrix4>> =
    std::sync::LazyLock::new(|| {
        std::sync::Mutex::new(create_matrix4(
            None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            None, None,
        ))
    });

// Source: upstream/packages/render/src/sceneRender.ts:383 (sha256:a51480322a2e1a42b1ef683e3d109a3f7b7da4b5747972cb7c26904713b11111)
static SCRATCH_LIGHT_DATA: std::sync::LazyLock<std::sync::Mutex<Vec<f32>>> =
    std::sync::LazyLock::new(|| {
        std::sync::Mutex::new(vec![0.0_f32; (scene_light_block_floats_constant) as usize])
    });
