// @generated from upstream/packages/types/src/SceneRenderList.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{Matrix4, Mesh, SceneLightBlock};

// Source: upstream/packages/types/src/SceneRenderList.ts:19 (sha256:e11b968da873cd528cfaa43d46b5ab9bef8b469ce02eb2d7ce0a7e02127f15e2)
#[derive(Clone)]
pub struct SceneRenderList {
    pub lights: SceneLightBlock,
    pub mesh_count: f64,
    pub view_projection: Matrix4,
    pub visible_meshes: Vec<Mesh>,
}
