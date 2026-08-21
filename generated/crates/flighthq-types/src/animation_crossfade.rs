// @generated from upstream/packages/types/src/AnimationCrossfade.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{AnimationChannel, AnimationPlayer, EasingFunction, EntityRuntime};

// Source: upstream/packages/types/src/AnimationCrossfade.ts:8 (sha256:fe06435797da8cf9bf102b8854a6814b5c53dc2c5bb25df2f0dbadd34d731f39)
#[derive(Clone, Default)]
pub struct AnimationCrossfadeChannel {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub channel: AnimationChannel,
    pub from_index: Option<f64>,
    pub to_index: Option<f64>,
}
impl PartialEq for AnimationCrossfadeChannel {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/AnimationCrossfade.ts:17 (sha256:863416025955d4c2f75507237c0651d71f10944bb0b639a679d76c2ab80bc2e1)
#[derive(Clone, Default)]
pub struct AnimationCrossfadeOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub curve: Option<EasingFunction>,
}
impl PartialEq for AnimationCrossfadeOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/AnimationCrossfade.ts:26 (sha256:592fba8ca0e3d4c3037c94796a1c24af517eaa4337ffd428ae340ca7f8c0bf29)
#[derive(Clone)]
pub struct AnimationCrossfade {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    #[doc(hidden)]
    pub __flight_entity_snapshot: Option<std::sync::Arc<dyn std::any::Any + Send + Sync>>,
    pub channels: Vec<AnimationCrossfadeChannel>,
    pub curve: EasingFunction,
    pub duration: f64,
    pub elapsed: f64,
    pub from: AnimationPlayer,
    pub from_sample: Vec<f32>,
    pub to: AnimationPlayer,
    pub to_sample: Vec<f32>,
    pub weight: f64,
}
impl PartialEq for AnimationCrossfade {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for AnimationCrossfade {
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
