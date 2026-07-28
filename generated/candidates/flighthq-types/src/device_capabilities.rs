// @generated from upstream/packages/types/src/DeviceCapabilities.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/DeviceCapabilities.ts:4 (sha256:feb134ddf3519fe7ed01774dd8bc12ec3f7521c5e507b4cf93efb4bc88b56ee0)
#[derive(Clone)]
pub struct DeviceCapabilities {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub has_keyboard: bool,
    pub has_mouse: bool,
    pub has_stylus: bool,
}
impl PartialEq for DeviceCapabilities {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
