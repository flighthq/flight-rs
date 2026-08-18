// @generated from upstream/packages/types/src/TransformMode2D.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/TransformMode2D.ts:16 (sha256:92216c7bb5ba79a01ce20433f1a6f7faf39bf5f71d8b9fc513620ade0ee5d721)
#[derive(Clone, Default)]
pub struct TransformMode2DRecord1 {
    pub __flight_identity: std::sync::Arc<()>,
    pub reflection: bool,
    pub rotation: bool,
    pub scale: bool,
    pub translation: bool,
}
impl PartialEq for TransformMode2DRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct TransformMode2D {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub normal: TransformMode2DRecord1,
    pub no_rotation_or_reflection: TransformMode2DRecord1,
    pub no_scale: TransformMode2DRecord1,
    pub no_scale_or_reflection: TransformMode2DRecord1,
    pub only_translation: TransformMode2DRecord1,
}
impl PartialEq for TransformMode2D {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub static TRANSFORM_MODE2_D: std::sync::LazyLock<TransformMode2D> =
    std::sync::LazyLock::new(|| TransformMode2D {
        __flight_identity: std::sync::Arc::new(()),
        normal: TransformMode2DRecord1 {
            __flight_identity: std::sync::Arc::new(()),
            reflection: true,
            rotation: true,
            scale: true,
            translation: true,
        },
        only_translation: TransformMode2DRecord1 {
            __flight_identity: std::sync::Arc::new(()),
            reflection: false,
            rotation: false,
            scale: false,
            translation: true,
        },
        no_rotation_or_reflection: TransformMode2DRecord1 {
            __flight_identity: std::sync::Arc::new(()),
            reflection: false,
            rotation: false,
            scale: true,
            translation: true,
        },
        no_scale: TransformMode2DRecord1 {
            __flight_identity: std::sync::Arc::new(()),
            reflection: true,
            rotation: true,
            scale: false,
            translation: true,
        },
        no_scale_or_reflection: TransformMode2DRecord1 {
            __flight_identity: std::sync::Arc::new(()),
            reflection: false,
            rotation: true,
            scale: false,
            translation: true,
        },
    });
