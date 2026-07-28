// @generated from upstream/packages/picking/src/pickScene.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_camera::get_camera_screen_to_world_ray;
use flighthq_geometry::{
    create_aabb, create_matrix4, create_ray3_d, create_vector3, get_ray3_d_point_at,
    intersect_ray3_d_aabb, intersect_ray3_d_triangle, inverse_matrix4,
};
use flighthq_mesh::{get_mesh_geometry_triangle_count, get_mesh_geometry_vertex_position};
use flighthq_node::{ensure_node_world_matrix4, get_node_runtime, get_node_world_matrix4};
use flighthq_scene::{get_scene_node_world_bounds, is_mesh};
use flighthq_types::{
    Aabb, AabbLike, Adjustment, Camera, ColorTransform, InteractionSignals, Kind, Material,
    Matrix4, Mesh, MeshGeometry, MeshMorph, Node, NodeData, NodeInteractionState, NodeSignals,
    NodeTraitsKey, Quaternion, Ray3D, SceneHit, SceneNode, Skin, Transform3DNode, Vector3,
};

#[derive(Clone)]
pub struct FlightPartialRecord1 {
    pub __flight_identity: std::sync::Arc<()>,
    pub data: Option<NodeData>,
    pub enabled: Option<bool>,
    pub kind: Option<Kind>,
    pub name: Option<String>,
    pub alpha: Option<f64>,
    pub visible: Option<bool>,
    pub position: Option<Vector3>,
    pub rotation: Option<Quaternion>,
    pub scale: Option<Vector3>,
    pub geometry: Option<MeshGeometry>,
    pub materials: Option<Vec<Option<Material>>>,
    pub morph: Option<MeshMorph>,
    pub skin: Option<Skin>,
}
impl PartialEq for FlightPartialRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone)]
pub struct FlightPartialRecord2 {
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
impl PartialEq for FlightPartialRecord2 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone)]
pub struct FlightPartialRecord3 {
    pub __flight_identity: std::sync::Arc<()>,
    pub data: Option<NodeData>,
    pub enabled: Option<bool>,
    pub kind: Option<Kind>,
    pub name: Option<String>,
    pub alpha: Option<f64>,
    pub visible: Option<bool>,
    pub position: Option<Vector3>,
    pub rotation: Option<Quaternion>,
    pub scale: Option<Vector3>,
}
impl PartialEq for FlightPartialRecord3 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone)]
pub struct FlightPartialRecord4 {
    pub __flight_identity: std::sync::Arc<()>,
    pub alpha: Option<f64>,
    pub visible: Option<bool>,
}
impl PartialEq for FlightPartialRecord4 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone)]
pub struct FlightPartialRecord5 {
    pub __flight_identity: std::sync::Arc<()>,
    pub position: Option<Vector3>,
    pub rotation: Option<Quaternion>,
    pub scale: Option<Vector3>,
}
impl PartialEq for FlightPartialRecord5 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/picking/src/pickScene.ts:24 (sha256:3a8513735f2e1ff5979966971cf58afa9e28db6bef5077145cf4f416dadfbf33)
#[derive(Clone)]
pub struct ScenePickOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub cull_backfaces: Option<bool>,
    pub max_distance: Option<f64>,
    pub predicate:
        Option<std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(Mesh) -> bool + Send + 'static>>>>,
}
impl PartialEq for ScenePickOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/picking/src/pickScene.ts:32 (sha256:5655906d45bab757bd15d1bc8bbf78b557e341a2171e90e7de4a9a8bb21ffa06)
#[derive(Clone)]
struct CreateSceneHitRecord6 {
    __flight_identity: std::sync::Arc<()>,
    distance: f64,
    node: Mesh,
    normal_x: f64,
    normal_y: f64,
    normal_z: f64,
    point_x: f64,
    point_y: f64,
    point_z: f64,
    triangle_index: f64,
    u: f64,
    v: f64,
    w: f64,
}
impl PartialEq for CreateSceneHitRecord6 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn create_scene_hit() -> SceneHit {
    return SceneHit {
        __flight_identity: std::sync::Arc::new(()),
        distance: 0.0_f64,
        node: crate::OpaqueHostValue::Null,
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
    };
}

