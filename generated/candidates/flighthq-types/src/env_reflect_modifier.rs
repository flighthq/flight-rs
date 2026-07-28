// @generated from upstream/packages/types/src/EnvReflectModifier.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/EnvReflectModifier.ts:14 (sha256:57ea745159357dca3900015d7ffddcedc6d076797e254ff4744078bf12bdfa9d)
#[derive(Clone)]
pub struct EnvReflectModifier {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub kind: String,
    pub slot: String,
    pub tint: f64,
    pub intensity: Option<f64>,
    pub fresnel_bias: Option<f64>,
    pub roughness: Option<f64>,
}
impl PartialEq for EnvReflectModifier {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/EnvReflectModifier.ts:23 (sha256:7c28ff397a54bc44bbd7c88e5def768d8b849f15f34174467996e3e4d0b08835)
pub const ENV_REFLECT_MODIFIER_KIND: &'static str = "EnvReflectModifier";
