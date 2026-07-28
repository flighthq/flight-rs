// @generated from upstream/packages/types/src/ModifierSlot.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/ModifierSlot.ts:18 (sha256:39f229da2717033e4fa985dbc4d87edcd1c9f7ca4f651c9876c60f018ab108b5)
#[derive(Clone)]
pub struct ModifierSlotValues {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub diffuse: String,
    pub effect: String,
    pub emissive: String,
    pub normal: String,
    pub specular: String,
    pub vertex: String,
}
impl PartialEq for ModifierSlotValues {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub static MODIFIER_SLOT: std::sync::LazyLock<ModifierSlotValues> =
    std::sync::LazyLock::new(|| ModifierSlotValues {
        __flight_identity: std::sync::Arc::new(()),
        diffuse: "Diffuse".to_owned(),
        effect: "Effect".to_owned(),
        emissive: "Emissive".to_owned(),
        normal: "Normal".to_owned(),
        specular: "Specular".to_owned(),
        vertex: "Vertex".to_owned(),
    });

// Source: upstream/packages/types/src/ModifierSlot.ts:27 (sha256:932db9db951da88867a47b2544214af8aa3cfbdafd673e01e3c347a32397b0ba)
pub type ModifierSlot = String;
