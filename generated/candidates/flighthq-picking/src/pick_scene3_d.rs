// @generated from upstream/packages/picking/src/pickScene3D.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_camera::get_camera3_d_screen_to_world_ray;
use flighthq_entity::create_entity;
use flighthq_geometry::{
    create_aabb, create_matrix4, create_ray3_d, create_vector3, get_ray3_d_point_at,
    intersect_ray3_d_aabb, intersect_ray3_d_triangle, inverse_matrix4, transform_aabb_by_matrix4,
};
use flighthq_mesh::{
    get_mesh_geometry_triangle_count, get_mesh_geometry_triangle_vertex_indices,
    get_mesh_geometry_vertex_position,
};
use flighthq_node::{ensure_node_world_matrix4, get_node_runtime, get_node_world_matrix4};
use flighthq_scene3d::{get_node3_d_world_bounds, is_mesh};
use flighthq_types::{
    Aabb, AabbLike, Camera3D, Matrix4, Mesh, MeshTriangleVertexIndices, Node, Node3D, Ray3D,
    Ray3DLike, Scene3DHit, Scene3DPickOptions, Transform3DNode, Vector3, Vector3Like,
};

// Source: upstream/packages/picking/src/pickScene3D.ts:35 (sha256:cbb2d28fb6ee2fc03c2d05ff19ca8f9e11ccf7111e4d95a8daead194c8724c31)
pub fn create_scene3_d_hit() -> Scene3DHit {
    return create_entity(Some(Scene3DHit {
        __flight_identity: std::sync::Arc::new(()),
        __flight_entity_snapshot: Default::default(),
        __flight_entity_runtime: Default::default(),
        distance: 0.0_f64,
        node: None,
        normal_x: 0.0_f64,
        normal_y: 0.0_f64,
        normal_z: 0.0_f64,
        point_x: 0.0_f64,
        point_y: 0.0_f64,
        point_z: 0.0_f64,
        triangle_index: (-1.0_f64),
        u: 0.0_f64,
        v: 0.0_f64,
        w: 0.0_f64,
    }));
}

// Source: upstream/packages/picking/src/pickScene3D.ts:58 (sha256:e2020501776df76841e88d458c23ebafaba49d010eedc443aa57fe6a61effea1)
pub fn pick_scene3_d(
    scene: &mut Node3D,
    camera: &Camera3D,
    screen_x: f64,
    screen_y: f64,
    out: &mut Scene3DHit,
    options: Option<Scene3DPickOptions>,
) -> Option<Scene3DHit> {
    if (!build_camera_pick_ray(
        &mut (*_CAMERA_RAY.lock().unwrap()),
        camera,
        screen_x,
        screen_y,
    )) {
        return None;
    }
    return pick_scene3_d_with_ray3_d(
        scene,
        &(*_CAMERA_RAY.lock().unwrap()),
        (out).clone(),
        ((options).clone()).clone(),
    );
}

// Source: upstream/packages/picking/src/pickScene3D.ts:73 (sha256:479e0c7c882e6b3cf3e4942139243dca1389fc5361779ba56593ca2782b2304f)
pub fn pick_scene3_d_all(
    scene: &mut Node3D,
    camera: &Camera3D,
    screen_x: f64,
    screen_y: f64,
    out_array: &mut Vec<Scene3DHit>,
    options: Option<Scene3DPickOptions>,
) -> Vec<Scene3DHit> {
    if (!build_camera_pick_ray(
        &mut (*_CAMERA_RAY.lock().unwrap()),
        camera,
        screen_x,
        screen_y,
    )) {
        out_array.clear();
        return out_array.clone();
    }
    return pick_scene3_d_all_with_ray3_d(
        scene,
        &(*_CAMERA_RAY.lock().unwrap()),
        (out_array).clone(),
        ((options).clone()).clone(),
    );
}

// Source: upstream/packages/picking/src/pickScene3D.ts:92 (sha256:ce39a9c0484fdbac99595ec88fae4c42c2db5e4fe7380bf44568bdb185507d0f)
pub fn pick_scene3_d_all_with_ray3_d(
    scene: &mut Node3D,
    ray: &Ray3D,
    mut out_array: Vec<Scene3DHit>,
    options: Option<Scene3DPickOptions>,
) -> Vec<Scene3DHit> {
    let count: std::sync::Arc<std::sync::Mutex<f64>> =
        std::sync::Arc::new(std::sync::Mutex::new(0.0_f64));
    for_each_scene3_d_ray_hit(scene, ray, &(options), &mut |hit: Scene3DHit| -> () {
        let mut slot: Option<Scene3DHit> = out_array
            .get((*count.lock().unwrap()).clone() as usize)
            .cloned();
        if ((slot).clone()).is_none() {
            slot = Some(create_scene3_d_hit());
            {
                let __flight_index = ((*count.lock().unwrap()).clone()) as usize;
                let __flight_value = ((slot).clone()).clone().unwrap();
                if __flight_index == out_array.len() {
                    out_array.push(__flight_value);
                } else {
                    out_array[__flight_index] = __flight_value;
                }
            };
        }
        copy_scene3_d_hit(slot.as_mut().unwrap(), &hit);
        {
            (*count.lock().unwrap()) += 1.0;
            (*count.lock().unwrap())
        };
    });
    out_array.truncate(((*count.lock().unwrap()).clone()) as usize);
    {
        let mut __flight_values = out_array;
        __flight_values
            .sort_by(|left, right| left.partial_cmp(right).unwrap_or(std::cmp::Ordering::Equal));
        __flight_values
    };
    return out_array;
}

// Source: upstream/packages/picking/src/pickScene3D.ts:125 (sha256:4bd3a72f7178070279d459335b54911d403a341a350c8ff3813f5e47804af422)
pub fn pick_scene3_d_with_ray3_d(
    scene: &mut Node3D,
    ray: &Ray3D,
    mut out: Scene3DHit,
    options: Option<Scene3DPickOptions>,
) -> Option<Scene3DHit> {
    let found: std::sync::Arc<std::sync::Mutex<bool>> =
        std::sync::Arc::new(std::sync::Mutex::new(false));
    let best_t: std::sync::Arc<std::sync::Mutex<f64>> =
        std::sync::Arc::new(std::sync::Mutex::new(f64::INFINITY));
    for_each_scene3_d_ray_hit(scene, ray, &(options), &mut |hit: Scene3DHit| -> () {
        if (hit.distance >= (*best_t.lock().unwrap()).clone()) {
            return;
        }
        (*best_t.lock().unwrap()) = hit.distance;
        (*found.lock().unwrap()) = true;
        copy_scene3_d_hit(&mut out, &hit);
    });
    return if (*found.lock().unwrap()).clone() {
        Some((out).clone())
    } else {
        None
    };
}

