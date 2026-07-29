// @generated from upstream/packages/types/src/ImageResource.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{AlphaType, EntityRuntime, ImageResourceCompressed, PixelFormat};

// Source: upstream/packages/types/src/ImageResource.ts:18 (sha256:e28dc5618fbdb55d16b93f846822b1c8f71235c796f967cfb1ae7ef45b99657c)
#[derive(Clone, Default)]
pub struct ImageResource {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    pub alpha_type: AlphaType,
    pub compressed: Option<ImageResourceCompressed>,
    pub data: Option<Vec<u8>>,
    pub format: PixelFormat,
    pub height: f64,
    pub source: Option<crate::OpaqueHostValue>,
    pub version: f64,
    pub width: f64,
}
impl PartialEq for ImageResource {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for ImageResource {
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