// Source: upstream/packages/picking/src/pickScene.ts:55 (sha256:0caa230cf562f365027e5dafab8ca15ae865ef1c778ddce59d5fefd8efb6b830)
pub fn pick_scene(
    scene: &mut SceneNode,
    camera: &Camera,
    screen_x: f64,
    screen_y: f64,
    out: &mut SceneHit,
    options: Option<ScenePickOptions>,
) -> Option<SceneHit> {
    if (!build_camera_pick_ray(
        &mut (*_CAMERA_RAY.lock().unwrap()),
        camera,
        screen_x,
        screen_y,
    )) {
        return None;
    }
    return pick_scene_with_ray3_d(
        scene,
        &(*_CAMERA_RAY.lock().unwrap()),
        out,
        Some(((options).clone().unwrap()).clone()),
    );
}

// Source: upstream/packages/picking/src/pickScene.ts:70 (sha256:3dd7d280c0ff31a6ab2183fd95d125d65cd8678a0c2ded83e361132f11d32827)
pub fn pick_scene_all(
    scene: &mut SceneNode,
    camera: &Camera,
    screen_x: f64,
    screen_y: f64,
    out_array: &mut Vec<SceneHit>,
    options: Option<ScenePickOptions>,
) -> Vec<SceneHit> {
    if (!build_camera_pick_ray(
        &mut (*_CAMERA_RAY.lock().unwrap()),
        camera,
        screen_x,
        screen_y,
    )) {
        out_array.clear();
        return out_array.clone();
    }
    return pick_scene_all_with_ray3_d(
        scene,
        &(*_CAMERA_RAY.lock().unwrap()),
        out_array,
        Some(((options).clone().unwrap()).clone()),
    );
}

// Source: upstream/packages/picking/src/pickScene.ts:89 (sha256:bf519e7c16b1976beaa738a40a3a935164bc914a56a0c2ff24b2b6017af57329)
pub fn pick_scene_all_with_ray3_d(
    scene: &mut SceneNode,
    ray: &Ray3D,
    mut out_array: Vec<SceneHit>,
    options: Option<ScenePickOptions>,
) -> Vec<SceneHit> {
    let count: std::sync::Arc<std::sync::Mutex<f64>> =
        std::sync::Arc::new(std::sync::Mutex::new(0.0_f64));
    for_each_scene_ray_hit(
        scene,
        ray,
        ((options).clone()).clone(),
        &mut |hit: SceneHit| -> () {
            let mut slot = out_array[(*count.lock().unwrap()).clone() as usize].clone();
            if (slot).is_none() {
                slot = create_scene_hit();
                {
                    let __flight_index = ((*count.lock().unwrap()).clone()) as usize;
                    let __flight_value = (slot).clone();
                    if __flight_index == out_array.len() {
                        out_array.push(__flight_value);
                    } else {
                        out_array[__flight_index] = __flight_value;
                    }
                };
            }
            copy_scene_hit(&mut slot, &hit);
            {
                (*count.lock().unwrap()) += 1.0;
                (*count.lock().unwrap())
            };
        },
    );
    out_array.truncate(((*count.lock().unwrap()).clone()) as usize);
    {
        let mut __flight_values = out_array;
        __flight_values
            .sort_by(|left, right| left.partial_cmp(right).unwrap_or(std::cmp::Ordering::Equal));
        __flight_values
    };
    return out_array;
}

