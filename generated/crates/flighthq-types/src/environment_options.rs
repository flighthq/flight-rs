// @generated from upstream/packages/types/src/EnvironmentOptions.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::Texture;

// Source: upstream/packages/types/src/EnvironmentOptions.ts:3 (sha256:940271c9a05acd493328d3b9af9bf9dbd26a76fc3e6e9424d8d9f91767e57a43)
#[derive(Clone, Default)]
pub struct EnvironmentOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub environment: Option<Texture>,
    pub intensity: Option<f64>,
}
impl PartialEq for EnvironmentOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
