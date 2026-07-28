// @generated from upstream/packages/types/src/Platform.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/Platform.ts:8 (sha256:a04ed6cec006a086a4d2d32f5db1c3f1d642b28d54d6d8ba609bb912bc9791c9)
pub type PlatformName = String;

// Source: upstream/packages/types/src/Platform.ts:11 (sha256:df06c4e36f45ac446a2f32a4810fd8c234d807a5f335568488f7a99f6e31b942)
pub type PlatformEndianness = String;

// Source: upstream/packages/types/src/Platform.ts:14 (sha256:3d95d4c6e2bfe44636f154e6348414c571495d2a23dce6577d1522f1c262f95e)
pub type PlatformEngine = String;

// Source: upstream/packages/types/src/Platform.ts:16 (sha256:82610121fe0e13d70e44e152b6a712ed0a26a6f86a7e2b07a96baacc3353b815)
pub type PlatformKind = String;

// Source: upstream/packages/types/src/Platform.ts:20 (sha256:28a791deb1179319a937bc7fa7b2a28f774d50b4a0ee34417f1d2a710a0bc749)
pub type PlatformRuntime = String;

// Source: upstream/packages/types/src/Platform.ts:22 (sha256:222171b0e13d4200b4ea0330662c0887e8c96736d29b9deb0660ca0ac11e9d9f)
#[derive(Clone)]
pub struct PlatformInfo {
    pub name: PlatformName,
    pub kind: PlatformKind,
    pub version: String,
    pub arch: String,
    pub locale: String,
    pub is_touch: bool,
    pub runtime: PlatformRuntime,
    pub engine: PlatformEngine,
    pub engine_version: String,
    pub endianness: PlatformEndianness,
    pub pointer_width: f64,
    pub os_build: String,
    pub distro: String,
    pub distro_version: String,
}

// Source: upstream/packages/types/src/Platform.ts:53 (sha256:4bfcc4232d8ccaaa9b9ddf32789c50d0d31ca62a52b5305151feec8b6f1b36eb)
#[derive(Clone)]
pub struct PlatformBackend {
    pub get_info: crate::OpaqueHostValue,
}
