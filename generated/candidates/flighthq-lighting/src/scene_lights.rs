// @generated from upstream/packages/lighting/src/sceneLights.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::{
    AmbientLight, DirectionalLight, HemisphereLight, PointLight, SceneLights, SpotLight,
};

#[derive(Clone, Default)]
pub struct FlightPartialRecord1 {
    pub __flight_identity: std::sync::Arc<()>,
    pub ambient: Option<AmbientLight>,
    pub directional: Option<DirectionalLight>,
    pub hemisphere: Option<Vec<HemisphereLight>>,
    pub point: Option<Vec<PointLight>>,
    pub spot: Option<Vec<SpotLight>>,
}
impl PartialEq for FlightPartialRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/lighting/src/sceneLights.ts:13 (sha256:2adbef4ccf600811480476a0d0aa3c9a4315bd3dc171d0f19ef264123089219d)
pub fn create_scene_lights(options: Option<FlightPartialRecord1>) -> SceneLights {
    return SceneLights {
        __flight_identity: std::sync::Arc::new(()),
        ambient: options.as_ref().and_then(|value| (value.ambient).clone()),
        directional: options
            .as_ref()
            .and_then(|value| (value.directional).clone()),
        hemisphere: Some(
            (options
                .as_ref()
                .and_then(|value| (value.hemisphere).clone()))
            .unwrap_or(vec![]),
        ),
        point: Some((options.as_ref().and_then(|value| (value.point).clone())).unwrap_or(vec![])),
        spot: Some((options.as_ref().and_then(|value| (value.spot).clone())).unwrap_or(vec![])),
    };
}
