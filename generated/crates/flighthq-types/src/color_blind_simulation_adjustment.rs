// @generated from upstream/packages/types/src/ColorBlindSimulationAdjustment.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::ColorScaleBiasLike;
use crate::{AdjustmentKind, EntityRuntime};

// Source: upstream/packages/types/src/ColorBlindSimulationAdjustment.ts:6 (sha256:a2960bbc2d69b424e26e023d3829d7a36122daf7aaafcf483fa30384caadf90e)
pub type ColorBlindType = String;

// Source: upstream/packages/types/src/ColorBlindSimulationAdjustment.ts:16 (sha256:cc26eba38e40df2c4d3158d516ad2524a08c987ec16f54f1c75b11d6d5209eef)
#[derive(Clone, Default)]
pub struct ColorBlindSimulationAdjustment {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    #[doc(hidden)]
    pub __flight_entity_snapshot: Option<std::sync::Arc<dyn std::any::Any + Send + Sync>>,
    pub kind: AdjustmentKind,
    pub color_matrix: Vec<f64>,
    pub intensity: Option<f64>,
    pub exposure: Option<f64>,
    pub color_scale_bias: ColorScaleBiasLike,
    pub type_: Option<ColorBlindType>,
    pub matrix: Vec<f64>,
    pub brightness: Option<f64>,
    pub contrast: Option<f64>,
}
impl PartialEq for ColorBlindSimulationAdjustment {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for ColorBlindSimulationAdjustment {
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
