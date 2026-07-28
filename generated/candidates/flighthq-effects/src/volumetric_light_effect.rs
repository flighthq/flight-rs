// @generated from upstream/packages/effects/src/volumetricLightEffect.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::VolumetricLightEffect;

// Source: upstream/packages/effects/src/volumetricLightEffect.ts:3 (sha256:169af6783f03baa47eb6fa38ca00da245644f336396e7b6809f9969533e0bafd)
#[derive(Clone)]
struct CreateVolumetricLightEffectRecord1 {
    __flight_identity: std::sync::Arc<()>,
}
impl PartialEq for CreateVolumetricLightEffectRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn create_volumetric_light_effect(
    options: Option<VolumetricLightEffect>,
) -> VolumetricLightEffect {
    let options = options.unwrap_or(VolumetricLightEffect {
        __flight_identity: std::sync::Arc::new(()),
        density: None,
        light_color: None,
        light_x: None,
        light_y: None,
        samples: None,
        scattering: None,
    });
    return VolumetricLightEffect {
        kind: "VolumetricLightEffect".to_owned(),
        ..((options).clone()).clone()
    };
}
