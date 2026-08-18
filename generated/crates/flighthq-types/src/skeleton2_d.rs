// @generated from upstream/packages/types/src/Skeleton2D.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{AttachmentSkin2D, Bone2D, EntityRuntime, Slot2D};

// Source: upstream/packages/types/src/Skeleton2D.ts:26 (sha256:4c5c7df2276c0ba36c720adc9c2f10a21508a54448a1f67dcb2587581d3ca5c2)
#[derive(Clone, Default)]
pub struct Skeleton2D {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    pub bone_matrices: Vec<f32>,
    pub bones: Vec<Bone2D>,
    pub inverse_bind_matrices: Vec<f32>,
    pub skins: Option<Vec<AttachmentSkin2D>>,
    pub slots: Option<Vec<Slot2D>>,
    pub world_matrices: Vec<f32>,
}
impl PartialEq for Skeleton2D {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for Skeleton2D {
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
