// @generated from upstream/packages/types/src/WgpuVelocityWriter.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{VelocityField, WgpuRenderState};

// Source: upstream/packages/types/src/WgpuVelocityWriter.ts:11 (sha256:669927618ba45f10b871136f1943ea25c1a7d4367b9becd2dd02df1d475c5715)
#[derive(Clone, Default)]
pub struct WgpuVelocityContext {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub state: WgpuRenderState,
    pub field: VelocityField,
    pub width: f64,
    pub height: f64,
    pub pixel_ratio: f64,
}
impl PartialEq for WgpuVelocityContext {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/WgpuVelocityWriter.ts:19 (sha256:409c58bb5361cc3e9951ad9343db92dc57c630c933328d445d46377cc95d504d)
pub type WgpuVelocityWriter = std::sync::Arc<
    std::sync::Mutex<
        Box<dyn FnMut(WgpuVelocityContext, crate::OpaqueHostValue) -> () + Send + 'static>,
    >,
>;
