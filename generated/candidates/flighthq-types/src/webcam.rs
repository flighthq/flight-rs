// @generated from upstream/packages/types/src/Webcam.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/Webcam.ts:6 (sha256:8a155725f097fe08a6e6b94c39e5299a98948fef45e08f3f98e6b1794681e7d3)
pub type WebcamSource = String;

// Source: upstream/packages/types/src/Webcam.ts:8 (sha256:d0b604cf45d81a7fea0ce444a0099767a7e3349f82643a649b142c6817fddb11)
#[derive(Clone)]
pub struct WebcamCaptureOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub source: Option<WebcamSource>,
    pub quality: Option<f64>,
    pub allow_editing: Option<bool>,
    pub max_duration_ms: Option<f64>,
}
impl PartialEq for WebcamCaptureOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Webcam.ts:16 (sha256:9cf8203c121b8cd475eac9dc25c820c259d432f3a426d0bd93f499a728e23b40)
#[derive(Clone)]
pub struct WebcamPhoto {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub data_url: String,
    pub width: f64,
    pub height: f64,
    pub format: String,
}
impl PartialEq for WebcamPhoto {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Webcam.ts:23 (sha256:d3e545a30f9b2abfac821914485200373bb53b52f54c064b8c228c62feccbbe1)
#[derive(Clone)]
pub struct WebcamVideo {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub data_url: String,
    pub duration: f64,
    pub format: String,
}
impl PartialEq for WebcamVideo {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Webcam.ts:29 (sha256:62f5506c799957c9d248ffe9f1d53a2d0be47ffc449f59480fdd3acb0a327f94)
#[derive(Clone)]
pub struct WebcamBackend {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub capture: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(WebcamCaptureOptions) -> crate::Promise<Option<WebcamPhoto>>
                    + Send
                    + 'static,
            >,
        >,
    >,
    pub capture_video: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(WebcamCaptureOptions) -> crate::Promise<Option<WebcamVideo>>
                    + Send
                    + 'static,
            >,
        >,
    >,
    pub request_permission:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> crate::Promise<bool> + Send + 'static>>>,
}
impl PartialEq for WebcamBackend {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
