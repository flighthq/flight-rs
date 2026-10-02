// @generated from upstream/packages/types/src/RiveImportRegistry.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{
    DisplayObject, EntityRuntime, ImportDiagnostic, RiveAdvancedBlend, RiveAnimationClip,
    RiveArtboardGraph, RiveCoreObject, RiveFileAsset, RiveLayoutImport, RivePathRecord,
    RiveSkeleton2DImport, RiveStateMachineDescriptor,
};

// Source: upstream/packages/types/src/RiveImportRegistry.ts:43 (sha256:d71aaf9048ee15153487212feea3bf79d0ca9486f35b55459abd4154c650c2c9)
#[derive(Clone, Default)]
pub struct RiveCoreObjectHandler {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub apply_artboard: Option<
        std::sync::Arc<
            std::sync::Mutex<Box<dyn FnMut(RiveArtboardImportContext) -> () + Send + 'static>>,
        >,
    >,
    pub apply_document: Option<
        std::sync::Arc<
            std::sync::Mutex<Box<dyn FnMut(RiveDocumentImportContext) -> () + Send + 'static>>,
        >,
    >,
    pub import_component: Option<
        std::sync::Arc<
            std::sync::Mutex<
                Box<
                    dyn FnMut(RiveArtboardImportContext, f64) -> Option<DisplayObject>
                        + Send
                        + 'static,
                >,
            >,
        >,
    >,
}
impl PartialEq for RiveCoreObjectHandler {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/RiveImportRegistry.ts:57 (sha256:35b14eec36e6e002c6cb8829a573d160863ba323b032e03ec3a2e59adb595859)
#[derive(Clone, Default)]
pub struct RiveArtboardImportContext {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    #[doc(hidden)]
    pub __flight_entity_snapshot: Option<std::sync::Arc<dyn std::any::Any + Send + Sync>>,
    pub advanced_blends: Vec<RiveAdvancedBlend>,
    pub animations: Vec<RiveAnimationClip>,
    pub artboard: RiveArtboardGraph,
    pub diagnostics: Option<Vec<ImportDiagnostic>>,
    pub font_names: Vec<String>,
    pub layouts: Vec<RiveLayoutImport>,
    pub nodes: Vec<Option<DisplayObject>>,
    pub objects: Vec<RiveCoreObject>,
    pub rebuilds: Vec<(
        f64,
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>,
    )>,
    pub registry: RiveImportRegistry,
    pub root: DisplayObject,
    pub shape_paths: Vec<(f64, Vec<RivePathRecord>)>,
    pub skeleton: Option<RiveSkeleton2DImport>,
    pub state_machines: Vec<RiveStateMachineDescriptor>,
}
impl PartialEq for RiveArtboardImportContext {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for RiveArtboardImportContext {
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

// Source: upstream/packages/types/src/RiveImportRegistry.ts:87 (sha256:6c1d3b04d3527fa35cf4503b8581e6734801758ae802adff843f40f8407b6406)
#[derive(Clone, Default)]
pub struct RiveDocumentImportContext {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    #[doc(hidden)]
    pub __flight_entity_snapshot: Option<std::sync::Arc<dyn std::any::Any + Send + Sync>>,
    pub assets: Vec<RiveFileAsset>,
    pub diagnostics: Option<Vec<ImportDiagnostic>>,
    pub objects: Vec<RiveCoreObject>,
    pub registry: RiveImportRegistry,
}
impl PartialEq for RiveDocumentImportContext {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for RiveDocumentImportContext {
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

// Source: upstream/packages/types/src/RiveImportRegistry.ts:101 (sha256:e8a5858d7606aad6d6c612a3d64f1d333a84c0a43360b5945f7053b844408f77)
#[derive(Clone, Default)]
pub struct RiveImportRegistry {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub handlers: Vec<(f64, RiveCoreObjectHandler)>,
}
impl PartialEq for RiveImportRegistry {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
