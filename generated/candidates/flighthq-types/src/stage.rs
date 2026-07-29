// @generated from upstream/packages/types/src/Stage.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{DisplayObject, EntityRuntime, ViewportAlign, ViewportScaleMode};

// Source: upstream/packages/types/src/Stage.ts:12 (sha256:395313cd947e137c2d949635cc2332004f36e418c238cd420bc5beb08214657b)
#[derive(Clone, Default)]
pub struct Stage {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    pub align: ViewportAlign,
    pub root: DisplayObject,
    pub scale_mode: ViewportScaleMode,
    pub color: Option<f64>,
    pub stage_height: f64,
    pub stage_width: f64,
}
impl PartialEq for Stage {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for Stage {
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

// Source: upstream/packages/types/src/Stage.ts:19 (sha256:2132e54a4e2c283ba473779d330f87294776820f1173f4f86d6fd606e9f39945)
pub type StageRuntime = crate::EntityRuntime;
