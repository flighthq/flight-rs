// @generated from upstream/packages/types/src/ClockOptions.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/ClockOptions.ts:3 (sha256:787fb04080d6bb77b20fec61602689e57360183e1178b3fadc77aca4fece3c43)
#[derive(Clone, Default)]
pub struct ClockOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub scale: Option<f64>,
    pub paused: Option<bool>,
}
impl PartialEq for ClockOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
