// @generated from upstream/packages/lighting/src/hemisphereLight.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_entity::{allocate_entity, finish_entity};
use flighthq_types::{
    EntityConstruction, HEMISPHERE_LIGHT_KIND as hemisphere_light_kind_constant, HemisphereLight,
    HemisphereLightOptions, UNITLESS_LIGHT_UNIT as unitless_light_unit_constant,
};

// Source: upstream/packages/lighting/src/hemisphereLight.ts:6 (sha256:a3cae011bb8e988142f96257e615cc7260d73399b622a90ea275ac2697e6c966)
pub fn clone_hemisphere_light(source: &HemisphereLight) -> HemisphereLight {
    return create_hemisphere_light(Some(HemisphereLightOptions {
        __flight_identity: std::sync::Arc::new(()),
        enabled: Some(source.enabled),
        ground_color: Some(source.ground_color),
        intensity: Some(source.intensity),
        intensity_unit: Some((source.intensity_unit).clone()),
        sky_color: Some(source.sky_color),
    }));
}

// Source: upstream/packages/lighting/src/hemisphereLight.ts:16 (sha256:93810366e09c9cb2763edfa6e36c72136360996ed3a3530ecd93921b37116eae)
pub fn create_hemisphere_light(options: Option<HemisphereLightOptions>) -> HemisphereLight {
    let mut out = allocate_entity();
    initialize_hemisphere_light((out).clone(), ((options).clone()).clone());
    return finish_entity((out).clone());
}

// Source: upstream/packages/lighting/src/hemisphereLight.ts:25 (sha256:f2da9f4190ffee8cc965d77276fbdd9f068045c705390d92ba167cc7e08b4d8a)
pub fn initialize_hemisphere_light(
    out: EntityConstruction<HemisphereLight>,
    options: Option<HemisphereLightOptions>,
) -> () {
    crate::host_set(
        "host.enabled",
        (options.as_ref().and_then(|value| value.enabled)).unwrap_or(true),
    );
    crate::host_set(
        "host.groundColor",
        (options.as_ref().and_then(|value| value.ground_color)).unwrap_or(4294967295.0_f64),
    );
    crate::host_set(
        "host.intensity",
        (options.as_ref().and_then(|value| value.intensity)).unwrap_or(1.0_f64),
    );
    crate::host_set(
        "host.intensityUnit",
        (options
            .as_ref()
            .and_then(|value| (value.intensity_unit).clone()))
        .unwrap_or((unitless_light_unit_constant).to_owned()),
    );
    crate::host_set("host.kind", hemisphere_light_kind_constant);
    crate::host_set(
        "host.skyColor",
        (options.as_ref().and_then(|value| value.sky_color)).unwrap_or(4294967295.0_f64),
    );
}