// Source: upstream/packages/picking/src/pickScene3D.ts:145 (sha256:7ef33900cfac491cc028bbbe32aa7557e676d1c0762d14ec7d3ed26adda86696)
fn build_camera_pick_ray(out: &mut Ray3D, camera: &Camera3D, screen_x: f64, screen_y: f64) -> bool {
    let aspect = if (match &((camera.projection).clone()) {
        crate::FlightUnion2::A(value) => (value).kind.clone(),
        crate::FlightUnion2::B(value) => (value).kind.clone(),
    } == "perspective")
    {
        camera.projection.aspect
    } else {
        1.0_f64
    };
    return get_camera3_d_screen_to_world_ray(out, camera, screen_x, screen_y, aspect);
}

// Source: upstream/packages/picking/src/pickScene3D.ts:152 (sha256:1dd6b4be16cca3b459af2244b3933a4c264dbca27b26f7085fdd4f6a8182762d)
fn copy_scene3_d_hit(out: &mut Scene3DHit, src: &Scene3DHit) -> () {
    out.node = (src.node).clone();
    out.distance = src.distance;
    out.triangle_index = src.triangle_index;
    out.u = src.u;
    out.v = src.v;
    out.w = src.w;
    out.point_x = src.point_x;
    out.point_y = src.point_y;
    out.point_z = src.point_z;
    out.normal_x = src.normal_x;
    out.normal_y = src.normal_y;
    out.normal_z = src.normal_z;
}

// Source: upstream/packages/picking/src/pickScene3D.ts:168 (sha256:4130ac89cc3e7fbea29ebf2fde781e846879149817949cabfda7873581f2a76f)
fn compare_scene3_d_hit_by_distance(a: &Scene3DHit, b: &Scene3DHit) -> f64 {
    return (a.distance - b.distance);
}

// Source: upstream/packages/picking/src/pickScene3D.ts:182 (sha256:fa9fb014116f0176d515e7d165142b61d271a47b981517ccd301068422271690)
fn for_each_scene3_d_ray_hit(
    scene: &mut Node3D,
    ray: &Ray3D,
    options: &Option<Scene3DPickOptions>,
    on_hit: &mut impl FnMut(Scene3DHit) -> (),
) -> () {
    let predicate = options.as_ref().and_then(|value| (value.predicate).clone());
    let max_distance = (options.as_ref().and_then(|value| value.max_distance))
        .clone()
        .unwrap_or(f64::INFINITY);
    let cull_backfaces = (options.as_ref().and_then(|value| value.cull_backfaces))
        .clone()
        .unwrap_or(false);
    pick_node(
        &mut (*scene).clone(),
        ray,
        &(predicate),
        max_distance,
        cull_backfaces,
        on_hit,
    );
}

