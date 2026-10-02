// @generated from upstream/packages/lighting/src/environment.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_entity::{allocate_entity, finish_entity};
use flighthq_types::{
    ENVIRONMENT_KIND as environment_kind_constant, EntityConstruction, Environment,
    EnvironmentOptions,
};

// Source: upstream/packages/lighting/src/environment.ts:7 (sha256:11abed9bfe4f79853669e0466c8425d3df4b6f0678ca91d83aceb2159d67b203)
pub fn clone_environment(source: &Environment) -> Environment {
    return create_environment(Some(EnvironmentOptions {
        __flight_identity: std::sync::Arc::new(()),
        enabled: Some(source.enabled),
        environment: (source.environment).clone(),
        intensity: Some(source.intensity),
    }));
}

// Source: upstream/packages/lighting/src/environment.ts:11 (sha256:1708cc7c10026590bdcbc8b1a15c15f9cba92e04eeba802f65f19764b484cb62)
pub fn create_environment(options: Option<EnvironmentOptions>) -> Environment {
    let mut out = allocate_entity();
    initialize_environment((out).clone(), ((options).clone()).clone());
    return finish_entity((out).clone());
}

// Source: upstream/packages/lighting/src/environment.ts:20 (sha256:afd3418a2df4891bf0bb562cf83a53a7990d2a42d3d5a7a0512d2a35122ccd14)
pub fn initialize_environment(
    out: EntityConstruction<Environment>,
    options: Option<EnvironmentOptions>,
) -> () {
    crate::host_set(
        "host.enabled",
        (options.as_ref().and_then(|value| value.enabled)).unwrap_or(true),
    );
    crate::host_set(
        "host.environment",
        options
            .as_ref()
            .and_then(|value| (value.environment).clone()),
    );
    crate::host_set(
        "host.intensity",
        (options.as_ref().and_then(|value| value.intensity)).unwrap_or(1.0_f64),
    );
    crate::host_set("host.kind", environment_kind_constant);
}
