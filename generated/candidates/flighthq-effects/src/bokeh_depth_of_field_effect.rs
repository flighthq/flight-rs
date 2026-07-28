// @generated from upstream/packages/effects/src/bokehDepthOfFieldEffect.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::BokehDepthOfFieldEffect;

#[derive(Clone, Default)]
pub struct FlightOmitRecord1 {
    pub __flight_identity: std::sync::Arc<()>,
    pub focus_distance: Option<f64>,
    pub focus_range: Option<f64>,
    pub max_blur: Option<f64>,
}
impl PartialEq for FlightOmitRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/effects/src/bokehDepthOfFieldEffect.ts:3 (sha256:0b78c1e4a0b23c1ecf901b3287c73a2234ea6f1d8c6d93ce36bd68c17f59d5c3)
#[derive(Clone, Default)]
struct CreateBokehDepthOfFieldEffectRecord2 {
    __flight_identity: std::sync::Arc<()>,
}
impl PartialEq for CreateBokehDepthOfFieldEffectRecord2 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn create_bokeh_depth_of_field_effect(
    options: Option<FlightOmitRecord1>,
) -> BokehDepthOfFieldEffect {
    let options = options.unwrap_or(FlightOmitRecord1 {
        __flight_identity: std::sync::Arc::new(()),
        focus_distance: None,
        focus_range: None,
        max_blur: None,
    });
    return {
        let __flight_spread_1 = options;
        BokehDepthOfFieldEffect {
            __flight_identity: std::sync::Arc::new(()),
            kind: "BokehDepthOfFieldEffect".to_owned(),
            focus_distance: __flight_spread_1.focus_distance,
            focus_range: __flight_spread_1.focus_range,
            max_blur: __flight_spread_1.max_blur,
            ..Default::default()
        }
    };
}
