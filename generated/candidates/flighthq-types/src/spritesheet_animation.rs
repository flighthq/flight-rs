// @generated from upstream/packages/types/src/SpritesheetAnimation.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{EntityRuntime, SpritesheetAnimationDirection};

// Source: upstream/packages/types/src/SpritesheetAnimation.ts:4 (sha256:8ac0c9d0f7a4d1503d6f2ba9bdb34706ca82ef0ab0b9c75b5d02cacf528d77e2)
#[derive(Clone, Default)]
pub struct SpritesheetAnimation {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    pub frames: Vec<f64>,
    pub frame_duration: f64,
    pub frame_durations: Option<Vec<f64>>,
    pub direction: SpritesheetAnimationDirection,
    pub loop_: bool,
    pub origin_x: f64,
    pub origin_y: f64,
}
impl PartialEq for SpritesheetAnimation {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for SpritesheetAnimation {
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
