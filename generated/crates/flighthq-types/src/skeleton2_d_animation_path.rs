// @generated from upstream/packages/types/src/Skeleton2DAnimationPath.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/Skeleton2DAnimationPath.ts:36 (sha256:27c5d2c784b7ff8751134a144ae560ffe3fd45c1a45ea6702c43ecd0be274b29)
#[derive(Clone, Default)]
pub struct Skeleton2DAnimationPathValues {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub rotation: String,
    pub scale: String,
    pub scale_x: String,
    pub scale_y: String,
    pub shear: String,
    pub shear_x: String,
    pub shear_y: String,
    pub translation: String,
    pub translation_x: String,
    pub translation_y: String,
}
impl PartialEq for Skeleton2DAnimationPathValues {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub static SKELETON2_D_ANIMATION_PATH: std::sync::LazyLock<Skeleton2DAnimationPathValues> =
    std::sync::LazyLock::new(|| Skeleton2DAnimationPathValues {
        __flight_identity: std::sync::Arc::new(()),
        translation: "Translation".to_owned(),
        translation_x: "TranslationX".to_owned(),
        translation_y: "TranslationY".to_owned(),
        rotation: "Rotation".to_owned(),
        scale: "Scale".to_owned(),
        scale_x: "ScaleX".to_owned(),
        scale_y: "ScaleY".to_owned(),
        shear: "Shear".to_owned(),
        shear_x: "ShearX".to_owned(),
        shear_y: "ShearY".to_owned(),
    });

// Source: upstream/packages/types/src/Skeleton2DAnimationPath.ts:49 (sha256:df7a3a26fb45827b834f461d662f6a8abac5a5540ac35215186247cdf281e990)
pub type Skeleton2DAnimationPath = crate::OpaqueHostValue;
