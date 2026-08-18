// @generated from upstream/packages/types/src/AnimationBlendTree.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{AnimationChannel, AnimationPlayer, AnimationSampleAccumulator, EntityRuntime};

// Source: upstream/packages/types/src/AnimationBlendTree.ts:9 (sha256:999935ff446a57ea847011240330cc4caeb5afe76fd06fddb5d816cbb15d5dfa)
#[derive(Clone, Default)]
pub struct AnimationBlendTreeInput {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    pub additive: bool,
    pub player: AnimationPlayer,
    pub weight: f64,
}
impl PartialEq for AnimationBlendTreeInput {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for AnimationBlendTreeInput {
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

// Source: upstream/packages/types/src/AnimationBlendTree.ts:16 (sha256:e210c92a3b1a0c7eff315023dfa318ee36312f9c0d0a14649dd030c858bd0fe9)
#[derive(Clone, Default)]
pub struct AnimationBlendTreeChannelSource {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub channel_index: f64,
    pub input_index: f64,
}
impl PartialEq for AnimationBlendTreeChannelSource {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/AnimationBlendTree.ts:23 (sha256:d73fbec5a2c4773d9e8eb53ed511928234ed9ff9989a54401b85df4ed2c1ad56)
#[derive(Clone, Default)]
pub struct AnimationBlendTreeChannel {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub accumulator: AnimationSampleAccumulator,
    pub channel: AnimationChannel,
    pub sources: Vec<AnimationBlendTreeChannelSource>,
}
impl PartialEq for AnimationBlendTreeChannel {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/AnimationBlendTree.ts:31 (sha256:054fcb71f93fcdf8767c5be098eb1a25dc1facb91452f1a2fdc42cb37556318c)
#[derive(Clone, Default)]
pub struct AnimationBlendTree {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    pub channels: Vec<AnimationBlendTreeChannel>,
    pub inputs: Vec<AnimationBlendTreeInput>,
    pub players: Vec<AnimationPlayer>,
    pub sample_scratch: Vec<f32>,
}
impl PartialEq for AnimationBlendTree {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for AnimationBlendTree {
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
