// @generated from upstream/packages/types/src/ElectronBackendOptions.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/ElectronBackendOptions.ts:1 (sha256:db3740f898dc031be9a51b957b0d43dd9b25a6c7da0920c22fc13d8dfda3df75)
#[derive(Clone, Default)]
pub struct ElectronBackendOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub storage_file_name: Option<String>,
}
impl PartialEq for ElectronBackendOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
