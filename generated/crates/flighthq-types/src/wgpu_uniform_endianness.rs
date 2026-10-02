// @generated from upstream/packages/types/src/WgpuUniformEndianness.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::PlatformEndianness;

// Source: upstream/packages/types/src/WgpuUniformEndianness.ts:3 (sha256:c142badb3541724ed5879c821f29e581cfab8056875cf69d7e83f7d05f877d9a)
pub type WgpuUniformEndiannessStatus = String;

// Source: upstream/packages/types/src/WgpuUniformEndianness.ts:5 (sha256:7e12e04625e19320e338419d21f623b7d39434e1a1ee233fb3d37bd2c247b115)
#[derive(Clone, Default)]
pub struct WgpuUniformEndiannessExplanation {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub endianness: PlatformEndianness,
    pub status: WgpuUniformEndiannessStatus,
}
impl PartialEq for WgpuUniformEndiannessExplanation {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
