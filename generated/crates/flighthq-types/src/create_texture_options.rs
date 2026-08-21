// @generated from upstream/packages/types/src/CreateTextureOptions.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::ImageResourceReference;

// Source: upstream/packages/types/src/CreateTextureOptions.ts:4 (sha256:d0ecbdda01016d281c0442a71e2d43a5c1664ab8e7a5b33a31ff3b38d3434003)
pub(crate) type CreateTextureVariantOptions = crate::OpaqueHostValue;

// Source: upstream/packages/types/src/CreateTextureOptions.ts:9 (sha256:df1e25f3faa5bf43fdc255628f5486a2f82d5155ce8b45e4be4d82ead392df8a)
#[derive(Clone, Default)]
pub struct CreateTextureOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub resource: Option<ImageResourceReference>,
}
impl PartialEq for CreateTextureOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/CreateTextureOptions.ts:17 (sha256:c5ac4dad09322cb49bb960424d2e180f10c87ab312249f15fafdecb0e2f4f268)
pub type CreateTexture2DOptions = CreateTextureOptions;
