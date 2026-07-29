// @generated from upstream/packages/types/src/WgpuMeshMaterialRenderer.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{Camera, Material, MeshGeometry, SceneLightBlock, SceneRenderProxy, WgpuRenderState};

// Source: upstream/packages/types/src/WgpuMeshMaterialRenderer.ts:25 (sha256:aaa91ed37d2e09b15e0b863d8959ba7237700fa37044cebe68c81ce99182eff3)
#[derive(Clone)]
pub struct WgpuMeshMaterialRenderer {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub bind: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(WgpuRenderState, Option<Material>, SceneLightBlock, Camera) -> ()
                    + Send
                    + 'static,
            >,
        >,
    >,
    pub draw: std::sync::Arc<
        std::sync::Mutex<
            Box<dyn FnMut(WgpuRenderState, SceneRenderProxy, MeshGeometry) -> () + Send + 'static>,
        >,
    >,
}
impl PartialEq for WgpuMeshMaterialRenderer {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
