// @generated from upstream/packages/effects/src/customShaderEffect.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::CustomShaderEffect;

#[derive(Clone, Default)]
pub struct FlightOmitRecord4056060894 {
    pub __flight_identity: std::sync::Arc<()>,
    pub shader_key: String,
    pub uniforms: Option<Vec<(String, crate::FlightUnion2<f64, Vec<f64>>)>>,
}
impl PartialEq for FlightOmitRecord4056060894 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/effects/src/customShaderEffect.ts:3 (sha256:9542b2a533be83b99807f767481c502e2e2e937e696bda8edfbf4375f0e41a57)
pub fn create_custom_shader_effect(options: &FlightOmitRecord4056060894) -> CustomShaderEffect {
    return {
        let __flight_spread_1 = (*options).clone();
        CustomShaderEffect {
            __flight_identity: std::sync::Arc::new(()),
            kind: "CustomShaderEffect".to_owned(),
            shader_key: (__flight_spread_1.shader_key).clone(),
            uniforms: (__flight_spread_1.uniforms).clone(),
            ..Default::default()
        }
    };
}
