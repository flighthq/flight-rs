// @generated from upstream/packages/types/src/Skeleton2DPathConstraint.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::Skeleton2DConstraintKind;

// Source: upstream/packages/types/src/Skeleton2DPathConstraint.ts:19 (sha256:3831dc7503c830297819df165667142b18595623fb5320f616539f5dbb48b1bd)
#[derive(Clone, Default)]
pub struct Skeleton2DPathConstraint {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub kind: Skeleton2DConstraintKind,
    pub mix: f64,
    pub bone_indices: Vec<f64>,
    pub mix_rotate: f64,
    pub mix_x: f64,
    pub mix_y: f64,
    pub position: f64,
    pub position_mode: Skeleton2DPathPositionMode,
    pub rotate_mode: Skeleton2DPathRotateMode,
    pub spacing: f64,
    pub spacing_mode: Skeleton2DPathSpacingMode,
    pub target_slot_index: f64,
}
impl PartialEq for Skeleton2DPathConstraint {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Skeleton2DPathConstraint.ts:39 (sha256:572f2a50d46db86da9e0deb3d92bed9d04e10676b0768626f6cfd21040eab893)
#[derive(Clone, Default)]
pub struct Skeleton2DPathPositionModeValues {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub fixed: String,
    pub percent: String,
}
impl PartialEq for Skeleton2DPathPositionModeValues {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub static SKELETON2_D_PATH_POSITION_MODE: std::sync::LazyLock<Skeleton2DPathPositionModeValues> =
    std::sync::LazyLock::new(|| Skeleton2DPathPositionModeValues {
        __flight_identity: std::sync::Arc::new(()),
        fixed: "Fixed".to_owned(),
        percent: "Percent".to_owned(),
    });

// Source: upstream/packages/types/src/Skeleton2DPathConstraint.ts:44 (sha256:4e4e1754f149587428b70762a7b51ca6e991b2fc2f9cadfcbee300948c503fdc)
pub type Skeleton2DPathPositionMode = crate::OpaqueHostValue;

// Source: upstream/packages/types/src/Skeleton2DPathConstraint.ts:46 (sha256:5a3c49661c431060b3288cc7e75ebc7df167b850765a4c8228a7808f8cc2dd85)
#[derive(Clone, Default)]
pub struct Skeleton2DPathRotateModeValues {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub chain: String,
    pub tangent: String,
}
impl PartialEq for Skeleton2DPathRotateModeValues {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub static SKELETON2_D_PATH_ROTATE_MODE: std::sync::LazyLock<Skeleton2DPathRotateModeValues> =
    std::sync::LazyLock::new(|| Skeleton2DPathRotateModeValues {
        __flight_identity: std::sync::Arc::new(()),
        chain: "Chain".to_owned(),
        tangent: "Tangent".to_owned(),
    });

// Source: upstream/packages/types/src/Skeleton2DPathConstraint.ts:51 (sha256:dd32036f5f711a2629a91f992cbf6c9320e1e8e55007bc32010cb077bf5782a8)
pub type Skeleton2DPathRotateMode = crate::OpaqueHostValue;

// Source: upstream/packages/types/src/Skeleton2DPathConstraint.ts:53 (sha256:196e3b45765116f196e867f1b7a70123161d789dc1d2e056b849950923ad3776)
#[derive(Clone, Default)]
pub struct Skeleton2DPathSpacingModeValues {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub fixed: String,
    pub length: String,
    pub percent: String,
}
impl PartialEq for Skeleton2DPathSpacingModeValues {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub static SKELETON2_D_PATH_SPACING_MODE: std::sync::LazyLock<Skeleton2DPathSpacingModeValues> =
    std::sync::LazyLock::new(|| Skeleton2DPathSpacingModeValues {
        __flight_identity: std::sync::Arc::new(()),
        fixed: "Fixed".to_owned(),
        length: "Length".to_owned(),
        percent: "Percent".to_owned(),
    });

// Source: upstream/packages/types/src/Skeleton2DPathConstraint.ts:59 (sha256:8d18ffbb34304e1f79152661900b900f85eda3ac5ea86ca9c7360542e4686113)
pub type Skeleton2DPathSpacingMode = crate::OpaqueHostValue;
