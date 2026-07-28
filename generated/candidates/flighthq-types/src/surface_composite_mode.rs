// @generated from upstream/packages/types/src/SurfaceCompositeMode.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/SurfaceCompositeMode.ts:12 (sha256:eabe6eecbd50db07c50b07fe9a8f69e42757f5e66c7498cae41d756f296ad9e0)
#[derive(Clone)]
pub struct SurfaceCompositeModeValues {
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
impl PartialEq for SurfaceCompositeModeValues {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub static SURFACE_COMPOSITE_MODE: std::sync::LazyLock<SurfaceCompositeModeValues> =
    std::sync::LazyLock::new(|| SurfaceCompositeModeValues {
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

// Source: upstream/packages/types/src/SurfaceCompositeMode.ts:41 (sha256:254e2b667d53978844b5189708134dc4153a6363a36ad1924cffcc83994d21a6)
pub type SurfaceCompositeMode = String;
