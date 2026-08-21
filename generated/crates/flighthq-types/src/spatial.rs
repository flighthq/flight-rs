// @generated from upstream/packages/types/src/Spatial.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::SpatialIndexingExplanation;

// Source: upstream/packages/types/src/Spatial.ts:30 (sha256:1960cbd630215366c451daa631d02519c177397e1affa0765e9d83fd4d85a1ab)
pub type SpatialObjectId = f64;

// Source: upstream/packages/types/src/Spatial.ts:36 (sha256:611d1da404b7ff516c17a1f9172563244173b9002f6ed08413d2b65738180fd2)
#[derive(Clone, Default)]
pub struct SpatialAabb2D {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub min_x: f64,
    pub min_y: f64,
    pub max_x: f64,
    pub max_y: f64,
}
impl PartialEq for SpatialAabb2D {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Spatial.ts:46 (sha256:ade59b2f2d517cdbda7de60ef266ff2e20d781bbe29a295411ab44ff58c12936)
#[derive(Clone, Default)]
pub struct SpatialPair {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub a: SpatialObjectId,
    pub b: SpatialObjectId,
}
impl PartialEq for SpatialPair {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Spatial.ts:58 (sha256:4fee3cba559983f72268f54eac22894bc2df930207c714003e1e1cd233b3484d)
#[derive(Clone)]
pub struct SpatialIndexBackend2D {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub insert_spatial_object: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut(SpatialObjectId, SpatialAabb2D) -> bool + Send + 'static>>,
    >,
    pub update_spatial_object: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut(SpatialObjectId, SpatialAabb2D) -> bool + Send + 'static>>,
    >,
    pub remove_spatial_object:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(SpatialObjectId) -> () + Send + 'static>>>,
    pub clear_spatial_index:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>,
    pub explain_spatial_indexing: std::sync::Arc<
        std::sync::Mutex<
            Box<dyn FnMut(SpatialObjectId) -> SpatialIndexingExplanation + Send + 'static>,
        >,
    >,
    pub query_spatial_pairs:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(Vec<SpatialPair>) -> () + Send + 'static>>>,
    pub query_spatial_region: std::sync::Arc<
        std::sync::Mutex<
            Box<dyn FnMut(SpatialAabb2D, Vec<SpatialObjectId>) -> () + Send + 'static>,
        >,
    >,
    pub query_spatial_point: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut(f64, f64, Vec<SpatialObjectId>) -> () + Send + 'static>>,
    >,
    pub query_spatial_ray: std::sync::Arc<
        std::sync::Mutex<
            Box<dyn FnMut(f64, f64, f64, f64, Vec<SpatialObjectId>) -> () + Send + 'static>,
        >,
    >,
}
impl PartialEq for SpatialIndexBackend2D {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Spatial.ts:91 (sha256:0c819f45914d1f8df3db19def4456e6ae5abd819d7bb9874d35d22464856c0d4)
#[derive(Clone)]
pub struct SpatialIndexRuntime2D {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub backend: SpatialIndexBackend2D,
}
impl PartialEq for SpatialIndexRuntime2D {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Spatial.ts:99 (sha256:07d5a0d57405fbe180954979475618abc336ad6400ea0d22b38a2c29b6721fde)
#[derive(Clone)]
pub struct SpatialIndex2D {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub runtime: SpatialIndexRuntime2D,
}
impl PartialEq for SpatialIndex2D {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
