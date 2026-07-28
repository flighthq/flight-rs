// @generated from upstream/packages/types/src/SceneMetadata.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/SceneMetadata.ts:4 (sha256:3810ff78b354f1ff97baaf4c1518bebbea1f3df522afa50f5d449004054ad09f)
#[derive(Clone, Default)]
pub struct SceneMetadata {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub copyright: Option<String>,
    pub generator: Option<String>,
    pub version: Option<String>,
}
impl PartialEq for SceneMetadata {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
