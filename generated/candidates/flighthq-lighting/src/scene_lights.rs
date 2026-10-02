// @generated from upstream/packages/lighting/src/sceneLights.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_entity::{allocate_entity, finish_entity};
use flighthq_types::{
    AmbientLight, DirectionalLight, EntityConstruction, HemisphereLight, PointLight, Scene3DLights,
    SpotLight,
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

// Source: upstream/packages/lighting/src/sceneLights.ts:4 (sha256:1f58f434a80346861540ed8e6c922112522524bb1c70742b4602b903fd589f02)
pub fn create_scene3_d_lights(options: Option<FlightPartialRecord1933629006>) -> Scene3DLights {
    let mut out = allocate_entity();
    initialize_scene3_d_lights(
        (out).clone(),
        ((options).clone()).as_ref().map(|__flight_value| {
            let __flight_source = &(__flight_value);
            FlightPartialRecord1933629006 {
                __flight_identity: std::sync::Arc::clone(&__flight_source.__flight_identity),
                ambient: (__flight_source.ambient).clone(),
                directional: (__flight_source.directional).clone(),
                hemisphere: (__flight_source.hemisphere).clone(),
                point: (__flight_source.point).clone(),
                spot: (__flight_source.spot).clone(),
            }
        }),
    );
    return finish_entity((out).clone());
}

// Source: upstream/packages/lighting/src/sceneLights.ts:19 (sha256:10c7eec3629b15d959a145a82ddbdc445c5eb8e53462e4c61eaf07f1e6dae22a)
pub fn initialize_scene3_d_lights(
    out: EntityConstruction<Scene3DLights>,
    options: Option<FlightPartialRecord1933629006>,
) -> () {
    crate::host_set(
        "host.ambient",
        options.as_ref().and_then(|value| (value.ambient).clone()),
    );
    crate::host_set(
        "host.directional",
        options
            .as_ref()
            .and_then(|value| (value.directional).clone()),
    );
    crate::host_set(
        "host.hemisphere",
        (options
            .as_ref()
            .and_then(|value| (value.hemisphere).clone()))
        .unwrap_or(vec![]),
    );
    crate::host_set(
        "host.point",
        (options.as_ref().and_then(|value| (value.point).clone())).unwrap_or(vec![]),
    );
    crate::host_set(
        "host.spot",
        (options.as_ref().and_then(|value| (value.spot).clone())).unwrap_or(vec![]),
    );
}
