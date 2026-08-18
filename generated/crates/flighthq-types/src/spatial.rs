// @generated from upstream/packages/types/src/Spatial.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::SpatialIndexingExplanation;

// Source: upstream/packages/types/src/Spatial.ts:15 (sha256:1960cbd630215366c451daa631d02519c177397e1affa0765e9d83fd4d85a1ab)
pub type SpatialObjectId = f64;

// Source: upstream/packages/types/src/Spatial.ts:21 (sha256:b6d4e18dccb6b633e55a76a547626386e1a252b89d1832b0ec714542a680deab)
#[derive(Clone, Default)]
pub struct SpatialAabb {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub min_x: f64,
    pub min_y: f64,
    pub max_x: f64,
    pub max_y: f64,
}
impl PartialEq for SpatialAabb {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Spatial.ts:31 (sha256:ade59b2f2d517cdbda7de60ef266ff2e20d781bbe29a295411ab44ff58c12936)
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

// Source: upstream/packages/types/src/Spatial.ts:43 (sha256:02933375483257bcbd8993de12e3259f62189014fb02d16cac0367f7654a01d2)
#[derive(Clone)]
pub struct SpatialIndexBackend {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub insert_spatial_object: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut(SpatialObjectId, SpatialAabb) -> bool + Send + 'static>>,
    >,
    pub update_spatial_object: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut(SpatialObjectId, SpatialAabb) -> bool + Send + 'static>>,
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
        std::sync::Mutex<Box<dyn FnMut(SpatialAabb, Vec<SpatialObjectId>) -> () + Send + 'static>>,
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
impl PartialEq for SpatialIndexBackend {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Spatial.ts:76 (sha256:1424d271e8a00942690c4056933117a55be6a35256b5034cd85771e9227f8f9b)
#[derive(Clone)]
pub struct SpatialIndexRuntime {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub backend: SpatialIndexBackend,
}
impl PartialEq for SpatialIndexRuntime {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Spatial.ts:84 (sha256:026a289ae13bcef28f32a641f01dfb745ed1a04ee4666a494efbd3c25a8c59dd)
#[derive(Clone)]
pub struct SpatialIndex {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub runtime: SpatialIndexRuntime,
}
impl PartialEq for SpatialIndex {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