// Source: upstream/packages/picking/src/pickScene3D.ts:196 (sha256:e9a7dc23665391a0b2fe1c9fc76bbb982964ebbbf207f44698ee8dbb8e6010ff)
fn pick_node(
    node: &mut Node3D,
    ray: &Ray3D,
    predicate: &Option<
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(Mesh) -> bool + Send + 'static>>>,
    >,
    max_distance: f64,
    cull_backfaces: bool,
    on_hit: &mut impl FnMut(Scene3DHit) -> (),
) -> () {
    if (!node.enabled) {
        return;
    }
    if (is_mesh((node).clone()))
        && (((predicate).is_none()) || (predicate.as_ref().unwrap().lock().unwrap()(node)))
    {
        ({
            #[derive(Clone, Default)]
            struct OutContextRecord5 {
                __flight_identity: std::sync::Arc<()>,
                x: f64,
                y: f64,
                z: f64,
            }
            impl PartialEq for OutContextRecord5 {
                fn eq(&self, other: &Self) -> bool {
                    std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
                }
            }

            || -> () {
                let posed_local_bounds = None::<Option<Aabb>>;
                if (posed_local_bounds).is_some() {
                    ensure_node_world_matrix4(&{
                        let __flight_source = &(node);
                        Transform3DNode {
                            __flight_identity: std::sync::Arc::clone(
                                &__flight_source.__flight_identity,
                            ),
                            __flight_entity_runtime: std::sync::Arc::clone(
                                &__flight_source.__flight_entity_runtime,
                            ),
                            __flight_entity_snapshot: __flight_source
                                .__flight_entity_snapshot
                                .clone()
                                .or_else(|| Some(std::sync::Arc::new((*__flight_source).clone()))),
                            data: (__flight_source.data).clone(),
                            enabled: __flight_source.enabled,
                            kind: (__flight_source.kind).clone(),
                            name: (__flight_source.name).clone(),
                            position: (__flight_source.position).clone(),
                            rotation: (__flight_source.rotation).clone(),
                            scale: (__flight_source.scale).clone(),
                        }
                    });
                    transform_aabb_by_matrix4(
                        &mut (*_WORLD_BOUNDS.lock().unwrap()),
                        &{
                            let __flight_source = &(posed_local_bounds.as_ref().unwrap());
                            AabbLike {
                                __flight_identity: std::sync::Arc::clone(
                                    &__flight_source.__flight_identity,
                                ),
                                max: (__flight_source.max).clone(),
                                min: (__flight_source.min).clone(),
                            }
                        },
                        &get_node_world_matrix4(&{
                            let __flight_source = &(node);
                            Transform3DNode {
                                __flight_identity: std::sync::Arc::clone(
                                    &__flight_source.__flight_identity,
                                ),
                                __flight_entity_runtime: std::sync::Arc::clone(
                                    &__flight_source.__flight_entity_runtime,
                                ),
                                __flight_entity_snapshot: __flight_source
                                    .__flight_entity_snapshot
                                    .clone()
                                    .or_else(|| {
                                        Some(std::sync::Arc::new((*__flight_source).clone()))
                                    }),
                                data: (__flight_source.data).clone(),
                                enabled: __flight_source.enabled,
                                kind: (__flight_source.kind).clone(),
                                name: (__flight_source.name).clone(),
                                position: (__flight_source.position).clone(),
                                rotation: (__flight_source.rotation).clone(),
                                scale: (__flight_source.scale).clone(),
                            }
                        }),
                    );
                } else {
                    get_node3_d_world_bounds(&mut (*_WORLD_BOUNDS.lock().unwrap()), &mut node);
                }
                if (intersect_ray3_d_aabb(
                    &{
                        let __flight_source = &(ray);
                        Ray3DLike {
                            __flight_identity: std::sync::Arc::clone(
                                &__flight_source.__flight_identity,
                            ),
                            __flight_entity_runtime: std::sync::Arc::clone(
                                &__flight_source.__flight_entity_runtime,
                            ),
                            __flight_entity_snapshot: __flight_source
                                .__flight_entity_snapshot
                                .clone(),
                            direction: (__flight_source.direction).clone(),
                            origin: (__flight_source.origin).clone(),
                        }
                    },
                    &{
                        let __flight_source = &(*_WORLD_BOUNDS.lock().unwrap());
                        AabbLike {
                            __flight_identity: std::sync::Arc::clone(
                                &__flight_source.__flight_identity,
                            ),
                            max: (__flight_source.max).clone(),
                            min: (__flight_source.min).clone(),
                        }
                    },
                ) < 0.0_f64)
                {
                    return;
                }
                ensure_node_world_matrix4(&{
                    let __flight_source = &(node);
                    Transform3DNode {
                        __flight_identity: std::sync::Arc::clone(
                            &__flight_source.__flight_identity,
                        ),
                        __flight_entity_runtime: std::sync::Arc::clone(
                            &__flight_source.__flight_entity_runtime,
                        ),
                        __flight_entity_snapshot: __flight_source
                            .__flight_entity_snapshot
                            .clone()
                            .or_else(|| Some(std::sync::Arc::new((*__flight_source).clone()))),
                        data: (__flight_source.data).clone(),
                        enabled: __flight_source.enabled,
                        kind: (__flight_source.kind).clone(),
                        name: (__flight_source.name).clone(),
                        position: (__flight_source.position).clone(),
                        rotation: (__flight_source.rotation).clone(),
                        scale: (__flight_source.scale).clone(),
                    }
                });
                let world_matrix = get_node_world_matrix4(&{
                    let __flight_source = &(node);
                    Transform3DNode {
                        __flight_identity: std::sync::Arc::clone(
                            &__flight_source.__flight_identity,
                        ),
                        __flight_entity_runtime: std::sync::Arc::clone(
                            &__flight_source.__flight_entity_runtime,
                        ),
                        __flight_entity_snapshot: __flight_source
                            .__flight_entity_snapshot
                            .clone()
                            .or_else(|| Some(std::sync::Arc::new((*__flight_source).clone()))),
                        data: (__flight_source.data).clone(),
                        enabled: __flight_source.enabled,
                        kind: (__flight_source.kind).clone(),
                        name: (__flight_source.name).clone(),
                        position: (__flight_source.position).clone(),
                        rotation: (__flight_source.rotation).clone(),
                        scale: (__flight_source.scale).clone(),
                    }
                });
                if (!inverse_matrix4(&mut (*_INVERSE_WORLD.lock().unwrap()), &world_matrix)) {
                    return;
                }
                transform_point_by_matrix4(
                    &mut (*_LOCAL_RAY.lock().unwrap()).origin,
                    &ray.origin,
                    &(*_INVERSE_WORLD.lock().unwrap()).m,
                );
                transform_direction_by_matrix4(
                    &mut (*_LOCAL_RAY.lock().unwrap()).direction,
                    &ray.direction,
                    &(*_INVERSE_WORLD.lock().unwrap()).m,
                );
                let geometry = node.geometry;
                let triangle_count = get_mesh_geometry_triangle_count(&geometry);
                {
                    let mut triangle = 0.0_f64;
                    while (triangle < triangle_count) {
                        if (!get_mesh_geometry_triangle_vertex_indices(
                            &mut (*_TRIANGLE.lock().unwrap()),
                            &geometry,
                            triangle,
                        )) {
                            {
                                triangle += 1.0;
                                triangle
                            };
                            continue;
                        }
                        get_mesh_geometry_vertex_position(
                            &mut (*_A.lock().unwrap()),
                            &geometry,
                            (*_TRIANGLE.lock().unwrap()).i0,
                        );
                        get_mesh_geometry_vertex_position(
                            &mut (*_B.lock().unwrap()),
                            &geometry,
                            (*_TRIANGLE.lock().unwrap()).i1,
                        );
                        get_mesh_geometry_vertex_position(
                            &mut (*_C.lock().unwrap()),
                            &geometry,
                            (*_TRIANGLE.lock().unwrap()).i2,
                        );
                        let t = intersect_ray3_d_triangle(
                            &{
                                let __flight_source = &(*_LOCAL_RAY.lock().unwrap());
                                Ray3DLike {
                                    __flight_identity: std::sync::Arc::clone(
                                        &__flight_source.__flight_identity,
                                    ),
                                    __flight_entity_runtime: std::sync::Arc::clone(
                                        &__flight_source.__flight_entity_runtime,
                                    ),
                                    __flight_entity_snapshot: __flight_source
                                        .__flight_entity_snapshot
                                        .clone(),
                                    direction: (__flight_source.direction).clone(),
                                    origin: (__flight_source.origin).clone(),
                                }
                            },
                            &{
                                let __flight_source = &(*_A.lock().unwrap());
                                Vector3Like {
                                    __flight_identity: std::sync::Arc::clone(
                                        &__flight_source.__flight_identity,
                                    ),
                                    __flight_entity_runtime: std::sync::Arc::clone(
                                        &__flight_source.__flight_entity_runtime,
                                    ),
                                    __flight_entity_snapshot: __flight_source
                                        .__flight_entity_snapshot
                                        .clone(),
                                    x: __flight_source.x,
                                    y: __flight_source.y,
                                    z: __flight_source.z,
                                }
                            },
                            &{
                                let __flight_source = &(*_B.lock().unwrap());
                                Vector3Like {
                                    __flight_identity: std::sync::Arc::clone(
                                        &__flight_source.__flight_identity,
                                    ),
                                    __flight_entity_runtime: std::sync::Arc::clone(
                                        &__flight_source.__flight_entity_runtime,
                                    ),
                                    __flight_entity_snapshot: __flight_source
                                        .__flight_entity_snapshot
                                        .clone(),
                                    x: __flight_source.x,
                                    y: __flight_source.y,
                                    z: __flight_source.z,
                                }
                            },
                            &{
                                let __flight_source = &(*_C.lock().unwrap());
                                Vector3Like {
                                    __flight_identity: std::sync::Arc::clone(
                                        &__flight_source.__flight_identity,
                                    ),
                                    __flight_entity_runtime: std::sync::Arc::clone(
                                        &__flight_source.__flight_entity_runtime,
                                    ),
                                    __flight_entity_snapshot: __flight_source
                                        .__flight_entity_snapshot
                                        .clone(),
                                    x: __flight_source.x,
                                    y: __flight_source.y,
                                    z: __flight_source.z,
                                }
                            },
                        );
                        if (t < 0.0_f64) || (t > max_distance) {
                            {
                                triangle += 1.0;
                                triangle
                            };
                            continue;
                        }
                        transform_point_by_matrix4(
                            &mut (*_WA.lock().unwrap()),
                            &(*_A.lock().unwrap()),
                            &world_matrix.m,
                        );
                        transform_point_by_matrix4(
                            &mut (*_WB.lock().unwrap()),
                            &(*_B.lock().unwrap()),
                            &world_matrix.m,
                        );
                        transform_point_by_matrix4(
                            &mut (*_WC.lock().unwrap()),
                            &(*_C.lock().unwrap()),
                            &world_matrix.m,
                        );
                        if (!write_face_normal(
                            &mut (*_WORLD_NORMAL.lock().unwrap()),
                            &(*_WA.lock().unwrap()),
                            &(*_WB.lock().unwrap()),
                            &(*_WC.lock().unwrap()),
                        )) {
                            {
                                triangle += 1.0;
                                triangle
                            };
                            continue;
                        }
                        if (cull_backfaces)
                            && ((((ray.direction.x * (*_WORLD_NORMAL.lock().unwrap()).x)
                                + (ray.direction.y * (*_WORLD_NORMAL.lock().unwrap()).y))
                                + (ray.direction.z * (*_WORLD_NORMAL.lock().unwrap()).z))
                                > 0.0_f64)
                        {
                            {
                                triangle += 1.0;
                                triangle
                            };
                            continue;
                        }
                        (*_HIT.lock().unwrap()).node = Some((node).clone());
                        (*_HIT.lock().unwrap()).distance = t;
                        (*_HIT.lock().unwrap()).triangle_index = triangle;
                        (*_HIT.lock().unwrap()).normal_x = (*_WORLD_NORMAL.lock().unwrap()).x;
                        (*_HIT.lock().unwrap()).normal_y = (*_WORLD_NORMAL.lock().unwrap()).y;
                        (*_HIT.lock().unwrap()).normal_z = (*_WORLD_NORMAL.lock().unwrap()).z;
                        get_ray3_d_point_at(
                            &mut (*_WORLD_POINT.lock().unwrap()),
                            &{
                                let __flight_source = &(ray);
                                Ray3DLike {
                                    __flight_identity: std::sync::Arc::clone(
                                        &__flight_source.__flight_identity,
                                    ),
                                    __flight_entity_runtime: std::sync::Arc::clone(
                                        &__flight_source.__flight_entity_runtime,
                                    ),
                                    __flight_entity_snapshot: __flight_source
                                        .__flight_entity_snapshot
                                        .clone(),
                                    direction: (__flight_source.direction).clone(),
                                    origin: (__flight_source.origin).clone(),
                                }
                            },
                            t,
                        );
                        (*_HIT.lock().unwrap()).point_x = (*_WORLD_POINT.lock().unwrap()).x;
                        (*_HIT.lock().unwrap()).point_y = (*_WORLD_POINT.lock().unwrap()).y;
                        (*_HIT.lock().unwrap()).point_z = (*_WORLD_POINT.lock().unwrap()).z;
                        get_ray3_d_point_at(
                            &mut (*_LOCAL_POINT.lock().unwrap()),
                            &{
                                let __flight_source = &(*_LOCAL_RAY.lock().unwrap());
                                Ray3DLike {
                                    __flight_identity: std::sync::Arc::clone(
                                        &__flight_source.__flight_identity,
                                    ),
                                    __flight_entity_runtime: std::sync::Arc::clone(
                                        &__flight_source.__flight_entity_runtime,
                                    ),
                                    __flight_entity_snapshot: __flight_source
                                        .__flight_entity_snapshot
                                        .clone(),
                                    direction: (__flight_source.direction).clone(),
                                    origin: (__flight_source.origin).clone(),
                                }
                            },
                            t,
                        );
                        write_barycentric(
                            &mut (*_HIT.lock().unwrap()),
                            &(*_LOCAL_POINT.lock().unwrap()),
                            &(*_A.lock().unwrap()),
                            &(*_B.lock().unwrap()),
                            &(*_C.lock().unwrap()),
                        );
                        on_hit((*_HIT.lock().unwrap()).clone());
                        {
                            triangle += 1.0;
                            triangle
                        };
                    }
                }
            }
        })();
    }
    let mut children = {
        let __flight_slot = get_node_runtime(&{
            let __flight_source = &(node);
            Node {
                __flight_identity: std::sync::Arc::clone(&__flight_source.__flight_identity),
                __flight_entity_runtime: std::sync::Arc::clone(
                    &__flight_source.__flight_entity_runtime,
                ),
                __flight_entity_snapshot: __flight_source
                    .__flight_entity_snapshot
                    .clone()
                    .or_else(|| Some(std::sync::Arc::new((*__flight_source).clone()))),
                data: (__flight_source.data).clone(),
                enabled: __flight_source.enabled,
                kind: (__flight_source.kind).clone(),
                name: (__flight_source.name).clone(),
            }
        })
        .__flight_generic_slot::<crate::NodeRuntimeStorage<Traits>>();
        let __flight_storage = __flight_slot.lock().unwrap();
        (__flight_storage.children).clone()
    };
    if (children).is_some() {
        {
            let mut i = 0.0_f64;
            while (i < (children.as_mut().unwrap().len() as f64)) {
                pick_node(
                    &mut {
                        let __flight_source = &(children.as_mut().unwrap()[i as usize].clone());
                        Node3D {
                            __flight_identity: std::sync::Arc::clone(
                                &__flight_source.__flight_identity,
                            ),
                            __flight_entity_runtime: std::sync::Arc::clone(
                                &__flight_source.__flight_entity_runtime,
                            ),
                            __flight_entity_snapshot: __flight_source
                                .__flight_entity_snapshot
                                .clone(),
                            data: (__flight_source.data).clone(),
                            enabled: __flight_source.enabled,
                            kind: (__flight_source.kind).clone(),
                            name: (__flight_source.name).clone(),
                            alpha: Default::default(),
                            visible: Default::default(),
                            position: Default::default(),
                            rotation: Default::default(),
                            scale: Default::default(),
                        }
                    },
                    ray,
                    predicate,
                    max_distance,
                    cull_backfaces,
                    on_hit,
                );
                {
                    i += 1.0;
                    i
                };
            }
        }
    }
}

