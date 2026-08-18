// @generated from upstream/packages/types/src/MeshGeometry.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{Aabb, EntityRuntime};

// Source: upstream/packages/types/src/MeshGeometry.ts:16 (sha256:8971a6ca5dffa70ea82130390dcbcc1204c899711fa3a0f5049ac287aac074bb)
pub type VertexSemantic = String;

// Source: upstream/packages/types/src/MeshGeometry.ts:19 (sha256:71a8621cdc1f6ded2aea076fdf5408d52b1cdff804e21778a5721c6e21e077a0)
pub type VertexFormat = String;

// Source: upstream/packages/types/src/MeshGeometry.ts:22 (sha256:24ab3d45fe78ea5b6a798db540885bf84b22db88c3fa45d25204becbe523cd20)
pub type PrimitiveTopology = String;

// Source: upstream/packages/types/src/MeshGeometry.ts:27 (sha256:ba5dd9a697a0d03ad2a6bd0554f6db2787b7536229bbdcfc395ac93cc8a6a889)
#[derive(Clone, Default)]
pub struct VertexAttribute {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub byte_offset: f64,
    pub format: VertexFormat,
    pub semantic: VertexSemantic,
}
impl PartialEq for VertexAttribute {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/MeshGeometry.ts:36 (sha256:9b14faf8c5337ca465ecc163e62107de498b98fb6e4e0b7c34f3d40514045479)
#[derive(Clone, Default)]
pub struct VertexAttributeLayout {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub attributes: Vec<VertexAttribute>,
    pub stride: f64,
}
impl PartialEq for VertexAttributeLayout {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/MeshGeometry.ts:44 (sha256:41d1dc5491ddd2855f1e3421ef84e991bbcd8ef4acd680dfb88ea7ecbc9da8ae)
#[derive(Clone, Default)]
pub struct MeshSubset {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub index_count: f64,
    pub index_offset: f64,
}
impl PartialEq for MeshSubset {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/MeshGeometry.ts:55 (sha256:ba607b6e53ee733bb8d2233cfaf78618ed0be454fbbe45809ec8977365013d83)
#[derive(Clone, Default)]
pub struct MeshGeometry {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    pub bounds: Option<Aabb>,
    pub indices: Option<Vec<u32>>,
    pub layout: VertexAttributeLayout,
    pub subsets: Vec<MeshSubset>,
    pub topology: PrimitiveTopology,
    pub version: f64,
    pub vertices: Vec<f32>,
}
impl PartialEq for MeshGeometry {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for MeshGeometry {
    fn __flight_entity_runtime(
        &self,
    ) -> &std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>> {
        &self.__flight_entity_runtime
    }
    fn __flight_fresh_clone(&self) -> Self {
        let mut cloned = self.clone();
        cloned.__flight_identity = std::sync::Arc::new(());
        cloned.__flight_entity_runtime = std::sync::Arc::new(std::sync::Mutex::new(
            self.__flight_entity_runtime.lock().unwrap().clone(),
        ));
        cloned
    }
}

// Source: upstream/packages/types/src/MeshGeometry.ts:68 (sha256:fbdba96bdb857f605c2728ff8a5430d0e9c4bd9b38a190dc047bef58e3259f4b)
#[derive(Clone)]
pub struct MeshGeometryGlData {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub __mesh_geometry_gl_data: crate::FlightSymbol,
}
impl PartialEq for MeshGeometryGlData {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/MeshGeometry.ts:75 (sha256:3b4297889845e25927078cdf90db397d4d03b88db80427dd743cffb2d74a0287)
#[derive(Clone)]
pub struct MeshGeometryWgpuData {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub __mesh_geometry_wgpu_data: crate::FlightSymbol,
}
impl PartialEq for MeshGeometryWgpuData {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/MeshGeometry.ts:98 (sha256:09771ab81a3d8b9386f482e7a9b0d8e8bb9af7a1576510bd07568070f8cde3bd)
pub type MeshGeometryRuntime = crate::EntityRuntime;
