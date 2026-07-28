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
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub on_cleared:
        Signal<std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>>,
    pub on_tile_changed: Signal<
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(f64, f64, f64) -> () + Send + 'static>>>,
    >,
    pub on_tiles_changed: Signal<
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(f64, f64, f64, f64) -> () + Send + 'static>>>,
    >,
}
impl PartialEq for TilemapSignals {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