// Source: upstream/packages/picking/src/pickScene3D.ts:229 (sha256:ce558c554f6203b7d00930608650821c3933df01a3443d0a61b2b2ff287afef3)
#[derive(Clone, Default)]
struct OutContextRecord5 {
    __flight_identity: std::sync::Arc<()>,
    x: f64,
    y: f64,
    z: f64,
}
impl PartialEq for OutContextRecord5 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

fn intersect_mesh_triangles(
    mesh: &mut Mesh,
    ray: &Ray3D,
    max_distance: f64,
    cull_backfaces: bool,
    on_hit: &mut impl FnMut(Scene3DHit) -> (),
) -> () {
    let posed_local_bounds = None::<Option<Aabb>>;
    if (posed_local_bounds).is_some() {
        ensure_node_world_matrix4(&{
            let __flight_source = &(mesh);
            Transform3DNode {
                __flight_identity: std::sync::Arc::clone(&__flight_source.__flight_identity),
                __flight_entity_runtime: std::sync::Arc::clone(
                    &__flight_source.__flight_entity_runtime,
                ),
                __flight_entity_snapshot: __flight_source
                    .__flight_entity_snapshot
                    .clone()
                    .or_else(|| Some(std::sync::Arc::new((*__flight_source).clone()))),
                data: (__flight_source.data).clone(),
                enabled: __flight_source.enabled,
                kind: (__flight_source.kind).clone(),
                name: (__flight_source.name).clone(),
                position: (__flight_source.position).clone(),
                rotation: (__flight_source.rotation).clone(),
                scale: (__flight_source.scale).clone(),
            }
        });
        transform_aabb_by_matrix4(
            &mut (*_WORLD_BOUNDS.lock().unwrap()),
            &{
                let __flight_source = &(posed_local_bounds.as_ref().unwrap());
                AabbLike {
                    __flight_identity: std::sync::Arc::clone(&__flight_source.__flight_identity),
                    max: (__flight_source.max).clone(),
                    min: (__flight_source.min).clone(),
                }
            },
            &get_node_world_matrix4(&{
                let __flight_source = &(mesh);
                Transform3DNode {
                    __flight_identity: std::sync::Arc::clone(&__flight_source.__flight_identity),
                    __flight_entity_runtime: std::sync::Arc::clone(
                        &__flight_source.__flight_entity_runtime,
                    ),
                    __flight_entity_snapshot: __flight_source
                        .__flight_entity_snapshot
                        .clone()
                        .or_else(|| Some(std::sync::Arc::new((*__flight_source).clone()))),
                    data: (__flight_source.data).clone(),
                    enabled: __flight_source.enabled,
                    kind: (__flight_source.kind).clone(),
                    name: (__flight_source.name).clone(),
                    position: (__flight_source.position).clone(),
                    rotation: (__flight_source.rotation).clone(),
                    scale: (__flight_source.scale).clone(),
                }
            }),
        );
    } else {
        get_node3_d_world_bounds(&mut (*_WORLD_BOUNDS.lock().unwrap()), mesh);
    }
    if (intersect_ray3_d_aabb(
        &{
            let __flight_source = &(ray);
            Ray3DLike {
                __flight_identity: std::sync::Arc::clone(&__flight_source.__flight_identity),
                __flight_entity_runtime: std::sync::Arc::clone(
                    &__flight_source.__flight_entity_runtime,
                ),
                __flight_entity_snapshot: __flight_source.__flight_entity_snapshot.clone(),
                direction: (__flight_source.direction).clone(),
                origin: (__flight_source.origin).clone(),
            }
        },
        &{
            let __flight_source = &(*_WORLD_BOUNDS.lock().unwrap());
            AabbLike {
                __flight_identity: std::sync::Arc::clone(&__flight_source.__flight_identity),
                max: (__flight_source.max).clone(),
                min: (__flight_source.min).clone(),
            }
        },
    ) < 0.0_f64)
    {
        return;
    }
    ensure_node_world_matrix4(&{
        let __flight_source = &(mesh);
        Transform3DNode {
            __flight_identity: std::sync::Arc::clone(&__flight_source.__flight_identity),
            __flight_entity_runtime: std::sync::Arc::clone(
                &__flight_source.__flight_entity_runtime,
            ),
            __flight_entity_snapshot: __flight_source
                .__flight_entity_snapshot
                .clone()
                .or_else(|| Some(std::sync::Arc::new((*__flight_source).clone()))),
            data: (__flight_source.data).clone(),
            enabled: __flight_source.enabled,
            kind: (__flight_source.kind).clone(),
            name: (__flight_source.name).clone(),
            position: (__flight_source.position).clone(),
            rotation: (__flight_source.rotation).clone(),
            scale: (__flight_source.scale).clone(),
        }
    });
    let world_matrix = get_node_world_matrix4(&{
        let __flight_source = &(mesh);
        Transform3DNode {
            __flight_identity: std::sync::Arc::clone(&__flight_source.__flight_identity),
            __flight_entity_runtime: std::sync::Arc::clone(
                &__flight_source.__flight_entity_runtime,
            ),
            __flight_entity_snapshot: __flight_source
                .__flight_entity_snapshot
                .clone()
                .or_else(|| Some(std::sync::Arc::new((*__flight_source).clone()))),
            data: (__flight_source.data).clone(),
            enabled: __flight_source.enabled,
            kind: (__flight_source.kind).clone(),
            name: (__flight_source.name).clone(),
            position: (__flight_source.position).clone(),
            rotation: (__flight_source.rotation).clone(),
            scale: (__flight_source.scale).clone(),
        }
    });
    if (!inverse_matrix4(&mut (*_INVERSE_WORLD.lock().unwrap()), &world_matrix)) {
        return;
    }
    transform_point_by_matrix4(
        &mut (*_LOCAL_RAY.lock().unwrap()).origin,
        &ray.origin,
        &(*_INVERSE_WORLD.lock().unwrap()).m,
    );
    transform_direction_by_matrix4(
        &mut (*_LOCAL_RAY.lock().unwrap()).direction,
        &ray.direction,
        &(*_INVERSE_WORLD.lock().unwrap()).m,
    );
    let triangle_count = get_mesh_geometry_triangle_count(&mesh.geometry);
    {
        let mut triangle = 0.0_f64;
        while (triangle < triangle_count) {
            if (!get_mesh_geometry_triangle_vertex_indices(
                &mut (*_TRIANGLE.lock().unwrap()),
                &mesh.geometry,
                triangle,
            )) {
                {
                    triangle += 1.0;
                    triangle
                };
                continue;
            }
            get_mesh_geometry_vertex_position(
                &mut (*_A.lock().unwrap()),
                &mesh.geometry,
                (*_TRIANGLE.lock().unwrap()).i0,
            );
            get_mesh_geometry_vertex_position(
                &mut (*_B.lock().unwrap()),
                &mesh.geometry,
                (*_TRIANGLE.lock().unwrap()).i1,
            );
            get_mesh_geometry_vertex_position(
                &mut (*_C.lock().unwrap()),
                &mesh.geometry,
                (*_TRIANGLE.lock().unwrap()).i2,
            );
            let t = intersect_ray3_d_triangle(
                &{
                    let __flight_source = &(*_LOCAL_RAY.lock().unwrap());
                    Ray3DLike {
                        __flight_identity: std::sync::Arc::clone(
                            &__flight_source.__flight_identity,
                        ),
                        __flight_entity_runtime: std::sync::Arc::clone(
                            &__flight_source.__flight_entity_runtime,
                        ),
                        __flight_entity_snapshot: __flight_source.__flight_entity_snapshot.clone(),
                        direction: (__flight_source.direction).clone(),
                        origin: (__flight_source.origin).clone(),
                    }
                },
                &{
                    let __flight_source = &(*_A.lock().unwrap());
                    Vector3Like {
                        __flight_identity: std::sync::Arc::clone(
                            &__flight_source.__flight_identity,
                        ),
                        __flight_entity_runtime: std::sync::Arc::clone(
                            &__flight_source.__flight_entity_runtime,
                        ),
                        __flight_entity_snapshot: __flight_source.__flight_entity_snapshot.clone(),
                        x: __flight_source.x,
                        y: __flight_source.y,
                        z: __flight_source.z,
                    }
                },
                &{
                    let __flight_source = &(*_B.lock().unwrap());
                    Vector3Like {
                        __flight_identity: std::sync::Arc::clone(
                            &__flight_source.__flight_identity,
                        ),
                        __flight_entity_runtime: std::sync::Arc::clone(
                            &__flight_source.__flight_entity_runtime,
                        ),
                        __flight_entity_snapshot: __flight_source.__flight_entity_snapshot.clone(),
                        x: __flight_source.x,
                        y: __flight_source.y,
                        z: __flight_source.z,
                    }
                },
                &{
                    let __flight_source = &(*_C.lock().unwrap());
                    Vector3Like {
                        __flight_identity: std::sync::Arc::clone(
                            &__flight_source.__flight_identity,
                        ),
                        __flight_entity_runtime: std::sync::Arc::clone(
                            &__flight_source.__flight_entity_runtime,
                        ),
                        __flight_entity_snapshot: __flight_source.__flight_entity_snapshot.clone(),
                        x: __flight_source.x,
                        y: __flight_source.y,
                        z: __flight_source.z,
                    }
                },
            );
            if (t < 0.0_f64) || (t > max_distance) {
                {
                    triangle += 1.0;
                    triangle
                };
                continue;
            }
            transform_point_by_matrix4(
                &mut (*_WA.lock().unwrap()),
                &(*_A.lock().unwrap()),
                &world_matrix.m,
            );
            transform_point_by_matrix4(
                &mut (*_WB.lock().unwrap()),
                &(*_B.lock().unwrap()),
                &world_matrix.m,
            );
            transform_point_by_matrix4(
                &mut (*_WC.lock().unwrap()),
                &(*_C.lock().unwrap()),
                &world_matrix.m,
            );
            if (!write_face_normal(
                &mut (*_WORLD_NORMAL.lock().unwrap()),
                &(*_WA.lock().unwrap()),
                &(*_WB.lock().unwrap()),
                &(*_WC.lock().unwrap()),
            )) {
                {
                    triangle += 1.0;
                    triangle
                };
                continue;
            }
            if (cull_backfaces)
                && ((((ray.direction.x * (*_WORLD_NORMAL.lock().unwrap()).x)
                    + (ray.direction.y * (*_WORLD_NORMAL.lock().unwrap()).y))
                    + (ray.direction.z * (*_WORLD_NORMAL.lock().unwrap()).z))
                    > 0.0_f64)
            {
                {
                    triangle += 1.0;
                    triangle
                };
                continue;
            }
            (*_HIT.lock().unwrap()).node = Some((*mesh).clone());
            (*_HIT.lock().unwrap()).distance = t;
            (*_HIT.lock().unwrap()).triangle_index = triangle;
            (*_HIT.lock().unwrap()).normal_x = (*_WORLD_NORMAL.lock().unwrap()).x;
            (*_HIT.lock().unwrap()).normal_y = (*_WORLD_NORMAL.lock().unwrap()).y;
            (*_HIT.lock().unwrap()).normal_z = (*_WORLD_NORMAL.lock().unwrap()).z;
            get_ray3_d_point_at(
                &mut (*_WORLD_POINT.lock().unwrap()),
                &{
                    let __flight_source = &(ray);
                    Ray3DLike {
                        __flight_identity: std::sync::Arc::clone(
                            &__flight_source.__flight_identity,
                        ),
                        __flight_entity_runtime: std::sync::Arc::clone(
                            &__flight_source.__flight_entity_runtime,
                        ),
                        __flight_entity_snapshot: __flight_source.__flight_entity_snapshot.clone(),
                        direction: (__flight_source.direction).clone(),
                        origin: (__flight_source.origin).clone(),
                    }
                },
                t,
            );
            (*_HIT.lock().unwrap()).point_x = (*_WORLD_POINT.lock().unwrap()).x;
            (*_HIT.lock().unwrap()).point_y = (*_WORLD_POINT.lock().unwrap()).y;
            (*_HIT.lock().unwrap()).point_z = (*_WORLD_POINT.lock().unwrap()).z;
            get_ray3_d_point_at(
                &mut (*_LOCAL_POINT.lock().unwrap()),
                &{
                    let __flight_source = &(*_LOCAL_RAY.lock().unwrap());
                    Ray3DLike {
                        __flight_identity: std::sync::Arc::clone(
                            &__flight_source.__flight_identity,
                        ),
                        __flight_entity_runtime: std::sync::Arc::clone(
                            &__flight_source.__flight_entity_runtime,
                        ),
                        __flight_entity_snapshot: __flight_source.__flight_entity_snapshot.clone(),
                        direction: (__flight_source.direction).clone(),
                        origin: (__flight_source.origin).clone(),
                    }
                },
                t,
            );
            write_barycentric(
                &mut (*_HIT.lock().unwrap()),
                &(*_LOCAL_POINT.lock().unwrap()),
                &(*_A.lock().unwrap()),
                &(*_B.lock().unwrap()),
                &(*_C.lock().unwrap()),
            );
            on_hit((*_HIT.lock().unwrap()).clone());
            {
                triangle += 1.0;
                triangle
            };
        }
    }
}

