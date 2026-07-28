// @generated from upstream/packages/path/src/pathMeshPool.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{tessellate_path, tessellate_path_typed};
use flighthq_types::{Path, PathMesh, PathMeshTyped};

// Source: upstream/packages/path/src/pathMeshPool.ts:14 (sha256:0fb4dd1ad7fe1954e0576d1a1e27ba67d60f8aefb082e56d5b8b94436e9fb22a)
pub fn acquire_path_mesh(path: &Path, tolerance: Option<f64>) -> PathMesh {
    let tolerance = tolerance.unwrap_or(0.25_f64);
    let mut mesh = if ((PATH_MESH_POOL.lock().unwrap().len() as f64) > 0.0_f64) {
        PATH_MESH_POOL.lock().unwrap().pop()
    } else {
        Some(PathMesh {
            __flight_identity: std::sync::Arc::new(()),
            vertices: vec![],
            indices: vec![],
        })
    };
    mesh.as_mut().unwrap().vertices.clear();
    mesh.as_mut().unwrap().indices.clear();
    let fresh = tessellate_path(path, Some(tolerance));
    {
        let mut i = 0.0_f64;
        while (i < (fresh.vertices.len() as f64)) {
            {
                let __flight_index = (i) as usize;
                let __flight_value = fresh.vertices[i as usize].clone();
                if __flight_index == mesh.as_mut().unwrap().vertices.len() {
                    mesh.as_mut().unwrap().vertices.push(__flight_value);
                } else {
                    mesh.as_mut().unwrap().vertices[__flight_index] = __flight_value;
                }
            };
            {
                i += 1.0;
                i
            };
        }
    }
    mesh.as_mut()
        .unwrap()
        .vertices
        .truncate((fresh.vertices.len() as f64) as usize);
    {
        let mut i = 0.0_f64;
        while (i < (fresh.indices.len() as f64)) {
            {
                let __flight_index = (i) as usize;
                let __flight_value = fresh.indices[i as usize].clone();
                if __flight_index == mesh.as_mut().unwrap().indices.len() {
                    mesh.as_mut().unwrap().indices.push(__flight_value);
                } else {
                    mesh.as_mut().unwrap().indices[__flight_index] = __flight_value;
                }
            };
            {
                i += 1.0;
                i
            };
        }
    }
    mesh.as_mut()
        .unwrap()
        .indices
        .truncate((fresh.indices.len() as f64) as usize);
    return (mesh).clone().unwrap();
}

// Source: upstream/packages/path/src/pathMeshPool.ts:31 (sha256:c9d3adad472e3b886825034c091dbd5d4a8bf6eff14d29a650a10bec9e08c24c)
pub fn acquire_path_mesh_typed(path: &Path, tolerance: Option<f64>) -> PathMeshTyped {
    let tolerance = tolerance.unwrap_or(0.25_f64);
    let fresh = tessellate_path_typed(path, Some(tolerance));
    let mut mesh = if ((TYPED_POOL.lock().unwrap().len() as f64) > 0.0_f64) {
        TYPED_POOL.lock().unwrap().pop()
    } else {
        Some(PathMeshTyped {
            __flight_identity: std::sync::Arc::new(()),
            vertices: vec![0.0_f32; (0.0_f64) as usize],
            indices: vec![0_u32; (0.0_f64) as usize],
        })
    };
    mesh.as_mut().unwrap().vertices = (fresh.vertices).clone();
    mesh.as_mut().unwrap().indices = (fresh.indices).clone();
    return (mesh).clone().unwrap();
}

// Source: upstream/packages/path/src/pathMeshPool.ts:44 (sha256:f5c2f0eff2d794b29210067122e292cffcb0cf475fc59259e276351f2c4f58dc)
pub fn release_path_mesh(mesh: &PathMesh) -> () {
    if ((PATH_MESH_POOL.lock().unwrap().len() as f64) < POOL_HIGH_WATER) {
        PATH_MESH_POOL
            .lock()
            .unwrap()
            .push(((*mesh).clone()).clone());
    }
}

// Source: upstream/packages/path/src/pathMeshPool.ts:51 (sha256:9582d078bf7442c6a43cbeffe18caeb00a564ad90a4e1eb434de32a0591c94e8)
pub fn release_path_mesh_typed(mesh: &PathMeshTyped) -> () {
    if ((TYPED_POOL.lock().unwrap().len() as f64) < POOL_HIGH_WATER) {
        TYPED_POOL.lock().unwrap().push(((*mesh).clone()).clone());
    }
}

// Source: upstream/packages/path/src/pathMeshPool.ts:60 (sha256:f3affaa528530f57896fdfa5317adbd8035d950a7c640fb3cfa0d626e1f5e3d4)
const POOL_HIGH_WATER: f64 = 64.0_f64;

// Source: upstream/packages/path/src/pathMeshPool.ts:61 (sha256:becbf9680ed4c0e1b1957625587bfcef0705d86d94110b5d26b32cbe1ea5f0de)
static PATH_MESH_POOL: std::sync::LazyLock<std::sync::Mutex<Vec<PathMesh>>> =
    std::sync::LazyLock::new(|| std::sync::Mutex::new(vec![]));

// Source: upstream/packages/path/src/pathMeshPool.ts:62 (sha256:b3dd8d553b0b51fe4534e2f63b821c051fc6569226ebcef113873eb86d1c3ce3)
static TYPED_POOL: std::sync::LazyLock<std::sync::Mutex<Vec<PathMeshTyped>>> =
    std::sync::LazyLock::new(|| std::sync::Mutex::new(vec![]));
