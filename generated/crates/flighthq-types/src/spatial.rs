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

// Source: upstream/packages/types/src/Spatial.ts:47 (sha256:2c97397bb5cc621ecf1dd250a0101d41ac521db48e397569e2ab36b12262d2ae)
#[derive(Clone, Default)]
pub struct SpatialAabb3D {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub min_x: f64,
    pub min_y: f64,
    pub min_z: f64,
    pub max_x: f64,
    pub max_y: f64,
    pub max_z: f64,
}
impl PartialEq for SpatialAabb3D {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Spatial.ts:63 (sha256:ade59b2f2d517cdbda7de60ef266ff2e20d781bbe29a295411ab44ff58c12936)
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

// Source: upstream/packages/types/src/Spatial.ts:81 (sha256:795d29bd71070bd81511837390ede5bf172170bc0166e2397ed4e0733ae00cb4)
#[derive(Clone, Default)]
pub struct SpatialFrustum3D {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub corners: Vec<f64>,
}
impl PartialEq for SpatialFrustum3D {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Spatial.ts:92 (sha256:4fee3cba559983f72268f54eac22894bc2df930207c714003e1e1cd233b3484d)
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

// Source: upstream/packages/types/src/Spatial.ts:125 (sha256:0c819f45914d1f8df3db19def4456e6ae5abd819d7bb9874d35d22464856c0d4)
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

// Source: upstream/packages/types/src/Spatial.ts:133 (sha256:07d5a0d57405fbe180954979475618abc336ad6400ea0d22b38a2c29b6721fde)
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

// Source: upstream/packages/types/src/Spatial.ts:147 (sha256:7aece00d5d3f7fcf77580f9bcf7eb2a6f240a7120c4779c857e10aae2a8c648f)
#[derive(Clone)]
pub struct SpatialIndexBackend3D {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub insert_spatial_object: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut(SpatialObjectId, SpatialAabb3D) -> bool + Send + 'static>>,
    >,
    pub update_spatial_object: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut(SpatialObjectId, SpatialAabb3D) -> bool + Send + 'static>>,
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
            Box<dyn FnMut(SpatialAabb3D, Vec<SpatialObjectId>) -> () + Send + 'static>,
        >,
    >,
    pub query_spatial_point: std::sync::Arc<
        std::sync::Mutex<
            Box<dyn FnMut(f64, f64, f64, Vec<SpatialObjectId>) -> () + Send + 'static>,
        >,
    >,
    pub query_spatial_ray: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(f64, f64, f64, f64, f64, f64, Vec<SpatialObjectId>) -> ()
                    + Send
                    + 'static,
            >,
        >,
    >,
}
impl PartialEq for SpatialIndexBackend3D {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Spatial.ts:178 (sha256:2fa3449878b450408afa887ac708dafca167934caf58dd07611dd6b31a0e85d2)
#[derive(Clone)]
pub struct SpatialIndexRuntime3D {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub backend: SpatialIndexBackend3D,
}
impl PartialEq for SpatialIndexRuntime3D {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Spatial.ts:187 (sha256:00fe0c5e27fb840cc18cc9502bc97102cfe47ce1322fec51d69c5870ed957228)
#[derive(Clone)]
pub struct SpatialIndex3D {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub runtime: SpatialIndexRuntime3D,
}
impl PartialEq for SpatialIndex3D {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
