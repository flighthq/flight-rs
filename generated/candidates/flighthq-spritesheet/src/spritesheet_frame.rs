// @generated from upstream/packages/spritesheet/src/spritesheetFrame.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::SpritesheetFrame;

#[derive(Clone, Default)]
pub struct FlightPartialRecord3721705896 {
    pub __flight_identity: std::sync::Arc<()>,
    pub id: Option<f64>,
    pub offset_x: Option<f64>,
    pub offset_y: Option<f64>,
    pub pivot_x: Option<f64>,
    pub pivot_y: Option<f64>,
    pub rotated: Option<bool>,
}
impl PartialEq for FlightPartialRecord3721705896 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/spritesheet/src/spritesheetFrame.ts:3 (sha256:10e55620f60192ae87bac3c53615f221d2e81c348c8138a1763ca17768ecac74)
pub fn create_spritesheet_frame(obj: Option<FlightPartialRecord3721705896>) -> SpritesheetFrame {
    return SpritesheetFrame {
        __flight_identity: std::sync::Arc::new(()),
        id: (obj.as_ref().and_then(|value| value.id)).unwrap_or(0.0_f64),
        offset_x: (obj.as_ref().and_then(|value| value.offset_x)).unwrap_or(0.0_f64),
        offset_y: (obj.as_ref().and_then(|value| value.offset_y)).unwrap_or(0.0_f64),
        pivot_x: obj.as_ref().and_then(|value| value.pivot_x),
        pivot_y: obj.as_ref().and_then(|value| value.pivot_y),
        rotated: (obj.as_ref().and_then(|value| value.rotated)).unwrap_or(false),
    };
}
