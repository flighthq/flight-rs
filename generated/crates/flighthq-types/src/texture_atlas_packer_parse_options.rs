// @generated from upstream/packages/types/src/TextureAtlasPackerParseOptions.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/TextureAtlasPackerParseOptions.ts:1 (sha256:1be5adb06eca1d693db5886cf34c1358c86298097fc1de5452c9df16d41ba319)
#[derive(Clone, Default)]
pub struct TextureAtlasPackerParseOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub strip_path_prefix: Option<bool>,
}
impl PartialEq for TextureAtlasPackerParseOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
