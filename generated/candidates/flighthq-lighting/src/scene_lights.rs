// @generated from upstream/packages/lighting/src/sceneLights.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_entity::create_entity;
use flighthq_types::{
    AmbientLight, DirectionalLight, HemisphereLight, PointLight, Scene3DLights, SpotLight,
};

#[derive(Clone, Default)]
pub struct FlightPartialRecord1933629006 {
    pub __flight_identity: std::sync::Arc<()>,
    pub ambient: Option<AmbientLight>,
    pub directional: Option<DirectionalLight>,
    pub hemisphere: Option<Vec<HemisphereLight>>,
    pub point: Option<Vec<PointLight>>,
    pub spot: Option<Vec<SpotLight>>,
}
impl PartialEq for FlightPartialRecord1933629006 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/lighting/src/sceneLights.ts:13 (sha256:6561c692f70a5977ed3104ebf86a2dcfaee5eb1ddd067d15207232d6d40b0418)
pub fn create_scene3_d_lights(options: Option<FlightPartialRecord1933629006>) -> Scene3DLights {
    return create_entity(Some(Scene3DLights {
        __flight_identity: std::sync::Arc::new(()),
        __flight_entity_snapshot: Default::default(),
        __flight_entity_runtime: Default::default(),
        ambient: options.as_ref().and_then(|value| (value.ambient).clone()),
        directional: options
            .as_ref()
            .and_then(|value| (value.directional).clone()),
        hemisphere: Some(
            (options
                .as_ref()
                .and_then(|value| (value.hemisphere).clone()))
            .clone()
            .unwrap_or(vec![]),
        ),
        point: Some(
            (options.as_ref().and_then(|value| (value.point).clone()))
                .clone()
                .unwrap_or(vec![]),
        ),
        spot: Some(
            (options.as_ref().and_then(|value| (value.spot).clone()))
                .clone()
                .unwrap_or(vec![]),
        ),
    }));
}
