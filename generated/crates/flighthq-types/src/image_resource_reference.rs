// @generated from upstream/packages/types/src/ImageResourceReference.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{AlphaType, Image, ImageBitmapComposition, ResourceResolutionState, Texture};

// Source: upstream/packages/types/src/ImageResourceReference.ts:27 (sha256:76202cb11b2750ad09d0ed21fdfde2a01e9d6133603fda83294ff0cc4e3846ca)
#[derive(Clone, Default)]
pub struct ImageResourceReferenceKindValues {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub embedded: String,
    pub external: String,
}
impl PartialEq for ImageResourceReferenceKindValues {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub static IMAGE_RESOURCE_REFERENCE_KIND: std::sync::LazyLock<ImageResourceReferenceKindValues> =
    std::sync::LazyLock::new(|| ImageResourceReferenceKindValues {
        __flight_identity: std::sync::Arc::new(()),
        embedded: "Embedded".to_owned(),
        external: "External".to_owned(),
    });

// Source: upstream/packages/types/src/ImageResourceReference.ts:32 (sha256:34acad0284315327c3fd7b697c781dcf38d9a3a4a3568731d05921df7165a25b)
pub type ImageResourceReferenceKind = crate::OpaqueHostValue;

// Source: upstream/packages/types/src/ImageResourceReference.ts:34 (sha256:063d0fc61d13332353c38bdbdd601819e53bb35e42fecd5ebdf2c78b9187ec82)
#[derive(Clone, Default)]
pub struct ImageResourceFailureKindValues {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub error: String,
    pub unavailable: String,
}
impl PartialEq for ImageResourceFailureKindValues {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub static IMAGE_RESOURCE_FAILURE_KIND: std::sync::LazyLock<ImageResourceFailureKindValues> =
    std::sync::LazyLock::new(|| ImageResourceFailureKindValues {
        __flight_identity: std::sync::Arc::new(()),
        error: "Error".to_owned(),
        unavailable: "Unavailable".to_owned(),
    });

// Source: upstream/packages/types/src/ImageResourceReference.ts:39 (sha256:7b99275c8a6e542c4acd459174d87500a404da82570783708d07632e55b52165)
pub type ImageResourceFailureKind = crate::OpaqueHostValue;

// Source: upstream/packages/types/src/ImageResourceReference.ts:43 (sha256:4dca6d23eef328dc9578b362b0d2a4ee611c3117b13167419d06d836724816e4)
#[derive(Clone, Default)]
pub struct ImageResourceFailure {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub kind: ImageResourceFailureKind,
    pub message: String,
    pub name: Option<String>,
}
impl PartialEq for ImageResourceFailure {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/ImageResourceReference.ts:49 (sha256:fedbae0458baefc2b2336a5b2f125e683123a1de1f4c8172537048df76524478)
#[derive(Clone, Default)]
struct ImageResourceReferenceBase {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub failure: Option<ImageResourceFailure>,
    pub mime_type: Option<String>,
    pub state: ResourceResolutionState,
    pub textures: Option<Vec<Texture>>,
}
impl PartialEq for ImageResourceReferenceBase {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/ImageResourceReference.ts:65 (sha256:e6fea874381080cf306cac6592fb8a7e892e1f7338561dc6752dcad8f3ca2f3e)
#[derive(Clone, Default)]
pub struct EmbeddedImageResourceReference {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub failure: Option<ImageResourceFailure>,
    pub mime_type: Option<String>,
    pub state: ResourceResolutionState,
    pub textures: Option<Vec<Texture>>,
    pub kind: String,
    pub alpha_type: AlphaType,
    pub bitmap_composition: Option<ImageBitmapComposition>,
    pub bytes: Vec<u8>,
}
impl PartialEq for EmbeddedImageResourceReference {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/ImageResourceReference.ts:82 (sha256:83368ede5ddc7fcbcce66b5935c92b02fc0af38e3c69255c04e34fc7c8e6beda)
#[derive(Clone, Default)]
pub struct ExternalImageResourceReference {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub failure: Option<ImageResourceFailure>,
    pub mime_type: Option<String>,
    pub state: ResourceResolutionState,
    pub textures: Option<Vec<Texture>>,
    pub kind: String,
    pub uri: String,
    pub base_path: Option<String>,
}
impl PartialEq for ExternalImageResourceReference {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/ImageResourceReference.ts:88 (sha256:d30a4d473877bc76bcd788da01b1d5a65b692ca3bb97cab44bc96507d32f2f8b)
pub type ImageResourceReference =
    crate::FlightUnion2<EmbeddedImageResourceReference, ExternalImageResourceReference>;

// Source: upstream/packages/types/src/ImageResourceReference.ts:96 (sha256:c8ad6312b70bd84d18b43d83cd1e374f8d1bcd07856603d949c39952f8d62616)
pub type ImageResourceFetch = std::sync::Arc<
    std::sync::Mutex<
        Box<
            dyn FnMut(
                    ExternalImageResourceReference,
                    crate::OpaqueHostValue,
                ) -> crate::FlightTask<Option<Image>>
                + Send
                + 'static,
        >,
    >,
>;

// Source: upstream/packages/types/src/ImageResourceReference.ts:101 (sha256:2f662693ebd3799d15687c98c8fb4ca8b0a67f4c59966d0898f8b94d88020ac1)
#[derive(Clone, Default)]
pub struct ImageResourceReferenceResolutionExplanation {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub failure: Option<ImageResourceFailure>,
    pub kind: ImageResourceReferenceKind,
    pub retryable: bool,
    pub state: ResourceResolutionState,
}
impl PartialEq for ImageResourceReferenceResolutionExplanation {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
