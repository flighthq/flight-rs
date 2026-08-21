// @generated from upstream/packages/types/src/GltfSchema.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/GltfSchema.ts:10 (sha256:7c45eb9ffede320985adfbeefe462de8bec2a209a1754b90c2938e77d38f88a4)
#[derive(Clone, Default)]
pub struct GltfDocumentRecord1 {
    pub __flight_identity: std::sync::Arc<()>,
    pub version: String,
}
impl PartialEq for GltfDocumentRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct GltfDocument {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub asset: Option<GltfDocumentRecord1>,
    pub cameras: Option<Vec<GltfCamera>>,
    pub scene: Option<f64>,
    pub scenes: Option<Vec<GltfScene3D>>,
    pub nodes: Option<Vec<GltfNode>>,
    pub meshes: Option<Vec<GltfMesh>>,
    pub materials: Option<Vec<GltfMaterial>>,
    pub textures: Option<Vec<GltfTexture>>,
    pub samplers: Option<Vec<GltfSampler>>,
    pub images: Option<Vec<GltfImage>>,
    pub animations: Option<Vec<GltfAnimation>>,
    pub accessors: Option<Vec<GltfAccessor>>,
    pub buffer_views: Option<Vec<GltfBufferView>>,
    pub buffers: Option<Vec<GltfBuffer>>,
    pub skins: Option<Vec<GltfSkin>>,
    pub extensions_used: Option<Vec<String>>,
    pub extensions_required: Option<Vec<String>>,
    pub extensions: Option<GltfDocumentExtensions>,
}
impl PartialEq for GltfDocument {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/GltfSchema.ts:31 (sha256:6bade5e485caee34db3848337c19201905ce4f65982b14d78750efbf48afa1ef)
#[derive(Clone, Default)]
pub struct GltfCameraRecord1 {
    pub __flight_identity: std::sync::Arc<()>,
    pub aspect_ratio: Option<f64>,
    pub yfov: f64,
    pub zfar: Option<f64>,
    pub znear: f64,
}
impl PartialEq for GltfCameraRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct GltfCameraRecord2 {
    pub __flight_identity: std::sync::Arc<()>,
    pub xmag: f64,
    pub ymag: f64,
    pub zfar: f64,
    pub znear: f64,
}
impl PartialEq for GltfCameraRecord2 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct GltfCamera {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub name: Option<String>,
    pub orthographic: Option<GltfCameraRecord2>,
    pub perspective: Option<GltfCameraRecord1>,
    pub type_: String,
}
impl PartialEq for GltfCamera {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/GltfSchema.ts:48 (sha256:4fcd06231e24ccf6ec886d0f641d5aa079e5bc9d102804793db58cff2cfdfe3e)
#[derive(Clone, Default)]
pub struct GltfDocumentExtensionsRecord1 {
    pub __flight_identity: std::sync::Arc<()>,
    pub lights: Vec<GltfPunctualLight>,
}
impl PartialEq for GltfDocumentExtensionsRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct GltfDocumentExtensions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub khr_lights_punctual: Option<GltfDocumentExtensionsRecord1>,
}
impl PartialEq for GltfDocumentExtensions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/GltfSchema.ts:53 (sha256:9a4b9d5d9174473533ebcc2cf171e61b59acfc5603eaae96e16d5c9e284a6034)
#[derive(Clone, Default)]
pub struct GltfPunctualLightRecord1 {
    pub __flight_identity: std::sync::Arc<()>,
    pub inner_cone_angle: Option<f64>,
    pub outer_cone_angle: Option<f64>,
}
impl PartialEq for GltfPunctualLightRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct GltfPunctualLight {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub color: Option<Vec<f64>>,
    pub intensity: Option<f64>,
    pub name: Option<String>,
    pub range: Option<f64>,
    pub spot: Option<GltfPunctualLightRecord1>,
    pub type_: String,
}
impl PartialEq for GltfPunctualLight {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/GltfSchema.ts:64 (sha256:6b316998000142bc2d6f2bff8916ea8ec263d84dc186618c7e8cdb3afa4a25db)
#[derive(Clone, Default)]
pub struct GltfAnimation {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub name: Option<String>,
    pub channels: Vec<GltfAnimationChannel>,
    pub samplers: Vec<GltfAnimationSampler>,
}
impl PartialEq for GltfAnimation {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/GltfSchema.ts:70 (sha256:99ebbcd124c01546a548a199915381b5de5916b1de258cedf8de0f24a750ad98)
#[derive(Clone, Default)]
pub struct GltfAnimationChannelRecord1 {
    pub __flight_identity: std::sync::Arc<()>,
    pub node: Option<f64>,
    pub path: String,
}
impl PartialEq for GltfAnimationChannelRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct GltfAnimationChannel {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub sampler: f64,
    pub target: GltfAnimationChannelRecord1,
}
impl PartialEq for GltfAnimationChannel {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/GltfSchema.ts:77 (sha256:4c48340b0452e329d091a9a2ff5c4d2390fa6f45b4e95f9516d36ed17b0ba4e4)
#[derive(Clone, Default)]
pub struct GltfAnimationSampler {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub input: f64,
    pub output: f64,
    pub interpolation: Option<String>,
}
impl PartialEq for GltfAnimationSampler {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/GltfSchema.ts:86 (sha256:ba823684611c91f967da59a9aa5ef72b6e2427d0f4e543c3532b4102bf4bc91b)
#[derive(Clone, Default)]
pub struct GltfMaterialRecord1 {
    pub __flight_identity: std::sync::Arc<()>,
    pub khr_materials_anisotropy: Option<GltfMaterialsAnisotropy>,
    pub khr_materials_clearcoat: Option<GltfMaterialsClearcoat>,
    pub khr_materials_emissive_strength: Option<GltfMaterialsEmissiveStrength>,
    pub khr_materials_ior: Option<GltfMaterialsIor>,
    pub khr_materials_iridescence: Option<GltfMaterialsIridescence>,
    pub khr_materials_pbr_specular_glossiness: Option<GltfMaterialsPbrSpecularGlossiness>,
    pub khr_materials_sheen: Option<GltfMaterialsSheen>,
    pub khr_materials_unlit: Option<Vec<(String, std::convert::Infallible)>>,
    pub khr_materials_specular: Option<GltfMaterialsSpecular>,
    pub khr_materials_transmission: Option<GltfMaterialsTransmission>,
    pub khr_materials_volume: Option<GltfMaterialsVolume>,
}
impl PartialEq for GltfMaterialRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct GltfMaterial {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub name: Option<String>,
    pub pbr_metallic_roughness: Option<GltfPbrMetallicRoughness>,
    pub normal_texture: Option<GltfNormalTextureInfo>,
    pub occlusion_texture: Option<GltfOcclusionTextureInfo>,
    pub emissive_texture: Option<GltfTextureInfo>,
    pub emissive_factor: Option<Vec<f64>>,
    pub alpha_mode: Option<String>,
    pub alpha_cutoff: Option<f64>,
    pub double_sided: Option<bool>,
    pub extensions: Option<GltfMaterialRecord1>,
}
impl PartialEq for GltfMaterial {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/GltfSchema.ts:120 (sha256:83c1c01a6b2e649e652d240c668774153c52e52624bfd52a543c1cbe9003104d)
#[derive(Clone, Default)]
pub struct GltfMaterialsAnisotropy {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub anisotropy_rotation: Option<f64>,
    pub anisotropy_strength: Option<f64>,
    pub anisotropy_texture: Option<GltfTextureInfo>,
}
impl PartialEq for GltfMaterialsAnisotropy {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/GltfSchema.ts:128 (sha256:e63f567e73fb625cd461e7c4e32905caf673bb294ce233d1d55fee2ca247f55b)
#[derive(Clone, Default)]
pub struct GltfMaterialsIor {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub ior: Option<f64>,
}
impl PartialEq for GltfMaterialsIor {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/GltfSchema.ts:134 (sha256:d2f8abfb3271aa7a4c8ee46bb342005a1ce9a8000cd85f3e8ab2bfcf89e3339a)
#[derive(Clone, Default)]
pub struct GltfMaterialsIridescence {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub iridescence_factor: Option<f64>,
    pub iridescence_ior: Option<f64>,
    pub iridescence_texture: Option<GltfTextureInfo>,
    pub iridescence_thickness_maximum: Option<f64>,
    pub iridescence_thickness_minimum: Option<f64>,
    pub iridescence_thickness_texture: Option<GltfTextureInfo>,
}
impl PartialEq for GltfMaterialsIridescence {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/GltfSchema.ts:147 (sha256:f9aa1773a943833e5c35fe099cf30754813b4015fd71ef9fb554652660c8a0db)
#[derive(Clone, Default)]
pub struct GltfMaterialsPbrSpecularGlossiness {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub diffuse_factor: Option<Vec<f64>>,
    pub diffuse_texture: Option<GltfTextureInfo>,
    pub glossiness_factor: Option<f64>,
    pub specular_factor: Option<Vec<f64>>,
    pub specular_glossiness_texture: Option<GltfTextureInfo>,
}
impl PartialEq for GltfMaterialsPbrSpecularGlossiness {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/GltfSchema.ts:157 (sha256:79337eae9ef84ccdf0f51f7070b6e147253c0002c938a30f6273d6194f222d4c)
#[derive(Clone, Default)]
pub struct GltfMaterialsSpecular {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub specular_color_factor: Option<Vec<f64>>,
    pub specular_color_texture: Option<GltfTextureInfo>,
    pub specular_factor: Option<f64>,
    pub specular_texture: Option<GltfTextureInfo>,
}
impl PartialEq for GltfMaterialsSpecular {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/GltfSchema.ts:166 (sha256:014105deb1858374094c92f1936969f206d49b3cf24ac30e85cece6216dfd92a)
#[derive(Clone, Default)]
pub struct GltfMaterialsTransmission {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub transmission_factor: Option<f64>,
    pub transmission_texture: Option<GltfTextureInfo>,
}
impl PartialEq for GltfMaterialsTransmission {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/GltfSchema.ts:174 (sha256:f802e7c01ce18b59223b3d3044e9143ab4318885fcc6ad7a7c94391b1d741864)
#[derive(Clone, Default)]
pub struct GltfMaterialsVolume {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub attenuation_color: Option<Vec<f64>>,
    pub attenuation_distance: Option<f64>,
    pub thickness_factor: Option<f64>,
    pub thickness_texture: Option<GltfTextureInfo>,
}
impl PartialEq for GltfMaterialsVolume {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/GltfSchema.ts:185 (sha256:da01f81b765da48a0f599d980bbce9f62d4d5ef931a1ae2c6702704011149140)
#[derive(Clone, Default)]
pub struct GltfMaterialsClearcoat {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub clearcoat_factor: Option<f64>,
    pub clearcoat_normal_texture: Option<GltfNormalTextureInfo>,
    pub clearcoat_roughness_factor: Option<f64>,
    pub clearcoat_roughness_texture: Option<GltfTextureInfo>,
    pub clearcoat_texture: Option<GltfTextureInfo>,
}
impl PartialEq for GltfMaterialsClearcoat {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/GltfSchema.ts:195 (sha256:391adfc653c039fd69c81d346d77d070ed4104d5bbb67913b6339a840b4712e7)
#[derive(Clone, Default)]
pub struct GltfMaterialsEmissiveStrength {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub emissive_strength: Option<f64>,
}
impl PartialEq for GltfMaterialsEmissiveStrength {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/GltfSchema.ts:201 (sha256:0b2d39871768853b5d9df83e34d4448e12c77c520ea09af05d924630c0bfe6d3)
#[derive(Clone, Default)]
pub struct GltfMaterialsSheen {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub sheen_color_factor: Option<Vec<f64>>,
    pub sheen_color_texture: Option<GltfTextureInfo>,
    pub sheen_roughness_factor: Option<f64>,
    pub sheen_roughness_texture: Option<GltfTextureInfo>,
}
impl PartialEq for GltfMaterialsSheen {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/GltfSchema.ts:208 (sha256:1cc0bbdb9da8c79e74bf002197e232abc9fcd8eb9c138301010459e38a2148a5)
#[derive(Clone, Default)]
pub struct GltfPbrMetallicRoughness {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub base_color_factor: Option<Vec<f64>>,
    pub base_color_texture: Option<GltfTextureInfo>,
    pub metallic_factor: Option<f64>,
    pub roughness_factor: Option<f64>,
    pub metallic_roughness_texture: Option<GltfTextureInfo>,
}
impl PartialEq for GltfPbrMetallicRoughness {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/GltfSchema.ts:223 (sha256:6210ba35b14169a522c08b35d273a181f2481177a9953d96530a76161880e0ce)
#[derive(Clone, Default)]
pub struct GltfTextureInfoRecord1 {
    pub __flight_identity: std::sync::Arc<()>,
    pub khr_texture_transform: Option<GltfTextureTransform>,
}
impl PartialEq for GltfTextureInfoRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct GltfTextureInfo {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub index: f64,
    pub tex_coord: Option<f64>,
    pub extensions: Option<GltfTextureInfoRecord1>,
}
impl PartialEq for GltfTextureInfo {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/GltfSchema.ts:233 (sha256:86e2895ab152b151cdfe2b5a0c9007dcf5a0b0871e0ea90e367f100b5a67cbc7)
#[derive(Clone, Default)]
pub struct GltfTextureTransform {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub offset: Option<Vec<f64>>,
    pub rotation: Option<f64>,
    pub scale: Option<Vec<f64>>,
    pub tex_coord: Option<f64>,
}
impl PartialEq for GltfTextureTransform {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/GltfSchema.ts:240 (sha256:d0b4f951da9cec3176dc1870a4df662474a533004ecede68858b9664d27faca8)
#[derive(Clone, Default)]
pub struct GltfNormalTextureInfoRecord1 {
    pub __flight_identity: std::sync::Arc<()>,
    pub khr_texture_transform: Option<GltfTextureTransform>,
}
impl PartialEq for GltfNormalTextureInfoRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct GltfNormalTextureInfo {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub index: f64,
    pub tex_coord: Option<f64>,
    pub extensions: Option<GltfNormalTextureInfoRecord1>,
    pub scale: Option<f64>,
}
impl PartialEq for GltfNormalTextureInfo {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/GltfSchema.ts:245 (sha256:081d04274e741f437f37e3a451316d1945f97c9238a3d90cff534c9c0da310c3)
#[derive(Clone, Default)]
pub struct GltfOcclusionTextureInfoRecord1 {
    pub __flight_identity: std::sync::Arc<()>,
    pub khr_texture_transform: Option<GltfTextureTransform>,
}
impl PartialEq for GltfOcclusionTextureInfoRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct GltfOcclusionTextureInfo {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub index: f64,
    pub tex_coord: Option<f64>,
    pub extensions: Option<GltfOcclusionTextureInfoRecord1>,
    pub strength: Option<f64>,
}
impl PartialEq for GltfOcclusionTextureInfo {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/GltfSchema.ts:253 (sha256:c1cd0e1179e2bf5123b64ed52de0a98625938360d08f0b5ae19c8503bba52c8c)
#[derive(Clone, Default)]
pub struct GltfTextureRecord1 {
    pub __flight_identity: std::sync::Arc<()>,
    pub source: f64,
}
impl PartialEq for GltfTextureRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct GltfTextureRecord2 {
    pub __flight_identity: std::sync::Arc<()>,
    pub khr_texture_basisu: Option<GltfTextureRecord1>,
}
impl PartialEq for GltfTextureRecord2 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct GltfTexture {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub extensions: Option<GltfTextureRecord2>,
    pub sampler: Option<f64>,
    pub source: Option<f64>,
}
impl PartialEq for GltfTexture {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/GltfSchema.ts:264 (sha256:caedd7f879707ef31d3944bc22b0d41c98c516d5bc34843dcb62bf983ce61116)
#[derive(Clone, Default)]
pub struct GltfSampler {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub mag_filter: Option<f64>,
    pub min_filter: Option<f64>,
    pub wrap_s: Option<f64>,
    pub wrap_t: Option<f64>,
}
impl PartialEq for GltfSampler {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/GltfSchema.ts:273 (sha256:b1fb41632f2a66a2fa070f737a37c92f27815963f3fd03ced13af040af9b342c)
#[derive(Clone, Default)]
pub struct GltfImage {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub uri: Option<String>,
    pub mime_type: Option<String>,
    pub buffer_view: Option<f64>,
}
impl PartialEq for GltfImage {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/GltfSchema.ts:279 (sha256:5285063ac147a4a6df3717447f73a61a2b8f1e28fd665fcda8ae0deb6d5a0dc2)
#[derive(Clone, Default)]
pub struct GltfScene3D {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub name: Option<String>,
    pub nodes: Option<Vec<f64>>,
}
impl PartialEq for GltfScene3D {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/GltfSchema.ts:284 (sha256:afc38d35e5d3d90a57875bee4cc3b27ce036d83c9a71a2dc1cf9cc87c49a48b1)
#[derive(Clone, Default)]
pub struct GltfNodeRecord1 {
    pub __flight_identity: std::sync::Arc<()>,
    pub light: f64,
}
impl PartialEq for GltfNodeRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct GltfNodeRecord2 {
    pub __flight_identity: std::sync::Arc<()>,
    pub khr_lights_punctual: Option<GltfNodeRecord1>,
}
impl PartialEq for GltfNodeRecord2 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct GltfNode {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub camera: Option<f64>,
    pub name: Option<String>,
    pub children: Option<Vec<f64>>,
    pub mesh: Option<f64>,
    pub skin: Option<f64>,
    pub matrix: Option<Vec<f64>>,
    pub translation: Option<Vec<f64>>,
    pub rotation: Option<Vec<f64>>,
    pub scale: Option<Vec<f64>>,
    pub extensions: Option<GltfNodeRecord2>,
}
impl PartialEq for GltfNode {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/GltfSchema.ts:302 (sha256:99d5895ce5d78c85874856583b91e5e3846638f94c7c4960052e0e50ebcd7e42)
#[derive(Clone, Default)]
pub struct GltfSkin {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub name: Option<String>,
    pub inverse_bind_matrices: Option<f64>,
    pub joints: Vec<f64>,
    pub skeleton: Option<f64>,
}
impl PartialEq for GltfSkin {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/GltfSchema.ts:309 (sha256:5aede4ffd3b27f4ce025fb39ae3066b9f955001895ec39a6ca333c4a5e3afce9)
#[derive(Clone, Default)]
pub struct GltfMesh {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub name: Option<String>,
    pub primitives: Vec<GltfPrimitive>,
    pub weights: Option<Vec<f64>>,
}
impl PartialEq for GltfMesh {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/GltfSchema.ts:320 (sha256:39e8583ed5ad4ff1c6b673277ffe8417fc5edb0df25b62d22caf5e739fe6ef6b)
#[derive(Clone, Default)]
pub struct GltfMorphTarget {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub normal: Option<f64>,
    pub position: Option<f64>,
    pub tangent: Option<f64>,
}
impl PartialEq for GltfMorphTarget {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/GltfSchema.ts:326 (sha256:b1593f698fc60799b6adb2664ed58ade70d87a995c9c1acc161188164456f74a)
#[derive(Clone, Default)]
pub struct GltfPrimitiveRecord1 {
    pub __flight_identity: std::sync::Arc<()>,
    pub khr_draco_mesh_compression: Option<GltfDracoMeshCompression>,
}
impl PartialEq for GltfPrimitiveRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct GltfPrimitiveRecord2 {
    pub __flight_identity: std::sync::Arc<()>,
    pub joints_0: Option<f64>,
    pub normal: Option<f64>,
    pub position: Option<f64>,
    pub tangent: Option<f64>,
    pub texcoord_0: Option<f64>,
    pub weights_0: Option<f64>,
}
impl PartialEq for GltfPrimitiveRecord2 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct GltfPrimitive {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub attributes: GltfPrimitiveRecord2,
    pub indices: Option<f64>,
    pub material: Option<f64>,
    pub mode: Option<f64>,
    pub extensions: Option<GltfPrimitiveRecord1>,
    pub targets: Option<Vec<GltfMorphTarget>>,
}
impl PartialEq for GltfPrimitive {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/GltfSchema.ts:350 (sha256:4b5a42d25a9a6682eea2c9b469f9ec5bdbdd9ca5473d5aea89b9dc8f38950a91)
#[derive(Clone, Default)]
pub struct GltfDracoMeshCompression {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub attributes: Vec<(String, f64)>,
    pub buffer_view: f64,
}
impl PartialEq for GltfDracoMeshCompression {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/GltfSchema.ts:356 (sha256:060a4edc94d656a61aa8a6534804a3d3caa5790b821ec7e46a42bebf1854dec3)
pub type GltfComponentType = f64;

// Source: upstream/packages/types/src/GltfSchema.ts:358 (sha256:03dc7fe0581a9c7dbc63705b22e8ac86750109328f7ff0b99fb0af259bd64851)
#[derive(Clone, Default)]
pub struct GltfAccessor {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub buffer_view: Option<f64>,
    pub byte_offset: Option<f64>,
    pub component_type: GltfComponentType,
    pub count: f64,
    pub normalized: Option<bool>,
    pub type_: String,
    pub sparse: Option<GltfAccessorSparse>,
}
impl PartialEq for GltfAccessor {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/GltfSchema.ts:372 (sha256:22e3169e6f7ca127b441532e8e6022724030e909f77bb315ba20ec1c0acea528)
#[derive(Clone, Default)]
pub struct GltfAccessorSparseRecord1 {
    pub __flight_identity: std::sync::Arc<()>,
    pub buffer_view: f64,
    pub byte_offset: Option<f64>,
}
impl PartialEq for GltfAccessorSparseRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct GltfAccessorSparseRecord2 {
    pub __flight_identity: std::sync::Arc<()>,
    pub buffer_view: f64,
    pub byte_offset: Option<f64>,
    pub component_type: f64,
}
impl PartialEq for GltfAccessorSparseRecord2 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct GltfAccessorSparse {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub count: f64,
    pub indices: GltfAccessorSparseRecord2,
    pub values: GltfAccessorSparseRecord1,
}
impl PartialEq for GltfAccessorSparse {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/GltfSchema.ts:378 (sha256:c5f8dca8a790fc9a896e9d555454fb836a8b608b1cb02b2b1551edc76d893dba)
#[derive(Clone, Default)]
pub struct GltfBufferView {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub buffer: f64,
    pub byte_length: f64,
    pub byte_offset: Option<f64>,
    pub byte_stride: Option<f64>,
}
impl PartialEq for GltfBufferView {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/GltfSchema.ts:385 (sha256:c8d71510538c1b8579f77d9c8e91250764f5815db704463db69ef95127c64e29)
#[derive(Clone, Default)]
pub struct GltfBuffer {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub byte_length: f64,
    pub uri: Option<String>,
}
impl PartialEq for GltfBuffer {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
