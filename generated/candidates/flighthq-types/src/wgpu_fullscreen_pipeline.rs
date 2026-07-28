// @generated from upstream/packages/types/src/WgpuFullscreenPipeline.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/WgpuFullscreenPipeline.ts:1 (sha256:a42d38ab253cf19743febd4203b0f39b3ebeac0927728f4ff2b5bd3c879867e0)
#[derive(Clone)]
pub struct WgpuFullscreenPipeline {
    pub pipeline: crate::OpaqueHostValue,
    pub pipeline_layout: crate::OpaqueHostValue,
    pub uniform_bind_group_layout: crate::OpaqueHostValue,
    pub texture_bind_group_layouts: Vec<crate::OpaqueHostValue>,
}
