// @generated from upstream/packages/types/src/TextureAtlasParseOptions.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/TextureAtlasParseOptions.ts:11 (sha256:e937b283d7eb6e2417cf75fe49ab8bb6b16b1d67a7064d11fe0b21f4f6d2721f)
#[derive(Clone, Default)]
pub struct TextureAtlasParseOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub strip_path_prefix: Option<bool>,
    pub image_width: Option<f64>,
    pub image_height: Option<f64>,
}
impl PartialEq for TextureAtlasParseOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
