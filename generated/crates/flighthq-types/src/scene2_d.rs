// @generated from upstream/packages/types/src/Scene2D.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{EntityRuntime, Node2D, ViewportAlign, ViewportScaleMode};

// Source: upstream/packages/types/src/Scene2D.ts:15 (sha256:4b69ee022fa00e41a62fd634e053d1b496dd1c92af980eabfe270367dda3a9ef)
#[derive(Clone, Default)]
pub struct Scene2D {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    pub align: ViewportAlign,
    pub color: Option<f64>,
    pub root: Node2D,
    pub scale_mode: ViewportScaleMode,
    pub scene2d_height: f64,
    pub scene2d_width: f64,
}
impl PartialEq for Scene2D {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for Scene2D {
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

// Source: upstream/packages/types/src/Scene2D.ts:24 (sha256:04f380ffa5e46aac36a6e154eb36bbd34f29ad0657c39a1b675586247e48f9be)
pub type Scene2DRuntime = crate::EntityRuntime;
