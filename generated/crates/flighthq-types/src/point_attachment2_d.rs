// @generated from upstream/packages/types/src/PointAttachment2D.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/PointAttachment2D.ts:13 (sha256:8b6a11c7ae1419ecec6e1cc58e9b3fe2f0501773aef0e74d574670c48299165a)
#[derive(Clone, Default)]
pub struct PointAttachment2D {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub kind: String,
    pub name: Option<String>,
    pub rotation: f64,
    pub x: f64,
    pub y: f64,
}
impl PartialEq for PointAttachment2D {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/PointAttachment2D.ts:20 (sha256:1ccef8406ba186476f2fdd0c10153f4d598bf031b84a6f2a83bcf63d59e87075)
pub const POINT_ATTACHMENT2_D_KIND: &'static str = "PointAttachment2D";
