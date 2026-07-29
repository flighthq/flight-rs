// @generated from upstream/packages/types/src/WebcamStream.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{EntityRuntime, WebcamFacingMode};

// Source: upstream/packages/types/src/WebcamStream.ts:3 (sha256:4dd420a3de66629fd9d20d6edebb45002d12629e66fe948f85923169759058ea)
#[derive(Clone, Default)]
pub struct WebcamStream {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    pub active: bool,
    pub device_id: String,
    pub facing_mode: Option<WebcamFacingMode>,
    pub frame_rate: f64,
    pub height: f64,
    pub id: String,
    pub width: f64,
}
impl PartialEq for WebcamStream {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for WebcamStream {
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
