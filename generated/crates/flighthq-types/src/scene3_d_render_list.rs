// @generated from upstream/packages/types/src/Scene3DRenderList.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{Matrix4, Mesh, Scene3DLightBlock};

// Source: upstream/packages/types/src/Scene3DRenderList.ts:19 (sha256:7e7d78288f957c8b5498a6e851712cd492da91353a70049a43a65b9d5abf86ed)
#[derive(Clone, Default)]
pub struct Scene3DRenderList {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub lights: Scene3DLightBlock,
    pub mesh_count: f64,
    pub view_projection: Matrix4,
    pub visible_meshes: Vec<Mesh>,
}
impl PartialEq for Scene3DRenderList {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
