// @generated from upstream/packages/types/src/WgpuMaterialRenderer.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/WgpuMaterialRenderer.ts:15 (sha256:076f59ee0f05fca1621199e0b4db7cdafa86abc8e5c8b7f43296974efee7867b)
#[derive(Clone)]
pub struct WgpuMaterialRenderer {
    pub instance_float_count: f64,
    pub get_shader_module: crate::OpaqueHostValue,
    pub pack_instance: Option<crate::OpaqueHostValue>,
}