// Source: upstream/packages/picking/src/pickScene3D.ts:297 (sha256:df0a38c5ab1d4d655012be266ba4c63823606570afa6cc6983276ef4beef9d10)
fn write_face_normal(out: &mut Vector3, a: &Vector3, b: &Vector3, c: &Vector3) -> bool {
    let e1x = (b.x - a.x);
    let e1y = (b.y - a.y);
    let e1z = (b.z - a.z);
    let e2x = (c.x - a.x);
    let e2y = (c.y - a.y);
    let e2z = (c.z - a.z);
    let nx = ((e1y * e2z) - (e1z * e2y));
    let ny = ((e1z * e2x) - (e1x * e2z));
    let nz = ((e1x * e2y) - (e1y * e2x));
    let length_squared = (((nx * nx) + (ny * ny)) + (nz * nz));
    if (length_squared == 0.0_f64) {
        return false;
    }
    let inv = (1.0_f64 / (length_squared).sqrt());
    out.x = (nx * inv);
    out.y = (ny * inv);
    out.z = (nz * inv);
    return true;
}

// Source: upstream/packages/picking/src/pickScene3D.ts:318 (sha256:060e3f9d06d012844c75e9ff77cf9eb56e8020f23d7cdd436fa73a73d096e21c)
fn write_barycentric(
    out: &mut Scene3DHit,
    p: &Vector3,
    a: &Vector3,
    b: &Vector3,
    c: &Vector3,
) -> () {
    let v0x = (b.x - a.x);
    let v0y = (b.y - a.y);
    let v0z = (b.z - a.z);
    let v1x = (c.x - a.x);
    let v1y = (c.y - a.y);
    let v1z = (c.z - a.z);
    let v2x = (p.x - a.x);
    let v2y = (p.y - a.y);
    let v2z = (p.z - a.z);
    let d00 = (((v0x * v0x) + (v0y * v0y)) + (v0z * v0z));
    let d01 = (((v0x * v1x) + (v0y * v1y)) + (v0z * v1z));
    let d11 = (((v1x * v1x) + (v1y * v1y)) + (v1z * v1z));
    let d20 = (((v2x * v0x) + (v2y * v0y)) + (v2z * v0z));
    let d21 = (((v2x * v1x) + (v2y * v1y)) + (v2z * v1z));
    let denom = ((d00 * d11) - (d01 * d01));
    if (denom == 0.0_f64) {
        out.u = 1.0_f64;
        out.v = 0.0_f64;
        out.w = 0.0_f64;
        return;
    }
    let inv = (1.0_f64 / denom);
    let v = (((d11 * d20) - (d01 * d21)) * inv);
    let w = (((d00 * d21) - (d01 * d20)) * inv);
    out.u = ((1.0_f64 - v) - w);
    out.v = v;
    out.w = w;
}

