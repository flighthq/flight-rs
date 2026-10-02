// @generated from upstream/packages/types/src/ImageResource.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{AlphaType, Bitmap, EntityRuntime, HostImageSource, PixelFormat, Surface};
use crate::{CompressedImageData, RenderTargetColorSpace, RenderTargetFormat};

// Source: upstream/packages/types/src/ImageResource.ts:12 (sha256:11fd7e972b1d94b617ca0074f8ac502d60afb357944c5155f1c24678c1d9ff03)
#[derive(Clone, Default)]
pub struct ImageResource {
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
    pub source: HostImageSource,
    pub compressed: CompressedImageData,
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

// Source: upstream/packages/types/src/ImageResource.ts:27 (sha256:19a459e2ee7ff89a4d16b9c3be5ba23c694fb45c74725f7bb39d1ab59d60b231)
#[derive(Clone)]
pub struct HostImageCapability {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub create_image_from_bitmap: Option<
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(Bitmap) -> ImageResource + Send + 'static>>>,
    >,
    pub create_image_from_surface: Option<
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(Surface) -> ImageResource + Send + 'static>>>,
    >,
    pub load_image_from_url: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(
                        String,
                        Option<String>,
                        Option<crate::OpaqueHostValue>,
                    ) -> crate::FlightTask<ImageResource>
                    + Send
                    + 'static,
            >,
        >,
    >,
}
impl PartialEq for HostImageCapability {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/ImageResource.ts:43 (sha256:3a7518481751d7a40e554f501deb30ac35c34638fef69da7620112b1b8ead2d8)
pub type ImageOperation = HostImageCapability;
