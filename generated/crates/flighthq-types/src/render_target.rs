// @generated from upstream/packages/types/src/RenderTarget.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{AlphaType, EntityRuntime};
use crate::{CompressedImageData, HostImageSource, PixelFormat};

// Source: upstream/packages/types/src/RenderTarget.ts:11 (sha256:a0d73980b6a720e494454a81bbc1cb444c98a3ff2a5958710bf704d1878506c8)
#[derive(Clone, Default)]
pub struct RenderTargetDimensions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub height: f64,
    pub width: f64,
}
impl PartialEq for RenderTargetDimensions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/RenderTarget.ts:16 (sha256:70e56e4c1b52244a919e2cc3bfa0d0ac1ad7f7bec423bf04d8594940a73d302b)
pub type RenderTargetFormat = String;

// Source: upstream/packages/types/src/RenderTarget.ts:20 (sha256:6c3d727afc6cb980b0d4bcb02ed55534fb2ecdf9131b6f1494e65520a8adc347)
pub type RenderTargetFormatPolicy = String;

// Source: upstream/packages/types/src/RenderTarget.ts:22 (sha256:b7e550ff29ab52a8095a9dc34717bc1c7bf9cf5a8e4b6f043e18baa25252f9b4)
pub type RenderTargetDepth = String;

// Source: upstream/packages/types/src/RenderTarget.ts:29 (sha256:935540e940b052c519a1ecf1aabe414ee3507f7c9da1ca6c4ac9721aef250751)
pub type RenderTargetColorSpace = String;

// Source: upstream/packages/types/src/RenderTarget.ts:34 (sha256:4836e7a68783fe392ee77bde7f52bc3fbe30974d6c2418018c2c0c3ee133cebf)
#[derive(Clone, Default)]
pub struct RenderTargetAxes {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub width: f64,
    pub height: f64,
    pub format: RenderTargetFormat,
    pub color_attachments: f64,
    pub color_formats: Vec<RenderTargetFormat>,
    pub sample_count: f64,
    pub depth: RenderTargetDepth,
    pub color_space: RenderTargetColorSpace,
}
impl PartialEq for RenderTargetAxes {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/RenderTarget.ts:45 (sha256:8a69a95f05fd20bf873fbb7f632a91085823f5ab64452146f7efb4416cdb6ba8)
#[derive(Clone, Default)]
pub struct RenderTargetAxisDifference {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub axis: RenderTargetAxes,
    pub effective: crate::OpaqueHostValue,
    pub requested: crate::OpaqueHostValue,
}
impl PartialEq for RenderTargetAxisDifference {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/RenderTarget.ts:51 (sha256:da72c1a2f0a6d414eacd3309bdec528f46802db9b52b5625942dfdf7bf81b7a6)
#[derive(Clone, Default)]
pub struct RenderTargetExplanation {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub differences: Vec<RenderTargetAxisDifference>,
    pub effective: RenderTargetAxes,
    pub requested: RenderTargetAxes,
}
impl PartialEq for RenderTargetExplanation {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/RenderTarget.ts:57 (sha256:f976a3e923d48395ab6e3ab23594c3979ad742550499012816e1aa6fada959dc)
#[derive(Clone, Default)]
pub struct RenderTargetDescriptor {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub width: f64,
    pub height: f64,
    pub format: Option<RenderTargetFormat>,
    pub color_attachments: Option<f64>,
    pub color_formats: Option<Vec<RenderTargetFormat>>,
    pub sample_count: Option<f64>,
    pub depth: Option<RenderTargetDepth>,
    pub color_space: Option<RenderTargetColorSpace>,
    pub clear_colors: Option<Vec<f64>>,
    pub clear_depth: Option<f64>,
}
impl PartialEq for RenderTargetDescriptor {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/RenderTarget.ts:87 (sha256:c7a251ae0b80f4ecea3ed0c7bf9d8f702baff476a5465d16cdf5e1d1bc427111)
#[derive(Clone, Default)]
pub struct RenderTarget {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    #[doc(hidden)]
    pub __flight_entity_snapshot: Option<std::sync::Arc<dyn std::any::Any + Send + Sync>>,
    pub width: f64,
    pub height: f64,
    pub format: PixelFormat,
    pub color_attachments: Option<f64>,
    pub color_formats: Option<Vec<RenderTargetFormat>>,
    pub sample_count: Option<f64>,
    pub depth: Option<RenderTargetDepth>,
    pub color_space: Option<RenderTargetColorSpace>,
    pub clear_colors: Option<Vec<f64>>,
    pub clear_depth: Option<f64>,
    pub alpha_type: AlphaType,
    pub gamut: String,
    pub kind: crate::OpaqueHostValue,
    pub version: f64,
    pub source: HostImageSource,
    pub compressed: CompressedImageData,
}
impl PartialEq for RenderTarget {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for RenderTarget {
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

// Source: upstream/packages/types/src/RenderTarget.ts:93 (sha256:f1ab7ec236b568f33e9b66eec91b29426d97375591f5060c6a649f9439d5d083)
#[derive(Clone, Default)]
pub struct ResolvedRenderTargetDescriptor {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub width: f64,
    pub height: f64,
    pub format: RenderTargetFormat,
    pub color_attachments: f64,
    pub color_formats: Vec<RenderTargetFormat>,
    pub sample_count: f64,
    pub depth: RenderTargetDepth,
    pub color_space: RenderTargetColorSpace,
    pub clear_colors: Vec<f64>,
    pub clear_depth: f64,
}
impl PartialEq for ResolvedRenderTargetDescriptor {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