// Source: upstream/packages/picking/src/pickScene3D.ts:355 (sha256:6d1a58e8862ff2afff75fad235796e008f91a90e09e74d1820966ee6f9823451)
fn transform_point_by_matrix4(out: &mut Vector3, p: &Vector3, m: &Vec<f32>) -> () {
    let x = p.x;
    let y = p.y;
    let z = p.z;
    out.x = (((((m[0.0_f64 as usize] as f64) * x) + ((m[4.0_f64 as usize] as f64) * y))
        + ((m[8.0_f64 as usize] as f64) * z))
        + (m[12.0_f64 as usize] as f64));
    out.y = (((((m[1.0_f64 as usize] as f64) * x) + ((m[5.0_f64 as usize] as f64) * y))
        + ((m[9.0_f64 as usize] as f64) * z))
        + (m[13.0_f64 as usize] as f64));
    out.z = (((((m[2.0_f64 as usize] as f64) * x) + ((m[6.0_f64 as usize] as f64) * y))
        + ((m[10.0_f64 as usize] as f64) * z))
        + (m[14.0_f64 as usize] as f64));
}

// Source: upstream/packages/picking/src/pickScene3D.ts:367 (sha256:2d6f1c2570d6991d74f6932f3587edc3a46fa26154ca26dd016f2b9ea76b8302)
fn transform_direction_by_matrix4(out: &mut Vector3, d: &Vector3, m: &Vec<f32>) -> () {
    let x = d.x;
    let y = d.y;
    let z = d.z;
    out.x = ((((m[0.0_f64 as usize] as f64) * x) + ((m[4.0_f64 as usize] as f64) * y))
        + ((m[8.0_f64 as usize] as f64) * z));
    out.y = ((((m[1.0_f64 as usize] as f64) * x) + ((m[5.0_f64 as usize] as f64) * y))
        + ((m[9.0_f64 as usize] as f64) * z));
    out.z = ((((m[2.0_f64 as usize] as f64) * x) + ((m[6.0_f64 as usize] as f64) * y))
        + ((m[10.0_f64 as usize] as f64) * z));
}

