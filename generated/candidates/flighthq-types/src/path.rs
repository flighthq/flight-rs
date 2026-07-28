// @generated from upstream/packages/types/src/Path.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::PathWinding;

// Source: upstream/packages/types/src/Path.ts:9 (sha256:960ebdfc83547ae67d1a8fd15067d051b34acf039c2bed0c979017e4a64f6658)
pub struct PathCommand;
impl PathCommand {
    pub const NO_OP: f64 = 0.0_f64;
    pub const MOVE_TO: f64 = 1.0_f64;
    pub const LINE_TO: f64 = 2.0_f64;
    pub const CURVE_TO: f64 = 3.0_f64;
    pub const WIDE_MOVE_TO: f64 = 4.0_f64;
    pub const WIDE_LINE_TO: f64 = 5.0_f64;
    pub const CUBIC_CURVE_TO: f64 = 6.0_f64;
    pub const CLOSE: f64 = 7.0_f64;
}

// Source: upstream/packages/types/src/Path.ts:20 (sha256:ee26808eff98759a4abeb57a3d14e181c1c047b5e2ca5f634a090e3ba16bd69b)
// TypeScript numeric namespace PathCommand is represented by its generated Rust constants.

// Source: upstream/packages/types/src/Path.ts:22 (sha256:9cbb0d15071e08ebc7e743943a37cb1426cb47860423d48eb0de4fd5729f9b81)
#[derive(Clone)]
pub struct Path {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub commands: Vec<f64>,
    pub data: Vec<f64>,
    pub winding: PathWinding,
}
impl PartialEq for Path {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
