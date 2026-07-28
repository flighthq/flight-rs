// @generated from upstream/packages/effects/src/convolutionEffect.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::ConvolutionEffect;

#[derive(Clone, Default)]
pub struct FlightOmitRecord1 {
    pub __flight_identity: std::sync::Arc<()>,
    pub matrix: Vec<f64>,
    pub matrix_x: f64,
    pub matrix_y: f64,
    pub bias: Option<f64>,
    pub clamp: Option<bool>,
    pub color: Option<f64>,
    pub divisor: Option<f64>,
    pub preserve_alpha: Option<bool>,
}
impl PartialEq for FlightOmitRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/effects/src/convolutionEffect.ts:3 (sha256:da09ff4b411a344966534cbd0fa54eea272b20411116ef4f0c27b20f6052566c)
pub fn create_convolution_effect(options: &FlightOmitRecord1) -> ConvolutionEffect {
    return {
        let __flight_spread_1 = options;
        ConvolutionEffect {
            __flight_identity: std::sync::Arc::new(()),
            kind: "ConvolutionEffect".to_owned(),
            matrix: (__flight_spread_1.matrix).clone(),
            matrix_x: __flight_spread_1.matrix_x,
            matrix_y: __flight_spread_1.matrix_y,
            bias: __flight_spread_1.bias,
            clamp: __flight_spread_1.clamp,
            color: __flight_spread_1.color,
            divisor: __flight_spread_1.divisor,
            preserve_alpha: __flight_spread_1.preserve_alpha,
            ..Default::default()
        }
    };
}
