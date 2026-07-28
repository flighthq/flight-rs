// @generated from upstream/packages/types/src/CustomShaderEffect.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/CustomShaderEffect.ts:2 (sha256:a6776413ec9f28def997d78140a464a8a6d290149e22743d706e661df8ab12db)
#[derive(Clone)]
pub struct CustomShaderEffect {
    pub kind: String,
    pub shader_key: String,
    pub uniforms: Option<crate::OpaqueHostValue>,
}
