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

// Source: upstream/packages/types/src/QuadBatch.ts:7 (sha256:1eb0802de4e5a5b34e6e585aa05bc5147a491e7e455d4719ac072ac9577c9caa)
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

// Source: upstream/packages/types/src/QuadBatch.ts:20 (sha256:75238ecb20b17ee00471a4ef63623e2e5b34bab0747cfd29e59d8662516e1942)
pub type QuadBatchRuntime = crate::EntityRuntime;

// Source: upstream/packages/types/src/QuadBatch.ts:31 (sha256:51f21dd1261747ad4e3ecb0223ce247c09417a3ad79113123cf7bde455ecfb17)
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
