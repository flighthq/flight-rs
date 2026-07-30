// @generated from upstream/packages/types/src/WgpuAdapterCapabilities.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/WgpuAdapterCapabilities.ts:1 (sha256:2fe9d81139cc8bc8bbac0afcccc562276e9af5032731679eff3ffa753eef5bac)
#[derive(Clone, Default)]
pub struct WgpuAdapterCapabilities {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub supports_float32_filterable: bool,
    pub supports_timestamp_query: bool,
    pub max_sample_count: f64,
    pub max_texture_dimension2_d: f64,
}
impl PartialEq for WgpuAdapterCapabilities {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
