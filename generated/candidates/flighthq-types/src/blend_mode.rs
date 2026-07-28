// @generated from upstream/packages/types/src/BlendMode.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/BlendMode.ts:16 (sha256:4e5b5258b1f5c2f68304f09935f1ec144026db13f7cb742b3d238e7d2a06d012)
#[derive(Clone)]
pub struct BlendModeValues {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub add: String,
    pub darken: String,
    pub lighten: String,
    pub multiply: String,
    pub normal: String,
    pub screen: String,
}
impl PartialEq for BlendModeValues {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub static BLEND_MODE: std::sync::LazyLock<BlendModeValues> =
    std::sync::LazyLock::new(|| BlendModeValues {
        __flight_identity: std::sync::Arc::new(()),
        add: "Add".to_owned(),
        darken: "Darken".to_owned(),
        lighten: "Lighten".to_owned(),
        multiply: "Multiply".to_owned(),
        normal: "Normal".to_owned(),
        screen: "Screen".to_owned(),
    });

// Source: upstream/packages/types/src/BlendMode.ts:25 (sha256:4f6c55129c8b1b15188a56aacc435998f0ea3a2f79600827c8279010bdffe586)
pub type BlendMode = String;
