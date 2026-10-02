// @generated from upstream/packages/types/src/HostImageDimensions.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::HostImageSource;

// Source: upstream/packages/types/src/HostImageDimensions.ts:4 (sha256:2efc261a95e6c65cf59f49eec830d8ca385659c18cb54b212a7f995cdd2df801)
#[derive(Clone, Default)]
pub struct HostImageDimensions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub height: f64,
    pub width: f64,
}
impl PartialEq for HostImageDimensions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/HostImageDimensions.ts:13 (sha256:085038717055389552d2c6e00e09f90d225cfff88d74a6f736ec89e407c704ac)
pub type HostImageDimensionResolver = std::sync::Arc<
    std::sync::Mutex<Box<dyn FnMut(HostImageSource, HostImageDimensions) -> bool + Send + 'static>>,
>;
