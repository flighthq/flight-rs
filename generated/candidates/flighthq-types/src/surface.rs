// @generated from upstream/packages/types/src/Surface.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{AlphaType, EntityRuntime, ImageResourceCompressed, PixelFormat};

// Source: upstream/packages/types/src/Surface.ts:8 (sha256:b1031fe4ef5c1d3deb40e43e9ac35498c8bc35ad4217086bd1f8732d1f78a0f3)
#[derive(Clone, Default)]
pub struct Surface {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    pub alpha_type: AlphaType,
    pub compressed: Option<ImageResourceCompressed>,
    pub data: Vec<u8>,
    pub format: PixelFormat,
    pub height: f64,
    pub source: Option<crate::OpaqueHostValue>,
    pub version: f64,
    pub width: f64,
    pub color_space: String,
}
impl PartialEq for Surface {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for Surface {
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
