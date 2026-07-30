// @generated from upstream/packages/types/src/FontVariation.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/FontVariation.ts:1 (sha256:738762ab3d2c7b637084d12086d9c57c38bcc6ed282d5a88e4cc2e2b361652bf)
#[derive(Clone, Default)]
pub struct FontVariation {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub axis: String,
    pub value: f64,
}
impl PartialEq for FontVariation {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/FontVariation.ts:5 (sha256:b4a196722bd6c808d653575ad7fa7604a02157f7a6be540881126a6b759ab058)
pub const FONT_VARIATION_ITALIC: &'static str = "ital";

// Source: upstream/packages/types/src/FontVariation.ts:6 (sha256:3f09deac54b34880ef6ae0a0deb11810107f32abe44871903de9b5e15737472a)
pub const FONT_VARIATION_OPTICAL_SIZE: &'static str = "opsz";

// Source: upstream/packages/types/src/FontVariation.ts:7 (sha256:67db176b24b23127548212de5bcaa6d4d79c3b6904398b1dcb1f3a36a3b124a8)
pub const FONT_VARIATION_SLANT: &'static str = "slnt";

// Source: upstream/packages/types/src/FontVariation.ts:8 (sha256:fcaee04d6544ff341ffce740fd6d1ab5af2f06b3e861be32f1c69c9c339b99c0)
pub const FONT_VARIATION_WEIGHT: &'static str = "wght";

// Source: upstream/packages/types/src/FontVariation.ts:9 (sha256:383af8ecbf307dd2f139d583c95072fc25ad157af21ac36f5bc5b38e297abd84)
pub const FONT_VARIATION_WIDTH: &'static str = "wdth";
