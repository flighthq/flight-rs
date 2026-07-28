// @generated from upstream/packages/types/src/Skeleton3D.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::SceneNode;

// Source: upstream/packages/types/src/Skeleton3D.ts:12 (sha256:1b17a0f01745f680f0a10d3d66e836133f3b89fc168d920c7604107f106948ae)
#[derive(Clone)]
pub struct Skeleton3D {
    pub inverse_bind_matrices: Vec<f32>,
    pub joint_matrices: Vec<f32>,
    pub joints: Vec<SceneNode>,
    pub names: Option<Option<Vec<String>>>,
}
