// @generated from upstream/packages/types/src/ImageEncoder.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{DecodedImage, ImageEncodeOptions};

// Source: upstream/packages/types/src/ImageEncoder.ts:7 (sha256:81c3b077ea9ec65ff9383839d556ff308eefdbf5eeb43a738ebe00f7ac000c22)
pub type ImageEncoder = std::sync::Arc<
    std::sync::Mutex<
        Box<
            dyn FnMut(DecodedImage, Option<ImageEncodeOptions>) -> crate::Promise<Vec<u8>>
                + Send
                + 'static,
        >,
    >,
>;
