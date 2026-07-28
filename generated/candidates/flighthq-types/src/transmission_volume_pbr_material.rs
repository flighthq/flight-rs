// @generated from upstream/packages/types/src/TransmissionVolumePbrMaterial.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{
    AlphaType, BlendMode, Kind, MaterialAlphaMode, StandardPbrMaterialProperties, Texture,
};

// Source: upstream/packages/types/src/TransmissionVolumePbrMaterial.ts:11 (sha256:a9dcfc45f73df391d7747d9e18ba602175b8421cc16144dbe110e62d58258b23)
#[derive(Clone, Default)]
pub struct TransmissionVolumePbrMaterial {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub kind: Kind,
    pub name: Option<String>,
    pub alpha_cutoff: f64,
    pub alpha_mode: MaterialAlphaMode,
    pub alpha_type: AlphaType,
    pub blend_mode: BlendMode,
    pub double_sided: bool,
    pub shader_key: String,
    pub textures: Option<Vec<(String, Texture)>>,
    pub uniforms: Option<Vec<(String, crate::FlightUnion2<f64, Vec<f64>>)>>,
    pub attenuation_color: f64,
    pub attenuation_distance: f64,
    pub ior: f64,
    pub standard: StandardPbrMaterialProperties,
    pub thickness: f64,
    pub thickness_map: Option<Texture>,
    pub transmission: f64,
    pub transmission_map: Option<Texture>,
}
impl PartialEq for TransmissionVolumePbrMaterial {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/TransmissionVolumePbrMaterial.ts:22 (sha256:37afb6e517436bf4a14c8f056b62c74bfa6006312e12fc5740dd49392ca86ae5)
pub const TRANSMISSION_VOLUME_PBR_MATERIAL_KIND: &'static str = "TransmissionVolumePbrMaterial";
