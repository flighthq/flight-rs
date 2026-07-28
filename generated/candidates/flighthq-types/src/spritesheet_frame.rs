// @generated from upstream/packages/types/src/SpritesheetFrame.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/SpritesheetFrame.ts:1 (sha256:4f827eaaaaaca75e8a9b87aa72d34e221739ae8994eeb32f74bf5b0018147a67)
#[derive(Clone)]
pub struct SpritesheetFrame {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub id: f64,
    pub offset_x: f64,
    pub offset_y: f64,
    pub pivot_x: Option<f64>,
    pub pivot_y: Option<f64>,
    pub rotated: bool,
}
impl PartialEq for SpritesheetFrame {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
