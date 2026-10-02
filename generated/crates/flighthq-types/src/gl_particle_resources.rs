// @generated from upstream/packages/types/src/GlParticleResources.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::GlParticleShader;

// Source: upstream/packages/types/src/GlParticleResources.ts:3 (sha256:3131b76f429169f6752e69eec1dbb8e0fc4b1522840eb1168eed36537a979613)
#[derive(Clone, Default)]
pub struct GlParticleResources {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub corner_buffer: crate::OpaqueHostValue,
    pub instance_buffer: crate::OpaqueHostValue,
    pub shader: GlParticleShader,
}
impl PartialEq for GlParticleResources {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