// Source: upstream/packages/picking/src/pickScene3D.ts:378 (sha256:7979dd41824c3beb60483579565e406f9e90058ff72ec6062230ef9a9ad86546)
static _CAMERA_RAY: std::sync::LazyLock<std::sync::Mutex<Ray3D>> = std::sync::LazyLock::new(|| {
    std::sync::Mutex::new(create_ray3_d(None, None, None, None, None, None))
});

// Source: upstream/packages/picking/src/pickScene3D.ts:379 (sha256:8dddddeded83d26bf03efb12b20fde203d20bffa895061f0d2af33bbd9695d2c)
static _LOCAL_RAY: std::sync::LazyLock<std::sync::Mutex<Ray3D>> = std::sync::LazyLock::new(|| {
    std::sync::Mutex::new(create_ray3_d(None, None, None, None, None, None))
});

// Source: upstream/packages/picking/src/pickScene3D.ts:380 (sha256:2e923ce8abf535735f38c48690cb07b6b2a9019d4ad7092f0ef4a5b8c3308d0a)
static _INVERSE_WORLD: std::sync::LazyLock<std::sync::Mutex<Matrix4>> =
    std::sync::LazyLock::new(|| {
        std::sync::Mutex::new(create_matrix4(
            None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            None, None,
        ))
    });