// Source: upstream/packages/picking/src/pickScene.ts:122 (sha256:af965e26f8798a8af44b6ba97627731b38c41c4aa93a1a014899181360b41591)
pub fn pick_scene_with_ray3_d(
    scene: &mut SceneNode,
    ray: &Ray3D,
    mut out: SceneHit,
    options: Option<ScenePickOptions>,
) -> Option<SceneHit> {
    let found: std::sync::Arc<std::sync::Mutex<bool>> =
        std::sync::Arc::new(std::sync::Mutex::new(false));
    let mut best_t = f64::INFINITY;
    for_each_scene_ray_hit(
        scene,
        ray,
        ((options).clone()).clone(),
        &mut |hit: SceneHit| -> () {
            if (hit.distance >= (*best_t.lock().unwrap()).clone()) {
                return;
            }
            (*best_t.lock().unwrap()) = hit.distance;
            (*found.lock().unwrap()) = true;
            copy_scene_hit(&mut out, &hit);
        },
    );
    return if (*found.lock().unwrap()).clone() {
        Some((out).clone())
    } else {
        None
    };
}

// Source: upstream/packages/picking/src/pickScene.ts:142 (sha256:b887b5500d096cee7e5260fdb7a827409247c9acd76891182714492ef2eb3f6d)
fn build_camera_pick_ray(out: &mut Ray3D, camera: &Camera, screen_x: f64, screen_y: f64) -> bool {
    let aspect = if (camera.projection.kind == "perspective") {
        camera.projection.aspect
    } else {
        1.0_f64
    };
    return get_camera_screen_to_world_ray(out, camera, screen_x, screen_y, aspect);
}

