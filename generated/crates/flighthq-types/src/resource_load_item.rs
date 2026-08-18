// @generated from upstream/packages/types/src/ResourceLoadItem.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::ResourceLoadBytesReporter;

// Source: upstream/packages/types/src/ResourceLoadItem.ts:3 (sha256:54beb2a0b9c06866f1cf5f3d92baa12d3d4fffd425bc4bbe32788145893e01fc)
#[derive(Clone)]
pub struct ResourceLoadItem<T> {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub bytes_hint: Option<f64>,
    pub group: Option<String>,
    pub key: Option<String>,
    pub load: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(crate::OpaqueHostValue, ResourceLoadBytesReporter) -> crate::FlightTask<T>
                    + Send
                    + 'static,
            >,
        >,
    >,
    pub on_bytes_progress:
        Option<std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(f64, f64) -> () + Send + 'static>>>>,
    pub priority: Option<f64>,
    pub retries: Option<f64>,
    pub timeout_ms: Option<f64>,
    pub weight: Option<f64>,
}
impl<T> PartialEq for ResourceLoadItem<T> {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
