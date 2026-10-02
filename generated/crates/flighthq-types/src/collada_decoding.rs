// @generated from upstream/packages/types/src/ColladaDecoding.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{ImportDiagnostic, Scene3DDocument, XmlElement};

// Source: upstream/packages/types/src/ColladaDecoding.ts:14 (sha256:4993af59ee903a1e6af871961b032218663182be33692fb0f9efc41e3943496a)
#[derive(Clone, Default)]
pub struct ColladaDecodedSkinRecord1 {
    pub __flight_identity: std::sync::Arc<()>,
    pub joint: String,
    pub weight: f64,
}
impl PartialEq for ColladaDecodedSkinRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct ColladaDecodedSkin {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub bind_shape_matrix: Vec<f64>,
    pub controller_id: String,
    pub geometry_ref: String,
    pub influences: Vec<Vec<ColladaDecodedSkinRecord1>>,
    pub inverse_bind_matrices: Vec<Vec<f64>>,
    pub joint_names: Vec<String>,
    pub joint_sids: Vec<String>,
}
impl PartialEq for ColladaDecodedSkin {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/ColladaDecoding.ts:23 (sha256:d2f9cf0e0cd947f84c5931d6dd7fe9058d00f64feb7371ae715a5efe2fa96e58)
#[derive(Clone, Default)]
pub struct ColladaDecodedAnimationChannel {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub target: String,
    pub times: Vec<f64>,
    pub values: Vec<f64>,
    pub interpolation: Vec<String>,
    pub in_tangents: Vec<f64>,
    pub out_tangents: Vec<f64>,
}
impl PartialEq for ColladaDecodedAnimationChannel {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/ColladaDecoding.ts:31 (sha256:f229396747083f873a10555f917668f6f78a1de4c4738902b7cec88bf3270b41)
#[derive(Clone, Default)]
pub struct ColladaPerspectiveCameraDefinition {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub aspect: f64,
    pub far: f64,
    pub fov_y: f64,
    pub kind: String,
    pub name: Option<String>,
    pub near: f64,
}
impl PartialEq for ColladaPerspectiveCameraDefinition {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/ColladaDecoding.ts:39 (sha256:991537a48115e07007ac1f995134fcbcf386fab69862d091d84daf82cd07afef)
#[derive(Clone, Default)]
pub struct ColladaOrthographicCameraDefinition {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub far: f64,
    pub half_height: f64,
    pub half_width: f64,
    pub kind: String,
    pub name: Option<String>,
    pub near: f64,
}
impl PartialEq for ColladaOrthographicCameraDefinition {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/ColladaDecoding.ts:47 (sha256:5e72445b5d650187c014687978998a25fda99b6005dbe3f205e8ca596710e73f)
pub type ColladaCameraDefinition =
    crate::FlightUnion2<ColladaOrthographicCameraDefinition, ColladaPerspectiveCameraDefinition>;

// Source: upstream/packages/types/src/ColladaDecoding.ts:48 (sha256:18eef8e9ca09ec06775207a3cca60486afe752b03784af7d35e8ab25337ecb5c)
#[derive(Clone, Default)]
pub struct ColladaDecodedMorph {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub controller_id: String,
    pub base_geometry: String,
    pub method: String,
    pub targets: Vec<String>,
    pub weights: Vec<f64>,
}
impl PartialEq for ColladaDecodedMorph {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/ColladaDecoding.ts:55 (sha256:9d2dbed479821fdb8fb3e28b934a33ebd46bbc42d3765b34094dd7f4d1309f84)
pub type ColladaLightKind = String;

// Source: upstream/packages/types/src/ColladaDecoding.ts:56 (sha256:1ae141c4694e0769594c0fdcb16ed7ba394f47f1bd7142ece1b9a1fccf4f3093)
#[derive(Clone, Default)]
pub struct ColladaLightDefinition {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub color: f64,
    pub decay: f64,
    pub inner_cone_degrees: f64,
    pub intensity: f64,
    pub kind: ColladaLightKind,
    pub name: Option<String>,
    pub outer_cone_degrees: f64,
    pub spot_blend: f64,
}
impl PartialEq for ColladaLightDefinition {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/ColladaDecoding.ts:84 (sha256:59d05701a24eeb7ec7dd4c562ed079fe5e89feb1ca1120c14012ad09d0bfcaca)
#[derive(Clone, Default)]
pub struct ColladaParseContext {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub animation_channels: Vec<ColladaDecodedAnimationChannel>,
    pub base_url: Option<String>,
    pub camera_definitions: Vec<(String, Option<ColladaCameraDefinition>)>,
    pub diagnostics: Vec<ImportDiagnostic>,
    pub document: Scene3DDocument,
    pub geometry_id_to_mesh_index: Vec<(String, f64)>,
    pub geometry_positions: Vec<(String, Vec<f64>)>,
    pub geometry_primitive_symbols: Vec<(String, Vec<String>)>,
    pub light_definitions: Vec<(String, Option<ColladaLightDefinition>)>,
    pub material_indices: Vec<(String, f64)>,
    pub morphs: Vec<(String, ColladaDecodedMorph)>,
    pub root: XmlElement,
    pub skins: Vec<(String, ColladaDecodedSkin)>,
}
impl PartialEq for ColladaParseContext {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/ColladaDecoding.ts:125 (sha256:f36359c691abd8bcf2cf1a6e116069fa9f94ca540db99dba77e1696587ed1923)
#[derive(Clone)]
pub struct ColladaElementDecoder {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub build: Option<
        std::sync::Arc<
            std::sync::Mutex<Box<dyn FnMut(ColladaBuildContext) -> () + Send + 'static>>,
        >,
    >,
    pub build_phase: Option<f64>,
    pub elements: Vec<String>,
    pub features: Vec<String>,
    pub decode: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut(ColladaParseContext) -> () + Send + 'static>>,
    >,
}
impl PartialEq for ColladaElementDecoder {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/ColladaDecoding.ts:168 (sha256:dcd314aa32cb5172cc939bb30ffd81dea50e09b71df00dadc5b858ab6bf450a6)
#[derive(Clone, Default)]
pub struct ColladaBuildContext {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub deferred_cameras: Vec<ColladaDeferredCameraBinding>,
    pub deferred_controllers: Vec<ColladaDeferredControllerBinding>,
    pub deferred_lights: Vec<ColladaDeferredLightBinding>,
    pub node_id_map: Vec<(String, f64)>,
    pub parse: ColladaParseContext,
    pub root_node_indices: Vec<f64>,
}
impl PartialEq for ColladaBuildContext {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/ColladaDecoding.ts:184 (sha256:7a8e83c89892d37f8300123e81434ddbd33fac688664c074c38d73cda39814b1)
#[derive(Clone, Default)]
pub struct ColladaDeferredControllerBinding {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub controller_id: String,
    pub material_overrides: Vec<(String, f64)>,
    pub node_index: f64,
    pub skeleton_root: Option<String>,
}
impl PartialEq for ColladaDeferredControllerBinding {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/ColladaDecoding.ts:192 (sha256:51f12e229782bafeb81cab72ed232dd9049f6ee372027c19c9350b0a7c914517)
#[derive(Clone, Default)]
pub struct ColladaDeferredCameraBinding {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub camera_id: String,
    pub node_index: f64,
}
impl PartialEq for ColladaDeferredCameraBinding {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/ColladaDecoding.ts:198 (sha256:d13e8a9cd8e149f7758cc58fa657bd506fef49205b41905757b92a92fc138f3d)
#[derive(Clone, Default)]
pub struct ColladaDeferredLightBinding {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub node_index: f64,
    pub url: String,
}
impl PartialEq for ColladaDeferredLightBinding {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
