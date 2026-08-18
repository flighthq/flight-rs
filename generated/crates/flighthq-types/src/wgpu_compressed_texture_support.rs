// @generated from upstream/packages/types/src/WgpuCompressedTextureSupport.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/WgpuCompressedTextureSupport.ts:3 (sha256:c11019022c1c3052ee6c48a87bd7a5a602518501ce5bc0b848fc3e5c2cc2232a)
#[derive(Clone, Default)]
pub struct WgpuCompressedTextureSupport {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub astc: bool,
    pub bc: bool,
    pub etc2: bool,
}
impl PartialEq for WgpuCompressedTextureSupport {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
