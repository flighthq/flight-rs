// @generated from upstream/packages/types/src/GlContext.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/GlContext.ts:5 (sha256:b08cabd4876e16a1d736a7268a939881109fd4a38490fd5f9c2e0c1afed69275)
pub(crate) type GlContextMember = String;

// Source: upstream/packages/types/src/GlContext.ts:250 (sha256:38fcc7afaaa3f22576bbd55664610f1c51c559deeae3051a537970af67defbe5)
#[derive(Clone, Default)]
pub struct GlContext {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    #[doc(hidden)]
    pub __flight_entity_snapshot: Option<std::sync::Arc<dyn std::any::Any + Send + Sync>>,
}
impl PartialEq for GlContext {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for GlContext {
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

// Source: upstream/packages/types/src/GlContext.ts:254 (sha256:96bce60c3c341a2c035e77d8f68f7c2497afc93c2b64ad7c458d57b1163fd8cb)
#[derive(Clone, Default)]
pub struct GlContextOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub antialias: Option<bool>,
    pub context_attributes: Option<crate::OpaqueHostValue>,
    pub power_preference: Option<crate::OpaqueHostValue>,
}
impl PartialEq for GlContextOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
