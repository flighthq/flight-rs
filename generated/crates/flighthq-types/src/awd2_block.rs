// @generated from upstream/packages/types/src/Awd2Block.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{EntityRuntime, ImportDiagnostic, MeshGeometry, Scene3DDocument};

// Source: upstream/packages/types/src/Awd2Block.ts:17 (sha256:0d15385a77498c40625bcb741270e1ff34261fcefc8eb94321003ead2d9e6fc0)
#[derive(Clone, Default)]
pub struct Awd2Block {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub block_id: f64,
    pub block_type: f64,
    pub data_end: f64,
    pub data_start: f64,
    pub geometry_wide: bool,
    pub matrix_wide: bool,
    pub source: Vec<u8>,
    pub view: crate::OpaqueHostValue,
}
impl PartialEq for Awd2Block {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Awd2Block.ts:35 (sha256:e2e83ddb246c5ecbec49ddaa1a7ce424389c8b96ae5e9d9f3c978126770d8f26)
#[derive(Clone)]
pub struct Awd2BlockHandler {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub block_types: Vec<f64>,
    pub build_phase: Option<f64>,
    pub deferred: Option<bool>,
    pub parse: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut(Awd2ParseState, Awd2Block) -> () + Send + 'static>>,
    >,
    pub build: Option<
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(Awd2ParseState) -> () + Send + 'static>>>,
    >,
}
impl PartialEq for Awd2BlockHandler {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Awd2Block.ts:62 (sha256:65f450584565b117a8bbb8c470f8bc03e69ed0310afac7180dc7c550370a3b33)
pub type Awd2BlockDispatch = Vec<(f64, Awd2BlockHandler)>;

// Source: upstream/packages/types/src/Awd2Block.ts:72 (sha256:1a9d3cd64a0bb43bf877ee291eb95f9e7eda687e0915a77b522452eb58517043)
#[derive(Clone, Default)]
pub struct Awd2ParseState {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    #[doc(hidden)]
    pub __flight_entity_snapshot: Option<std::sync::Arc<dyn std::any::Any + Send + Sync>>,
    pub cameras: Vec<(f64, Awd2ParsedCamera)>,
    pub containers: Vec<(f64, Awd2ParsedContainer)>,
    pub diagnostics: Option<Vec<ImportDiagnostic>>,
    pub document: Scene3DDocument,
    pub geometries: Vec<(f64, Vec<Awd2ParsedGeometry>)>,
    pub light_pickers: Vec<(f64, Awd2ParsedLightPicker)>,
    pub lights: Vec<(f64, Awd2ParsedLight)>,
    pub materials: Vec<(f64, Awd2ParsedMaterial)>,
    pub mesh_instances: Vec<(f64, Awd2ParsedMeshInstance)>,
    pub node_index_for_block: Vec<(f64, f64)>,
    pub resolve_material:
        Option<std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(f64) -> f64 + Send + 'static>>>>,
    pub skeleton_animations: Vec<(f64, Awd2ParsedSkeletonAnimation)>,
    pub skeleton_joint_node_indices: Vec<f64>,
    pub skeleton_poses: Vec<(f64, Awd2ParsedSkeletonPose)>,
    pub skeletons: Vec<(f64, Awd2ParsedSkeleton)>,
    pub skin_index: Option<f64>,
    pub source: Vec<u8>,
    pub textures: Vec<(f64, Awd2ParsedTexture)>,
    pub view: crate::OpaqueHostValue,
}
impl PartialEq for Awd2ParseState {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for Awd2ParseState {
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

// Source: upstream/packages/types/src/Awd2Block.ts:107 (sha256:9ba024830ea488e04d31660f34c720b7bb48cbbe7d864593f66bf04f8c7a6c7c)
#[derive(Clone, Default)]
pub struct Awd2ParsedCamera {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub bottom: f64,
    pub fov: f64,
    pub left: f64,
    pub name: String,
    pub parent_id: f64,
    pub projection_type: f64,
    pub right: f64,
    pub top: f64,
    pub transform: Vec<f64>,
}
impl PartialEq for Awd2ParsedCamera {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Awd2Block.ts:119 (sha256:814f405287c4550ef2a10cb15bf4c158c841f6a58b04051faaf6900540f26fb2)
#[derive(Clone, Default)]
pub struct Awd2ParsedContainer {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub name: String,
    pub parent_id: f64,
    pub transform: Vec<f64>,
}
impl PartialEq for Awd2ParsedContainer {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Awd2Block.ts:125 (sha256:112f14372bab1067246ad34bd3eb507456bfec227bf9d7d94902586415e5644f)
#[derive(Clone, Default)]
pub struct Awd2ParsedGeometry {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub geometry: MeshGeometry,
    pub skinned: bool,
}
impl PartialEq for Awd2ParsedGeometry {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Awd2Block.ts:134 (sha256:5c215a2919670bbedd93cf96587198753733014f4235e94c997dc16993ba6947)
#[derive(Clone, Default)]
pub struct Awd2ParsedJoint {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub name: String,
    pub parent_index: f64,
    pub transform: Vec<f64>,
}
impl PartialEq for Awd2ParsedJoint {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Awd2Block.ts:140 (sha256:6964708201843413162ec18d89e7856d318993eede04aa6e2f0ebbf5b0c1b88e)
#[derive(Clone, Default)]
pub struct Awd2ParsedLight {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub ambient: f64,
    pub ambient_rgb: f64,
    pub casts_shadow: bool,
    pub diffuse: f64,
    pub direction_x: f64,
    pub direction_y: f64,
    pub direction_z: f64,
    pub fall_off: f64,
    pub has_radius: bool,
    pub light_type: f64,
    pub name: String,
    pub parent_id: f64,
    pub radius: f64,
    pub rgb: f64,
    pub specular: f64,
    pub transform: Vec<f64>,
}
impl PartialEq for Awd2ParsedLight {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Awd2Block.ts:168 (sha256:0702272c863a84097bbfef4b35c7337927548f5fab968f351dc5f2c1b4028b22)
#[derive(Clone, Default)]
pub struct Awd2ParsedLightPicker {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub light_ids: Vec<f64>,
    pub name: String,
}
impl PartialEq for Awd2ParsedLightPicker {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Awd2Block.ts:173 (sha256:efa0c6a6c10ada2c3d435788a1d65e298f81d6174207e8b73514aacf851b4eda)
#[derive(Clone, Default)]
pub struct Awd2ParsedMaterial {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub alpha: Option<f64>,
    pub color: Option<f64>,
    pub diffuse_texture_id: f64,
    pub gloss: Option<f64>,
    pub name: String,
    pub normal_texture_id: f64,
    pub num_methods: f64,
    pub specular_color: Option<f64>,
    pub specular_strength: Option<f64>,
    pub specular_texture_id: f64,
}
impl PartialEq for Awd2ParsedMaterial {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Awd2Block.ts:187 (sha256:f34ba2ef489378642d001566145ad9378d3bd433d12d98c1b1fb30796c01f77b)
#[derive(Clone, Default)]
pub struct Awd2ParsedMeshInstance {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub geometry_id: f64,
    pub material_ids: Vec<f64>,
    pub name: String,
    pub parent_id: f64,
    pub transform: Vec<f64>,
}
impl PartialEq for Awd2ParsedMeshInstance {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Awd2Block.ts:195 (sha256:f58a942e92eb039a87395fc7181efc8021a271c71819378abe941199c560d742)
#[derive(Clone, Default)]
pub struct Awd2ParsedSkeleton {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub joints: Vec<Awd2ParsedJoint>,
    pub name: String,
}
impl PartialEq for Awd2ParsedSkeleton {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Awd2Block.ts:200 (sha256:1ff47761390e17a6596dfe1c63c87197caeefda37f78ae43d377d6554752937c)
#[derive(Clone, Default)]
pub struct Awd2ParsedSkeletonAnimationRecord1 {
    pub __flight_identity: std::sync::Arc<()>,
    pub duration: f64,
    pub pose_block_id: f64,
}
impl PartialEq for Awd2ParsedSkeletonAnimationRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct Awd2ParsedSkeletonAnimation {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub name: String,
    pub poses: Vec<Awd2ParsedSkeletonAnimationRecord1>,
}
impl PartialEq for Awd2ParsedSkeletonAnimation {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Awd2Block.ts:205 (sha256:151e5b34fc5c0a3672021149b81a571cc715d41f25674c96934c3f7c2e016ecc)
#[derive(Clone, Default)]
pub struct Awd2ParsedSkeletonPose {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub joint_transforms: Vec<Option<Vec<f64>>>,
    pub name: String,
}
impl PartialEq for Awd2ParsedSkeletonPose {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Awd2Block.ts:216 (sha256:21cb0ba5dafb6ade6031e43ca206a3787583d517734566498f86f593bcf1de1f)
#[derive(Clone, Default)]
pub struct Awd2ParsedTexture {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub bytes: Option<Vec<u8>>,
    pub mime_type: Option<String>,
    pub name: String,
    pub url: Option<String>,
}
impl PartialEq for Awd2ParsedTexture {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
