// @generated from upstream/packages/types/src/WgpuMeshMaterialRenderer.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{
    Camera3D, Material, MeshGeometry, Scene3DLightBlock, Scene3DRenderProxy, WgpuRenderState,
};

// Source: upstream/packages/types/src/WgpuMeshMaterialRenderer.ts:24 (sha256:4bf374dfff501c605c32e2ac9ac275016db035dbf3c676e31faa554243ee19ae)
#[derive(Clone)]
pub struct WgpuMeshMaterialRenderer {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub bind: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(WgpuRenderState, Option<Material>, Scene3DLightBlock, Camera3D) -> ()
                    + Send
                    + 'static,
            >,
        >,
    >,
    pub draw: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(WgpuRenderState, Scene3DRenderProxy, MeshGeometry) -> () + Send + 'static,
            >,
        >,
    >,
}
impl PartialEq for WgpuMeshMaterialRenderer {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
