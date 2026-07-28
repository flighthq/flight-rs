// @generated from upstream/packages/scene/src/sceneNodeBounds.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::is_mesh;
use flighthq_geometry::{create_aabb, set_aabb, transform_aabb_by_matrix4, union_aabb};
use flighthq_mesh::compute_mesh_geometry_bounds;
use flighthq_node::{ensure_node_world_matrix4, get_node_runtime, get_node_world_matrix4};
use flighthq_types::{Aabb, AabbLike, SceneNode};

// Source: upstream/packages/scene/src/sceneNodeBounds.ts:17 (sha256:e9ec8a7d672be1190905130363567af6863dab015f5b9797260ea23938340559)
pub fn get_scene_node_world_bounds(out: &mut AabbLike, node: &mut SceneNode) -> () {
    set_aabb(
        out,
        f64::INFINITY,
        f64::INFINITY,
        f64::INFINITY,
        f64::NEG_INFINITY,
        f64::NEG_INFINITY,
        f64::NEG_INFINITY,
    );
    _accumulate_world_bounds(out, node);
}

// Source: upstream/packages/scene/src/sceneNodeBounds.ts:31 (sha256:768fc425f96dd1e17f1de801e0ec10b80fa383c41cdbfeeddd3dec2f6030d02c)
fn _accumulate_world_bounds(out: &mut AabbLike, node: &mut SceneNode) -> () {
    if is_mesh(node) {
        let mut geom = node.geometry;
        let mut local_bounds = geom.bounds;
        if (local_bounds).is_none() {
            compute_mesh_geometry_bounds(&mut (*_SCRATCH_LOCAL_AABB.lock().unwrap()), &geom);
            local_bounds = (*_SCRATCH_LOCAL_AABB.lock().unwrap()).clone();
        }
        if (local_bounds.min.x <= local_bounds.max.x) {
            ensure_node_world_matrix4(node);
            let world_matrix = get_node_world_matrix4(node);
            transform_aabb_by_matrix4(
                &mut (*_SCRATCH_WORLD_AABB.lock().unwrap()),
                &local_bounds,
                &world_matrix,
            );
            {
                let __flight_argument_1 = (out).clone();
                union_aabb(
                    out,
                    &__flight_argument_1,
                    &(*_SCRATCH_WORLD_AABB.lock().unwrap()),
                )
            };
        }
    }
    let mut children = (get_node_runtime(node).children).clone();
    if (children).is_some() {
        {
            let mut i = 0.0_f64;
            while (i < (children.as_mut().unwrap().len() as f64)) {
                _accumulate_world_bounds(out, &mut children.as_mut().unwrap()[i as usize].clone());
                {
                    i += 1.0;
                    i
                };
            }
        }
    }
}

// Source: upstream/packages/scene/src/sceneNodeBounds.ts:56 (sha256:6fe5187a5a345646b6e1b1b86592ae73ec716e3ddd479281737882422c0e848d)
static _SCRATCH_LOCAL_AABB: std::sync::LazyLock<std::sync::Mutex<Aabb>> =
    std::sync::LazyLock::new(|| {
        std::sync::Mutex::new(create_aabb(None, None, None, None, None, None))
    });

// Source: upstream/packages/scene/src/sceneNodeBounds.ts:57 (sha256:f4adb2a3cb6957f412db3d2b35ccb6c2409c20971edfb289edbd9458ad7af15d)
static _SCRATCH_WORLD_AABB: std::sync::LazyLock<std::sync::Mutex<Aabb>> =
    std::sync::LazyLock::new(|| {
        std::sync::Mutex::new(create_aabb(None, None, None, None, None, None))
    });
