// @generated from upstream/packages/types/src/RiveImportOptions.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{PathBooleanKernel, RiveImportRegistry};

// Source: upstream/packages/types/src/RiveImportOptions.ts:8 (sha256:9aec3b59a37145b560efe7a16f71119e11936d4229074963cb33c907325459e3)
pub type RiveRegistrar =
    std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(RiveImportRegistry) -> () + Send + 'static>>>;

// Source: upstream/packages/types/src/RiveImportOptions.ts:18 (sha256:2ded9db1028715796d004327a2201bddd8e7ceb5b25e7079e56fd6b1f92580d9)
pub type RivePathBooleanRegistrar = std::sync::Arc<
    std::sync::Mutex<Box<dyn FnMut(PathBooleanKernel, RiveImportRegistry) -> () + Send + 'static>>,
>;

// Source: upstream/packages/types/src/RiveImportOptions.ts:40 (sha256:38646d8edff554e8ad44624e96cfee39363ec5c37467da2dedfa205011c455b9)
#[derive(Clone, Default)]
pub struct RiveImportOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub path_boolean_registrars: Option<Vec<RivePathBooleanRegistrar>>,
    pub registrars: Option<Vec<RiveRegistrar>>,
}
impl PartialEq for RiveImportOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
