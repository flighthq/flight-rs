// @generated from upstream/packages/spatial/src/spatialIndex.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::create_uniform_grid_spatial_backend;
use flighthq_types::{
    SpatialAabb, SpatialIndex, SpatialIndexBackend, SpatialIndexRuntime, SpatialObjectId,
    SpatialPair,
};

// Source: upstream/packages/spatial/src/spatialIndex.ts:6 (sha256:b569c666dbed07e990633e21bd3707863b4f738242d391ffbc670850df32b361)
pub fn clear_spatial_index(index: &SpatialIndex) -> () {
    {
        let __flight_callback = (index.runtime.backend.clear_spatial_index).clone();
        let __flight_result = __flight_callback.lock().unwrap()();
        __flight_result
    };
}

// Source: upstream/packages/spatial/src/spatialIndex.ts:14 (sha256:a9ee82a8cbfcee7b4dbbd1207b576aee29868228eebb6134e636f7913abdf751)
pub fn create_spatial_index(backend: Option<SpatialIndexBackend>) -> SpatialIndex {
    return SpatialIndex {
        __flight_identity: std::sync::Arc::new(()),
        runtime: SpatialIndexRuntime {
            __flight_identity: std::sync::Arc::new(()),
            backend: (backend).unwrap_or(create_uniform_grid_spatial_backend(
                DEFAULT_SPATIAL_CELL_SIZE,
            )),
        },
    };
}

// Source: upstream/packages/spatial/src/spatialIndex.ts:24 (sha256:eca14adc799a2882f5c25ccb7678c29efd1eb7e01eb382ffe276b8608cb3e65a)
pub fn insert_spatial_object(
    index: &SpatialIndex,
    id: SpatialObjectId,
    bounds: &SpatialAabb,
) -> () {
    {
        let __flight_callback = (index.runtime.backend.insert_spatial_object).clone();
        let __flight_result = __flight_callback.lock().unwrap()(id, (*bounds).clone());
        __flight_result
    };
}

// Source: upstream/packages/spatial/src/spatialIndex.ts:35 (sha256:99a501b2c067988b97eed0c5f3b7515e2856d722058b7204981978c941e02d97)
pub fn query_spatial_pairs(index: &SpatialIndex, out: &Vec<SpatialPair>) -> () {
    {
        let __flight_callback = (index.runtime.backend.query_spatial_pairs).clone();
        let __flight_result = __flight_callback.lock().unwrap()((*out).clone());
        __flight_result
    };
}

// Source: upstream/packages/spatial/src/spatialIndex.ts:40 (sha256:e983b3dfc837242c7f4072594750c739bca55a8872ce75d7bb1396be9dbc0e14)
pub fn query_spatial_point(index: &SpatialIndex, x: f64, y: f64, out: &Vec<SpatialObjectId>) -> () {
    {
        let __flight_callback = (index.runtime.backend.query_spatial_point).clone();
        let __flight_result = __flight_callback.lock().unwrap()(x, y, (*out).clone());
        __flight_result
    };
}

// Source: upstream/packages/spatial/src/spatialIndex.ts:46 (sha256:07383c2e2f03c104f3cfe8e5981c57c7fcf342026cef8958dbf77c6f4784e453)
pub fn query_spatial_ray(
    index: &SpatialIndex,
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

// Source: upstream/packages/spatial/src/spatialIndex.ts:58 (sha256:9a77bb0e2dabc2ecd53476534359d877aef1d5ed8e10acb2dc4c3db4257a7163)
pub fn query_spatial_region(
    index: &SpatialIndex,
    region: &SpatialAabb,
    out: &Vec<SpatialObjectId>,
) -> () {
    {
        let __flight_callback = (index.runtime.backend.query_spatial_region).clone();
        let __flight_result = __flight_callback.lock().unwrap()((*region).clone(), (*out).clone());
        __flight_result
    };
}

// Source: upstream/packages/spatial/src/spatialIndex.ts:67 (sha256:d4214d0512a47335e7709689b49acc362f44f025152aafe1e0ff598f1ec48dda)
pub fn remove_spatial_object(index: &SpatialIndex, id: SpatialObjectId) -> () {
    {
        let __flight_callback = (index.runtime.backend.remove_spatial_object).clone();
        let __flight_result = __flight_callback.lock().unwrap()(id);
        __flight_result
    };
}

// Source: upstream/packages/spatial/src/spatialIndex.ts:72 (sha256:1dcca48b14dce1a4ea68fa3f725bbbe49cfa12899aa9b6f41845e7e616809535)
pub fn update_spatial_object(
    index: &SpatialIndex,
    id: SpatialObjectId,
    bounds: &SpatialAabb,
) -> () {
    {
        let __flight_callback = (index.runtime.backend.update_spatial_object).clone();
        let __flight_result = __flight_callback.lock().unwrap()(id, (*bounds).clone());
        __flight_result
    };
}

// Source: upstream/packages/spatial/src/spatialIndex.ts:82 (sha256:4a01517b03a38bc08ab05f2faa8ae5076d48f2ed3d5ecc5a4bc0ae64e14d93bd)
const DEFAULT_SPATIAL_CELL_SIZE: f64 = 128.0_f64;
