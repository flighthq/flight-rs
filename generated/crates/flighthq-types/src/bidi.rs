// @generated from upstream/packages/types/src/Bidi.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::EntityRuntime;

// Source: upstream/packages/types/src/Bidi.ts:17 (sha256:a921aaf5358a53ec7c95b5a56e9874b12076236a378cd316012751e55a5fe30e)
pub type BidiClass = String;

// Source: upstream/packages/types/src/Bidi.ts:45 (sha256:1911c2b14f496522dfc77beb85c35012bb02656d3599b9c49e8c0edbe893524c)
#[derive(Clone)]
pub struct BidiClassKernel {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    #[doc(hidden)]
    pub __flight_entity_snapshot: Option<std::sync::Arc<dyn std::any::Any + Send + Sync>>,
    pub get_bidi_class:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(f64) -> BidiClass + Send + 'static>>>,
}
impl PartialEq for BidiClassKernel {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for BidiClassKernel {
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

// Source: upstream/packages/types/src/Bidi.ts:50 (sha256:e0a2087a7b4e8ff632349c63e11bf5e3ca2236ef83d0ac3f10db676ba807bc1e)
pub type BidiClassKernelKind = String;

// Source: upstream/packages/types/src/Bidi.ts:55 (sha256:e365b61fd0458c984cc691a4ab95a23b2ea2a19da4e24bda85a07938d13a45be)
#[derive(Clone, Default)]
pub struct BidiClassKernelExplanation {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub kernel: BidiClassKernelKind,
    pub coverage: String,
    pub covered_code_point_ranges: Vec<BidiCodePointRange>,
    pub fallback_class: Option<BidiClass>,
    pub table_valid: Option<bool>,
}
impl PartialEq for BidiClassKernelExplanation {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Bidi.ts:63 (sha256:fae3970fd134a8a56588fd44c529e66daf0b3133727508e13c4621c1a3dc0a32)
#[derive(Clone, Default)]
pub struct BidiCodePointRange {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub end: f64,
    pub start: f64,
}
impl PartialEq for BidiCodePointRange {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Bidi.ts:69 (sha256:a3ae7f591e8cbfa974b032d934b6908ad8c864fd09c2efeeef2c527bc565a815)
pub type TextBidiGuard =
    std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(f64) -> () + Send + 'static>>>;

// Source: upstream/packages/types/src/Bidi.ts:73 (sha256:5987936effa39e8e38c087c8b968fa7420711eb6e15f429664afc7fe903ccbac)
pub type BidiDirection = String;

// Source: upstream/packages/types/src/Bidi.ts:79 (sha256:6858c9f5f04f80c3b6f9cac539ffd824b9b3a8502bd78f86753a90a20ce92d4a)
#[derive(Clone, Default)]
pub struct BidiRun {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub start: f64,
    pub end: f64,
    pub level: f64,
    pub direction: String,
}
impl PartialEq for BidiRun {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
