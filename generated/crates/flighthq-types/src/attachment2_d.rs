// @generated from upstream/packages/types/src/Attachment2D.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::EntityRuntime;
use crate::{PathWinding, Skin2D};

// Source: upstream/packages/types/src/Attachment2D.ts:9 (sha256:dba844fc255ad45d2663ff159d35ac9ff2f9e04c965f64b94e4b274ecba2e9ff)
#[derive(Clone, Default)]
pub struct Attachment2D {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    #[doc(hidden)]
    pub __flight_entity_snapshot: Option<std::sync::Arc<dyn std::any::Any + Send + Sync>>,
    pub kind: String,
    pub name: Option<String>,
    pub rotation: f64,
    pub x: f64,
    pub y: f64,
    pub commands: Vec<f64>,
    pub point_count: f64,
    pub skin: Option<Skin2D>,
    pub vertices: Option<Vec<f32>>,
    pub winding: PathWinding,
    pub end_slot_index: f64,
}
impl PartialEq for Attachment2D {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for Attachment2D {
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
