// @generated from upstream/packages/types/src/ElectronHostOptions.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::DesktopOsProfile;

// Source: upstream/packages/types/src/ElectronHostOptions.ts:3 (sha256:b582db3dc510041ee1010838116813a68b99376aa7fabd0127d4bc4364fe33a9)
#[derive(Clone, Default)]
pub struct ElectronHostOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub platform: DesktopOsProfile,
    pub storage_file_name: Option<String>,
    pub updater_feed_url: Option<String>,
}
impl PartialEq for ElectronHostOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
