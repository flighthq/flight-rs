// @generated from upstream/packages/types/src/Scene2DResources.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{
    AudioDecoderRegistry, AudioResourceFetch, AudioResourceReference, HostAudioDecodeCapabilities,
    ImageDecodeFallback, ImageResourceFetch, ImageResourceReference, Node2D, Scene2DDocument,
    Scene2DSlotReference, Signal,
};

// Source: upstream/packages/types/src/Scene2DResources.ts:9 (sha256:3f083b92610afb95b6bb4b027ea8d76bd8bdef8903d58e06f4c13712ea089b58)
#[derive(Clone, Default)]
pub struct Scene2DDocumentFetchProgress {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub loaded: f64,
    pub total: f64,
    pub url: String,
}
impl PartialEq for Scene2DDocumentFetchProgress {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Scene2DResources.ts:15 (sha256:16ac47d6c98caac94b4f85f5625af3f556dea20c078b174a671e1763c55df1bf)
pub type Scene2DDocumentFetcher = std::sync::Arc<
    std::sync::Mutex<
        Box<
            dyn FnMut(
                    String,
                    crate::OpaqueHostValue,
                    Option<
                        Signal<
                            std::sync::Arc<
                                std::sync::Mutex<
                                    Box<
                                        dyn FnMut(Scene2DDocumentFetchProgress) -> ()
                                            + Send
                                            + 'static,
                                    >,
                                >,
                            >,
                        >,
                    >,
                ) -> crate::FlightTask<Option<Vec<u8>>>
                + Send
                + 'static,
        >,
    >,
>;

// Source: upstream/packages/types/src/Scene2DResources.ts:21 (sha256:d3e3e93673f8146adaec1a684e18664858256639e63ab9971ecfa60cafe81001)
#[derive(Clone, Default)]
pub struct Scene2DDocumentImportContext {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub mime_type: Option<String>,
    pub url: Option<String>,
}
impl PartialEq for Scene2DDocumentImportContext {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Scene2DResources.ts:26 (sha256:90d041f681117d784a12a0a469245da0e278512f9bb8b3ca5e0316bf239436c4)
pub type Scene2DDocumentImporter = std::sync::Arc<
    std::sync::Mutex<
        Box<
            dyn FnMut(Vec<u8>, Scene2DDocumentImportContext) -> Option<Scene2DDocument>
                + Send
                + 'static,
        >,
    >,
>;

// Source: upstream/packages/types/src/Scene2DResources.ts:31 (sha256:9ece9d13fc94c72949ddffe0966829153af52d97ba4b1cdabbcc1881f279c783)
pub type Scene2DDocumentImporterMatcher = std::sync::Arc<
    std::sync::Mutex<
        Box<dyn FnMut(Vec<u8>, Scene2DDocumentImportContext) -> bool + Send + 'static>,
    >,
>;

// Source: upstream/packages/types/src/Scene2DResources.ts:36 (sha256:9523426899391d8cd5c2462fccfa0dca60efe2831b143081adb1052f85a03370)
#[derive(Clone)]
pub struct Scene2DDocumentImporterEntry {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub import_document: Scene2DDocumentImporter,
    pub kind: String,
    pub matches: Scene2DDocumentImporterMatcher,
}
impl PartialEq for Scene2DDocumentImporterEntry {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Scene2DResources.ts:42 (sha256:3b69cede98b56074815dfade20e45df2d3d6fe6003ff2945e11675b3084e8e67)
#[derive(Clone, Default)]
pub struct Scene2DDocumentImporterRegistry {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub entries: Vec<Scene2DDocumentImporterEntry>,
}
impl PartialEq for Scene2DDocumentImporterRegistry {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Scene2DResources.ts:46 (sha256:d0bf53d413ef0e0c58236e2e46788e15c4552d536b092bb0afcf3889bd6fcde6)
#[derive(Clone, Default)]
pub struct Scene2DDocumentLoadOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub mime_type: Option<String>,
    pub progress: Option<
        Signal<
            std::sync::Arc<
                std::sync::Mutex<
                    Box<dyn FnMut(Scene2DDocumentFetchProgress) -> () + Send + 'static>,
                >,
            >,
        >,
    >,
    pub signal: Option<crate::OpaqueHostValue>,
}
impl PartialEq for Scene2DDocumentLoadOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Scene2DResources.ts:53 (sha256:70d4b5d9d94bab78f01c871ebb141a660a5605a44963e3eb561a7ed636efca55)
pub type Scene2DSlotContentResolver = std::sync::Arc<
    std::sync::Mutex<Box<dyn FnMut(Scene2DSlotReference) -> Option<Node2D> + Send + 'static>>,
>;

// Source: upstream/packages/types/src/Scene2DResources.ts:55 (sha256:995decd658aa7163b0cfc5deb43d6240236e6e96088005582adb272c8a9aa0c8)
#[derive(Clone, Default)]
pub struct Scene2DSlotResolution {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub content: Node2D,
    pub reference: Scene2DSlotReference,
}
impl PartialEq for Scene2DSlotResolution {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Scene2DResources.ts:60 (sha256:36711126c79e5c002ce45cf20b5b39b9d1b0dcd276ec5c8a75cd8a277eb71ba2)
#[derive(Clone, Default)]
pub struct Scene2DResources {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub document: Scene2DDocument,
    pub resolved: Vec<Scene2DSlotResolution>,
    pub root: Node2D,
    pub unresolved: Vec<Scene2DSlotReference>,
}
impl PartialEq for Scene2DResources {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Scene2DResources.ts:67 (sha256:c03dcf7f8e0f93b3281e8afde226ee3ce0923dc885f17050b31c68d04133247c)
#[derive(Clone, Default)]
pub struct ResolveScene2DResourcesOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub resolve_slot_content: Option<Scene2DSlotContentResolver>,
    pub select: Option<
        std::sync::Arc<
            std::sync::Mutex<Box<dyn FnMut(Scene2DSlotReference) -> bool + Send + 'static>>,
        >,
    >,
}
impl PartialEq for ResolveScene2DResourcesOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Scene2DResources.ts:72 (sha256:6fbb20bb037c60472cf3aa107bbc4f826f01a4635d34d6aa337b728d63d1b69e)
#[derive(Clone)]
pub struct Scene2DImageResourceLoadProgress {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub loaded: f64,
    pub reference: ImageResourceReference,
    pub total: f64,
}
impl PartialEq for Scene2DImageResourceLoadProgress {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Scene2DResources.ts:80 (sha256:8702cd3175279211f913ce9f5519c3364baafa5c89b80562dc7b80d52063117f)
#[derive(Clone, Default)]
pub struct Scene2DImageResources {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub document: Scene2DDocument,
    pub resolved: Vec<ImageResourceReference>,
    pub unresolved: Vec<ImageResourceReference>,
}
impl PartialEq for Scene2DImageResources {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Scene2DResources.ts:86 (sha256:6ed2e1794b6ad046755e1d244fdf47f816a08ac47cf2092550a529a1dc5efa58)
#[derive(Clone, Default)]
pub struct LoadScene2DImageResourcesOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub decode: Option<ImageDecodeFallback>,
    pub fetch: Option<ImageResourceFetch>,
    pub progress: Option<
        Signal<
            std::sync::Arc<
                std::sync::Mutex<
                    Box<dyn FnMut(Scene2DImageResourceLoadProgress) -> () + Send + 'static>,
                >,
            >,
        >,
    >,
    pub select: Option<
        std::sync::Arc<
            std::sync::Mutex<Box<dyn FnMut(ImageResourceReference) -> bool + Send + 'static>>,
        >,
    >,
    pub signal: Option<crate::OpaqueHostValue>,
}
impl PartialEq for LoadScene2DImageResourcesOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Scene2DResources.ts:95 (sha256:0f22176591c8e15c7bdfaa6eb34dcf219a667961bcad08349a7d86243cc90e0c)
#[derive(Clone)]
pub struct Scene2DAudioResourceLoadProgress {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub loaded: f64,
    pub reference: AudioResourceReference,
    pub total: f64,
}
impl PartialEq for Scene2DAudioResourceLoadProgress {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Scene2DResources.ts:101 (sha256:3fcff2266a2e00babf73dc8ca5adf012e8bc74cc9f466656a0949c435c622ec9)
#[derive(Clone, Default)]
pub struct Scene2DAudioResources {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub document: Scene2DDocument,
    pub resolved: Vec<AudioResourceReference>,
    pub unresolved: Vec<AudioResourceReference>,
}
impl PartialEq for Scene2DAudioResources {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Scene2DResources.ts:107 (sha256:d5e0a28eaaa61cb7a39a9fcb029651ae9a50124a297153f58b48fffccf579366)
#[derive(Clone, Default)]
pub struct LoadScene2DAudioResourcesOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub audio_decode: Option<HostAudioDecodeCapabilities>,
    pub decoders: Option<AudioDecoderRegistry>,
    pub fetch: Option<AudioResourceFetch>,
    pub progress: Option<
        Signal<
            std::sync::Arc<
                std::sync::Mutex<
                    Box<dyn FnMut(Scene2DAudioResourceLoadProgress) -> () + Send + 'static>,
                >,
            >,
        >,
    >,
    pub select: Option<
        std::sync::Arc<
            std::sync::Mutex<Box<dyn FnMut(AudioResourceReference) -> bool + Send + 'static>>,
        >,
    >,
    pub signal: Option<crate::OpaqueHostValue>,
}
impl PartialEq for LoadScene2DAudioResourcesOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Scene2DResources.ts:124 (sha256:0dc1c780147b6cd32e08424f32cfe57e400e4b570ccd295f2e0482d34bb714d1)
#[derive(Clone, Default)]
pub struct Scene2DResourceCoverageCount {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub resolved: f64,
    pub total: f64,
}
impl PartialEq for Scene2DResourceCoverageCount {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Scene2DResources.ts:131 (sha256:886b717b881e9e364eed5048dada4a72dea98b836d2f55b73daeca625401bbaa)
#[derive(Clone, Default)]
pub struct Scene2DResourceCoverageExplanation {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub audio_resources: Scene2DResourceCoverageCount,
    pub complete: bool,
    pub image_resources: Scene2DResourceCoverageCount,
    pub required_slots: Scene2DResourceCoverageCount,
}
impl PartialEq for Scene2DResourceCoverageExplanation {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Scene2DResources.ts:138 (sha256:2854bf2c59493f213a69eb2d75998232cc21bda1e7acd5c61634bb020045db92)
pub type Scene2DResourceFailureOperation = String;

// Source: upstream/packages/types/src/Scene2DResources.ts:145 (sha256:bb27c9ea091ecc00b8d637c1898f3d0e0c9e5f7ac80d4b61b361a65cbdab287a)
pub type Scene2DResourceFailureReason = String;

// Source: upstream/packages/types/src/Scene2DResources.ts:155 (sha256:e204b86168a14d08b49f74157a1df0e862b32b1c625be4a7b3d15e77508fdea4)
#[derive(Clone, Default)]
pub struct Scene2DResourceFailureNotice {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub operation: Scene2DResourceFailureOperation,
    pub reason: Scene2DResourceFailureReason,
    pub total: f64,
    pub unresolved: f64,
    pub url: Option<String>,
}
impl PartialEq for Scene2DResourceFailureNotice {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Scene2DResources.ts:163 (sha256:775edf7684682c5ab9bef1493a8c48957a83ff2820f8ea68abaa2ef3c5aacc18)
pub type Scene2DResourceFailureGuard = std::sync::Arc<
    std::sync::Mutex<Box<dyn FnMut(Scene2DResourceFailureNotice) -> () + Send + 'static>>,
>;
