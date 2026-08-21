// @generated from upstream/packages/types/src/Image.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{AlphaType, EntityRuntime, HostImageSource};
use crate::{CompressedImageData, PixelFormat, RenderTargetColorSpace, RenderTargetFormat};

// Source: upstream/packages/types/src/Image.ts:10 (sha256:51e00f5e14e6675877996f841832f52d79a46af08bfbf48612c260fe982ae958)
#[derive(Clone, Default)]
pub struct Image {
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
}
impl PartialEq for Image {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for Image {
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

// Source: upstream/packages/types/src/Image.ts:25 (sha256:eb1c241d794b5545f5457e91837b7a41e1948432da4b4664766c831666dbc786)
#[derive(Clone)]
pub struct ImageBackend {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub load_image_from_url: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(
                        String,
                        Option<String>,
                        Option<crate::OpaqueHostValue>,
                    ) -> crate::FlightTask<Image>
                    + Send
                    + 'static,
            >,
        >,
    >,
}
impl PartialEq for ImageBackend {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
