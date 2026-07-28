// @generated from upstream/packages/types/src/ScreenChangeEvent.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::ScreenInfo;

// Source: upstream/packages/types/src/ScreenChangeEvent.ts:5 (sha256:faa1e5f1205ed222ba5f08eb05504acc4b5aae035bea67374b328feb7e99ce2e)
pub type ScreenChangeKind = String;

// Source: upstream/packages/types/src/ScreenChangeEvent.ts:9 (sha256:0df84ed31b7ce6e6a43f4157117de2810ac834a62a1ed693d18653775a3ddabb)
#[derive(Clone, Default)]
pub struct ScreenChangedMetrics {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub bounds: bool,
    pub work_area: bool,
    pub scale_factor: bool,
    pub orientation: bool,
}
impl PartialEq for ScreenChangedMetrics {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/ScreenChangeEvent.ts:18 (sha256:6e7d0cde46a0a46d22bbb899faccdde996f0a657c7ed717975ec556572ac03c4)
#[derive(Clone, Default)]
pub struct ScreenChangeEvent {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub kind: ScreenChangeKind,
    pub screen: ScreenInfo,
    pub changed_metrics: Option<ScreenChangedMetrics>,
}
impl PartialEq for ScreenChangeEvent {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
