// @generated from upstream/packages/types/src/WgpuEffectPipeline.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::WgpuEffectBlendMode;

// Source: upstream/packages/types/src/WgpuEffectPipeline.ts:3 (sha256:0def27f503d792f5c38b473b5e9fbcfc235b9945fc28b8b0a2b972ab472f6d4c)
#[derive(Clone, Default)]
pub struct WgpuEffectPipeline {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub pipeline: crate::OpaqueHostValue,
    pub blend_mode: WgpuEffectBlendMode,
    pub compile_for_format: Option<
        std::sync::Arc<
            std::sync::Mutex<
                Box<dyn FnMut(crate::OpaqueHostValue) -> crate::OpaqueHostValue + Send + 'static>,
            >,
        >,
    >,
    pub variants: Option<Vec<(crate::OpaqueHostValue, crate::OpaqueHostValue)>>,
}
impl PartialEq for WgpuEffectPipeline {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
