// @generated from upstream/packages/types/src/MorphShape.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{
    BlendMode, ClipRegion, EntityRuntime, Kind, Material, MaterialData, Matrix, Path, PathMorph,
    ShapeCommandToken,
};

// Source: upstream/packages/types/src/MorphShape.ts:6 (sha256:0f9f71ff1557793611652434f7be0c4c1277647dc9314a7ebeb461cb0b163816)
#[derive(Clone, Default)]
pub struct MorphShapePathBinding {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub morph: PathMorph,
    pub path: Path,
}
impl PartialEq for MorphShapePathBinding {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/MorphShape.ts:11 (sha256:7b64793cbbbbba919e192888fc21a8521ab2e265520986d959157929a2d9b0a3)
#[derive(Clone, Default)]
pub struct MorphShapeColorEndpoint {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub alpha: Option<f64>,
    pub color: f64,
}
impl PartialEq for MorphShapeColorEndpoint {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/MorphShape.ts:17 (sha256:f5a125c830ec328239b3260b832a2d808ac31e1e0735b68100978abf63435bde)
#[derive(Clone, Default)]
pub struct MorphShapeGradientEndpoint {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub alphas: Vec<f64>,
    pub colors: Vec<f64>,
    pub focal_point_ratio: Option<f64>,
    pub matrix: Option<Matrix>,
    pub ratios: Vec<f64>,
}
impl PartialEq for MorphShapeGradientEndpoint {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/MorphShape.ts:26 (sha256:9d2cd07cf8c6818d54a0ba8350226da2a67d4ec2833d7c3ce798ba6483b825a8)
#[derive(Clone, Default)]
pub struct MorphShapeGradientEndpointExplanation {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub end_stop_count: f64,
    pub reason: MorphShapeGradientEndpointReason,
    pub start_stop_count: f64,
    pub supported: bool,
}
impl PartialEq for MorphShapeGradientEndpointExplanation {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/MorphShape.ts:33 (sha256:c4a45d443af674418c90775cf7da6dd22f2bb6b934922c24ee2e95f029085f48)
pub type MorphShapeGradientEndpointReason = String;

// Source: upstream/packages/types/src/MorphShape.ts:40 (sha256:9f08ed93b7149ba994f4cc6c6ddd65acb4d6899ce85afdd0cc31c16d82ebd125)
#[derive(Clone, Default)]
pub struct MorphShapeLineEndpoint {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub alpha: Option<f64>,
    pub color: f64,
    pub thickness: f64,
}
impl PartialEq for MorphShapeLineEndpoint {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/MorphShape.ts:44 (sha256:deb9192df6311aa1b59b2284491c46b35ec62c4f8f1f323d0288b412d8ea626a)
#[derive(Clone, Default)]
pub struct MorphShapeColorPaintBinding {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub command_index: f64,
    pub end_alpha: f64,
    pub end_color: f64,
    pub kind: String,
    pub start_alpha: f64,
    pub start_color: f64,
}
impl PartialEq for MorphShapeColorPaintBinding {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/MorphShape.ts:55 (sha256:abe5022d0bb36a4950a6d8414708a5f4e7b35787588996db82d6a1e6a8508691)
#[derive(Clone, Default)]
pub struct MorphShapeGradientPaintBinding {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub command_index: f64,
    pub command_key: String,
    pub end_alphas: Vec<f64>,
    pub end_colors: Vec<f64>,
    pub end_focal_point_ratio: f64,
    pub end_matrix: Option<Matrix>,
    pub end_ratios: Vec<f64>,
    pub kind: String,
    pub start_alphas: Vec<f64>,
    pub start_colors: Vec<f64>,
    pub start_focal_point_ratio: f64,
    pub start_matrix: Option<Matrix>,
    pub start_ratios: Vec<f64>,
}
impl PartialEq for MorphShapeGradientPaintBinding {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/MorphShape.ts:73 (sha256:d2bd3807c916ad7a1b39dc5e0fc94f1c1715228123376d205490cd4be74e8551)
#[derive(Clone, Default)]
pub struct MorphShapeLinePaintBinding {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub command_index: f64,
    pub end_alpha: f64,
    pub end_color: f64,
    pub end_thickness: f64,
    pub kind: String,
    pub start_alpha: f64,
    pub start_color: f64,
    pub start_thickness: f64,
}
impl PartialEq for MorphShapeLinePaintBinding {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/MorphShape.ts:86 (sha256:46d1902e4791c0c3bb03b695d38fe38e8e872c3c57c2a037c57d52e5f7de0c7d)
#[derive(Clone, Default)]
pub struct MorphShapeTexturePaintBinding {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub command_index: f64,
    pub command_key: String,
    pub end_matrix: Matrix,
    pub kind: String,
    pub start_matrix: Matrix,
}
impl PartialEq for MorphShapeTexturePaintBinding {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/MorphShape.ts:94 (sha256:9cd4331b9ec4b4b03e89b5be06bf65c194e2dfa7dbdf97fef6c53c0225498736)
pub type MorphShapePaintBinding = crate::FlightUnion2<
    MorphShapeColorPaintBinding,
    crate::FlightUnion2<
        MorphShapeGradientPaintBinding,
        crate::FlightUnion2<MorphShapeLinePaintBinding, MorphShapeTexturePaintBinding>,
    >,
>;

// Source: upstream/packages/types/src/MorphShape.ts:104 (sha256:3c3ad2fcb2496c19ddf40cd7c5c6c20d5ddde69456be127c40066abd544b30e8)
#[derive(Clone, Default)]
pub struct MorphShapeData {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub commands: Vec<ShapeCommandToken>,
    pub morph: PathMorph,
    pub path: Path,
    pub paint_bindings: Vec<MorphShapePaintBinding>,
    pub path_bindings: Vec<MorphShapePathBinding>,
    pub progress: f64,
}
impl PartialEq for MorphShapeData {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/MorphShape.ts:112 (sha256:3430df21fd836c68fa70333ee05e90b679458d94d704d5f228bfeab974055a63)
pub type MorphShapeRuntime = crate::EntityRuntime;

// Source: upstream/packages/types/src/MorphShape.ts:114 (sha256:4d520958150bb3f2e2c1beebf07d580ca947c836dca809a68b34ea205143529c)
#[derive(Clone, Default)]
pub struct MorphShape {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    pub data: MorphShapeData,
    pub enabled: bool,
    pub kind: Kind,
    pub name: Option<String>,
    pub alpha: f64,
    pub visible: bool,
    pub blend_mode: Option<BlendMode>,
    pub clip: Option<ClipRegion>,
    pub material: Option<Material>,
    pub material_data: Option<MaterialData>,
    pub pivot_x: f64,
    pub pivot_y: f64,
    pub rotation: f64,
    pub scale_x: f64,
    pub scale_y: f64,
    pub skew_x: f64,
    pub skew_y: f64,
    pub x: f64,
    pub y: f64,
}
impl PartialEq for MorphShape {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for MorphShape {
    fn __flight_entity_runtime(
        &self,
    ) -> &std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>> {
        &self.__flight_entity_runtime
    }
    fn __flight_fresh_clone(&self) -> Self {
        let mut cloned = self.clone();
        cloned.__flight_identity = std::sync::Arc::new(());
        cloned.__flight_entity_runtime = std::sync::Arc::new(std::sync::Mutex::new(
            self.__flight_entity_runtime.lock().unwrap().clone(),
        ));
        cloned
    }
}

// Source: upstream/packages/types/src/MorphShape.ts:118 (sha256:9bc7088b2617f41cff6cd6528deba0cd7af491bfc11eee1a377d3e97e83f297e)
pub const MORPH_SHAPE_KIND: &'static str = "MorphShape";
