// @generated from upstream/packages/types/src/SpritesheetFrameData.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::EntityRuntime;

// Source: upstream/packages/types/src/SpritesheetFrameData.ts:3 (sha256:6e9ac8ddb562cadf548f96bb0997808d4e4ca3ab0480d5522dc5d5019f33da0e)
#[derive(Clone, Default)]
pub struct SpritesheetFrameData {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    #[doc(hidden)]
    pub __flight_entity_snapshot: Option<std::sync::Arc<dyn std::any::Any + Send + Sync>>,
    pub height: f64,
    pub name: String,
    pub offset_x: f64,
    pub offset_y: f64,
    pub pivot_x: Option<f64>,
    pub pivot_y: Option<f64>,
    pub rotated: bool,
    pub source_height: f64,
    pub source_width: f64,
    pub width: f64,
    pub x: f64,
    pub y: f64,
}
impl PartialEq for SpritesheetFrameData {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for SpritesheetFrameData {
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
