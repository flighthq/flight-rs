// @generated from upstream/packages/types/src/MeshGeometry.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{Aabb, MeshMorphBindPose, MeshSkinBindPose};

// Source: upstream/packages/types/src/MeshGeometry.ts:15 (sha256:8971a6ca5dffa70ea82130390dcbcc1204c899711fa3a0f5049ac287aac074bb)
pub type VertexSemantic = String;

// Source: upstream/packages/types/src/MeshGeometry.ts:18 (sha256:71a8621cdc1f6ded2aea076fdf5408d52b1cdff804e21778a5721c6e21e077a0)
pub type VertexFormat = String;

// Source: upstream/packages/types/src/MeshGeometry.ts:21 (sha256:24ab3d45fe78ea5b6a798db540885bf84b22db88c3fa45d25204becbe523cd20)
pub type PrimitiveTopology = String;

// Source: upstream/packages/types/src/MeshGeometry.ts:26 (sha256:ba5dd9a697a0d03ad2a6bd0554f6db2787b7536229bbdcfc395ac93cc8a6a889)
#[derive(Clone)]
pub struct VertexAttribute {
    pub byte_offset: f64,
    pub format: VertexFormat,
    pub semantic: VertexSemantic,
}

// Source: upstream/packages/types/src/MeshGeometry.ts:35 (sha256:9b14faf8c5337ca465ecc163e62107de498b98fb6e4e0b7c34f3d40514045479)
#[derive(Clone)]
pub struct VertexAttributeLayout {
    pub attributes: Vec<VertexAttribute>,
    pub stride: f64,
}

// Source: upstream/packages/types/src/MeshGeometry.ts:43 (sha256:41d1dc5491ddd2855f1e3421ef84e991bbcd8ef4acd680dfb88ea7ecbc9da8ae)
#[derive(Clone)]
pub struct MeshSubset {
    pub index_count: f64,
    pub index_offset: f64,
}

// Source: upstream/packages/types/src/MeshGeometry.ts:54 (sha256:ba607b6e53ee733bb8d2233cfaf78618ed0be454fbbe45809ec8977365013d83)
#[derive(Clone)]
pub struct MeshGeometry {
    pub bounds: Option<Aabb>,
    pub indices: Option<crate::OpaqueHostValue>,
    pub layout: VertexAttributeLayout,
    pub subsets: Vec<MeshSubset>,
    pub topology: PrimitiveTopology,
    pub version: f64,
    pub vertices: Vec<f32>,
}

// Source: upstream/packages/types/src/MeshGeometry.ts:67 (sha256:fbdba96bdb857f605c2728ff8a5430d0e9c4bd9b38a190dc047bef58e3259f4b)
#[derive(Clone)]
pub struct MeshGeometryGlData {
    pub __mesh_geometry_gl_data: crate::OpaqueHostValue,
}

// Source: upstream/packages/types/src/MeshGeometry.ts:74 (sha256:3b4297889845e25927078cdf90db397d4d03b88db80427dd743cffb2d74a0287)
#[derive(Clone)]
pub struct MeshGeometryWgpuData {
    pub __mesh_geometry_wgpu_data: crate::OpaqueHostValue,
}

// Source: upstream/packages/types/src/MeshGeometry.ts:85 (sha256:973da6177456239f25e9a96ed6ef48c2ed2d80cfcce006894fd4acd3c2cf1dcb)
#[derive(Clone)]
pub struct MeshGeometryRuntime {
    pub binding: Option<crate::OpaqueHostValue>,
    pub morph_bind_pose: Option<MeshMorphBindPose>,
    pub skin_bind_pose: Option<MeshSkinBindPose>,
    pub webgl_data: Option<MeshGeometryGlData>,
    pub webgpu_data: Option<MeshGeometryWgpuData>,
}
