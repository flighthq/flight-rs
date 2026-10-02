// @generated from upstream/packages/types/src/ClipRegion.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{EntityRuntime, PathWinding, Rectangle};

// Source: upstream/packages/types/src/ClipRegion.ts:20 (sha256:239d5bd7a469f5e125c3709c41dda9dcc4f4e4718b97e84a85e23e33f731c1fb)
#[derive(Clone, Default)]
pub struct ClipRegion {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    #[doc(hidden)]
    pub __flight_entity_snapshot: Option<std::sync::Arc<dyn std::any::Any + Send + Sync>>,
    pub rect: Rectangle,
    pub contours: Option<Vec<Vec<f64>>>,
    pub winding: PathWinding,
    pub version: f64,
}
impl PartialEq for ClipRegion {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for ClipRegion {
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

// Source: upstream/packages/types/src/ClipRegion.ts:28 (sha256:d514dd2259734f78e1efae273b4e71691092b555a9b2832cc782063f2ca54cce)
#[derive(Clone, Default)]
pub struct ClipRegionExplanation {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub conservative: bool,
    pub status: String,
}
impl PartialEq for ClipRegionExplanation {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/ClipRegion.ts:33 (sha256:29bf2b5d74fea2f349eb338d095277a7e76dcd2ab49743eeb3eecfc924efab18)
#[derive(Clone, Default)]
pub struct ClipRegionContoursExplanation {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub contour_index: f64,
    pub coordinate_count: f64,
    pub reason: String,
}
impl PartialEq for ClipRegionContoursExplanation {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/ClipRegion.ts:39 (sha256:e8bcd2ed85ea7fb27b745cdfdc8f1407b997c0120423d2384bf31bba793698f6)
pub type ClipRegionContoursGuard = std::sync::Arc<
    std::sync::Mutex<
        Box<dyn FnMut(ClipRegionContoursExplanation, Vec<Vec<f64>>) -> () + Send + 'static>,
    >,
>;

// Source: upstream/packages/types/src/ClipRegion.ts:48 (sha256:6c511de0c14602672dd1c83729f1a59405f3ebbae160c52232c3273ce0ab59c6)
pub type ClipRegionReleaseGuard =
    std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(ClipRegion) -> () + Send + 'static>>>;

// Source: upstream/packages/types/src/ClipRegion.ts:50 (sha256:46e81be2835e0d723ff7454424ca04078d7375d2a5354e0539d8918ba2016ea5)
pub type ClipRegionUseGuard =
    std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(ClipRegion) -> () + Send + 'static>>>;
