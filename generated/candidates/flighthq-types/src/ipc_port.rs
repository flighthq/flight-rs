// @generated from upstream/packages/types/src/IpcPort.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/IpcPort.ts:10 (sha256:eb686338f579e0999842a3055f14260281d1ab06fbd0af5ae1b1108cfcccec02)
#[derive(Clone, Default)]
pub struct IpcPort {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub _port_id: f64,
}
impl PartialEq for IpcPort {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
