// @generated from upstream/packages/spring/src/springConfig.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_math::TAU as tau_constant;
use flighthq_types::SpringConfig;

// Source: upstream/packages/spring/src/springConfig.ts:7 (sha256:a148d196e2fc6801132f1329aca2ae6964067fdec64ab14ecaa68d6c580fd67d)
pub fn create_spring_config(frequency: f64, damping_ratio: f64) -> SpringConfig {
    return SpringConfig {
        __flight_identity: std::sync::Arc::new(()),
        damping_ratio: damping_ratio,
        frequency: frequency,
    };
}

// Source: upstream/packages/spring/src/springConfig.ts:15 (sha256:c879c318f9fe2b0d70283e4fc50e7cebe3b3c5981567e2f59bc15552a3d1c4bd)
pub fn create_spring_config_from_physical(stiffness: f64, damping: f64, mass: f64) -> SpringConfig {
    return SpringConfig {
        __flight_identity: std::sync::Arc::new(()),
        damping_ratio: (damping / (2.0_f64 * (stiffness * mass).sqrt())),
        frequency: ((stiffness / mass).sqrt() / tau_constant),
    };
}
