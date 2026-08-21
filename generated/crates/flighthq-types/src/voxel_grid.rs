// @generated from upstream/packages/types/src/VoxelGrid.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{AlphaType, EntityRuntime, PixelFormat};
use crate::{CompressedImageData, HostImageSource, RenderTargetColorSpace, RenderTargetFormat};

// Source: upstream/packages/types/src/VoxelGrid.ts:7 (sha256:197ba83ef36b4c0c0ab2e8995c74f35c2bbe61c4f92621633ea33158b02b0953)
#[derive(Clone, Default)]
pub struct VoxelGrid {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    #[doc(hidden)]
    pub __flight_entity_snapshot: Option<std::sync::Arc<dyn std::any::Any + Send + Sync>>,
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
    pub depth: f64,
}
impl PartialEq for VoxelGrid {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for VoxelGrid {
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
