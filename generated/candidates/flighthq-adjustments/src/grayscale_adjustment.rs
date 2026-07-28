// @generated from upstream/packages/adjustments/src/grayscaleAdjustment.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::GrayscaleAdjustment;

// Source: upstream/packages/adjustments/src/grayscaleAdjustment.ts:6 (sha256:c5def2b47e2ea5ec0351a431b3d66ef5a279c4f282694cb747b52790d7f73466)
#[derive(Clone)]
struct CreateGrayscaleAdjustmentRecord1 {
    __flight_identity: std::sync::Arc<()>,
}
impl PartialEq for CreateGrayscaleAdjustmentRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn create_grayscale_adjustment(options: Option<GrayscaleAdjustment>) -> GrayscaleAdjustment {
    let options = options.unwrap_or(GrayscaleAdjustment {
        __flight_identity: std::sync::Arc::new(()),
        intensity: None,
    });
    let intensity = (options.intensity).unwrap_or(1.0_f64);
    let k = intensity;
    let j = (1.0_f64 - intensity);
    let lr = (0.2126_f64 * k);
    let lg = (0.7152_f64 * k);
    let lb = (0.0722_f64 * k);
    let color_matrix = vec![
        (j + lr),
        lg,
        lb,
        0.0_f64,
        0.0_f64,
        lr,
        (j + lg),
        lb,
        0.0_f64,
        0.0_f64,
        lr,
        lg,
        (j + lb),
        0.0_f64,
        0.0_f64,
        0.0_f64,
        0.0_f64,
        0.0_f64,
        1.0_f64,
        0.0_f64,
    ];
    return GrayscaleAdjustment {
        kind: "GrayscaleAdjustment".to_owned(),
        color_matrix: (color_matrix).clone(),
        ..((options).clone()).clone()
    };
}
