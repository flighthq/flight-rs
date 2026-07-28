// @generated from upstream/packages/types/src/HemisphereLight.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/HemisphereLight.ts:5 (sha256:68701c0cc28dd9805f8d8fb3d66e5ef450aebc299ed1638873ae1bef483dc713)
#[derive(Clone)]
pub struct HemisphereLight {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub kind: String,
    pub ground_color: f64,
    pub intensity: f64,
    pub sky_color: f64,
}
impl PartialEq for HemisphereLight {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/HemisphereLight.ts:12 (sha256:268a35ad8a408e13922dadea642d2b952717c9e53a8781f99f739e86a679b98a)
pub const HEMISPHERE_LIGHT_KIND: &'static str = "HemisphereLight";
