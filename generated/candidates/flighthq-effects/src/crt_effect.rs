// @generated from upstream/packages/effects/src/crtEffect.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::CrtEffect;

// Source: upstream/packages/effects/src/crtEffect.ts:3 (sha256:45c15cf0b5f39d30b11eefbc49b6d4f93c662f10428e5a88b145a96a0f11dd34)
#[derive(Clone)]
struct CreateCrtEffectRecord1 {
    __flight_identity: std::sync::Arc<()>,
}
impl PartialEq for CreateCrtEffectRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn create_crt_effect(options: Option<CrtEffect>) -> CrtEffect {
    let options = options.unwrap_or(CrtEffect {
        __flight_identity: std::sync::Arc::new(()),
        curvature: None,
        scanline_intensity: None,
        vignette: None,
        aberration: None,
    });
    return CrtEffect {
        kind: "CrtEffect".to_owned(),
        ..((options).clone()).clone()
    };
}
