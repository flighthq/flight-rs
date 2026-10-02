// @generated from upstream/packages/types/src/Scene3DResources.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{
    EasingFunction, EntityRuntime, GltfCoreFeatureHandler, GltfExtensionHandler,
    HostImageDecodeCapabilities, ImageResourceFetch, ImageResourceReference, ImportDiagnostic,
    Kind, Material, PbrExtension, ResourceLoader, Scene3D, Signal, Texture, TextureSource,
};

// Source: upstream/packages/types/src/Scene3DResources.ts:15 (sha256:01f0ceade3aac271f87c16f033daa7d69535116b19065ebfb5f58a01d2dda8db)
pub type Scene3DMaterialTextureLister =
    std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(Material, Vec<Texture>) -> () + Send + 'static>>>;

// Source: upstream/packages/types/src/Scene3DResources.ts:17 (sha256:6babb1c5402ff9b0b204b67c86e80e74f20cae02f7c2fcd2e93a92de3e53a149)
pub type Scene3DPbrExtensionTextureLister = std::sync::Arc<
    std::sync::Mutex<Box<dyn FnMut(PbrExtension, Vec<Texture>) -> () + Send + 'static>>,
>;

// Source: upstream/packages/types/src/Scene3DResources.ts:19 (sha256:4d763e0c564ab4352cae345b6f5b5ba2235fff30c2364a59784b0a71d7d34694)
#[derive(Clone, Default)]
pub struct Scene3DMaterialTextureRegistry {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub extension_listers: Vec<(Kind, Scene3DPbrExtensionTextureLister)>,
    pub listers: Vec<(Kind, Scene3DMaterialTextureLister)>,
}
impl PartialEq for Scene3DMaterialTextureRegistry {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Scene3DResources.ts:24 (sha256:7d25d63115b4659cae73153f540ed76992ea46aa1e9a39249f538763bf9723b4)
#[derive(Clone)]
pub struct Scene3DResourceEvent {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub ref_: ImageResourceReference,
    pub texture: Texture,
}
impl PartialEq for Scene3DResourceEvent {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Scene3DResources.ts:29 (sha256:14df95c658c23c444b497393e0390fda0becc4ce05fc99b7008d81e0a4958cb6)
#[derive(Clone)]
pub struct Scene3DResourceSignals {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    #[doc(hidden)]
    pub __flight_entity_snapshot: Option<std::sync::Arc<dyn std::any::Any + Send + Sync>>,
    pub on_resource_failed: Signal<
        std::sync::Arc<
            std::sync::Mutex<Box<dyn FnMut(Scene3DResourceEvent) -> () + Send + 'static>>,
        >,
    >,
    pub on_resource_resolved: Signal<
        std::sync::Arc<
            std::sync::Mutex<Box<dyn FnMut(Scene3DResourceEvent) -> () + Send + 'static>>,
        >,
    >,
}
impl PartialEq for Scene3DResourceSignals {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for Scene3DResourceSignals {
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

// Source: upstream/packages/types/src/Scene3DResources.ts:37 (sha256:026fb36aa0a601968ec46914833f9a723f127e819a9ea78e2c9c86a3bf8fbb67)
#[derive(Clone)]
pub struct Scene3DResourceResolver {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    #[doc(hidden)]
    pub __flight_entity_snapshot: Option<std::sync::Arc<dyn std::any::Any + Send + Sync>>,
    pub fetch: ImageResourceFetch,
    pub image_decode: Option<HostImageDecodeCapabilities>,
    pub registry: Scene3DMaterialTextureRegistry,
}
impl PartialEq for Scene3DResourceResolver {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for Scene3DResourceResolver {
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

// Source: upstream/packages/types/src/Scene3DResources.ts:43 (sha256:deb162e4cf16e09458ecfaefa8dcb066cd761f118171518bc49865d995798bb0)
#[derive(Clone, Default)]
pub struct Scene3DResourceResolverOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub fetch: Option<ImageResourceFetch>,
    pub image_decode: Option<HostImageDecodeCapabilities>,
    pub max_concurrent: Option<f64>,
    pub registry: Option<Scene3DMaterialTextureRegistry>,
}
impl PartialEq for Scene3DResourceResolverOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Scene3DResources.ts:52 (sha256:5f43309acf411b68d7cd0a08d4f3a7df1421e528c360bf73dcdd9e8ae703a7cb)
#[derive(Clone)]
pub struct Scene3DResourceInFlight {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub controller: crate::OpaqueHostValue,
    pub promise: crate::FlightTask<()>,
    pub subscribers: Vec<Texture>,
}
impl PartialEq for Scene3DResourceInFlight {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Scene3DResources.ts:58 (sha256:cca48392cc0563408359655051fa56fa10bc7f87bc7ff62ad06dc65a87430355)
#[derive(Clone)]
pub struct Scene3DResourceResolverRuntime {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub in_flight: Vec<(ImageResourceReference, Scene3DResourceInFlight)>,
    pub loader: ResourceLoader,
    pub resolved: Vec<(ImageResourceReference, TextureSource)>,
    pub signals: Option<Scene3DResourceSignals>,
}
impl PartialEq for Scene3DResourceResolverRuntime {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Scene3DResources.ts:65 (sha256:0259e213d5666b2c65056546e063ceac1243ed344bf9c6691e3675ad8eaedc4d)
pub static SCENE3_D_RESOURCE_RESOLVER_RUNTIME_KEY: std::sync::LazyLock<crate::FlightSymbol> =
    std::sync::LazyLock::new(|| crate::FlightSymbol::new());

// Source: upstream/packages/types/src/Scene3DResources.ts:67 (sha256:0b52a40bbc8fb8dc2cc458d66e76617981dc892035a929e293a7434c5ad5baf7)
#[derive(Clone)]
pub struct Scene3DResourceResolverWithRuntime {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    #[doc(hidden)]
    pub __flight_entity_snapshot: Option<std::sync::Arc<dyn std::any::Any + Send + Sync>>,
    pub fetch: ImageResourceFetch,
    pub image_decode: Option<HostImageDecodeCapabilities>,
    pub registry: Scene3DMaterialTextureRegistry,
}
impl PartialEq for Scene3DResourceResolverWithRuntime {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for Scene3DResourceResolverWithRuntime {
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

// Source: upstream/packages/types/src/Scene3DResources.ts:74 (sha256:ff7486fdf9c9b07857bdf223b02568d13c0947fe70b5b1f32b9dc7c05921323d)
#[derive(Clone, Default)]
pub struct Scene3DDocumentLoadProgress {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub loaded: f64,
    pub phase: String,
    pub total: f64,
    pub url: String,
}
impl PartialEq for Scene3DDocumentLoadProgress {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Scene3DResources.ts:81 (sha256:811d5256b35b30b8267cc953837aee302c5df3f4d4141e03795cd86e46de00cd)
#[derive(Clone, Default)]
pub struct Scene3DDocumentLoadOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub progress: Option<
        Signal<
            std::sync::Arc<
                std::sync::Mutex<
                    Box<dyn FnMut(Scene3DDocumentLoadProgress) -> () + Send + 'static>,
                >,
            >,
        >,
    >,
    pub signal: Option<crate::OpaqueHostValue>,
}
impl PartialEq for Scene3DDocumentLoadOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Scene3DResources.ts:88 (sha256:5a5238187e4c5e434e31fd41478eec4e89c2261bc288a5bf817bc2e3f7254c6f)
#[derive(Clone, Default)]
pub struct GltfScene3DDocumentLoadOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub progress: Option<
        Signal<
            std::sync::Arc<
                std::sync::Mutex<
                    Box<dyn FnMut(Scene3DDocumentLoadProgress) -> () + Send + 'static>,
                >,
            >,
        >,
    >,
    pub signal: Option<crate::OpaqueHostValue>,
    pub core_feature_handlers: Option<Vec<GltfCoreFeatureHandler>>,
    pub diagnostics: Option<Vec<ImportDiagnostic>>,
    pub extension_handlers: Option<Vec<GltfExtensionHandler>>,
}
impl PartialEq for GltfScene3DDocumentLoadOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Scene3DResources.ts:94 (sha256:90c5993fc86d056681f160bf5188346b9089c5c1b86713991fcbd5d19c1e556c)
#[derive(Clone, Default)]
pub struct Scene3DResourceLoadProgress {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub loaded: f64,
    pub total: f64,
}
impl PartialEq for Scene3DResourceLoadProgress {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Scene3DResources.ts:99 (sha256:dfdca76dfa65f51610d11b887bd276bde1c69d7af5b586f48f9e66520d8b48dd)
#[derive(Clone, Default)]
pub struct LoadScene3DResourcesOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub select: Option<
        std::sync::Arc<
            std::sync::Mutex<
                Box<dyn FnMut(Texture, ImageResourceReference) -> bool + Send + 'static>,
            >,
        >,
    >,
    pub priority: Option<
        std::sync::Arc<
            std::sync::Mutex<
                Box<dyn FnMut(Texture, ImageResourceReference) -> f64 + Send + 'static>,
            >,
        >,
    >,
    pub progress: Option<
        Signal<
            std::sync::Arc<
                std::sync::Mutex<
                    Box<dyn FnMut(Scene3DResourceLoadProgress) -> () + Send + 'static>,
                >,
            >,
        >,
    >,
}
impl PartialEq for LoadScene3DResourcesOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Scene3DResources.ts:103 (sha256:0263393af9b060aa43c9453e548b688d97815cefe8efc830f989f029dacc8b12)
#[derive(Clone, Default)]
pub struct ResolveScene3DResourcesOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub select: Option<
        std::sync::Arc<
            std::sync::Mutex<
                Box<dyn FnMut(Texture, ImageResourceReference) -> bool + Send + 'static>,
            >,
        >,
    >,
}
impl PartialEq for ResolveScene3DResourcesOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Scene3DResources.ts:107 (sha256:a12571971c419e96f7482d4a994d45aa49bbe8bfa275dfd6ee4cece9788e46ba)
#[derive(Clone)]
pub struct Scene3DResourceResolution {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub ref_: ImageResourceReference,
    pub textures: Vec<Texture>,
    pub source: TextureSource,
}
impl PartialEq for Scene3DResourceResolution {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Scene3DResources.ts:113 (sha256:fd0034753fb96b06dd10887f34d1a26749f99d3191d57857e82a8f56e7389f9e)
#[derive(Clone, Default)]
pub struct Scene3DResources {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub resolved: Vec<Scene3DResourceResolution>,
    pub scene: Scene3D,
    pub unresolved: Vec<Scene3DResourceWorkingSet>,
}
impl PartialEq for Scene3DResources {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Scene3DResources.ts:119 (sha256:84cac90e561c9a73f0b2efd23becd660e8f91bd52660f1b4de9ec04115707e13)
#[derive(Clone)]
pub struct Scene3DResourceWorkingSet {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub ref_: ImageResourceReference,
    pub textures: Vec<Texture>,
}
impl PartialEq for Scene3DResourceWorkingSet {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Scene3DResources.ts:124 (sha256:d79ab9e791a13111f376b1f2591035d0cb4fb970fc474901493fbb87fc73ee90)
#[derive(Clone, Default)]
pub struct UpdateScene3DResourceStreamingOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub select: Option<
        std::sync::Arc<
            std::sync::Mutex<
                Box<dyn FnMut(Texture, ImageResourceReference) -> bool + Send + 'static>,
            >,
        >,
    >,
    pub priority: Option<
        std::sync::Arc<
            std::sync::Mutex<
                Box<dyn FnMut(Texture, ImageResourceReference) -> f64 + Send + 'static>,
            >,
        >,
    >,
}
impl PartialEq for UpdateScene3DResourceStreamingOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Scene3DResources.ts:128 (sha256:3dc39fe6e13776266146466005ef9ea1305b4f723974ceaabf3513c2147d098b)
#[derive(Clone, Default)]
pub struct Scene3DResourceRevealOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub ease: Option<EasingFunction>,
    pub fade_seconds: Option<f64>,
    pub from: Option<f64>,
}
impl PartialEq for Scene3DResourceRevealOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
