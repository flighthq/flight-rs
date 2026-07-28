// @generated from upstream/packages/types/src/TilemapSignals.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::Signal;

// Source: upstream/packages/types/src/TilemapSignals.ts:3 (sha256:27106cd4450b0c994aa8c9c764d46c27bd47d4208ae31cad2c1dd206bcd34bb8)
#[derive(Clone)]
pub struct TilemapSignals {
    pub on_cleared: Signal,
    pub on_tile_changed: Signal,
    pub on_tiles_changed: Signal,
}
