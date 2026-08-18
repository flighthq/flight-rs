// @generated from upstream/packages/types/src/AdvancedBlendMode.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/AdvancedBlendMode.ts:27 (sha256:be037cb98a35e427b901acc356ea2834fdfc70e0ae2391c0de6495d34076d940)
#[derive(Clone, Default)]
pub struct AdvancedBlendModeValues {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub color: String,
    pub color_burn: String,
    pub color_dodge: String,
    pub darken: String,
    pub difference: String,
    pub exclusion: String,
    pub hard_light: String,
    pub hue: String,
    pub lighten: String,
    pub luminosity: String,
    pub overlay: String,
    pub saturation: String,
    pub soft_light: String,
}
impl PartialEq for AdvancedBlendModeValues {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub static ADVANCED_BLEND_MODE: std::sync::LazyLock<AdvancedBlendModeValues> =
    std::sync::LazyLock::new(|| AdvancedBlendModeValues {
        __flight_identity: std::sync::Arc::new(()),
        color: "Color".to_owned(),
        color_burn: "ColorBurn".to_owned(),
        color_dodge: "ColorDodge".to_owned(),
        darken: "Darken".to_owned(),
        difference: "Difference".to_owned(),
        exclusion: "Exclusion".to_owned(),
        hard_light: "HardLight".to_owned(),
        hue: "Hue".to_owned(),
        lighten: "Lighten".to_owned(),
        luminosity: "Luminosity".to_owned(),
        overlay: "Overlay".to_owned(),
        saturation: "Saturation".to_owned(),
        soft_light: "SoftLight".to_owned(),
    });

// Source: upstream/packages/types/src/AdvancedBlendMode.ts:43 (sha256:25438bb79392c179d6d1b277bfe5874c72ac2454536e9167f524ab61e6de88f9)
pub type AdvancedBlendMode = String;
