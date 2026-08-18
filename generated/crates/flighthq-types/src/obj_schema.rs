// @generated from upstream/packages/types/src/ObjSchema.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/ObjSchema.ts:15 (sha256:a0ff3bb6c313241d67f54ed79691cf0d9db0c05f32660cb6cadfebb83f7a847a)
#[derive(Clone, Default)]
pub struct ObjMaterial {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub ambient: Vec<f64>,
    pub anisotropy: Option<f64>,
    pub anisotropy_rotation: Option<f64>,
    pub clearcoat: Option<f64>,
    pub clearcoat_roughness: Option<f64>,
    pub diffuse: Vec<f64>,
    pub dissolve: f64,
    pub emissive: Option<Vec<f64>>,
    pub illumination: f64,
    pub map_ambient: Option<String>,
    pub map_bump: Option<String>,
    pub map_diffuse: Option<String>,
    pub map_dissolve: Option<String>,
    pub map_emissive: Option<String>,
    pub map_metallic: Option<String>,
    pub map_normal: Option<String>,
    pub map_roughness: Option<String>,
    pub map_specular: Option<String>,
    pub metallic: Option<f64>,
    pub name: String,
    pub roughness: Option<f64>,
    pub sheen: Option<f64>,
    pub specular: Vec<f64>,
    pub specular_exponent: f64,
}
impl PartialEq for ObjMaterial {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/ObjSchema.ts:69 (sha256:3626b9f608cce766361e792daf164174173bb55816dee1f3729ad8e38f2aae6b)
#[derive(Clone, Default)]
pub struct ObjMaterialLibrary {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub materials: Vec<(String, ObjMaterial)>,
}
impl PartialEq for ObjMaterialLibrary {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
