// @generated from upstream/packages/render-wgpu/src/wgpuAdapterCapabilities.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::WgpuAdapterCapabilities;

// Source: upstream/packages/render-wgpu/src/wgpuAdapterCapabilities.ts:10 (sha256:ca34c9174b5865bf135a544dcd4fdf1af36c1168128aba9968e053fce59408a4)
pub fn get_wgpu_adapter_capabilities(adapter: crate::OpaqueHostValue) -> WgpuAdapterCapabilities {
    let features = crate::host_value::<crate::OpaqueHostValue>("host.features");
    let limits = crate::host_value::<crate::OpaqueHostValue>("host.limits");
    let supports_float32_filterable = crate::host_value::<()>("host.has");
    let supports_timestamp_query = crate::host_value::<()>("host.has");
    let max_texture_dimension2_d =
        (crate::host_value::<Option<f64>>("host.maxTextureDimension2D")).unwrap_or(8192.0_f64);
    let max_sample_count = 4.0_f64;
    return WgpuAdapterCapabilities {
        __flight_identity: std::sync::Arc::new(()),
        max_sample_count: max_sample_count,
        max_texture_dimension2_d: max_texture_dimension2_d,
        supports_float32_filterable: supports_float32_filterable,
        supports_timestamp_query: supports_timestamp_query,
    };
}
