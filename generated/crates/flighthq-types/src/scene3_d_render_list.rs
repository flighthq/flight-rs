// @generated from upstream/packages/types/src/Scene3DRenderList.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{InstancedMesh, Matrix4, Mesh, Scene3DLightBlock};

// Source: upstream/packages/types/src/Scene3DRenderList.ts:20 (sha256:7876e650c536ec6dff8ebacf154832629bd706eab3a5ec13b303a59dda2d6ae2)
#[derive(Clone, Default)]
pub struct Scene3DRenderList {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub instanced_mesh_count: f64,
    pub lights: Scene3DLightBlock,
    pub mesh_count: f64,
    pub view_projection: Matrix4,
    pub visible_instanced_meshes: Vec<InstancedMesh>,
    pub visible_meshes: Vec<Mesh>,
}
impl PartialEq for Scene3DRenderList {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
