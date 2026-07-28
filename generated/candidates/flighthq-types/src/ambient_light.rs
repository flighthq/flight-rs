// @generated from upstream/packages/types/src/AmbientLight.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/AmbientLight.ts:5 (sha256:038c7c0147123d4e1e5d7f4c3520a6d94a50bf8dbfe80a02f4cf0df03fccb7be)
#[derive(Clone)]
pub struct AmbientLight {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub kind: String,
    pub color: f64,
    pub intensity: f64,
}
impl PartialEq for AmbientLight {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/AmbientLight.ts:11 (sha256:788fd63e0464486184100bc1e10435a7ea3ff99bcfb70addcaa3c98ccdad28dc)
pub const AMBIENT_LIGHT_KIND: &'static str = "AmbientLight";
