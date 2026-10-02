// @generated from upstream/packages/types/src/HostFontLoading.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/HostFontLoading.ts:1 (sha256:680fe7879cc3d7c52899cf22f424cb89135fd2a7622b9509ce47f9ad50f40b6e)
#[derive(Clone)]
pub struct HostFontLoadingCapability {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub add_font_face: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut(crate::OpaqueHostValue) -> () + Send + 'static>>,
    >,
    pub check_font_face:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(String) -> bool + Send + 'static>>>,
    pub load_font_faces: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(String) -> crate::FlightTask<Vec<crate::OpaqueHostValue>>
                    + Send
                    + 'static,
            >,
        >,
    >,
    pub when_ready: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut() -> crate::FlightTask<()> + Send + 'static>>,
    >,
}
impl PartialEq for HostFontLoadingCapability {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
