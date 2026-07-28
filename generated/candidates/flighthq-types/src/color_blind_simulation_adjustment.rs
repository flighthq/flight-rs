// @generated from upstream/packages/types/src/ColorBlindSimulationAdjustment.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/ColorBlindSimulationAdjustment.ts:6 (sha256:a2960bbc2d69b424e26e023d3829d7a36122daf7aaafcf483fa30384caadf90e)
pub type ColorBlindType = String;

// Source: upstream/packages/types/src/ColorBlindSimulationAdjustment.ts:16 (sha256:cc26eba38e40df2c4d3158d516ad2524a08c987ec16f54f1c75b11d6d5209eef)
#[derive(Clone)]
pub struct ColorBlindSimulationAdjustment {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub kind: String,
    pub color_matrix: Vec<f64>,
    pub type_: Option<ColorBlindType>,
}
impl PartialEq for ColorBlindSimulationAdjustment {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
