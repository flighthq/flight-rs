// @generated from upstream/packages/types/src/Scene3D.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{AnimationClip, EntityRuntime, ImageResourceReference, Node3D, Scene3DMetadata};

// Source: upstream/packages/types/src/Scene3D.ts:19 (sha256:8e8dc8c875ecaca8382bb48bd3c72eba7174629efc6829a70b5ac92772d1c0db)
#[derive(Clone, Default)]
pub struct Scene3D {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    pub animations: Vec<(String, AnimationClip)>,
    pub metadata: Option<Scene3DMetadata>,
    pub resources: Vec<ImageResourceReference>,
    pub root: Node3D,
}
impl PartialEq for Scene3D {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for Scene3D {
    fn __flight_entity_runtime(
        &self,
    ) -> &std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>> {
        &self.__flight_entity_runtime
    }
    fn __flight_fresh_clone(&self) -> Self {
        let mut cloned = self.clone();
        cloned.__flight_identity = std::sync::Arc::new(());
        cloned.__flight_entity_runtime = std::sync::Arc::new(std::sync::Mutex::new(
            self.__flight_entity_runtime.lock().unwrap().clone(),
        ));
        cloned
    }
}

// Source: upstream/packages/types/src/Scene3D.ts:26 (sha256:0c45f59ae863e0965aa0abd684cb508545232b0ed2e31741a4bf6e4c8eeb8719)
pub type Scene3DRuntime = crate::EntityRuntime;
