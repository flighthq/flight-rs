// @generated from upstream/packages/types/src/WgpuRenderStateSignals.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::Signal;

// Source: upstream/packages/types/src/WgpuRenderStateSignals.ts:2 (sha256:c966119bf05e13b095e009b74d8b35ef8c17d7d33051531eddcd92efbd825349)
#[derive(Clone)]
pub struct WgpuRenderStateSignals {
    pub on_device_lost: Signal,
    pub on_context_resize: Signal,
}
