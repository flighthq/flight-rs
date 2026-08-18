// @generated from upstream/packages/types/src/Scene3DDocument.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{
    AnimationTrack, ImageResourceReference, Kind, Light, MaterialLike, Matrix4Like, MeshGeometry,
    MeshMorph, Projection, Scene3DAnimationPath, Scene3DMetadata, Transform3D,
};

// Source: upstream/packages/types/src/Scene3DDocument.ts:39 (sha256:8917d122db3e102ae4d684a953b0aace8b57597d4e4b6b10c66a3af8f3b19094)
#[derive(Clone, Default)]
pub struct Scene3DDocument {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub animations: Vec<Scene3DDocumentAnimation>,
    pub cameras: Vec<Scene3DDocumentCamera>,
    pub lights: Vec<Scene3DDocumentLight>,
    pub materials: Vec<MaterialLike>,
    pub meshes: Vec<Scene3DDocumentMesh>,
    pub metadata: Option<Scene3DMetadata>,
    pub nodes: Vec<Scene3DDocumentNode>,
    pub resources: Vec<ImageResourceReference>,
    pub scenes: Vec<Scene3DDocumentScene>,
    pub skins: Vec<Scene3DDocumentSkin>,
}
impl PartialEq for Scene3DDocument {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Scene3DDocument.ts:58 (sha256:a8ef4a7ea531050cec610877be552a931f72e175df17f057f47368157bf38370)
#[derive(Clone, Default)]
pub struct Scene3DDocumentAnimation {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub channels: Vec<Scene3DDocumentAnimationChannel>,
    pub duration: f64,
    pub name: Option<String>,
}
impl PartialEq for Scene3DDocumentAnimation {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Scene3DDocument.ts:67 (sha256:258c79e2d19618e8e7ef730e623d6f1e465e1aa1d937bb12084895e04348fbae)
#[derive(Clone, Default)]
pub struct Scene3DDocumentAnimationChannel {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub node: f64,
    pub path: Scene3DAnimationPath,
    pub track: AnimationTrack,
}
impl PartialEq for Scene3DDocumentAnimationChannel {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Scene3DDocument.ts:77 (sha256:4d3bf49c6cce7ec39a272d01ceacaf6ce3f7bc1d2979c8cb26cc49e085d76d02)
#[derive(Clone)]
pub struct Scene3DDocumentCamera {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub far: f64,
    pub name: Option<String>,
    pub near: f64,
    pub node: Option<f64>,
    pub projection: Projection,
    pub transform: Transform3D,
}
impl PartialEq for Scene3DDocumentCamera {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Scene3DDocument.ts:102 (sha256:4fd9e5cd40feb8c2bd878f06b8d4d8e90398b0b1560e85161c7f6f94b934bd04)
#[derive(Clone, Default)]
pub struct Scene3DDocumentLight {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub descriptor: Light,
    pub name: Option<String>,
    pub node: Option<f64>,
    pub transform: Transform3D,
}
impl PartialEq for Scene3DDocumentLight {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Scene3DDocument.ts:113 (sha256:c49c9ca13d552a40f3f674729c178d641b948943835cef7d9f5125e8bada05dd)
#[derive(Clone, Default)]
pub struct Scene3DDocumentMesh {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub geometry: MeshGeometry,
    pub materials: Vec<f64>,
    pub morph: Option<MeshMorph>,
    pub name: Option<String>,
    pub skin: Option<f64>,
}
impl PartialEq for Scene3DDocumentMesh {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Scene3DDocument.ts:125 (sha256:9ed61dc079468b2826972b414de55e5725087be3219b8eea7e4ddf7716ade10c)
#[derive(Clone, Default)]
pub struct Scene3DDocumentNode {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub children: Vec<f64>,
    pub kind: Kind,
    pub mesh: Option<f64>,
    pub name: Option<String>,
    pub transform: Transform3D,
}
impl PartialEq for Scene3DDocumentNode {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Scene3DDocument.ts:135 (sha256:6ce66003f1c3681606bdd962b865f63465e80ac078292ae9dba05deeb7ce1be5)
#[derive(Clone, Default)]
pub struct Scene3DDocumentScene {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub name: Option<String>,
    pub root_nodes: Vec<f64>,
}
impl PartialEq for Scene3DDocumentScene {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Scene3DDocument.ts:143 (sha256:99d5db456051b144f5a7546cb5310d3973a539fa028b6c83f345e820de966b20)
#[derive(Clone, Default)]
pub struct Scene3DDocumentSkin {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub inverse_bind: Vec<Matrix4Like>,
    pub joints: Vec<f64>,
}
impl PartialEq for Scene3DDocumentSkin {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
