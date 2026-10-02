// @generated from upstream/packages/types/src/TexturePackerAtlasParseOptions.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/TexturePackerAtlasParseOptions.ts:1 (sha256:aed4c62771e9f426551823cb4abf100becb1abafc0cba7cc6990e095950b9315)
#[derive(Clone, Default)]
pub struct TexturePackerAtlasParseOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub strip_path_prefix: Option<bool>,
}
impl PartialEq for TexturePackerAtlasParseOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
