// @generated from upstream/packages/types/src/WgpuRenderTexture.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{RenderTexture, WgpuRenderState, WgpuRenderTarget, WgpuRenderTargetPool};

// Source: upstream/packages/types/src/WgpuRenderTexture.ts:6 (sha256:2ec2ec6678145349a96ebcf33e25545566d1fcffc5017c52d13d89ac8178b393)
pub type WgpuRenderTextureStatus = String;

// Source: upstream/packages/types/src/WgpuRenderTexture.ts:8 (sha256:d8a2c37edbe1cd43d94d9db1777975309683b22d99e2e8638e1699b5976b05f8)
#[derive(Clone, Default)]
pub struct WgpuRenderTextureExplanation {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub height: f64,
    pub status: WgpuRenderTextureStatus,
    pub width: f64,
}
impl PartialEq for WgpuRenderTextureExplanation {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/WgpuRenderTexture.ts:15 (sha256:a5243909363d6d37ff704867c3df7bbceae877b0eac612d58b88af423e746572)
#[derive(Clone, Default)]
pub struct WgpuRenderTextureEntry {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub status: WgpuRenderTextureStatus,
    pub target: WgpuRenderTarget,
}
impl PartialEq for WgpuRenderTextureEntry {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/WgpuRenderTexture.ts:22 (sha256:8a2e66fa93ab54d34cd36ee7491780879d7f4f726c841200a9a168247ce7152c)
#[derive(Clone, Default)]
pub struct WgpuRenderTexturePool {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub device: Option<crate::OpaqueHostValue>,
    pub destroyed: bool,
    pub effect_targets: WgpuRenderTargetPool,
    pub free: Vec<RenderTexture>,
    pub leased: Vec<RenderTexture>,
}
impl PartialEq for WgpuRenderTexturePool {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/WgpuRenderTexture.ts:30 (sha256:1ff4489a9bfc7ddf6bcfe0bdf2650cfd22848ad67e613739bbafcae9bcd9aa27)
pub type WgpuRenderTextureGuard = std::sync::Arc<
    std::sync::Mutex<
        Box<
            dyn FnMut(WgpuRenderState, RenderTexture, WgpuRenderTextureExplanation) -> ()
                + Send
                + 'static,
        >,
    >,
>;
