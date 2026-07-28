// @generated from upstream/packages/types/src/Bidi.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/Bidi.ts:15 (sha256:a921aaf5358a53ec7c95b5a56e9874b12076236a378cd316012751e55a5fe30e)
pub type BidiClass = String;

// Source: upstream/packages/types/src/Bidi.ts:43 (sha256:f4160c0cbf94d8c4af995e8f99ad429f38b0e778dda9c2db30fef97d0428f64b)
#[derive(Clone)]
pub struct BidiClassBackend {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub get_bidi_class:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(f64) -> BidiClass + Send + 'static>>>,
}
impl PartialEq for BidiClassBackend {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Bidi.ts:50 (sha256:5987936effa39e8e38c087c8b968fa7420711eb6e15f429664afc7fe903ccbac)
pub type BidiDirection = String;

// Source: upstream/packages/types/src/Bidi.ts:56 (sha256:6858c9f5f04f80c3b6f9cac539ffd824b9b3a8502bd78f86753a90a20ce92d4a)
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
