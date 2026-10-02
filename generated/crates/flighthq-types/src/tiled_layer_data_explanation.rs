// @generated from upstream/packages/types/src/TiledLayerDataExplanation.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/TiledLayerDataExplanation.ts:9 (sha256:b91f4a7c2f5d0885c175bb59cdbd39fdecdd9cd546f53263d2a1d4589cb21b87)
pub type TiledLayerDataFailure = String;

// Source: upstream/packages/types/src/TiledLayerDataExplanation.ts:11 (sha256:56e1d2983636233040da74edd8fb1f632baf9bf61cfe272b0a7ccb7d13f97be9)
#[derive(Clone)]
pub struct TiledLayerDataExplanation {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub preserved_as_zero_grid: bool,
    pub reason: crate::FlightUnion2<TiledLayerDataFailure, String>,
}
impl PartialEq for TiledLayerDataExplanation {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
