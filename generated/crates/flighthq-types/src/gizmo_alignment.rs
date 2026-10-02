// @generated from upstream/packages/types/src/GizmoAlignment.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/GizmoAlignment.ts:1 (sha256:0e62550361b92e8741e31ce8286ee0e04f018c34f6769ab2c379637777398a11)
pub type GizmoAlignment = String;

// Source: upstream/packages/types/src/GizmoAlignment.ts:4 (sha256:e9e04744264e652219e4bdfb8cdb42c8b7312d8a44fdbff4deb565651015ef6d)
#[derive(Clone, Default)]
pub struct GizmoSmartGuideResult {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub delta_x: f64,
    pub delta_y: f64,
    pub guide_x: Option<f64>,
    pub guide_y: Option<f64>,
}
impl PartialEq for GizmoSmartGuideResult {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
