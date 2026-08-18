// @generated from upstream/packages/types/src/BitmapCompositeMode.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/BitmapCompositeMode.ts:12 (sha256:5cfd43cb55c41bff3c46b1422537761e5848693c284a45a5a7c976a043ddacba)
#[derive(Clone, Default)]
pub struct BitmapCompositeModeValues {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub add: String,
    pub clear: String,
    pub color_burn: String,
    pub color_dodge: String,
    pub copy: String,
    pub darken: String,
    pub destination_atop: String,
    pub destination_in: String,
    pub destination_out: String,
    pub destination_over: String,
    pub difference: String,
    pub exclusion: String,
    pub hard_light: String,
    pub invert: String,
    pub lighten: String,
    pub multiply: String,
    pub normal: String,
    pub overlay: String,
    pub screen: String,
    pub soft_light: String,
    pub source_atop: String,
    pub source_in: String,
    pub source_out: String,
    pub source_over: String,
    pub subtract: String,
    pub xor: String,
}
impl PartialEq for BitmapCompositeModeValues {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub static BITMAP_COMPOSITE_MODE: std::sync::LazyLock<BitmapCompositeModeValues> =
    std::sync::LazyLock::new(|| BitmapCompositeModeValues {
        __flight_identity: std::sync::Arc::new(()),
        add: "Add".to_owned(),
        clear: "Clear".to_owned(),
        color_burn: "ColorBurn".to_owned(),
        color_dodge: "ColorDodge".to_owned(),
        copy: "Copy".to_owned(),
        darken: "Darken".to_owned(),
        destination_atop: "DestinationAtop".to_owned(),
        destination_in: "DestinationIn".to_owned(),
        destination_out: "DestinationOut".to_owned(),
        destination_over: "DestinationOver".to_owned(),
        difference: "Difference".to_owned(),
        exclusion: "Exclusion".to_owned(),
        hard_light: "HardLight".to_owned(),
        invert: "Invert".to_owned(),
        lighten: "Lighten".to_owned(),
        multiply: "Multiply".to_owned(),
        normal: "Normal".to_owned(),
        overlay: "Overlay".to_owned(),
        screen: "Screen".to_owned(),
        soft_light: "SoftLight".to_owned(),
        source_atop: "SourceAtop".to_owned(),
        source_in: "SourceIn".to_owned(),
        source_out: "SourceOut".to_owned(),
        source_over: "SourceOver".to_owned(),
        subtract: "Subtract".to_owned(),
        xor: "Xor".to_owned(),
    });

// Source: upstream/packages/types/src/BitmapCompositeMode.ts:41 (sha256:3a669b9de9962a59bd68a5de8f5271fb5aaa4a6d4c3e210b256d356dccb90812)
pub type BitmapCompositeMode = String;
