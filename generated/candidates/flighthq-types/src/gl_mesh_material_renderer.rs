// @generated from upstream/packages/types/src/GlMeshMaterialRenderer.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{Camera, GlRenderState, Material, MeshGeometry, SceneLightBlock, SceneRenderProxy};

// Source: upstream/packages/types/src/GlMeshMaterialRenderer.ts:25 (sha256:601984d260e89715b21fd3d60676a63ef39a7e8937c30a9d37787f3db08e2c71)
#[derive(Clone)]
pub struct GlMeshMaterialRenderer {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub bind: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(GlRenderState, Option<Material>, SceneLightBlock, Camera) -> ()
                    + Send
                    + 'static,
            >,
        >,
    >,
    pub draw: std::sync::Arc<
        std::sync::Mutex<
            Box<dyn FnMut(GlRenderState, SceneRenderProxy, MeshGeometry) -> () + Send + 'static>,
        >,
    >,
}
impl PartialEq for GlMeshMaterialRenderer {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
