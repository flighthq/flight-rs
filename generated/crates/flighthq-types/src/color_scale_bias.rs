// @generated from upstream/packages/types/src/ColorScaleBias.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::EntityRuntime;

// Source: upstream/packages/types/src/ColorScaleBias.ts:9 (sha256:6fbc41d7a9065b4fc4a101d3ffc68df9a3c2756746b347495023a341d63b3dc0)
#[derive(Clone, Default)]
pub struct ColorScaleBias {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    #[doc(hidden)]
    pub __flight_entity_snapshot: Option<std::sync::Arc<dyn std::any::Any + Send + Sync>>,
    pub alpha_scale: f64,
    pub alpha_bias: f64,
    pub blue_scale: f64,
    pub blue_bias: f64,
    pub green_scale: f64,
    pub green_bias: f64,
    pub red_scale: f64,
    pub red_bias: f64,
}
impl PartialEq for ColorScaleBias {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for ColorScaleBias {
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

// Source: upstream/packages/types/src/ColorScaleBias.ts:20 (sha256:0d5cd177cd7f80b2dbf217f89bfa66d4c9eb46bd47561cc5b6573e6ea169e9aa)
pub type ColorScaleBiasLike = ColorScaleBias;
