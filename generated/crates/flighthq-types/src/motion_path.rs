// @generated from upstream/packages/types/src/MotionPath.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{EntityRuntime, Path};

// Source: upstream/packages/types/src/MotionPath.ts:14 (sha256:8b134fdf89edc14002bdfb7772da3967fb91db8069a65fbc6d84ed7295a1a8d1)
pub type MotionPathLoopMode = String;

// Source: upstream/packages/types/src/MotionPath.ts:24 (sha256:f948037e710792e688e6ceae9840880dadd57af89ad93dc943ed6ddf8c4eba89)
#[derive(Clone, Default)]
pub struct MotionPath {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    #[doc(hidden)]
    pub __flight_entity_snapshot: Option<std::sync::Arc<dyn std::any::Any + Send + Sync>>,
    pub direction: f64,
    pub distance: f64,
    pub length: f64,
    pub loop_mode: MotionPathLoopMode,
    pub path: Path,
    pub speed: f64,
}
impl PartialEq for MotionPath {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for MotionPath {
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
