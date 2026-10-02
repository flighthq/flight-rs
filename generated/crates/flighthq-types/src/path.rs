// @generated from upstream/packages/types/src/Path.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{EntityRuntime, PathWinding};

// Source: upstream/packages/types/src/Path.ts:11 (sha256:275a20fa9ba3d4965c632c9fe6d4d0f6b42bcf35b6c88950fccd1aff0a1a3e29)
pub struct PathCommand;
impl PathCommand {
    pub const NO_OP: f64 = 0.0_f64;
    pub const MOVE_TO: f64 = 1.0_f64;
    pub const LINE_TO: f64 = 2.0_f64;
    pub const QUADRATIC_CURVE_TO: f64 = 3.0_f64;
    pub const WIDE_MOVE_TO: f64 = 4.0_f64;
    pub const WIDE_LINE_TO: f64 = 5.0_f64;
    pub const CUBIC_CURVE_TO: f64 = 6.0_f64;
    pub const CLOSE: f64 = 7.0_f64;
}

// Source: upstream/packages/types/src/Path.ts:22 (sha256:ee26808eff98759a4abeb57a3d14e181c1c047b5e2ca5f634a090e3ba16bd69b)
// TypeScript numeric namespace PathCommand is represented by its generated Rust constants.

// Source: upstream/packages/types/src/Path.ts:24 (sha256:386dcd084dcb346f972fc537a54c540f7f9170c167e580823cbe50b1655135f3)
#[derive(Clone, Default)]
pub struct Path {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    #[doc(hidden)]
    pub __flight_entity_snapshot: Option<std::sync::Arc<dyn std::any::Any + Send + Sync>>,
    pub commands: Vec<f64>,
    pub data: Vec<f64>,
    pub winding: PathWinding,
}
impl PartialEq for Path {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for Path {
    fn __flight_entity_runtime(
        &self,
    ) -> &std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>> {
        &self.__flight_entity_runtime
    }
    fn __flight_entity_snapshot(&self) -> &Option<std::sync::Arc<dyn std::any::Any + Send + Sync>> {
        &self.__flight_entity_snapshot
    }
    fn __flight_fresh_clone(&self) -> Self {
        let mut cloned = self.clone();
        cloned.__flight_identity = std::sync::Arc::new(());
        cloned.__flight_entity_runtime = std::sync::Arc::new(std::sync::Mutex::new(
            self.__flight_entity_runtime.lock().unwrap().clone(),
        ));
        cloned
    }
}
