// @generated from upstream/packages/types/src/AttachmentSkin2D.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::Attachment2D;

// Source: upstream/packages/types/src/AttachmentSkin2D.ts:7 (sha256:5b923770aadf08c459c517186e28a6ca1a7ffcabd35a29d8d97ca742aa95996c)
#[derive(Clone, Default)]
pub struct SkinAttachment2D {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub attachment: Attachment2D,
    pub name: String,
    pub slot_index: f64,
}
impl PartialEq for SkinAttachment2D {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/AttachmentSkin2D.ts:27 (sha256:ee92865a82180e0ce674675c8bac3e1b141b827e87a26ad52b628cff1b27174c)
#[derive(Clone, Default)]
pub struct AttachmentSkin2D {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub attachments: Vec<SkinAttachment2D>,
    pub name: String,
}
impl PartialEq for AttachmentSkin2D {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
