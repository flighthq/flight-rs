// @generated from upstream/packages/types/src/SurfaceFingerprint.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/SurfaceFingerprint.ts:8 (sha256:6c43a11149663476fb3665ff362a924d87baea2483dfd3feea3f9bd31436f8ad)
#[derive(Clone, Default)]
pub struct SurfaceFingerprint {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub grid_size: f64,
    pub cells: Vec<u8>,
}
impl PartialEq for SurfaceFingerprint {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
