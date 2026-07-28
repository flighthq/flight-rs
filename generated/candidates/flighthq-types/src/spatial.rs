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
    pub min_x: f64,
    pub min_y: f64,
    pub max_x: f64,
    pub max_y: f64,
}

// Source: upstream/packages/types/src/Spatial.ts:29 (sha256:ade59b2f2d517cdbda7de60ef266ff2e20d781bbe29a295411ab44ff58c12936)
#[derive(Clone)]
pub struct SpatialPair {
    pub a: SpatialObjectId,
    pub b: SpatialObjectId,
}

// Source: upstream/packages/types/src/Spatial.ts:41 (sha256:101e7cb0bb559c74c2ef75198f65dbea449724d03d36caf094594d5aebcb2418)
#[derive(Clone)]
pub struct SpatialIndexBackend {
    pub insert_spatial_object: crate::OpaqueHostValue,
    pub update_spatial_object: crate::OpaqueHostValue,
    pub remove_spatial_object: crate::OpaqueHostValue,
    pub clear_spatial_index: crate::OpaqueHostValue,
    pub query_spatial_pairs: crate::OpaqueHostValue,
    pub query_spatial_region: crate::OpaqueHostValue,
    pub query_spatial_point: crate::OpaqueHostValue,
    pub query_spatial_ray: crate::OpaqueHostValue,
}

// Source: upstream/packages/types/src/Spatial.ts:63 (sha256:1424d271e8a00942690c4056933117a55be6a35256b5034cd85771e9227f8f9b)
#[derive(Clone)]
pub struct SpatialIndexRuntime {
    pub backend: SpatialIndexBackend,
}

// Source: upstream/packages/types/src/Spatial.ts:71 (sha256:026a289ae13bcef28f32a641f01dfb745ed1a04ee4666a494efbd3c25a8c59dd)
#[derive(Clone)]
pub struct SpatialIndex {
    pub runtime: SpatialIndexRuntime,
}
