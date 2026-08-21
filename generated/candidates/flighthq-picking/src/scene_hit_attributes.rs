// @generated from upstream/packages/picking/src/sceneHitAttributes.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_geometry::{create_matrix4, create_vector3, inverse_matrix4};
use flighthq_mesh::{
    get_mesh_geometry_triangle_subset_index, get_mesh_geometry_triangle_vertex_indices,
    get_mesh_geometry_vertex_normal, get_mesh_geometry_vertex_tangent,
    get_mesh_geometry_vertex_uv0,
};
use flighthq_node::{ensure_node_world_matrix4, get_node_world_matrix4};
use flighthq_types::{
    Material, Matrix4, MeshTriangleVertexIndices, Ray3D, Scene3DHit, Transform3DNode, Vector2,
    Vector2Like, Vector3, Vector3Like, Vector4,
};

#[derive(Clone, Default)]
pub struct SharedStructuralRecord1 {
    pub __flight_identity: std::sync::Arc<()>,
    pub w: f64,
    pub x: f64,
    pub y: f64,
    pub z: f64,
}
impl PartialEq for SharedStructuralRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/picking/src/sceneHitAttributes.ts:13 (sha256:ad3932ec0f435e0bdcba2a3cf451223eb909ea42af9c2ae2e0a359ac35c3e0f6)
pub fn get_scene3_d_hit_material(hit: &Scene3DHit) -> Option<Material> {
    let node = (hit.node).clone();
    if (node).is_none() {
        return None;
    }
    let subset_index = get_mesh_geometry_triangle_subset_index(
        &node.as_ref().unwrap().geometry,
        hit.triangle_index,
    );
    return if (subset_index < 0.0_f64) {
        None
    } else {
        node.as_ref().unwrap().materials[subset_index as usize].clone()
    };
}

// Source: upstream/packages/picking/src/sceneHitAttributes.ts:22 (sha256:56c7fc01967d3dbbade5499654be82c02ed776203cc610e3a1439c6c8f4f0512)
pub fn get_scene3_d_hit_subset_index(hit: &Scene3DHit) -> f64 {
    let node = (hit.node).clone();
    return if (node).is_none() {
        (-1.0_f64)
    } else {
        get_mesh_geometry_triangle_subset_index(
            &node.as_ref().unwrap().geometry,
            hit.triangle_index,
        )
    };
}

// Source: upstream/packages/picking/src/sceneHitAttributes.ts:29 (sha256:7475248321b679720ebdf6a99cbd01184c8431742c19c438905651aa850b3203)
#[derive(Clone, Default)]
struct OutContextRecord6 {
    __flight_identity: std::sync::Arc<()>,
    x: f64,
    y: f64,
}
impl PartialEq for OutContextRecord6 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn get_scene3_d_hit_uv0(out: &mut Vector2Like, hit: &Scene3DHit) -> bool {
    let node = (hit.node).clone();
    if (node).is_none() {
        return false;
    }
    if (!get_mesh_geometry_triangle_vertex_indices(
        &mut (*_TRIANGLE.lock().unwrap()),
        &node.as_ref().unwrap().geometry,
        hit.triangle_index,
    )) {
        return false;
    }
    if (!get_mesh_geometry_vertex_uv0(
        &mut (*_UV0.lock().unwrap()),
        &node.as_ref().unwrap().geometry,
        (*_TRIANGLE.lock().unwrap()).i0,
    )) {
        return false;
    }
    if (!get_mesh_geometry_vertex_uv0(
        &mut (*_UV1.lock().unwrap()),
        &node.as_ref().unwrap().geometry,
        (*_TRIANGLE.lock().unwrap()).i1,
    )) {
        return false;
    }
    if (!get_mesh_geometry_vertex_uv0(
        &mut (*_UV2.lock().unwrap()),
        &node.as_ref().unwrap().geometry,
        (*_TRIANGLE.lock().unwrap()).i2,
    )) {
        return false;
    }
    out.x = (((hit.u * (*_UV0.lock().unwrap()).x) + (hit.v * (*_UV1.lock().unwrap()).x))
        + (hit.w * (*_UV2.lock().unwrap()).x));
    out.y = (((hit.u * (*_UV0.lock().unwrap()).y) + (hit.v * (*_UV1.lock().unwrap()).y))
        + (hit.w * (*_UV2.lock().unwrap()).y));
    return true;
}

