// @generated from upstream/packages/types/src/RiveDocument.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{
    AdvancedBlendMode, AnimationClip, DisplayObject, EntityRuntime, ImageResourceReference,
    LayoutTree, PathWinding, Scene2DSlotReference, Skeleton2D,
};

// Source: upstream/packages/types/src/RiveDocument.ts:29 (sha256:cdeb52cf0299cae7fb221a910a8870562e02597a4b6fffb0080eba7d1c560b9d)
pub struct RiveFieldType;
impl RiveFieldType {
    pub const Uint: f64 = 0.0_f64;
    pub const String: f64 = 1.0_f64;
    pub const Double: f64 = 2.0_f64;
    pub const Color: f64 = 3.0_f64;
}

// Source: upstream/packages/types/src/RiveDocument.ts:40 (sha256:8a1c95d151dc3ce717d89261436cc333c669d5c8bf000ba7c2a32da872958fbb)
// TypeScript numeric namespace RiveFieldType is represented by its generated Rust constants.

// Source: upstream/packages/types/src/RiveDocument.ts:51 (sha256:9921052c086827727e89862281184278a08ad2de8c492ef93a8c52996447e1e9)
pub type RiveValue = crate::FlightUnion2<f64, crate::FlightUnion2<String, Vec<u8>>>;

