// @generated from upstream/packages/lighting/src/ambientLight.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_entity::{allocate_entity, finish_entity};
use flighthq_types::{
    AMBIENT_LIGHT_KIND as ambient_light_kind_constant, AmbientLight, AmbientLightOptions,
    EntityConstruction, UNITLESS_LIGHT_UNIT as unitless_light_unit_constant,
};

// Source: upstream/packages/lighting/src/ambientLight.ts:6 (sha256:2cbc2e1d7ccfc39d7b5e970e5f222c74b489a961061de3276016b341069f976d)
pub fn clone_ambient_light(source: &AmbientLight) -> AmbientLight {
    return create_ambient_light(Some(AmbientLightOptions {
        __flight_identity: std::sync::Arc::new(()),
        color: Some(source.color),
        enabled: Some(source.enabled),
        intensity: Some(source.intensity),
        intensity_unit: Some((source.intensity_unit).clone()),
    }));
}

// Source: upstream/packages/lighting/src/ambientLight.ts:15 (sha256:1980f4b6c69f6705ba9b694e34bf948ddaab06248f68390929d1983d6db4dfa0)
pub fn create_ambient_light(options: Option<AmbientLightOptions>) -> AmbientLight {
    let mut out = allocate_entity();
    initialize_ambient_light((out).clone(), ((options).clone()).clone());
    return finish_entity((out).clone());
}

// Source: upstream/packages/lighting/src/ambientLight.ts:23 (sha256:7df6ea753e191d76686524e44cac1c141cea537451fb27f8e94d6afa1c24532e)
pub fn initialize_ambient_light(
    out: EntityConstruction<AmbientLight>,
    options: Option<AmbientLightOptions>,
) -> () {
    crate::host_set(
        "host.color",
        (options.as_ref().and_then(|value| value.color)).unwrap_or(4294967295.0_f64),
    );
    crate::host_set(
        "host.enabled",
        (options.as_ref().and_then(|value| value.enabled)).unwrap_or(true),
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
    crate::host_set("host.kind", ambient_light_kind_constant);
}