// Source: upstream/packages/picking/src/pickScene.ts:149 (sha256:fe96fb1c0c37574e4c91c28e45cd0aab29bdc66a3c029b3e96d340dc10f4799a)
fn copy_scene_hit(out: &mut SceneHit, src: &SceneHit) -> () {
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

// Source: upstream/packages/picking/src/pickScene.ts:165 (sha256:ea152bd4e892ab4ab31a2cd858c8b1bec371e632da71e1330c10aaf9fbc988c8)
fn compare_scene_hit_by_distance(a: &SceneHit, b: &SceneHit) -> f64 {
    return (a.distance - b.distance);
}

// Source: upstream/packages/picking/src/pickScene.ts:179 (sha256:0ffa4779962fe93c7ba6c0aa1303d01d7e817e04ceb3c7127d948ec0a165aaac)
fn for_each_scene_ray_hit(
    scene: &mut SceneNode,
    ray: &Ray3D,
    options: Option<ScenePickOptions>,
    on_hit: &mut impl FnMut(SceneHit) -> (),
) -> () {
    let predicate = options.as_ref().and_then(|value| (value.predicate).clone());
    let max_distance =
        (options.as_ref().and_then(|value| value.max_distance)).unwrap_or(f64::INFINITY);
    let cull_backfaces = (options.as_ref().and_then(|value| value.cull_backfaces)).unwrap_or(false);
    pick_node(
        &mut scene,
        ray,
        ((predicate).clone()).clone(),
        max_distance,
        cull_backfaces,
        on_hit,
    );
}

// Source: upstream/packages/picking/src/pickScene.ts:193 (sha256:01555430f75e4626e1d862df48ca5fbb9512888750e4c52aa5ccd3f12bf5ecc3)
fn pick_node(
    node: &mut SceneNode,
    ray: &Ray3D,
    predicate: Option<
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(Mesh) -> bool + Send + 'static>>>,
    >,
    max_distance: f64,
    cull_backfaces: bool,
    on_hit: &mut impl FnMut(SceneHit) -> (),
) -> () {
    if (!node.enabled) {
        return;
    }
    if (is_mesh(node))
        && (((predicate).is_none()) || (predicate.as_ref().unwrap().lock().unwrap()(node)))
    {
        intersect_mesh_triangles(node, ray, max_distance, cull_backfaces, on_hit);
    }
    let mut children = (get_node_runtime(&Node {
        __flight_identity: std::sync::Arc::clone(&(node).__flight_identity),
        data: ((node).data).clone(),
        enabled: (node).enabled,
        kind: ((node).kind).clone(),
        name: ((node).name).clone(),
    })
    .children)
        .clone();
    if (children).is_some() {
        {
            let mut i = 0.0_f64;
            while (i < (children.as_mut().unwrap().len() as f64)) {
                pick_node(
                    &mut children.as_mut().unwrap()[i as usize].clone(),
                    ray,
                    ((predicate).clone()).clone(),
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

// Source: upstream/packages/picking/src/pickScene.ts:215 (sha256:1f6f8b8fbbf74c1bd727d9ff65ceb80dc8c2a9c8d0944bac5868984429344bb1)
#[derive(Clone)]
struct OutContextRecord6 {
    __flight_identity: std::sync::Arc<()>,
    x: f64,
    y: f64,
    z: f64,
}
impl PartialEq for OutContextRecord6 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

fn intersect_mesh_triangles(
    mesh: &mut Mesh,
    ray: &Ray3D,
    max_distance: f64,
    cull_backfaces: bool,
    on_hit: &mut impl FnMut(SceneHit) -> (),
) -> () {
    get_scene_node_world_bounds(&mut (*_WORLD_BOUNDS.lock().unwrap()), mesh);
    if (intersect_ray3_d_aabb(
        ray,
        &AabbLike {
            __flight_identity: std::sync::Arc::clone(
                &(*_WORLD_BOUNDS.lock().unwrap()).__flight_identity,
            ),
            max: ((*_WORLD_BOUNDS.lock().unwrap()).max).clone(),
            min: ((*_WORLD_BOUNDS.lock().unwrap()).min).clone(),
        },
    ) < 0.0_f64)
    {
        return;
    }
    ensure_node_world_matrix4(&Transform3DNode {
        __flight_identity: std::sync::Arc::clone(&(mesh).__flight_identity),
        data: ((mesh).data).clone(),
        enabled: (mesh).enabled,
        kind: ((mesh).kind).clone(),
        name: ((mesh).name).clone(),
        position: ((mesh).position).clone(),
        rotation: ((mesh).rotation).clone(),
        scale: ((mesh).scale).clone(),
    });
    let world_matrix = get_node_world_matrix4(&Transform3DNode {
        __flight_identity: std::sync::Arc::clone(&(mesh).__flight_identity),
        data: ((mesh).data).clone(),
        enabled: (mesh).enabled,
        kind: ((mesh).kind).clone(),
        name: ((mesh).name).clone(),
        position: ((mesh).position).clone(),
        rotation: ((mesh).rotation).clone(),
        scale: ((mesh).scale).clone(),
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
    let indices = (mesh.geometry.indices).clone();
    let triangle_count = get_mesh_geometry_triangle_count(&mesh.geometry);
    {
        let mut triangle = 0.0_f64;
        while (triangle < triangle_count) {
            let base = (triangle * 3.0_f64);
            let i0 = if (indices).is_some() {
                (indices.as_ref().unwrap()[base as usize] as f64) as u32
            } else {
                (base) as u32
            };
            let i1 = if (indices).is_some() {
                (indices.as_ref().unwrap()[(base + 1.0_f64) as usize] as f64) as u32
            } else {
                (base + 1.0_f64) as u32
            };
            let i2 = if (indices).is_some() {
                (indices.as_ref().unwrap()[(base + 2.0_f64) as usize] as f64) as u32
            } else {
                (base + 2.0_f64) as u32
            };
            get_mesh_geometry_vertex_position(&mut (*_A.lock().unwrap()), &mesh.geometry, i0);
            get_mesh_geometry_vertex_position(&mut (*_B.lock().unwrap()), &mesh.geometry, i1);
            get_mesh_geometry_vertex_position(&mut (*_C.lock().unwrap()), &mesh.geometry, i2);
            let t = intersect_ray3_d_triangle(
                &(*_LOCAL_RAY.lock().unwrap()),
                &(*_A.lock().unwrap()),
                &(*_B.lock().unwrap()),
                &(*_C.lock().unwrap()),
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
            (*_HIT.lock().unwrap()).node = (*mesh).clone();
            (*_HIT.lock().unwrap()).distance = t;
            (*_HIT.lock().unwrap()).triangle_index = triangle;
            (*_HIT.lock().unwrap()).normal_x = (*_WORLD_NORMAL.lock().unwrap()).x;
            (*_HIT.lock().unwrap()).normal_y = (*_WORLD_NORMAL.lock().unwrap()).y;
            (*_HIT.lock().unwrap()).normal_z = (*_WORLD_NORMAL.lock().unwrap()).z;
            get_ray3_d_point_at(&mut (*_WORLD_POINT.lock().unwrap()), ray, t);
            (*_HIT.lock().unwrap()).point_x = (*_WORLD_POINT.lock().unwrap()).x;
            (*_HIT.lock().unwrap()).point_y = (*_WORLD_POINT.lock().unwrap()).y;
            (*_HIT.lock().unwrap()).point_z = (*_WORLD_POINT.lock().unwrap()).z;
            get_ray3_d_point_at(
                &mut (*_LOCAL_POINT.lock().unwrap()),
                &(*_LOCAL_RAY.lock().unwrap()),
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

// Source: upstream/packages/picking/src/pickScene.ts:281 (sha256:df0a38c5ab1d4d655012be266ba4c63823606570afa6cc6983276ef4beef9d10)
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

// Source: upstream/packages/picking/src/pickScene.ts:302 (sha256:4a05e3c710c0a168432cc4e8712238de8e568cc10a04437a197b8d7d1ba499d4)
fn write_barycentric(out: &mut SceneHit, p: &Vector3, a: &Vector3, b: &Vector3, c: &Vector3) -> () {
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

// Source: upstream/packages/picking/src/pickScene.ts:339 (sha256:6d1a58e8862ff2afff75fad235796e008f91a90e09e74d1820966ee6f9823451)
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

// Source: upstream/packages/picking/src/pickScene.ts:351 (sha256:2d6f1c2570d6991d74f6932f3587edc3a46fa26154ca26dd016f2b9ea76b8302)
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

// Source: upstream/packages/picking/src/pickScene.ts:362 (sha256:7979dd41824c3beb60483579565e406f9e90058ff72ec6062230ef9a9ad86546)
static _CAMERA_RAY: std::sync::LazyLock<std::sync::Mutex<Ray3D>> = std::sync::LazyLock::new(|| {
    std::sync::Mutex::new(create_ray3_d(None, None, None, None, None, None))
});

// Source: upstream/packages/picking/src/pickScene.ts:363 (sha256:8dddddeded83d26bf03efb12b20fde203d20bffa895061f0d2af33bbd9695d2c)
static _LOCAL_RAY: std::sync::LazyLock<std::sync::Mutex<Ray3D>> = std::sync::LazyLock::new(|| {
    std::sync::Mutex::new(create_ray3_d(None, None, None, None, None, None))
});

// Source: upstream/packages/picking/src/pickScene.ts:364 (sha256:2e923ce8abf535735f38c48690cb07b6b2a9019d4ad7092f0ef4a5b8c3308d0a)
static _INVERSE_WORLD: std::sync::LazyLock<std::sync::Mutex<Matrix4>> =
    std::sync::LazyLock::new(|| {
        std::sync::Mutex::new(create_matrix4(
            None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            None, None,
        ))
    });

// Source: upstream/packages/picking/src/pickScene.ts:365 (sha256:eb3ad316a98e5e2b2c4608a94c4491b7b33bd4a28fb0ff1d8aca241c6acd2c70)
static _WORLD_BOUNDS: std::sync::LazyLock<std::sync::Mutex<Aabb>> =
    std::sync::LazyLock::new(|| {
        std::sync::Mutex::new(create_aabb(None, None, None, None, None, None))
    });

// Source: upstream/packages/picking/src/pickScene.ts:366 (sha256:74120b352e1a6c864b3198669a332c52713c85c093924ce2ef578f5c3807d228)
static _A: std::sync::LazyLock<std::sync::Mutex<Vector3>> =
    std::sync::LazyLock::new(|| std::sync::Mutex::new(create_vector3(None, None, None)));

// Source: upstream/packages/picking/src/pickScene.ts:367 (sha256:b436ccb1327ad93d5b769f33ce8fa68bd396078a628042d054602ca2814f4403)
static _B: std::sync::LazyLock<std::sync::Mutex<Vector3>> =
    std::sync::LazyLock::new(|| std::sync::Mutex::new(create_vector3(None, None, None)));

// Source: upstream/packages/picking/src/pickScene.ts:368 (sha256:662079c6fddc4d7b784be8caa1a98d2f6f713cec57348e64bfcb62e46d0a2e35)
static _C: std::sync::LazyLock<std::sync::Mutex<Vector3>> =
    std::sync::LazyLock::new(|| std::sync::Mutex::new(create_vector3(None, None, None)));

// Source: upstream/packages/picking/src/pickScene.ts:369 (sha256:bfbaf85a1c059344d6fb34a05f7d7713282cc01bb68de012fe17f14a2e657d51)
static _WA: std::sync::LazyLock<std::sync::Mutex<Vector3>> =
    std::sync::LazyLock::new(|| std::sync::Mutex::new(create_vector3(None, None, None)));

// Source: upstream/packages/picking/src/pickScene.ts:370 (sha256:f7427028ab1385dde8b9c7c47c49069aeb80fa4543a2b6b00e3c15961646d3e0)
static _WB: std::sync::LazyLock<std::sync::Mutex<Vector3>> =
    std::sync::LazyLock::new(|| std::sync::Mutex::new(create_vector3(None, None, None)));

// Source: upstream/packages/picking/src/pickScene.ts:371 (sha256:197107cb694ff6b870f0ae2eba404a21e17d42424019e4c094122ebbff24eda9)
static _WC: std::sync::LazyLock<std::sync::Mutex<Vector3>> =
    std::sync::LazyLock::new(|| std::sync::Mutex::new(create_vector3(None, None, None)));

// Source: upstream/packages/picking/src/pickScene.ts:372 (sha256:2838df34807f2d0106e1148a78e37fcbd8cae5b4ee55e291faad177ac9bac9c2)
static _WORLD_NORMAL: std::sync::LazyLock<std::sync::Mutex<Vector3>> =
    std::sync::LazyLock::new(|| std::sync::Mutex::new(create_vector3(None, None, None)));

// Source: upstream/packages/picking/src/pickScene.ts:373 (sha256:1f7a7e1d1fc8b3f2d77f0ae830adf96363b5fd5aaa553b37929e7190833e04d6)
static _LOCAL_POINT: std::sync::LazyLock<std::sync::Mutex<Vector3>> =
    std::sync::LazyLock::new(|| std::sync::Mutex::new(create_vector3(None, None, None)));

// Source: upstream/packages/picking/src/pickScene.ts:374 (sha256:2d97130b8aae7c12490473ccacf5f13c42c0b9a3a715a6d92e33fe20f8e7de6f)
static _WORLD_POINT: std::sync::LazyLock<std::sync::Mutex<Vector3>> =
    std::sync::LazyLock::new(|| std::sync::Mutex::new(create_vector3(None, None, None)));

// Source: upstream/packages/picking/src/pickScene.ts:375 (sha256:3a44690e0e6735930046b4b0b03319ad0db7ad21c7efcf3857b455fda4e1707b)
static _HIT: std::sync::LazyLock<std::sync::Mutex<SceneHit>> =
    std::sync::LazyLock::new(|| std::sync::Mutex::new(create_scene_hit()));
