// @generated from upstream/packages/easing/src/easeSmoothstep.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::ScalarRemap;

// Source: upstream/packages/easing/src/easeSmoothstep.ts:6 (sha256:a6b92c8f9e8f5e06f3ab799ed9364f4bcca4a31dfe7f8a18f1b23854eea635c2)
pub fn ease_smootherstep(t: f64) -> f64 {
    return (((t * t) * t) * ((t * ((t * 6.0_f64) - 15.0_f64)) + 10.0_f64));
}

// Source: upstream/packages/easing/src/easeSmoothstep.ts:9 (sha256:0225292e8faf9fba57ffe77c4b47ee3429a515303805de99d1b0199e6afc5079)
pub fn ease_smoothstep(t: f64) -> f64 {
    return ((t * t) * (3.0_f64 - (2.0_f64 * t)));
}

// Source: upstream/packages/easing/src/easeSmoothstep.ts:15 (sha256:c04bda7527c6948312842851fc16dab67a0ed67089471fec720e26e56aee96e6)
pub fn ease_smoothstep_range(edge0: f64, edge1: f64) -> ScalarRemap {
    return std::sync::Arc::new(move |x: f64| -> f64 {
        let t = (0.0_f64).max((1.0_f64).min(((x - edge0) / (edge1 - edge0))));
        return ((t * t) * (3.0_f64 - (2.0_f64 * t)));
    });
}
