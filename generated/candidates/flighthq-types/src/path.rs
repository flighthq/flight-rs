// @generated from upstream/packages/types/src/Path.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::PathWinding;

// Source: upstream/packages/types/src/Path.ts:9 (sha256:960ebdfc83547ae67d1a8fd15067d051b34acf039c2bed0c979017e4a64f6658)
// TypeScript value namespace PathCommand is represented by its generated Rust type.

// Source: upstream/packages/types/src/Path.ts:20 (sha256:ee26808eff98759a4abeb57a3d14e181c1c047b5e2ca5f634a090e3ba16bd69b)
pub type PathCommand = crate::OpaqueHostValue;

// Source: upstream/packages/types/src/Path.ts:22 (sha256:9cbb0d15071e08ebc7e743943a37cb1426cb47860423d48eb0de4fd5729f9b81)
#[derive(Clone)]
pub struct Path {
    pub commands: Vec<f64>,
    pub data: Vec<f64>,
    pub winding: PathWinding,
}
