// @generated from upstream/packages/types/src/HasTransform3D.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{Kind, Matrix4, NodeData, Quaternion, Vector3};

// Source: upstream/packages/types/src/HasTransform3D.ts:12 (sha256:ac6499b869984707d016242a71546b88681eeb1961c741ac5cae8a0e70f63a28)
#[derive(Clone)]
pub struct HasTransform3D {
    pub position: Vector3,
    pub rotation: Quaternion,
    pub scale: Vector3,
}

// Source: upstream/packages/types/src/HasTransform3D.ts:18 (sha256:1d64b827bad156d3d07cd1ca90f99aecb21a157026d06c1920d015992b2d3e2e)
#[derive(Clone)]
pub struct HasTransform3DRuntime {
    pub binding: Option<crate::OpaqueHostValue>,
    pub local_matrix4: Option<Matrix4>,
    pub local_matrix4_detached: bool,
    pub world_matrix4: Option<Matrix4>,
}

// Source: upstream/packages/types/src/HasTransform3D.ts:28 (sha256:da9298db6d25709679d19c3277a12efc13fb2ea8c4793a128cb1288dba978dcf)
#[derive(Clone)]
pub struct Transform3DNode {
    pub data: Option<NodeData>,
    pub enabled: bool,
    pub kind: Kind,
    pub name: Option<String>,
    pub position: Vector3,
    pub rotation: Quaternion,
    pub scale: Vector3,
}
