// @generated from upstream/packages/types/src/Bitmap.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{AlphaType, EntityRuntime, PixelFormat};
use crate::{CompressedImageData, HostImageSource, RenderTargetColorSpace, RenderTargetFormat};

// Source: upstream/packages/types/src/Bitmap.ts:9 (sha256:8806f84841153875eb1d57f486d1529304a35136d032d28da2ad579b45064d37)
#[derive(Clone, Default)]
pub struct Bitmap {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    pub alpha_type: AlphaType,
    pub gamut: String,
    pub height: f64,
    pub kind: crate::OpaqueHostValue,
    pub version: f64,
    pub width: f64,
    pub format: PixelFormat,
    pub color_attachments: Option<f64>,
    pub color_formats: Option<Vec<RenderTargetFormat>>,
    pub sample_count: Option<f64>,
    pub color_space: Option<RenderTargetColorSpace>,
    pub clear_colors: Option<Vec<f64>>,
    pub clear_depth: Option<f64>,
    pub source: HostImageSource,
    pub compressed: CompressedImageData,
    pub data: Vec<u8>,
}
impl PartialEq for Bitmap {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for Bitmap {
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
