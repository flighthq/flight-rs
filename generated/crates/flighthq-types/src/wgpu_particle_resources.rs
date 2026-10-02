// @generated from upstream/packages/types/src/WgpuParticleResources.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/WgpuParticleResources.ts:1 (sha256:2d597d5196ced4b9608a64a847b73ee7d190a61def77d508605e5daf66604793)
#[derive(Clone, Default)]
pub struct WgpuParticleResources {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub instance_bind_group_layout: crate::OpaqueHostValue,
    pub module: crate::OpaqueHostValue,
    pub pipeline_layout: crate::OpaqueHostValue,
    pub pipelines: Vec<(crate::OpaqueHostValue, crate::OpaqueHostValue)>,
}
impl PartialEq for WgpuParticleResources {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
