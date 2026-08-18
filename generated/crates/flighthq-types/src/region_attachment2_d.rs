// @generated from upstream/packages/types/src/RegionAttachment2D.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/RegionAttachment2D.ts:10 (sha256:40a66fe8322fa267e67064792d5723c3deb89468073fc92abe8cb4df5603bcbf)
#[derive(Clone, Default)]
pub struct RegionAttachment2D {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub kind: String,
    pub name: Option<String>,
    pub height: f64,
    pub rotation: f64,
    pub scale_x: f64,
    pub scale_y: f64,
    pub width: f64,
    pub x: f64,
    pub y: f64,
}
impl PartialEq for RegionAttachment2D {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/RegionAttachment2D.ts:20 (sha256:98fde18a5807eef9f5b1104707d8dced62a2fa5b1cf9e6ee2e9e97bff83cf18d)
pub const REGION_ATTACHMENT2_D_KIND: &'static str = "RegionAttachment2D";
