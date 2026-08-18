// @generated from upstream/packages/scene3d-gl/src/explainGlScene3DForwardLightSelection.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::{
    GlScene3DForwardLightList, MAX_FORWARD_LIGHTS as max_forward_lights_constant,
    Scene3DForwardLightSelectionExplanation, Scene3DLightsLike,
};

// Source: upstream/packages/scene3d-gl/src/explainGlScene3DForwardLightSelection.ts:10 (sha256:1f53901d289dc6473e7c31324008563abf9ac568c0ed89bd51de3c69d69e1428)
pub fn explain_gl_scene3_d_forward_light_selection(
    lights: &Scene3DLightsLike,
    selection: Option<GlScene3DForwardLightList>,
) -> Scene3DForwardLightSelectionExplanation {
    let point_light_count =
        (lights.point.as_ref().map(|value| value.len() as f64)).unwrap_or(0.0_f64);
    let spot_light_count =
        (lights.spot.as_ref().map(|value| value.len() as f64)).unwrap_or(0.0_f64);
    let selection_prepared = (selection).is_some();
    return Scene3DForwardLightSelectionExplanation {
        __flight_identity: std::sync::Arc::new(()),
        point_light_count: point_light_count,
        reason: if selection_prepared {
            "selection-prepared".to_owned()
        } else {
            if (point_light_count > max_forward_lights_constant)
                || (spot_light_count > max_forward_lights_constant)
            {
                "selection-required".to_owned()
            } else {
                "within-budget".to_owned()
            }
        },
        selection_prepared: selection_prepared,
        spot_light_count: spot_light_count,
    };
}
