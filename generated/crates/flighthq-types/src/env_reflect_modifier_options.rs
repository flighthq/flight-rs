// @generated from upstream/packages/types/src/EnvReflectModifierOptions.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/EnvReflectModifierOptions.ts:1 (sha256:dbf49aefbe4ffe9612339955a74a9bde2c3a25b4ddeeae0b3d484ddd27a61eec)
#[derive(Clone, Default)]
pub struct EnvReflectModifierOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub tint: Option<f64>,
    pub intensity: Option<f64>,
    pub fresnel_bias: Option<f64>,
    pub roughness: Option<f64>,
}
impl PartialEq for EnvReflectModifierOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
