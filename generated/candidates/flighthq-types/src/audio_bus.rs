// @generated from upstream/packages/types/src/AudioBus.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/AudioBus.ts:1 (sha256:9f8283e71d3fb1b5347a20c656b6b51ff9382e92c0458d5a72626a811a67ff42)
#[derive(Clone)]
pub struct AudioBus {
    pub gain: f64,
    pub muted: bool,
    pub name: String,
    pub pan: f64,
}

// Source: upstream/packages/types/src/AudioBus.ts:7 (sha256:9210d61ab3b5c83f9cee9723c9836095f16b8e0b3ce41a73704d227c93b6dfe7)
#[derive(Clone)]
pub struct AudioBusOptions {
    pub gain: Option<f64>,
    pub muted: Option<bool>,
    pub name: Option<String>,
    pub pan: Option<f64>,
}

// Source: upstream/packages/types/src/AudioBus.ts:13 (sha256:7acb2b5e3d87026eb58835d1bf113471e0a26970a8374b702e764ec9c30dfa65)
#[derive(Clone)]
pub struct AudioMixer {
    pub master_gain: f64,
    pub master_muted: bool,
}

// Source: upstream/packages/types/src/AudioBus.ts:17 (sha256:35f7cc79617b727435ae8469587bc72f31b854a18d193482ab5ecc4bf433a64e)
#[derive(Clone)]
pub struct AudioMixerOptions {
    pub master_gain: Option<f64>,
    pub master_muted: Option<bool>,
}
