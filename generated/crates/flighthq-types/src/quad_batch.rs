// @generated from upstream/packages/types/src/QuadBatch.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{
    BlendMode, ClipRegion, EntityRuntime, Kind, Material, MaterialData, QuadTransformType,
    TextureAtlas,
};

// Source: upstream/packages/types/src/QuadBatch.ts:7 (sha256:c5ddb66c3aa664642f434b204e28cd767990fb68dccd61d95ddce1217b271f85)
#[derive(Clone, Default)]
pub struct QuadBatchData {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub atlas: Option<TextureAtlas>,
    pub ids: Vec<u16>,
    pub instance_count: f64,
    pub material_data: Option<Vec<Option<MaterialData>>>,
    pub transforms: Vec<f32>,
    pub transform_type: QuadTransformType,
}
impl PartialEq for QuadBatchData {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/QuadBatch.ts:20 (sha256:e420cd628a1440e52a58f0ec478200b7597ac27f4757124600f5951f22965216)
pub type QuadBatchRuntime = crate::EntityRuntime;

// Source: upstream/packages/types/src/QuadBatch.ts:31 (sha256:899537b7d2a81e3752bb2c1fc97d945d22692f778fc4c242542ee226df28fef4)
#[derive(Clone, Default)]
pub struct QuadBatch {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    pub data: QuadBatchData,
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
impl PartialEq for QuadBatch {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for QuadBatch {
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

// Source: upstream/packages/types/src/QuadBatch.ts:35 (sha256:4a42050cba3214e1f705bc48c73a663b704ae3d048f879dbbbff7cdc5066415c)
pub const QUAD_BATCH_KIND: &'static str = "QuadBatch";
