// @generated from upstream/packages/types/src/GlMeshMaterialRenderer.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{
    Camera3D, GlRenderState, Material, MeshGeometry, Scene3DLightBlock, Scene3DRenderProxy,
};

// Source: upstream/packages/types/src/GlMeshMaterialRenderer.ts:24 (sha256:4315aa771a093d71a942538086627e43448bb3c4e8294c89b4337c8c6a201899)
#[derive(Clone)]
pub struct GlMeshMaterialRenderer {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub bind: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(GlRenderState, Option<Material>, Scene3DLightBlock, Camera3D) -> ()
                    + Send
                    + 'static,
            >,
        >,
    >,
    pub draw: std::sync::Arc<
        std::sync::Mutex<
            Box<dyn FnMut(GlRenderState, Scene3DRenderProxy, MeshGeometry) -> () + Send + 'static>,
        >,
    >,
}
impl PartialEq for GlMeshMaterialRenderer {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
