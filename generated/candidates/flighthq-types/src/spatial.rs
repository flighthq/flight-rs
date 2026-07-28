// @generated from upstream/packages/types/src/Spatial.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/Spatial.ts:13 (sha256:1960cbd630215366c451daa631d02519c177397e1affa0765e9d83fd4d85a1ab)
pub type SpatialObjectId = f64;

// Source: upstream/packages/types/src/Spatial.ts:19 (sha256:b6d4e18dccb6b633e55a76a547626386e1a252b89d1832b0ec714542a680deab)
#[derive(Clone)]
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

// Source: upstream/packages/types/src/Spatial.ts:29 (sha256:ade59b2f2d517cdbda7de60ef266ff2e20d781bbe29a295411ab44ff58c12936)
#[derive(Clone)]
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

// Source: upstream/packages/types/src/Spatial.ts:41 (sha256:101e7cb0bb559c74c2ef75198f65dbea449724d03d36caf094594d5aebcb2418)
#[derive(Clone)]
pub struct SpatialIndexBackend {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub insert_spatial_object: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut(SpatialObjectId, SpatialAabb) -> () + Send + 'static>>,
    >,
    pub update_spatial_object: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut(SpatialObjectId, SpatialAabb) -> () + Send + 'static>>,
    >,
    pub remove_spatial_object:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(SpatialObjectId) -> () + Send + 'static>>>,
    pub clear_spatial_index:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>,
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

// Source: upstream/packages/types/src/Spatial.ts:63 (sha256:1424d271e8a00942690c4056933117a55be6a35256b5034cd85771e9227f8f9b)
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

// Source: upstream/packages/types/src/Spatial.ts:71 (sha256:026a289ae13bcef28f32a641f01dfb745ed1a04ee4666a494efbd3c25a8c59dd)
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
