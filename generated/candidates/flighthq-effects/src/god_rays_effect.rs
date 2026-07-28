// @generated from upstream/packages/effects/src/godRaysEffect.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::GodRaysEffect;

// Source: upstream/packages/effects/src/godRaysEffect.ts:3 (sha256:1f649a2052d40e3944766a2f2827a6fdfb1db84b877ce1bad8b1be86e206b95b)
#[derive(Clone)]
struct CreateGodRaysEffectRecord1 {
    __flight_identity: std::sync::Arc<()>,
}
impl PartialEq for CreateGodRaysEffectRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn create_god_rays_effect(options: Option<GodRaysEffect>) -> GodRaysEffect {
    let options = options.unwrap_or(GodRaysEffect {
        __flight_identity: std::sync::Arc::new(()),
        center_x: None,
        center_y: None,
        density: None,
        decay: None,
        weight: None,
        exposure: None,
        samples: None,
    });
    return GodRaysEffect {
        kind: "GodRaysEffect".to_owned(),
        ..((options).clone()).clone()
    };
}
