// @generated from upstream/packages/types/src/SwfTagHandler.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{
    AudioResourceReference, ImageResourceReference, ImportDiagnostic, Node2D, SwfJpegAlphaPayload,
    SwfTagParseResult, SwfTagParseState, SwfTagReader, SwfTagRectangle, SwfTagTimelineState,
    SwfTimeline,
};

// Source: upstream/packages/types/src/SwfTagHandler.ts:27 (sha256:335e0c9b70a3aaf0afb244c08247f553053b10c009a9247431ea7ae82147a2ab)
#[derive(Clone)]
pub struct SwfTagHandler {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub instantiate: Option<SwfTagHandlerInstantiation>,
    pub tags: Vec<f64>,
    pub parse: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(
                        SwfTagReader,
                        f64,
                        SwfTagParseState,
                        SwfTagTimelineState,
                        Option<Vec<ImportDiagnostic>>,
                    ) -> bool
                    + Send
                    + 'static,
            >,
        >,
    >,
    pub finish_timeline: Option<
        std::sync::Arc<
            std::sync::Mutex<
                Box<dyn FnMut(SwfTagParseState, SwfTagTimelineState) -> () + Send + 'static>,
            >,
        >,
    >,
    pub resolve: Option<
        std::sync::Arc<
            std::sync::Mutex<Box<dyn FnMut(SwfTagParseState, SwfTimeline) -> () + Send + 'static>>,
        >,
    >,
}
impl PartialEq for SwfTagHandler {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/SwfTagHandler.ts:68 (sha256:083b76e90e05fa400edb0566c15d57230d8342745ffbe011f68a12962e3f51e5)
#[derive(Clone, Default)]
pub struct SwfTagHandlerInstantiation {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub create_resources: Option<
        std::sync::Arc<
            std::sync::Mutex<
                Box<dyn FnMut(SwfTagParseResult, SwfTagHandlerResources) -> () + Send + 'static>,
            >,
        >,
    >,
    pub create_placement_node: Option<
        std::sync::Arc<
            std::sync::Mutex<
                Box<
                    dyn FnMut(
                            SwfTagParseResult,
                            f64,
                            Option<SwfTagRectangle>,
                            Option<Vec<ImportDiagnostic>>,
                        ) -> Option<Node2D>
                        + Send
                        + 'static,
                >,
            >,
        >,
    >,
    pub has_placement_content: Option<
        std::sync::Arc<
            std::sync::Mutex<Box<dyn FnMut(SwfTagParseResult, f64) -> bool + Send + 'static>>,
        >,
    >,
}
impl PartialEq for SwfTagHandlerInstantiation {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/SwfTagHandler.ts:92 (sha256:f13e386a8acbf155c7db4f14354ab18b8c1e8678355f44f5c33f91bcc1f294a5)
#[derive(Clone, Default)]
pub struct SwfTagHandlerResources {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub audio: Vec<AudioResourceReference>,
    pub images: Vec<ImageResourceReference>,
    pub jpeg_alpha_payloads: Vec<SwfJpegAlphaPayload>,
}
impl PartialEq for SwfTagHandlerResources {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/SwfTagHandler.ts:103 (sha256:10b2d794fed3f7ad3209649c37260dee0cc68f5d22c1f55ae2ff57bfcd2c40f8)
pub type SwfTagHandlerDispatch = Vec<(f64, SwfTagHandler)>;
