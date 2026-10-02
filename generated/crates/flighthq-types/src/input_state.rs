// @generated from upstream/packages/types/src/InputState.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::EntityRuntime;

// Source: upstream/packages/types/src/InputState.ts:3 (sha256:ddc084173b8ba19cfd8f9bacf5886d9ff11f051c2235ca69f738c511a4f29c32)
#[derive(Clone, Default)]
pub struct InputState {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    #[doc(hidden)]
    pub __flight_entity_snapshot: Option<std::sync::Arc<dyn std::any::Any + Send + Sync>>,
    pub axis_values: Vec<(f64, f64)>,
    pub gamepad_buttons_down: Vec<f64>,
    pub just_pressed_gamepad_buttons: Vec<f64>,
    pub just_pressed_keys: Vec<f64>,
    pub just_released_gamepad_buttons: Vec<f64>,
    pub just_released_keys: Vec<f64>,
    pub keys_down: Vec<f64>,
    pub pointer_buttons_down: Vec<(f64, f64)>,
}
impl PartialEq for InputState {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for InputState {
    fn __flight_entity_runtime(
        &self,
    ) -> &std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>> {
        &self.__flight_entity_runtime
    }
    fn __flight_entity_snapshot(&self) -> &Option<std::sync::Arc<dyn std::any::Any + Send + Sync>> {
        &self.__flight_entity_snapshot
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