// Source: upstream/packages/types/src/RiveDocument.ts:58 (sha256:33b8ffeb2ffb3539affbe33b3665d4d8946af0486ae79f57a1ac3062d75617c5)
#[derive(Clone)]
pub struct RiveProperty {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub key: f64,
    pub type_: f64,
    pub value: RiveValue,
}
impl PartialEq for RiveProperty {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/RiveDocument.ts:68 (sha256:9252f9146b93933f51443521632f05794eed7f39a6e8059a7ae12d86167e16ac)
#[derive(Clone, Default)]
pub struct RiveCoreObject {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub properties: Vec<RiveProperty>,
    pub type_key: f64,
}
impl PartialEq for RiveCoreObject {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/RiveDocument.ts:74 (sha256:8e069d866e7b7c1b2e1dea84680b2c1aaee9261c3158f463f678a735872982a9)
#[derive(Clone, Default)]
pub struct RivePropertyFieldType {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub key: f64,
    pub type_: f64,
}
impl PartialEq for RivePropertyFieldType {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/RiveDocument.ts:84 (sha256:58430c568341539d07a196fb12b680ac253a2933a30332887826fd3f61b676c8)
#[derive(Clone, Default)]
pub struct RiveDocumentHeader {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub file_id: f64,
    pub major_version: f64,
    pub minor_version: f64,
    pub table_of_contents: Vec<RivePropertyFieldType>,
}
impl PartialEq for RiveDocumentHeader {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/RiveDocument.ts:92 (sha256:e42dc11ee2ff26991d6469eb849a675a3e0860983b512bdedb280cb3251648ce)
#[derive(Clone, Default)]
pub struct RiveDocument {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub header: RiveDocumentHeader,
    pub objects: Vec<RiveCoreObject>,
}
impl PartialEq for RiveDocument {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/RiveDocument.ts:104 (sha256:44aafe6b8ad37be7a692fd5ee540a56e2b48628f12925791a38e546b9f3e5987)
#[derive(Clone, Default)]
pub struct RiveArtboardGraph {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub objects: Vec<RiveCoreObject>,
    pub stream_end: f64,
    pub stream_start: f64,
    pub parent_indices: Vec<f64>,
}
impl PartialEq for RiveArtboardGraph {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/RiveDocument.ts:121 (sha256:b94308ee6f6c3f184de149e37a867b3ecc4c6c90f11919097a9ae45e880c5869)
#[derive(Clone, Default)]
pub struct RiveObjectGraph {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    #[doc(hidden)]
    pub __flight_entity_snapshot: Option<std::sync::Arc<dyn std::any::Any + Send + Sync>>,
    pub artboards: Vec<RiveArtboardGraph>,
}
impl PartialEq for RiveObjectGraph {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for RiveObjectGraph {
    fn __flight_entity_runtime(
        &self,
    ) -> &std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>> {
        &self.__flight_entity_runtime
    }
    fn __flight_entity_snapshot(&self) -> &Option<std::sync::Arc<dyn std::any::Any + Send + Sync>> {
        &self.__flight_entity_snapshot
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

// Source: upstream/packages/types/src/RiveDocument.ts:131 (sha256:e705df1c2ba082092310edcd7d71a4484273ee3cfd2d09ba1a644921b06566be)
#[derive(Clone, Default)]
pub struct RiveFileAsset {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub bytes: Option<Vec<u8>>,
    pub cdn_base_url: String,
    pub height: f64,
    pub kind: String,
    pub name: String,
    pub width: f64,
}
impl PartialEq for RiveFileAsset {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/RiveDocument.ts:148 (sha256:31564c3188fd747a9d408e99aecaaba1cddc025ae1f5d38e77240374602407a9)
#[derive(Clone, Default)]
pub struct RiveStateMachineTransition {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub duration: f64,
    pub exit_time: f64,
    pub flags: f64,
    pub to_state_id: f64,
}
impl PartialEq for RiveStateMachineTransition {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/RiveDocument.ts:156 (sha256:2d4a0be4d9f429ebcd6c5bf67f62693b2474d9ca1516b9856db4ade253bd1f27)
#[derive(Clone, Default)]
pub struct RiveStateMachineState {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub animation_id: f64,
    pub kind: String,
    pub transitions: Vec<RiveStateMachineTransition>,
}
impl PartialEq for RiveStateMachineState {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/RiveDocument.ts:162 (sha256:9197666b3856e564e31c71cefa69d6d11ef711156ae6738c9b5e9b59f58d9189)
#[derive(Clone, Default)]
pub struct RiveStateMachineLayer {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub name: String,
    pub states: Vec<RiveStateMachineState>,
}
impl PartialEq for RiveStateMachineLayer {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/RiveDocument.ts:168 (sha256:34d6971787f262e086374d04d6c4b54ceba2c61b8ce606434cbddf9ccd7807b3)
#[derive(Clone, Default)]
pub struct RiveStateMachineInput {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub kind: String,
    pub name: String,
    pub value: Option<crate::FlightUnion2<bool, f64>>,
}
impl PartialEq for RiveStateMachineInput {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/RiveDocument.ts:174 (sha256:eb014e2b1324041bf82a3175f87bf017bc5096e87254e1958b25ed04c5151e36)
#[derive(Clone, Default)]
pub struct RiveStateMachineDescriptor {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub inputs: Vec<RiveStateMachineInput>,
    pub layers: Vec<RiveStateMachineLayer>,
    pub name: String,
}
impl PartialEq for RiveStateMachineDescriptor {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/RiveDocument.ts:185 (sha256:406119ae48152c16b948d10ce07ac2ade01fa71316068d003e8bf5cf62d0c935)
#[derive(Clone, Default)]
pub struct RiveAdvancedBlend {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub mode: AdvancedBlendMode,
    pub node: DisplayObject,
}
impl PartialEq for RiveAdvancedBlend {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/RiveDocument.ts:195 (sha256:47b94215610c19e71a30e4d6e086fe9bad1d833b5d2ac65af5cdc1404e610909)
#[derive(Clone, Default)]
pub struct RiveAnimationLoopValues {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub loop_: String,
    pub one_shot: String,
    pub ping_pong: String,
}
impl PartialEq for RiveAnimationLoopValues {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub static RIVE_ANIMATION_LOOP: std::sync::LazyLock<RiveAnimationLoopValues> =
    std::sync::LazyLock::new(|| RiveAnimationLoopValues {
        __flight_identity: std::sync::Arc::new(()),
        one_shot: "OneShot".to_owned(),
        loop_: "Loop".to_owned(),
        ping_pong: "PingPong".to_owned(),
    });

// Source: upstream/packages/types/src/RiveDocument.ts:204 (sha256:94ee1af77fc2286ee4fda85cf0452aca04267d17dc6a36d5ab0005b0af8aca22)
pub type RiveAnimationLoop = String;

// Source: upstream/packages/types/src/RiveDocument.ts:219 (sha256:8c4002c22ea6d5f8a2e393febdda30e138443222b317625a807780d25afb12d2)
#[derive(Clone, Default)]
pub struct RiveSkeleton2DImport {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    #[doc(hidden)]
    pub __flight_entity_snapshot: Option<std::sync::Arc<dyn std::any::Any + Send + Sync>>,
    pub bone_indices: Vec<f64>,
    pub skeleton: Skeleton2D,
}
impl PartialEq for RiveSkeleton2DImport {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for RiveSkeleton2DImport {
    fn __flight_entity_runtime(
        &self,
    ) -> &std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>> {
        &self.__flight_entity_runtime
    }
    fn __flight_entity_snapshot(&self) -> &Option<std::sync::Arc<dyn std::any::Any + Send + Sync>> {
        &self.__flight_entity_snapshot
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

// Source: upstream/packages/types/src/RiveDocument.ts:232 (sha256:dc6ce844b2d7284290edebd13abd09b94bfd1db87d5835c3cda7848181a7bd0e)
#[derive(Clone, Default)]
pub struct RiveAnimationClip {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub clip: AnimationClip,
    pub loop_: RiveAnimationLoop,
    pub name: String,
    pub speed: f64,
    pub work_area_end: Option<f64>,
    pub work_area_start: Option<f64>,
}
impl PartialEq for RiveAnimationClip {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/RiveDocument.ts:260 (sha256:5863379cdbbf2f34ce15f4b7e5736fdc239bb96dd9fb7e979e83e779410de604)
#[derive(Clone, Default)]
pub struct RiveLayoutImport {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub targets: Vec<DisplayObject>,
    pub tree: LayoutTree,
}
impl PartialEq for RiveLayoutImport {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/RiveDocument.ts:270 (sha256:54a42f3cc19e59e72c52226519c6261639c69b68c0d659eb8edb42326eea8415)
#[derive(Clone, Default)]
pub struct RiveArtboardImport {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub advanced_blends: Vec<RiveAdvancedBlend>,
    pub animations: Vec<RiveAnimationClip>,
    pub state_machines: Vec<RiveStateMachineDescriptor>,
    pub height: f64,
    pub layouts: Vec<RiveLayoutImport>,
    pub name: String,
    pub root: DisplayObject,
    pub skeleton: Option<RiveSkeleton2DImport>,
    pub width: f64,
}
impl PartialEq for RiveArtboardImport {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/RiveDocument.ts:298 (sha256:e4d60e61db494330a741e425877bee07c00c78ef63991a768ca871e4c6ea67e2)
#[derive(Clone, Default)]
pub struct RiveWeightedPointKindValues {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub in_: String,
    pub out: String,
    pub point: String,
}
impl PartialEq for RiveWeightedPointKindValues {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub static RIVE_WEIGHTED_POINT_KIND: std::sync::LazyLock<RiveWeightedPointKindValues> =
    std::sync::LazyLock::new(|| RiveWeightedPointKindValues {
        __flight_identity: std::sync::Arc::new(()),
        point: "Point".to_owned(),
        in_: "In".to_owned(),
        out: "Out".to_owned(),
    });

// Source: upstream/packages/types/src/RiveDocument.ts:307 (sha256:286245f2740828e387c6284a6b569957a9ede33d9163db9d881538e801ac1900)
pub type RiveWeightedPointKind = String;

// Source: upstream/packages/types/src/RiveDocument.ts:317 (sha256:601a40b2969bbc822a6e4a0ed6f221a387c39d978d0aa0856d9cfb8755349044)
#[derive(Clone, Default)]
pub struct RiveWeightedPoint {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub kind: RiveWeightedPointKind,
    pub vertex: f64,
    pub x: f64,
    pub y: f64,
}
impl PartialEq for RiveWeightedPoint {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/RiveDocument.ts:329 (sha256:c9e4515a60d200d26308fa2a4d98c62ed83db38350d9545f6ef795ad4dd0edc7)
#[derive(Clone, Default)]
pub struct RivePathRecord {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub commands: Vec<f64>,
    pub data: Vec<f64>,
    pub path_index: f64,
    pub winding: PathWinding,
}
impl PartialEq for RivePathRecord {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/RiveDocument.ts:342 (sha256:3a3598b0a4e769f2d12f6f1bb59f6e3c6bac19aebd1ccd3fbe431b88520a2ce3)
#[derive(Clone, Default)]
pub struct RiveScene2DDocumentResult {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    #[doc(hidden)]
    pub __flight_entity_snapshot: Option<std::sync::Arc<dyn std::any::Any + Send + Sync>>,
    pub image_resources: Vec<ImageResourceReference>,
    pub imported: RiveDocumentImportResult,
    pub root: DisplayObject,
    pub slots: Vec<Scene2DSlotReference>,
}
impl PartialEq for RiveScene2DDocumentResult {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for RiveScene2DDocumentResult {
    fn __flight_entity_runtime(
        &self,
    ) -> &std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>> {
        &self.__flight_entity_runtime
    }
    fn __flight_entity_snapshot(&self) -> &Option<std::sync::Arc<dyn std::any::Any + Send + Sync>> {
        &self.__flight_entity_snapshot
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

// Source: upstream/packages/types/src/RiveDocument.ts:350 (sha256:4cd52ff4547df9122d019730085b2e9817cd95b330d8e981ae520fa71ac94905)
#[derive(Clone, Default)]
pub struct RiveDocumentImportResult {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    #[doc(hidden)]
    pub __flight_entity_snapshot: Option<std::sync::Arc<dyn std::any::Any + Send + Sync>>,
    pub artboards: Vec<RiveArtboardImport>,
    pub assets: Vec<RiveFileAsset>,
}
impl PartialEq for RiveDocumentImportResult {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for RiveDocumentImportResult {
    fn __flight_entity_runtime(
        &self,
    ) -> &std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>> {
        &self.__flight_entity_runtime
    }
    fn __flight_entity_snapshot(&self) -> &Option<std::sync::Arc<dyn std::any::Any + Send + Sync>> {
        &self.__flight_entity_snapshot
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
