// @generated from upstream/packages/effects/src/renderEffectInputs.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::{RenderEffect, RenderEffectInput};

// Source: upstream/packages/effects/src/renderEffectInputs.ts:6 (sha256:8e534120dcdbf329e9116561445cd1a478f1b19214d6ca856900967e326493ab)
pub fn get_render_effect_inputs(effect: &RenderEffect) -> Vec<RenderEffectInput> {
    return RENDER_EFFECT_INPUTS
        .iter()
        .find(|(key, _)| key == &(effect.kind).clone())
        .map(|(_, value)| value)
        .expect("TypeScript Record key was absent")
        .clone();
}

// Source: upstream/packages/effects/src/renderEffectInputs.ts:12 (sha256:db943485909439bf70566b8da8fd95459f9bbd14193b2f378bd10082031894fd)
pub fn get_render_effect_kinds() -> Vec<String> {
    return ((*RENDER_EFFECT_KINDS).clone()).clone();
}

// Source: upstream/packages/effects/src/renderEffectInputs.ts:20 (sha256:521f6769122f8081687ddf9766be96a1f99c6ac738d7569f8f0963a5ca8160f1)
static RENDER_EFFECT_INPUTS: std::sync::LazyLock<Vec<(String, Vec<RenderEffectInput>)>> =
    std::sync::LazyLock::new(|| {
        let mut __flight_record = Vec::new();
        __flight_record.push(("AutoExposureEffect".to_owned(), vec!["Hdr".to_owned()]));
        __flight_record.push(("BloomEffect".to_owned(), vec!["Hdr".to_owned()]));
        __flight_record.push((
            "BokehDepthOfFieldEffect".to_owned(),
            vec!["Depth".to_owned()],
        ));
        __flight_record.push((
            "CameraMotionBlurEffect".to_owned(),
            vec!["Motion".to_owned()],
        ));
        __flight_record.push(("ContactShadowsEffect".to_owned(), vec!["Depth".to_owned()]));
        __flight_record.push(("MotionBlurEffect".to_owned(), vec!["Motion".to_owned()]));
        __flight_record.push(("ScreenSpaceFogEffect".to_owned(), vec!["Depth".to_owned()]));
        __flight_record.push(("SsaoEffect".to_owned(), vec!["Depth".to_owned()]));
        __flight_record.push(("SsrEffect".to_owned(), vec!["Depth".to_owned()]));
        __flight_record.push(("TaaEffect".to_owned(), vec!["Temporal".to_owned()]));
        __flight_record.push(("TiltShiftEffect".to_owned(), vec!["Depth".to_owned()]));
        __flight_record.push(("ToneMapEffect".to_owned(), vec!["Hdr".to_owned()]));
        __flight_record.push(("VolumetricLightEffect".to_owned(), vec!["Depth".to_owned()]));
        __flight_record
    });

// Source: upstream/packages/effects/src/renderEffectInputs.ts:38 (sha256:b3c86ec4be9d41ebbfc855b5b5fbc2e41e2d656fe13358ac8c462c1af7e01928)
pub static RENDER_EFFECT_KINDS: std::sync::LazyLock<Vec<String>> = std::sync::LazyLock::new(|| {
    vec![
        "AutoExposureEffect".to_owned(),
        "BarrelDistortionEffect".to_owned(),
        "BevelEffect".to_owned(),
        "BlendEffect".to_owned(),
        "BloomEffect".to_owned(),
        "BlurEffect".to_owned(),
        "BokehDepthOfFieldEffect".to_owned(),
        "CameraMotionBlurEffect".to_owned(),
        "ChromaticAberrationEffect".to_owned(),
        "ContactShadowsEffect".to_owned(),
        "ConvolutionEffect".to_owned(),
        "CrtEffect".to_owned(),
        "CustomShaderEffect".to_owned(),
        "DirectionalBlurEffect".to_owned(),
        "DisplacementEffect".to_owned(),
        "DitherEffect".to_owned(),
        "DropShadowEffect".to_owned(),
        "FilmEmulationEffect".to_owned(),
        "FilmGrainEffect".to_owned(),
        "FxaaEffect".to_owned(),
        "GlitchEffect".to_owned(),
        "GodRaysEffect".to_owned(),
        "GradientBevelEffect".to_owned(),
        "GradientGlowEffect".to_owned(),
        "HalftoneEffect".to_owned(),
        "InnerGlowEffect".to_owned(),
        "InnerShadowEffect".to_owned(),
        "KuwaharaEffect".to_owned(),
        "LensDirtEffect".to_owned(),
        "LensDistortionEffect".to_owned(),
        "LensFlareEffect".to_owned(),
        "MedianEffect".to_owned(),
        "MotionBlurEffect".to_owned(),
        "OuterGlowEffect".to_owned(),
        "OutlineEffect".to_owned(),
        "PanniniProjectionEffect".to_owned(),
        "PixelateEffect".to_owned(),
        "PosterizeEffect".to_owned(),
        "RadialBlurEffect".to_owned(),
        "ScanlinesEffect".to_owned(),
        "ScreenSpaceFogEffect".to_owned(),
        "SharpenEffect".to_owned(),
        "SketchEffect".to_owned(),
        "SmaaEffect".to_owned(),
        "SsaoEffect".to_owned(),
        "SsrEffect".to_owned(),
        "TaaEffect".to_owned(),
        "TiltShiftEffect".to_owned(),
        "ToneMapEffect".to_owned(),
        "VignetteEffect".to_owned(),
        "VolumetricLightEffect".to_owned(),
        "WhiteBalanceEffect".to_owned(),
    ]
});