// Source: upstream/packages/picking/src/sceneHitAttributes.ts:45 (sha256:ff0c52908eeea87aa83308b2d1c33f0de37e2e007116a0daf8915c3d053178db)
#[derive(Clone, Default)]
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

pub fn get_scene3_d_hit_vertex_normal(out: &mut Vector3Like, hit: &Scene3DHit) -> bool {
    let node = (hit.node).clone();
    if (node).is_none() {
        return false;
    }
    if (!get_mesh_geometry_triangle_vertex_indices(
        &mut (*_TRIANGLE.lock().unwrap()),
        &node.as_ref().unwrap().geometry,
        hit.triangle_index,
    )) {
        return false;
    }
    if (!get_mesh_geometry_vertex_normal(
        &mut (*_NORMAL0.lock().unwrap()),
        &node.as_ref().unwrap().geometry,
        (*_TRIANGLE.lock().unwrap()).i0,
    )) {
        return false;
    }
    if (!get_mesh_geometry_vertex_normal(
        &mut (*_NORMAL1.lock().unwrap()),
        &node.as_ref().unwrap().geometry,
        (*_TRIANGLE.lock().unwrap()).i1,
    )) {
        return false;
    }
    if (!get_mesh_geometry_vertex_normal(
        &mut (*_NORMAL2.lock().unwrap()),
        &node.as_ref().unwrap().geometry,
        (*_TRIANGLE.lock().unwrap()).i2,
    )) {
        return false;
    }
    let nx = (((hit.u * (*_NORMAL0.lock().unwrap()).x) + (hit.v * (*_NORMAL1.lock().unwrap()).x))
        + (hit.w * (*_NORMAL2.lock().unwrap()).x));
    let ny = (((hit.u * (*_NORMAL0.lock().unwrap()).y) + (hit.v * (*_NORMAL1.lock().unwrap()).y))
        + (hit.w * (*_NORMAL2.lock().unwrap()).y));
    let nz = (((hit.u * (*_NORMAL0.lock().unwrap()).z) + (hit.v * (*_NORMAL1.lock().unwrap()).z))
        + (hit.w * (*_NORMAL2.lock().unwrap()).z));
    ensure_node_world_matrix4(&{
        let __flight_source = &(node.as_ref().unwrap());
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
    if (!inverse_matrix4(
        &mut (*_INVERSE_WORLD.lock().unwrap()),
        &get_node_world_matrix4(&{
            let __flight_source = &(node.as_ref().unwrap());
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
    )) {
        return false;
    }
    return write_normalized(
        out,
        (((((*_INVERSE_WORLD.lock().unwrap()).m[0.0_f64 as usize] as f64) * nx)
            + (((*_INVERSE_WORLD.lock().unwrap()).m[1.0_f64 as usize] as f64) * ny))
            + (((*_INVERSE_WORLD.lock().unwrap()).m[2.0_f64 as usize] as f64) * nz)),
        (((((*_INVERSE_WORLD.lock().unwrap()).m[4.0_f64 as usize] as f64) * nx)
            + (((*_INVERSE_WORLD.lock().unwrap()).m[5.0_f64 as usize] as f64) * ny))
            + (((*_INVERSE_WORLD.lock().unwrap()).m[6.0_f64 as usize] as f64) * nz)),
        (((((*_INVERSE_WORLD.lock().unwrap()).m[8.0_f64 as usize] as f64) * nx)
            + (((*_INVERSE_WORLD.lock().unwrap()).m[9.0_f64 as usize] as f64) * ny))
            + (((*_INVERSE_WORLD.lock().unwrap()).m[10.0_f64 as usize] as f64) * nz)),
    );
}

// Source: upstream/packages/picking/src/sceneHitAttributes.ts:71 (sha256:67c93374f216447ac19f717b40253302c9ff003420a4c11dc8f8b43e20e4588a)
pub fn get_scene3_d_hit_vertex_tangent(
    out: &mut SharedStructuralRecord1,
    hit: &Scene3DHit,
) -> bool {
    let node = (hit.node).clone();
    if (node).is_none() {
        return false;
    }
    if (!get_mesh_geometry_triangle_vertex_indices(
        &mut (*_TRIANGLE.lock().unwrap()),
        &node.as_ref().unwrap().geometry,
        hit.triangle_index,
    )) {
        return false;
    }
    if (!get_mesh_geometry_vertex_tangent(
        &mut (*_TANGENT0.lock().unwrap()),
        &node.as_ref().unwrap().geometry,
        (*_TRIANGLE.lock().unwrap()).i0,
    )) {
        return false;
    }
    if (!get_mesh_geometry_vertex_tangent(
        &mut (*_TANGENT1.lock().unwrap()),
        &node.as_ref().unwrap().geometry,
        (*_TRIANGLE.lock().unwrap()).i1,
    )) {
        return false;
    }
    if (!get_mesh_geometry_vertex_tangent(
        &mut (*_TANGENT2.lock().unwrap()),
        &node.as_ref().unwrap().geometry,
        (*_TRIANGLE.lock().unwrap()).i2,
    )) {
        return false;
    }
    if (!({
        #[derive(Clone, Default)]
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

        || -> bool {
            let node = (hit.node).clone();
            if (node.as_ref().unwrap()).is_none() {
                return false;
            }
            if (!get_mesh_geometry_triangle_vertex_indices(
                &mut (*_TRIANGLE.lock().unwrap()),
                &node.as_ref().unwrap().geometry,
                hit.triangle_index,
            )) {
                return false;
            }
            if (!get_mesh_geometry_vertex_normal(
                &mut (*_NORMAL0.lock().unwrap()),
                &node.as_ref().unwrap().geometry,
                (*_TRIANGLE.lock().unwrap()).i0,
            )) {
                return false;
            }
            if (!get_mesh_geometry_vertex_normal(
                &mut (*_NORMAL1.lock().unwrap()),
                &node.as_ref().unwrap().geometry,
                (*_TRIANGLE.lock().unwrap()).i1,
            )) {
                return false;
            }
            if (!get_mesh_geometry_vertex_normal(
                &mut (*_NORMAL2.lock().unwrap()),
                &node.as_ref().unwrap().geometry,
                (*_TRIANGLE.lock().unwrap()).i2,
            )) {
                return false;
            }
            let nx = (((hit.u * (*_NORMAL0.lock().unwrap()).x)
                + (hit.v * (*_NORMAL1.lock().unwrap()).x))
                + (hit.w * (*_NORMAL2.lock().unwrap()).x));
            let ny = (((hit.u * (*_NORMAL0.lock().unwrap()).y)
                + (hit.v * (*_NORMAL1.lock().unwrap()).y))
                + (hit.w * (*_NORMAL2.lock().unwrap()).y));
            let nz = (((hit.u * (*_NORMAL0.lock().unwrap()).z)
                + (hit.v * (*_NORMAL1.lock().unwrap()).z))
                + (hit.w * (*_NORMAL2.lock().unwrap()).z));
            ensure_node_world_matrix4(&{
                let __flight_source = &(node.as_ref().unwrap());
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
            if (!inverse_matrix4(
                &mut (*_INVERSE_WORLD.lock().unwrap()),
                &get_node_world_matrix4(&{
                    let __flight_source = &(node.as_ref().unwrap());
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
                }),
            )) {
                return false;
            }
            return (|| -> bool {
                let length_squared = (((((((m[0.0_f64 as usize] as f64) * nx)
                    + ((m[1.0_f64 as usize] as f64) * ny))
                    + ((m[2.0_f64 as usize] as f64) * nz))
                    * ((((m[0.0_f64 as usize] as f64) * nx)
                        + ((m[1.0_f64 as usize] as f64) * ny))
                        + ((m[2.0_f64 as usize] as f64) * nz)))
                    + (((((m[4.0_f64 as usize] as f64) * nx)
                        + ((m[5.0_f64 as usize] as f64) * ny))
                        + ((m[6.0_f64 as usize] as f64) * nz))
                        * ((((m[4.0_f64 as usize] as f64) * nx)
                            + ((m[5.0_f64 as usize] as f64) * ny))
                            + ((m[6.0_f64 as usize] as f64) * nz))))
                    + (((((m[8.0_f64 as usize] as f64) * nx)
                        + ((m[9.0_f64 as usize] as f64) * ny))
                        + ((m[10.0_f64 as usize] as f64) * nz))
                        * ((((m[8.0_f64 as usize] as f64) * nx)
                            + ((m[9.0_f64 as usize] as f64) * ny))
                            + ((m[10.0_f64 as usize] as f64) * nz))));
                if (length_squared == 0.0_f64) {
                    return false;
                }
                let inverse_length = (1.0_f64 / (length_squared).sqrt());
                (*_WORLD_NORMAL.lock().unwrap()).x = (((((m[0.0_f64 as usize] as f64) * nx)
                    + ((m[1.0_f64 as usize] as f64) * ny))
                    + ((m[2.0_f64 as usize] as f64) * nz))
                    * inverse_length);
                (*_WORLD_NORMAL.lock().unwrap()).y = (((((m[4.0_f64 as usize] as f64) * nx)
                    + ((m[5.0_f64 as usize] as f64) * ny))
                    + ((m[6.0_f64 as usize] as f64) * nz))
                    * inverse_length);
                (*_WORLD_NORMAL.lock().unwrap()).z = (((((m[8.0_f64 as usize] as f64) * nx)
                    + ((m[9.0_f64 as usize] as f64) * ny))
                    + ((m[10.0_f64 as usize] as f64) * nz))
                    * inverse_length);
                return true;
            })();
        }
    })()) {
        return false;
    }
    let tx = (((hit.u * (*_TANGENT0.lock().unwrap()).x)
        + (hit.v * (*_TANGENT1.lock().unwrap()).x))
        + (hit.w * (*_TANGENT2.lock().unwrap()).x));
    let ty = (((hit.u * (*_TANGENT0.lock().unwrap()).y)
        + (hit.v * (*_TANGENT1.lock().unwrap()).y))
        + (hit.w * (*_TANGENT2.lock().unwrap()).y));
    let tz = (((hit.u * (*_TANGENT0.lock().unwrap()).z)
        + (hit.v * (*_TANGENT1.lock().unwrap()).z))
        + (hit.w * (*_TANGENT2.lock().unwrap()).z));
    let tw = (((hit.u * (*_TANGENT0.lock().unwrap()).w)
        + (hit.v * (*_TANGENT1.lock().unwrap()).w))
        + (hit.w * (*_TANGENT2.lock().unwrap()).w));
    let mut wx = ((((get_node_world_matrix4(&{
        let __flight_source = &(node.as_ref().unwrap());
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
    })
    .m[0.0_f64 as usize] as f64)
        * tx)
        + ((get_node_world_matrix4(&{
            let __flight_source = &(node.as_ref().unwrap());
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
        })
        .m[4.0_f64 as usize] as f64)
            * ty))
        + ((get_node_world_matrix4(&{
            let __flight_source = &(node.as_ref().unwrap());
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
        })
        .m[8.0_f64 as usize] as f64)
            * tz));
    let mut wy = ((((get_node_world_matrix4(&{
        let __flight_source = &(node.as_ref().unwrap());
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
    })
    .m[1.0_f64 as usize] as f64)
        * tx)
        + ((get_node_world_matrix4(&{
            let __flight_source = &(node.as_ref().unwrap());
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
        })
        .m[5.0_f64 as usize] as f64)
            * ty))
        + ((get_node_world_matrix4(&{
            let __flight_source = &(node.as_ref().unwrap());
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
        })
        .m[9.0_f64 as usize] as f64)
            * tz));
    let mut wz = ((((get_node_world_matrix4(&{
        let __flight_source = &(node.as_ref().unwrap());
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
    })
    .m[2.0_f64 as usize] as f64)
        * tx)
        + ((get_node_world_matrix4(&{
            let __flight_source = &(node.as_ref().unwrap());
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
        })
        .m[6.0_f64 as usize] as f64)
            * ty))
        + ((get_node_world_matrix4(&{
            let __flight_source = &(node.as_ref().unwrap());
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
        })
        .m[10.0_f64 as usize] as f64)
            * tz));
    let projection = (((wx * (*_WORLD_NORMAL.lock().unwrap()).x)
        + (wy * (*_WORLD_NORMAL.lock().unwrap()).y))
        + (wz * (*_WORLD_NORMAL.lock().unwrap()).z));
    wx -= (projection * (*_WORLD_NORMAL.lock().unwrap()).x);
    wy -= (projection * (*_WORLD_NORMAL.lock().unwrap()).y);
    wz -= (projection * (*_WORLD_NORMAL.lock().unwrap()).z);
    let length_squared = (((wx * wx) + (wy * wy)) + (wz * wz));
    if (length_squared == 0.0_f64) {
        return false;
    }
    let inverse_length = (1.0_f64 / (length_squared).sqrt());
    let determinant = ((((get_node_world_matrix4(&{
        let __flight_source = &(node.as_ref().unwrap());
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
    })
    .m[0.0_f64 as usize] as f64)
        * (((get_node_world_matrix4(&{
            let __flight_source = &(node.as_ref().unwrap());
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
        })
        .m[5.0_f64 as usize] as f64)
            * (get_node_world_matrix4(&{
                let __flight_source = &(node.as_ref().unwrap());
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
            })
            .m[10.0_f64 as usize] as f64))
            - ((get_node_world_matrix4(&{
                let __flight_source = &(node.as_ref().unwrap());
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
            })
            .m[9.0_f64 as usize] as f64)
                * (get_node_world_matrix4(&{
                    let __flight_source = &(node.as_ref().unwrap());
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
                })
                .m[6.0_f64 as usize] as f64))))
        - ((get_node_world_matrix4(&{
            let __flight_source = &(node.as_ref().unwrap());
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
        })
        .m[4.0_f64 as usize] as f64)
            * (((get_node_world_matrix4(&{
                let __flight_source = &(node.as_ref().unwrap());
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
            })
            .m[1.0_f64 as usize] as f64)
                * (get_node_world_matrix4(&{
                    let __flight_source = &(node.as_ref().unwrap());
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
                })
                .m[10.0_f64 as usize] as f64))
                - ((get_node_world_matrix4(&{
                    let __flight_source = &(node.as_ref().unwrap());
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
                })
                .m[9.0_f64 as usize] as f64)
                    * (get_node_world_matrix4(&{
                        let __flight_source = &(node.as_ref().unwrap());
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
                    })
                    .m[2.0_f64 as usize] as f64)))))
        + ((get_node_world_matrix4(&{
            let __flight_source = &(node.as_ref().unwrap());
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
        })
        .m[8.0_f64 as usize] as f64)
            * (((get_node_world_matrix4(&{
                let __flight_source = &(node.as_ref().unwrap());
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
            })
            .m[1.0_f64 as usize] as f64)
                * (get_node_world_matrix4(&{
                    let __flight_source = &(node.as_ref().unwrap());
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
                })
                .m[6.0_f64 as usize] as f64))
                - ((get_node_world_matrix4(&{
                    let __flight_source = &(node.as_ref().unwrap());
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
                })
                .m[5.0_f64 as usize] as f64)
                    * (get_node_world_matrix4(&{
                        let __flight_source = &(node.as_ref().unwrap());
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
                    })
                    .m[2.0_f64 as usize] as f64)))));
    out.x = (wx * inverse_length);
    out.y = (wy * inverse_length);
    out.z = (wz * inverse_length);
    out.w = (if (tw < 0.0_f64) { (-1.0_f64) } else { 1.0_f64 }
        * if (determinant < 0.0_f64) {
            (-1.0_f64)
        } else {
            1.0_f64
        });
    return true;
}

// Source: upstream/packages/picking/src/sceneHitAttributes.ts:110 (sha256:a2b5c2aa268bed597d43dedf89481c279fdb7673417486baa0c2b550c68609ee)
pub fn is_scene3_d_hit_front_facing(hit: &Scene3DHit, ray: &Ray3D) -> bool {
    return ((((ray.direction.x * hit.normal_x) + (ray.direction.y * hit.normal_y))
        + (ray.direction.z * hit.normal_z))
        <= 0.0_f64);
}

// Source: upstream/packages/picking/src/sceneHitAttributes.ts:114 (sha256:910db783dacd0310a472b876a5ba1d4a3f25f8bbf101e0162ebf2949c67e5099)
fn write_normalized(out: &mut Vector3Like, x: f64, y: f64, z: f64) -> bool {
    let length_squared = (((x * x) + (y * y)) + (z * z));
    if (length_squared == 0.0_f64) {
        return false;
    }
    let inverse_length = (1.0_f64 / (length_squared).sqrt());
    out.x = (x * inverse_length);
    out.y = (y * inverse_length);
    out.z = (z * inverse_length);
    return true;
}

// Source: upstream/packages/picking/src/sceneHitAttributes.ts:124 (sha256:2e923ce8abf535735f38c48690cb07b6b2a9019d4ad7092f0ef4a5b8c3308d0a)
static _INVERSE_WORLD: std::sync::LazyLock<std::sync::Mutex<Matrix4>> =
    std::sync::LazyLock::new(|| {
        std::sync::Mutex::new(create_matrix4(
            None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            None, None,
        ))
    });

// Source: upstream/packages/picking/src/sceneHitAttributes.ts:125 (sha256:ba37508919c17fa2a3d49530a9df3bb58644109a6c5a428650f463c86f552e06)
static _NORMAL0: std::sync::LazyLock<std::sync::Mutex<Vector3>> =
    std::sync::LazyLock::new(|| std::sync::Mutex::new(create_vector3(None, None, None)));

// Source: upstream/packages/picking/src/sceneHitAttributes.ts:126 (sha256:e2c6c12780bb5a7dfc6859c6ed4b40e2e5dbd26701e2e1561aed5d302891d076)
static _NORMAL1: std::sync::LazyLock<std::sync::Mutex<Vector3>> =
    std::sync::LazyLock::new(|| std::sync::Mutex::new(create_vector3(None, None, None)));

// Source: upstream/packages/picking/src/sceneHitAttributes.ts:127 (sha256:36a85bba7a16842b8d9c9383e358ce0287a98a98610e63dc9381b8ca245bfb27)
static _NORMAL2: std::sync::LazyLock<std::sync::Mutex<Vector3>> =
    std::sync::LazyLock::new(|| std::sync::Mutex::new(create_vector3(None, None, None)));

// Source: upstream/packages/picking/src/sceneHitAttributes.ts:128 (sha256:69a65837c41d1b7c9aac78a410a744122e043af30ad5733a2f716df05ffc637d)
static _TANGENT0: std::sync::LazyLock<std::sync::Mutex<Vector4>> = std::sync::LazyLock::new(|| {
    std::sync::Mutex::new(Vector4 {
        __flight_identity: std::sync::Arc::new(()),
        __flight_entity_snapshot: Default::default(),
        __flight_entity_runtime: Default::default(),
        w: 0.0_f64,
        x: 0.0_f64,
        y: 0.0_f64,
        z: 0.0_f64,
    })
});

// Source: upstream/packages/picking/src/sceneHitAttributes.ts:129 (sha256:001ae638415a1c878b77ea7e69ed8dd709c155e0b0767c7298c4c48d8913d379)
static _TANGENT1: std::sync::LazyLock<std::sync::Mutex<Vector4>> = std::sync::LazyLock::new(|| {
    std::sync::Mutex::new(Vector4 {
        __flight_identity: std::sync::Arc::new(()),
        __flight_entity_snapshot: Default::default(),
        __flight_entity_runtime: Default::default(),
        w: 0.0_f64,
        x: 0.0_f64,
        y: 0.0_f64,
        z: 0.0_f64,
    })
});

// Source: upstream/packages/picking/src/sceneHitAttributes.ts:130 (sha256:095238a717116d0b3f07ae171062cd69b012e3a721768176a8e2017041f31d6f)
static _TANGENT2: std::sync::LazyLock<std::sync::Mutex<Vector4>> = std::sync::LazyLock::new(|| {
    std::sync::Mutex::new(Vector4 {
        __flight_identity: std::sync::Arc::new(()),
        __flight_entity_snapshot: Default::default(),
        __flight_entity_runtime: Default::default(),
        w: 0.0_f64,
        x: 0.0_f64,
        y: 0.0_f64,
        z: 0.0_f64,
    })
});

// Source: upstream/packages/picking/src/sceneHitAttributes.ts:131 (sha256:df5ac1fdfc2c2642879e09f3014359524426eceaccf1b1b97ec616a2184c0b35)
static _TRIANGLE: std::sync::LazyLock<std::sync::Mutex<MeshTriangleVertexIndices>> =
    std::sync::LazyLock::new(|| {
        std::sync::Mutex::new(MeshTriangleVertexIndices {
            __flight_identity: std::sync::Arc::new(()),
            i0: 0.0_f64,
            i1: 0.0_f64,
            i2: 0.0_f64,
        })
    });

// Source: upstream/packages/picking/src/sceneHitAttributes.ts:132 (sha256:f6ca65debfd19b07c1ea6d60315effa384b9fc59d501c70841fb588d01bdd5ad)
static _UV0: std::sync::LazyLock<std::sync::Mutex<Vector2>> = std::sync::LazyLock::new(|| {
    std::sync::Mutex::new(Vector2 {
        __flight_identity: std::sync::Arc::new(()),
        __flight_entity_snapshot: Default::default(),
        __flight_entity_runtime: Default::default(),
        x: 0.0_f64,
        y: 0.0_f64,
    })
});

// Source: upstream/packages/picking/src/sceneHitAttributes.ts:133 (sha256:f9c52c2de3011656a02a09b756858912bb2c7c08f90c033653f7236a9645f9d3)
static _UV1: std::sync::LazyLock<std::sync::Mutex<Vector2>> = std::sync::LazyLock::new(|| {
    std::sync::Mutex::new(Vector2 {
        __flight_identity: std::sync::Arc::new(()),
        __flight_entity_snapshot: Default::default(),
        __flight_entity_runtime: Default::default(),
        x: 0.0_f64,
        y: 0.0_f64,
    })
});

// Source: upstream/packages/picking/src/sceneHitAttributes.ts:134 (sha256:113dba9230c4eea18bf202357bd64a385359bc090b7b30cccf9063bc1843dbdf)
static _UV2: std::sync::LazyLock<std::sync::Mutex<Vector2>> = std::sync::LazyLock::new(|| {
    std::sync::Mutex::new(Vector2 {
        __flight_identity: std::sync::Arc::new(()),
        __flight_entity_snapshot: Default::default(),
        __flight_entity_runtime: Default::default(),
        x: 0.0_f64,
        y: 0.0_f64,
    })
});

// Source: upstream/packages/picking/src/sceneHitAttributes.ts:135 (sha256:2838df34807f2d0106e1148a78e37fcbd8cae5b4ee55e291faad177ac9bac9c2)
static _WORLD_NORMAL: std::sync::LazyLock<std::sync::Mutex<Vector3>> =
    std::sync::LazyLock::new(|| std::sync::Mutex::new(create_vector3(None, None, None)));
