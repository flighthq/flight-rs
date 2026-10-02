// @generated from upstream/packages/types/src/HostBitmapReadback.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{Bitmap, BitmapReadbackBlockReason, HostImageSource};

// Source: upstream/packages/types/src/HostBitmapReadback.ts:7 (sha256:c13ea723eec48135bbdaac8997736b62f75b31a49bf0f730e96b0950c8ceebaa)
pub type BitmapReadbackMode = String;

// Source: upstream/packages/types/src/HostBitmapReadback.ts:9 (sha256:91c6e9e6aa8237c6825f551d97667ad5cf884fa664fd22ed6ce872a2a45fa2e9)
pub type BitmapReadbackBackendReason = BitmapReadbackBlockReason;

// Source: upstream/packages/types/src/HostBitmapReadback.ts:11 (sha256:7a36339399ddbb130f968fee5ccf98b5a007e7a6ae08288e2f1ef5c1940f84f9)
#[derive(Clone, Default)]
pub struct BitmapReadbackOutcome {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub bitmap: Option<Bitmap>,
    pub reason: BitmapReadbackBackendReason,
}
impl PartialEq for BitmapReadbackOutcome {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/HostBitmapReadback.ts:16 (sha256:032c734d21948098dcb83b9f461255134ef590fe35756f9e283cf78261ece38c)
#[derive(Clone)]
pub struct HostBitmapReadbackCapability {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub read_bitmap: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(HostImageSource, f64, f64, BitmapReadbackMode) -> BitmapReadbackOutcome
                    + Send
                    + 'static,
            >,
        >,
    >,
}
impl PartialEq for HostBitmapReadbackCapability {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
