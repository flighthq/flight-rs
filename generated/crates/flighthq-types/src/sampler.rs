// @generated from upstream/packages/types/src/Sampler.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::EntityRuntime;

// Source: upstream/packages/types/src/Sampler.ts:4 (sha256:b4c7c5f0936a76e753f97d07405cbd81dd9b05623d785710b8a5fc2f21183ee0)
pub type TextureWrap = String;

// Source: upstream/packages/types/src/Sampler.ts:8 (sha256:5963a37968ed6e884a4afffa8bb58f3596d65bffdb5740fb3e83e281da2405ca)
pub type TextureFilter = String;

// Source: upstream/packages/types/src/Sampler.ts:19 (sha256:08ac581426ed70f9190c38f52b7d331b546b4a23faafe9b65902b84d69d76d50)
#[derive(Clone, Default)]
pub struct Sampler {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    #[doc(hidden)]
    pub __flight_entity_snapshot: Option<std::sync::Arc<dyn std::any::Any + Send + Sync>>,
    pub anisotropy: f64,
    pub mag_filter: TextureFilter,
    pub min_filter: TextureFilter,
    pub mipmaps: bool,
    pub wrap_u: TextureWrap,
    pub wrap_v: TextureWrap,
}
impl PartialEq for Sampler {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for Sampler {
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

// Source: upstream/packages/types/src/Sampler.ts:28 (sha256:4984430992060b1ac4b1fbcc3dd8b57f6b4e5026f9e4abbfb2a6cf272238fe2f)
pub type SamplerLike = Sampler;