// Source: upstream/packages/picking/src/pickScene3D.ts:381 (sha256:eb3ad316a98e5e2b2c4608a94c4491b7b33bd4a28fb0ff1d8aca241c6acd2c70)
static _WORLD_BOUNDS: std::sync::LazyLock<std::sync::Mutex<Aabb>> =
    std::sync::LazyLock::new(|| {
        std::sync::Mutex::new(create_aabb(None, None, None, None, None, None))
    });

// Source: upstream/packages/picking/src/pickScene3D.ts:382 (sha256:74120b352e1a6c864b3198669a332c52713c85c093924ce2ef578f5c3807d228)
static _A: std::sync::LazyLock<std::sync::Mutex<Vector3>> =
    std::sync::LazyLock::new(|| std::sync::Mutex::new(create_vector3(None, None, None)));

// Source: upstream/packages/picking/src/pickScene3D.ts:383 (sha256:b436ccb1327ad93d5b769f33ce8fa68bd396078a628042d054602ca2814f4403)
static _B: std::sync::LazyLock<std::sync::Mutex<Vector3>> =
    std::sync::LazyLock::new(|| std::sync::Mutex::new(create_vector3(None, None, None)));

// Source: upstream/packages/picking/src/pickScene3D.ts:384 (sha256:662079c6fddc4d7b784be8caa1a98d2f6f713cec57348e64bfcb62e46d0a2e35)
static _C: std::sync::LazyLock<std::sync::Mutex<Vector3>> =
    std::sync::LazyLock::new(|| std::sync::Mutex::new(create_vector3(None, None, None)));

// Source: upstream/packages/picking/src/pickScene3D.ts:385 (sha256:bfbaf85a1c059344d6fb34a05f7d7713282cc01bb68de012fe17f14a2e657d51)
static _WA: std::sync::LazyLock<std::sync::Mutex<Vector3>> =
    std::sync::LazyLock::new(|| std::sync::Mutex::new(create_vector3(None, None, None)));

// Source: upstream/packages/picking/src/pickScene3D.ts:386 (sha256:f7427028ab1385dde8b9c7c47c49069aeb80fa4543a2b6b00e3c15961646d3e0)
static _WB: std::sync::LazyLock<std::sync::Mutex<Vector3>> =
    std::sync::LazyLock::new(|| std::sync::Mutex::new(create_vector3(None, None, None)));

// Source: upstream/packages/picking/src/pickScene3D.ts:387 (sha256:197107cb694ff6b870f0ae2eba404a21e17d42424019e4c094122ebbff24eda9)
static _WC: std::sync::LazyLock<std::sync::Mutex<Vector3>> =
    std::sync::LazyLock::new(|| std::sync::Mutex::new(create_vector3(None, None, None)));

// Source: upstream/packages/picking/src/pickScene3D.ts:388 (sha256:2838df34807f2d0106e1148a78e37fcbd8cae5b4ee55e291faad177ac9bac9c2)
static _WORLD_NORMAL: std::sync::LazyLock<std::sync::Mutex<Vector3>> =
    std::sync::LazyLock::new(|| std::sync::Mutex::new(create_vector3(None, None, None)));

// Source: upstream/packages/picking/src/pickScene3D.ts:389 (sha256:1f7a7e1d1fc8b3f2d77f0ae830adf96363b5fd5aaa553b37929e7190833e04d6)
static _LOCAL_POINT: std::sync::LazyLock<std::sync::Mutex<Vector3>> =
    std::sync::LazyLock::new(|| std::sync::Mutex::new(create_vector3(None, None, None)));

// Source: upstream/packages/picking/src/pickScene3D.ts:390 (sha256:2d97130b8aae7c12490473ccacf5f13c42c0b9a3a715a6d92e33fe20f8e7de6f)
static _WORLD_POINT: std::sync::LazyLock<std::sync::Mutex<Vector3>> =
    std::sync::LazyLock::new(|| std::sync::Mutex::new(create_vector3(None, None, None)));

// Source: upstream/packages/picking/src/pickScene3D.ts:391 (sha256:e3f9518b8ad23dae383ef9fc0240c4b760924a9be4959ac97bd5a450806b017f)
static _HIT: std::sync::LazyLock<std::sync::Mutex<Scene3DHit>> =
    std::sync::LazyLock::new(|| std::sync::Mutex::new(create_scene3_d_hit()));

// Source: upstream/packages/picking/src/pickScene3D.ts:392 (sha256:df5ac1fdfc2c2642879e09f3014359524426eceaccf1b1b97ec616a2184c0b35)
static _TRIANGLE: std::sync::LazyLock<std::sync::Mutex<MeshTriangleVertexIndices>> =
    std::sync::LazyLock::new(|| {
        std::sync::Mutex::new(MeshTriangleVertexIndices {
            __flight_identity: std::sync::Arc::new(()),
            i0: 0.0_f64,
            i1: 0.0_f64,
            i2: 0.0_f64,
        })
    });
