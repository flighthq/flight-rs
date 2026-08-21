// @generated from upstream/packages/spatial/src/spatialIndex.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::create_uniform_grid_spatial_backend2_d;
use flighthq_types::{
    SpatialAabb2D, SpatialIndex2D, SpatialIndexBackend2D, SpatialIndexRuntime2D, SpatialObjectId,
    SpatialPair,
};

// Source: upstream/packages/spatial/src/spatialIndex.ts:12 (sha256:f0386a0e8955833fbb71a924e0495beae9c9fd4d2a7e938114daa40db4afd9ed)
pub fn clear_spatial_index2_d(index: &SpatialIndex2D) -> () {
    {
        let __flight_callback = (index.runtime.backend.clear_spatial_index).clone();
        let __flight_result = __flight_callback.lock().unwrap()();
        __flight_result
    };
}

// Source: upstream/packages/spatial/src/spatialIndex.ts:20 (sha256:e123060713c506ddc4bdc72889098bab5bfb60ce5d105397b5bcb23a89ede9e7)
pub fn create_spatial_index2_d(backend: Option<SpatialIndexBackend2D>) -> SpatialIndex2D {
    return SpatialIndex2D {
        __flight_identity: std::sync::Arc::new(()),
        runtime: SpatialIndexRuntime2D {
            __flight_identity: std::sync::Arc::new(()),
            backend: (backend)
                .clone()
                .unwrap_or(create_uniform_grid_spatial_backend2_d(
                    DEFAULT_SPATIAL_CELL_SIZE,
                )),
        },
    };
}

// Source: upstream/packages/spatial/src/spatialIndex.ts:33 (sha256:c1baaec8fe63f8be7230ff0ef28412d60945cd7b8d222fdf118df72bb2742dec)
pub fn insert_spatial_object2_d(
    index: &SpatialIndex2D,
    id: SpatialObjectId,
    bounds: &SpatialAabb2D,
) -> bool {
    return {
        let __flight_callback = (index.runtime.backend.insert_spatial_object).clone();
        let __flight_result = __flight_callback.lock().unwrap()(id, (*bounds).clone());
        __flight_result
    };
}

// Source: upstream/packages/spatial/src/spatialIndex.ts:44 (sha256:77b4fa4d725b2281fdb0d2ea760d12c0113a0559d64511bf1b2ae91239d50a38)
pub fn query_spatial_pairs2_d(index: &SpatialIndex2D, out: &Vec<SpatialPair>) -> () {
    {
        let __flight_callback = (index.runtime.backend.query_spatial_pairs).clone();
        let __flight_result = __flight_callback.lock().unwrap()((*out).clone());
        __flight_result
    };
}

// Source: upstream/packages/spatial/src/spatialIndex.ts:49 (sha256:a082db65e049f99df130842bb42ad8acfdd3cfaa97da8f1df6cd9a41c0e1eb1d)
pub fn query_spatial_point2_d(
    index: &SpatialIndex2D,
    x: f64,
    y: f64,
    out: &Vec<SpatialObjectId>,
) -> () {
    {
        let __flight_callback = (index.runtime.backend.query_spatial_point).clone();
        let __flight_result = __flight_callback.lock().unwrap()(x, y, (*out).clone());
        __flight_result
    };
}

// Source: upstream/packages/spatial/src/spatialIndex.ts:60 (sha256:b05ceebf74009694225a0a7871af30600d72168bbea451b06425e876b4aa8914)
pub fn query_spatial_ray2_d(
    index: &SpatialIndex2D,
    x: f64,
    y: f64,
    dx: f64,
    dy: f64,
    out: &Vec<SpatialObjectId>,
) -> () {
    {
        let __flight_callback = (index.runtime.backend.query_spatial_ray).clone();
        let __flight_result = __flight_callback.lock().unwrap()(x, y, dx, dy, (*out).clone());
        __flight_result
    };
}

// Source: upstream/packages/spatial/src/spatialIndex.ts:72 (sha256:330cfa96557c2b6fa4257538bd9512b40458ebf2580edeaa7b6d1f8ef6223ef4)
pub fn query_spatial_region2_d(
    index: &SpatialIndex2D,
    region: &SpatialAabb2D,
    out: &Vec<SpatialObjectId>,
) -> () {
    {
        let __flight_callback = (index.runtime.backend.query_spatial_region).clone();
        let __flight_result = __flight_callback.lock().unwrap()((*region).clone(), (*out).clone());
        __flight_result
    };
}

// Source: upstream/packages/spatial/src/spatialIndex.ts:81 (sha256:60c5aac51d73e443cad383421f0912805f0760889f67f77b5604ac0ae4005dd9)
pub fn remove_spatial_object2_d(index: &SpatialIndex2D, id: SpatialObjectId) -> () {
    {
        let __flight_callback = (index.runtime.backend.remove_spatial_object).clone();
        let __flight_result = __flight_callback.lock().unwrap()(id);
        __flight_result
    };
}

// Source: upstream/packages/spatial/src/spatialIndex.ts:89 (sha256:1a893e0b5cd596d56732fbb9173ab14364f804d05811d86ceea9206ccb726ad3)
pub fn update_spatial_object2_d(
    index: &SpatialIndex2D,
    id: SpatialObjectId,
    bounds: &SpatialAabb2D,
) -> bool {
    return {
        let __flight_callback = (index.runtime.backend.update_spatial_object).clone();
        let __flight_result = __flight_callback.lock().unwrap()(id, (*bounds).clone());
        __flight_result
    };
}

// Source: upstream/packages/spatial/src/spatialIndex.ts:99 (sha256:4a01517b03a38bc08ab05f2faa8ae5076d48f2ed3d5ecc5a4bc0ae64e14d93bd)
const DEFAULT_SPATIAL_CELL_SIZE: f64 = 128.0_f64;
