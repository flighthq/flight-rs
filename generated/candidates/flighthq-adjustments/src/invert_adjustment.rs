// @generated from upstream/packages/adjustments/src/invertAdjustment.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::InvertAdjustment;

// Source: upstream/packages/adjustments/src/invertAdjustment.ts:6 (sha256:d16be5a43825d22ec95f513b635f5e8a8f26441525c5b2aee10ed8895edb2ac5)
#[derive(Clone)]
struct CreateInvertAdjustmentRecord1 {
    __flight_identity: std::sync::Arc<()>,
}
impl PartialEq for CreateInvertAdjustmentRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn create_invert_adjustment(options: Option<InvertAdjustment>) -> InvertAdjustment {
    let options = options.unwrap_or(InvertAdjustment {
        __flight_identity: std::sync::Arc::new(()),
        intensity: None,
    });
    let intensity = (options.intensity).unwrap_or(1.0_f64);
    let s = (1.0_f64 - (2.0_f64 * intensity));
    let o = (255.0_f64 * intensity);
    let color_matrix = vec![
        s, 0.0_f64, 0.0_f64, 0.0_f64, o, 0.0_f64, s, 0.0_f64, 0.0_f64, o, 0.0_f64, 0.0_f64, s,
        0.0_f64, o, 0.0_f64, 0.0_f64, 0.0_f64, 1.0_f64, 0.0_f64,
    ];
    return InvertAdjustment {
        kind: "InvertAdjustment".to_owned(),
        color_matrix: (color_matrix).clone(),
        ..((options).clone()).clone()
    };
}
