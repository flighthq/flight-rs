// @generated from upstream/packages/effects/src/renderEffectDefaults.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::RenderEffect;

// Source: upstream/packages/effects/src/renderEffectDefaults.ts:12 (sha256:026a7af589b7e7987ffbe4bfe04ffab31ac80701a4c415478623c93f77160df7)
#[derive(Clone)]
struct GetRenderEffectDefaultsRecord1 {
    __flight_identity: std::sync::Arc<()>,
}
impl PartialEq for GetRenderEffectDefaultsRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn get_render_effect_defaults(kind: String) -> Vec<(String, crate::OpaqueHostValue)> {
    let entry = DEFAULTS
        .iter()
        .find(|(key, _)| key == &(kind).clone())
        .map(|(_, value)| value)
        .expect("TypeScript Record key was absent")
        .clone();
    if false {
        return {
            let mut __flight_record = Vec::new();
            __flight_record
        };
    }
    return (entry).clone();
}

// Source: upstream/packages/effects/src/renderEffectDefaults.ts:25 (sha256:04a3be167a91b72060e8e19aa030b5439b865ec9648b860a734eafb372275e71)
pub fn normalize_render_effect(effect: &RenderEffect, out: &mut RenderEffect) -> bool {
    let entry = DEFAULTS
        .iter()
        .find(|(key, _)| key == &(effect.kind).clone())
        .map(|(_, value)| value)
        .expect("TypeScript Record key was absent")
        .clone();
    if false {
        return false;
    }
    let effect_rec = effect;
    let mut out_rec = out;
    for key in (crate::host_value::<()>("host.keys")).iter().cloned() {
        out_rec
            .iter()
            .find(|(key, _)| key == &key)
            .map(|(_, value)| value)
            .expect("TypeScript Record key was absent") = if (effect_rec
            .iter()
            .find(|(key, _)| key == &key)
            .map(|(_, value)| value)
            .expect("TypeScript Record key was absent")
            .clone())
        .is_some()
        {
            effect_rec
                .iter()
                .find(|(key, _)| key == &key)
                .map(|(_, value)| value)
                .expect("TypeScript Record key was absent")
                .clone()
        } else {
            entry
                .iter()
                .find(|(key, _)| key == &key)
                .map(|(_, value)| value)
                .expect("TypeScript Record key was absent")
                .clone()
        };
    }
    for key in (crate::host_value::<()>("host.keys")).iter().cloned() {
        if (!{
            let __flight_key = key;
            entry.iter().any(|(key, _)| key == &__flight_key)
        }) {
            out_rec
                .iter()
                .find(|(key, _)| key == &key)
                .map(|(_, value)| value)
                .expect("TypeScript Record key was absent") = effect_rec
                .iter()
                .find(|(key, _)| key == &key)
                .map(|(_, value)| value)
                .expect("TypeScript Record key was absent")
                .clone();
        }
    }
    return true;
}

// Source: upstream/packages/effects/src/renderEffectDefaults.ts:45 (sha256:4dda80e2c8be73ff8cd0146fac1fd4e23086dc254e6b9a36371d7719a055efd8)
static DEFAULTS: std::sync::LazyLock<Vec<(String, Vec<(String, crate::OpaqueHostValue)>)>> =
    std::sync::LazyLock::new(|| {
        let mut __flight_record = Vec::new();
        __flight_record.push(("AutoExposureEffect".to_owned(), {
            let mut __flight_record = Vec::new();
            __flight_record.push((
                "adaptationSpeed".to_owned(),
                crate::OpaqueHostValue::Number(1.0_f64),
            ));
            __flight_record.push((
                "exposureCompensation".to_owned(),
                crate::OpaqueHostValue::Number(0.0_f64),
            ));
            __flight_record.push((
                "maxExposure".to_owned(),
                crate::OpaqueHostValue::Number(2.0_f64),
            ));
            __flight_record.push(("minExposure".to_owned(), (-2.0_f64)));
            __flight_record
        }));
        __flight_record.push(("BarrelDistortionEffect".to_owned(), {
            let mut __flight_record = Vec::new();
            __flight_record.push(("amount".to_owned(), crate::OpaqueHostValue::Number(0.5_f64)));
            __flight_record.push(("scale".to_owned(), crate::OpaqueHostValue::Number(0.9_f64)));
            __flight_record
        }));
        __flight_record.push(("BevelEffect".to_owned(), {
            let mut __flight_record = Vec::new();
            __flight_record.push(("angle".to_owned(), crate::OpaqueHostValue::Number(45.0_f64)));
            __flight_record.push((
                "bevelType".to_owned(),
                crate::OpaqueHostValue::String("inner".to_owned()),
            ));
            __flight_record.push(("blurX".to_owned(), crate::OpaqueHostValue::Number(4.0_f64)));
            __flight_record.push(("blurY".to_owned(), crate::OpaqueHostValue::Number(4.0_f64)));
            __flight_record.push((
                "distance".to_owned(),
                crate::OpaqueHostValue::Number(4.0_f64),
            ));
            __flight_record.push((
                "highlightAlpha".to_owned(),
                crate::OpaqueHostValue::Number(1.0_f64),
            ));
            __flight_record.push((
                "highlightColor".to_owned(),
                crate::OpaqueHostValue::Number(16777215.0_f64),
            ));
            __flight_record.push((
                "quality".to_owned(),
                crate::OpaqueHostValue::Number(1.0_f64),
            ));
            __flight_record.push((
                "shadowAlpha".to_owned(),
                crate::OpaqueHostValue::Number(1.0_f64),
            ));
            __flight_record.push((
                "shadowColor".to_owned(),
                crate::OpaqueHostValue::Number(0.0_f64),
            ));
            __flight_record.push((
                "sourceMode".to_owned(),
                crate::OpaqueHostValue::String("draw".to_owned()),
            ));
            __flight_record.push((
                "strength".to_owned(),
                crate::OpaqueHostValue::Number(1.0_f64),
            ));
            __flight_record
        }));
        __flight_record.push(("BlendEffect".to_owned(), {
            let mut __flight_record = Vec::new();
            __flight_record.push((
                "opacity".to_owned(),
                crate::OpaqueHostValue::Number(1.0_f64),
            ));
            __flight_record
        }));
        __flight_record.push(("BloomEffect".to_owned(), {
            let mut __flight_record = Vec::new();
            __flight_record.push((
                "brightness".to_owned(),
                crate::OpaqueHostValue::Number(1.0_f64),
            ));
            __flight_record.push((
                "mipCount".to_owned(),
                crate::OpaqueHostValue::Number(0.0_f64),
            ));
            __flight_record.push(("passes".to_owned(), crate::OpaqueHostValue::Number(1.0_f64)));
            __flight_record.push(("radius".to_owned(), crate::OpaqueHostValue::Number(8.0_f64)));
            __flight_record.push((
                "threshold".to_owned(),
                crate::OpaqueHostValue::Number(0.8_f64),
            ));
            __flight_record.push((
                "thresholdKnee".to_owned(),
                crate::OpaqueHostValue::Number(0.5_f64),
            ));
            __flight_record
        }));
        __flight_record.push(("BlurEffect".to_owned(), {
            let mut __flight_record = Vec::new();
            __flight_record.push(("blurX".to_owned(), crate::OpaqueHostValue::Number(4.0_f64)));
            __flight_record.push(("blurY".to_owned(), crate::OpaqueHostValue::Number(4.0_f64)));
            __flight_record
        }));
        __flight_record.push(("BokehDepthOfFieldEffect".to_owned(), {
            let mut __flight_record = Vec::new();
            __flight_record.push((
                "apertureBlades".to_owned(),
                crate::OpaqueHostValue::Number(6.0_f64),
            ));
            __flight_record.push((
                "maxBlurRadius".to_owned(),
                crate::OpaqueHostValue::Number(16.0_f64),
            ));
            __flight_record.push((
                "samples".to_owned(),
                crate::OpaqueHostValue::Number(16.0_f64),
            ));
            __flight_record
        }));
        __flight_record.push(("CameraMotionBlurEffect".to_owned(), {
            let mut __flight_record = Vec::new();
            __flight_record.push((
                "samples".to_owned(),
                crate::OpaqueHostValue::Number(8.0_f64),
            ));
            __flight_record
        }));
        __flight_record.push(("ChromaticAberrationEffect".to_owned(), {
            let mut __flight_record = Vec::new();
            __flight_record.push((
                "fringeStrength".to_owned(),
                crate::OpaqueHostValue::Number(0.01_f64),
            ));
            __flight_record.push(("radial".to_owned(), crate::OpaqueHostValue::Bool(true)));
            __flight_record.push((
                "samples".to_owned(),
                crate::OpaqueHostValue::Number(3.0_f64),
            ));
            __flight_record
        }));
        __flight_record.push(("ContactShadowsEffect".to_owned(), {
            let mut __flight_record = Vec::new();
            __flight_record.push((
                "distance".to_owned(),
                crate::OpaqueHostValue::Number(0.5_f64),
            ));
            __flight_record.push((
                "opacity".to_owned(),
                crate::OpaqueHostValue::Number(0.6_f64),
            ));
            __flight_record.push((
                "samples".to_owned(),
                crate::OpaqueHostValue::Number(16.0_f64),
            ));
            __flight_record.push((
                "smoothness".to_owned(),
                crate::OpaqueHostValue::Number(0.5_f64),
            ));
            __flight_record
        }));
        __flight_record.push(("ConvolutionEffect".to_owned(), {
            let mut __flight_record = Vec::new();
            __flight_record.push(("bias".to_owned(), crate::OpaqueHostValue::Number(0.0_f64)));
            __flight_record.push(("clamp".to_owned(), crate::OpaqueHostValue::Bool(true)));
            __flight_record.push((
                "preserveAlpha".to_owned(),
                crate::OpaqueHostValue::Bool(true),
            ));
            __flight_record
        }));
        __flight_record.push(("CrtEffect".to_owned(), {
            let mut __flight_record = Vec::new();
            __flight_record.push((
                "curvature".to_owned(),
                crate::OpaqueHostValue::Number(0.1_f64),
            ));
            __flight_record.push((
                "scanlineIntensity".to_owned(),
                crate::OpaqueHostValue::Number(0.5_f64),
            ));
            __flight_record.push((
                "vignette".to_owned(),
                crate::OpaqueHostValue::Number(0.4_f64),
            ));
            __flight_record.push((
                "aberration".to_owned(),
                crate::OpaqueHostValue::Number(0.0_f64),
            ));
            __flight_record
        }));
        __flight_record.push(("DirectionalBlurEffect".to_owned(), {
            let mut __flight_record = Vec::new();
            __flight_record.push(("angle".to_owned(), crate::OpaqueHostValue::Number(0.0_f64)));
            __flight_record.push((
                "samples".to_owned(),
                crate::OpaqueHostValue::Number(8.0_f64),
            ));
            __flight_record
        }));
        __flight_record.push(("DisplacementEffect".to_owned(), {
            let mut __flight_record = Vec::new();
            __flight_record.push((
                "frequency".to_owned(),
                crate::OpaqueHostValue::Number(12.0_f64),
            ));
            __flight_record.push((
                "intensity".to_owned(),
                crate::OpaqueHostValue::Number(8.0_f64),
            ));
            __flight_record
        }));
        __flight_record.push(("DitherEffect".to_owned(), {
            let mut __flight_record = Vec::new();
            __flight_record.push((
                "levels".to_owned(),
                crate::OpaqueHostValue::Number(16.0_f64),
            ));
            __flight_record
        }));
        __flight_record.push(("DropShadowEffect".to_owned(), {
            let mut __flight_record = Vec::new();
            __flight_record.push(("alpha".to_owned(), crate::OpaqueHostValue::Number(1.0_f64)));
            __flight_record.push(("angle".to_owned(), crate::OpaqueHostValue::Number(45.0_f64)));
            __flight_record.push(("blurX".to_owned(), crate::OpaqueHostValue::Number(4.0_f64)));
            __flight_record.push(("blurY".to_owned(), crate::OpaqueHostValue::Number(4.0_f64)));
            __flight_record.push((
                "distance".to_owned(),
                crate::OpaqueHostValue::Number(4.0_f64),
            ));
            __flight_record.push((
                "quality".to_owned(),
                crate::OpaqueHostValue::Number(1.0_f64),
            ));
            __flight_record.push((
                "sourceMode".to_owned(),
                crate::OpaqueHostValue::String("draw".to_owned()),
            ));
            __flight_record.push((
                "strength".to_owned(),
                crate::OpaqueHostValue::Number(1.0_f64),
            ));
            __flight_record
        }));
        __flight_record.push(("FilmEmulationEffect".to_owned(), {
            let mut __flight_record = Vec::new();
            __flight_record.push((
                "gateWeave".to_owned(),
                crate::OpaqueHostValue::Number(0.0_f64),
            ));
            __flight_record.push((
                "grainIntensity".to_owned(),
                crate::OpaqueHostValue::Number(0.1_f64),
            ));
            __flight_record.push((
                "halationRadius".to_owned(),
                crate::OpaqueHostValue::Number(4.0_f64),
            ));
            __flight_record.push((
                "halationStrength".to_owned(),
                crate::OpaqueHostValue::Number(0.3_f64),
            ));
            __flight_record
        }));
        __flight_record.push(("FilmGrainEffect".to_owned(), {
            let mut __flight_record = Vec::new();
            __flight_record.push((
                "intensity".to_owned(),
                crate::OpaqueHostValue::Number(0.2_f64),
            ));
            __flight_record.push(("size".to_owned(), crate::OpaqueHostValue::Number(1.0_f64)));
            __flight_record.push(("seed".to_owned(), crate::OpaqueHostValue::Number(0.0_f64)));
            __flight_record
        }));
        __flight_record.push(("FxaaEffect".to_owned(), {
            let mut __flight_record = Vec::new();
            __flight_record.push((
                "edgeThreshold".to_owned(),
                crate::OpaqueHostValue::Number(0.0312_f64),
            ));
            __flight_record.push((
                "subpixel".to_owned(),
                crate::OpaqueHostValue::Number(0.75_f64),
            ));
            __flight_record
        }));
        __flight_record.push(("GlitchEffect".to_owned(), {
            let mut __flight_record = Vec::new();
            __flight_record.push((
                "blockSize".to_owned(),
                crate::OpaqueHostValue::Number(24.0_f64),
            ));
            __flight_record.push((
                "colorShift".to_owned(),
                crate::OpaqueHostValue::Number(8.0_f64),
            ));
            __flight_record.push((
                "intensity".to_owned(),
                crate::OpaqueHostValue::Number(0.5_f64),
            ));
            __flight_record.push(("seed".to_owned(), crate::OpaqueHostValue::Number(0.0_f64)));
            __flight_record
        }));
        __flight_record.push(("GodRaysEffect".to_owned(), {
            let mut __flight_record = Vec::new();
            __flight_record.push((
                "centerX".to_owned(),
                crate::OpaqueHostValue::Number(0.5_f64),
            ));
            __flight_record.push((
                "centerY".to_owned(),
                crate::OpaqueHostValue::Number(0.5_f64),
            ));
            __flight_record.push(("decay".to_owned(), crate::OpaqueHostValue::Number(0.96_f64)));
            __flight_record.push((
                "density".to_owned(),
                crate::OpaqueHostValue::Number(0.96_f64),
            ));
            __flight_record.push((
                "exposure".to_owned(),
                crate::OpaqueHostValue::Number(0.1_f64),
            ));
            __flight_record.push((
                "samples".to_owned(),
                crate::OpaqueHostValue::Number(100.0_f64),
            ));
            __flight_record.push(("weight".to_owned(), crate::OpaqueHostValue::Number(0.4_f64)));
            __flight_record
        }));
        __flight_record.push(("GradientBevelEffect".to_owned(), {
            let mut __flight_record = Vec::new();
            __flight_record.push(("angle".to_owned(), crate::OpaqueHostValue::Number(45.0_f64)));
            __flight_record.push((
                "bevelType".to_owned(),
                crate::OpaqueHostValue::String("inner".to_owned()),
            ));
            __flight_record.push(("blurX".to_owned(), crate::OpaqueHostValue::Number(4.0_f64)));
            __flight_record.push(("blurY".to_owned(), crate::OpaqueHostValue::Number(4.0_f64)));
            __flight_record.push((
                "distance".to_owned(),
                crate::OpaqueHostValue::Number(4.0_f64),
            ));
            __flight_record.push((
                "quality".to_owned(),
                crate::OpaqueHostValue::Number(1.0_f64),
            ));
            __flight_record.push((
                "sourceMode".to_owned(),
                crate::OpaqueHostValue::String("draw".to_owned()),
            ));
            __flight_record.push((
                "strength".to_owned(),
                crate::OpaqueHostValue::Number(1.0_f64),
            ));
            __flight_record
        }));
        __flight_record.push(("GradientGlowEffect".to_owned(), {
            let mut __flight_record = Vec::new();
            __flight_record.push(("blurX".to_owned(), crate::OpaqueHostValue::Number(6.0_f64)));
            __flight_record.push(("blurY".to_owned(), crate::OpaqueHostValue::Number(6.0_f64)));
            __flight_record.push((
                "quality".to_owned(),
                crate::OpaqueHostValue::Number(1.0_f64),
            ));
            __flight_record.push((
                "sourceMode".to_owned(),
                crate::OpaqueHostValue::String("draw".to_owned()),
            ));
            __flight_record.push((
                "strength".to_owned(),
                crate::OpaqueHostValue::Number(1.0_f64),
            ));
            __flight_record
        }));
        __flight_record.push(("HalftoneEffect".to_owned(), {
            let mut __flight_record = Vec::new();
            __flight_record.push((
                "angle".to_owned(),
                crate::OpaqueHostValue::Number(0.785_f64),
            ));
            __flight_record.push(("scale".to_owned(), crate::OpaqueHostValue::Number(8.0_f64)));
            __flight_record
        }));
        __flight_record.push(("InnerGlowEffect".to_owned(), {
            let mut __flight_record = Vec::new();
            __flight_record.push(("alpha".to_owned(), crate::OpaqueHostValue::Number(1.0_f64)));
            __flight_record.push(("blurX".to_owned(), crate::OpaqueHostValue::Number(6.0_f64)));
            __flight_record.push(("blurY".to_owned(), crate::OpaqueHostValue::Number(6.0_f64)));
            __flight_record.push((
                "color".to_owned(),
                crate::OpaqueHostValue::Number(16711680.0_f64),
            ));
            __flight_record.push((
                "quality".to_owned(),
                crate::OpaqueHostValue::Number(1.0_f64),
            ));
            __flight_record.push((
                "sourceMode".to_owned(),
                crate::OpaqueHostValue::String("draw".to_owned()),
            ));
            __flight_record.push((
                "strength".to_owned(),
                crate::OpaqueHostValue::Number(1.0_f64),
            ));
            __flight_record
        }));
        __flight_record.push(("InnerShadowEffect".to_owned(), {
            let mut __flight_record = Vec::new();
            __flight_record.push(("alpha".to_owned(), crate::OpaqueHostValue::Number(1.0_f64)));
            __flight_record.push(("angle".to_owned(), crate::OpaqueHostValue::Number(45.0_f64)));
            __flight_record.push(("blurX".to_owned(), crate::OpaqueHostValue::Number(4.0_f64)));
            __flight_record.push(("blurY".to_owned(), crate::OpaqueHostValue::Number(4.0_f64)));
            __flight_record.push((
                "distance".to_owned(),
                crate::OpaqueHostValue::Number(4.0_f64),
            ));
            __flight_record.push((
                "quality".to_owned(),
                crate::OpaqueHostValue::Number(1.0_f64),
            ));
            __flight_record.push((
                "sourceMode".to_owned(),
                crate::OpaqueHostValue::String("draw".to_owned()),
            ));
            __flight_record.push((
                "strength".to_owned(),
                crate::OpaqueHostValue::Number(1.0_f64),
            ));
            __flight_record
        }));
        __flight_record.push(("KuwaharaEffect".to_owned(), {
            let mut __flight_record = Vec::new();
            __flight_record.push(("radius".to_owned(), crate::OpaqueHostValue::Number(3.0_f64)));
            __flight_record
        }));
        __flight_record.push(("LensDirtEffect".to_owned(), {
            let mut __flight_record = Vec::new();
            __flight_record.push((
                "intensity".to_owned(),
                crate::OpaqueHostValue::Number(1.0_f64),
            ));
            __flight_record.push((
                "threshold".to_owned(),
                crate::OpaqueHostValue::Number(0.55_f64),
            ));
            __flight_record
        }));
        __flight_record.push(("LensDistortionEffect".to_owned(), {
            let mut __flight_record = Vec::new();
            __flight_record.push(("amount".to_owned(), crate::OpaqueHostValue::Number(0.5_f64)));
            __flight_record.push(("scale".to_owned(), crate::OpaqueHostValue::Number(0.9_f64)));
            __flight_record
        }));
        __flight_record.push(("LensFlareEffect".to_owned(), {
            let mut __flight_record = Vec::new();
            __flight_record.push(("ghosts".to_owned(), crate::OpaqueHostValue::Number(4.0_f64)));
            __flight_record.push(("halo".to_owned(), crate::OpaqueHostValue::Number(0.5_f64)));
            __flight_record.push((
                "intensity".to_owned(),
                crate::OpaqueHostValue::Number(1.0_f64),
            ));
            __flight_record.push((
                "threshold".to_owned(),
                crate::OpaqueHostValue::Number(0.9_f64),
            ));
            __flight_record
        }));
        __flight_record.push(("MedianEffect".to_owned(), {
            let mut __flight_record = Vec::new();
            __flight_record.push(("radius".to_owned(), crate::OpaqueHostValue::Number(1.0_f64)));
            __flight_record
        }));
        __flight_record.push(("MotionBlurEffect".to_owned(), {
            let mut __flight_record = Vec::new();
            __flight_record.push((
                "intensity".to_owned(),
                crate::OpaqueHostValue::Number(1.0_f64),
            ));
            __flight_record.push((
                "samples".to_owned(),
                crate::OpaqueHostValue::Number(8.0_f64),
            ));
            __flight_record.push((
                "shutterAngle".to_owned(),
                crate::OpaqueHostValue::Number(180.0_f64),
            ));
            __flight_record.push((
                "target".to_owned(),
                crate::OpaqueHostValue::String("both".to_owned()),
            ));
            __flight_record
        }));
        __flight_record.push(("OuterGlowEffect".to_owned(), {
            let mut __flight_record = Vec::new();
            __flight_record.push(("alpha".to_owned(), crate::OpaqueHostValue::Number(1.0_f64)));
            __flight_record.push(("blurX".to_owned(), crate::OpaqueHostValue::Number(6.0_f64)));
            __flight_record.push(("blurY".to_owned(), crate::OpaqueHostValue::Number(6.0_f64)));
            __flight_record.push((
                "color".to_owned(),
                crate::OpaqueHostValue::Number(16711680.0_f64),
            ));
            __flight_record.push((
                "quality".to_owned(),
                crate::OpaqueHostValue::Number(1.0_f64),
            ));
            __flight_record.push((
                "sourceMode".to_owned(),
                crate::OpaqueHostValue::String("draw".to_owned()),
            ));
            __flight_record.push((
                "strength".to_owned(),
                crate::OpaqueHostValue::Number(1.0_f64),
            ));
            __flight_record
        }));
        __flight_record.push(("OutlineEffect".to_owned(), {
            let mut __flight_record = Vec::new();
            __flight_record.push((
                "color".to_owned(),
                crate::OpaqueHostValue::Number(255.0_f64),
            ));
            __flight_record.push((
                "thickness".to_owned(),
                crate::OpaqueHostValue::Number(1.0_f64),
            ));
            __flight_record.push((
                "threshold".to_owned(),
                crate::OpaqueHostValue::Number(0.1_f64),
            ));
            __flight_record
        }));
        __flight_record.push(("PanniniProjectionEffect".to_owned(), {
            let mut __flight_record = Vec::new();
            __flight_record.push((
                "compression".to_owned(),
                crate::OpaqueHostValue::Number(0.5_f64),
            ));
            __flight_record.push(("crop".to_owned(), crate::OpaqueHostValue::Number(0.0_f64)));
            __flight_record
        }));
        __flight_record.push(("PixelateEffect".to_owned(), {
            let mut __flight_record = Vec::new();
            __flight_record.push(("size".to_owned(), crate::OpaqueHostValue::Number(8.0_f64)));
            __flight_record
        }));
        __flight_record.push(("PosterizeEffect".to_owned(), {
            let mut __flight_record = Vec::new();
            __flight_record.push(("levels".to_owned(), crate::OpaqueHostValue::Number(8.0_f64)));
            __flight_record
        }));
        __flight_record.push(("RadialBlurEffect".to_owned(), {
            let mut __flight_record = Vec::new();
            __flight_record.push((
                "centerX".to_owned(),
                crate::OpaqueHostValue::Number(0.5_f64),
            ));
            __flight_record.push((
                "centerY".to_owned(),
                crate::OpaqueHostValue::Number(0.5_f64),
            ));
            __flight_record.push((
                "samples".to_owned(),
                crate::OpaqueHostValue::Number(8.0_f64),
            ));
            __flight_record.push((
                "strength".to_owned(),
                crate::OpaqueHostValue::Number(0.1_f64),
            ));
            __flight_record
        }));
        __flight_record.push(("ScanlinesEffect".to_owned(), {
            let mut __flight_record = Vec::new();
            __flight_record.push((
                "count".to_owned(),
                crate::OpaqueHostValue::Number(480.0_f64),
            ));
            __flight_record.push((
                "intensity".to_owned(),
                crate::OpaqueHostValue::Number(0.25_f64),
            ));
            __flight_record
        }));
        __flight_record.push(("ScreenSpaceFogEffect".to_owned(), {
            let mut __flight_record = Vec::new();
            __flight_record.push((
                "density".to_owned(),
                crate::OpaqueHostValue::Number(0.5_f64),
            ));
            __flight_record.push(("far".to_owned(), crate::OpaqueHostValue::Number(1000.0_f64)));
            __flight_record.push(("near".to_owned(), crate::OpaqueHostValue::Number(10.0_f64)));
            __flight_record
        }));
        __flight_record.push(("SharpenEffect".to_owned(), {
            let mut __flight_record = Vec::new();
            __flight_record.push(("amount".to_owned(), crate::OpaqueHostValue::Number(0.5_f64)));
            __flight_record
        }));
        __flight_record.push(("SketchEffect".to_owned(), {
            let mut __flight_record = Vec::new();
            __flight_record.push((
                "strength".to_owned(),
                crate::OpaqueHostValue::Number(1.0_f64),
            ));
            __flight_record
        }));
        __flight_record.push(("SmaaEffect".to_owned(), {
            let mut __flight_record = Vec::new();
            __flight_record.push((
                "threshold".to_owned(),
                crate::OpaqueHostValue::Number(0.1_f64),
            ));
            __flight_record
        }));
        __flight_record.push(("SsaoEffect".to_owned(), {
            let mut __flight_record = Vec::new();
            __flight_record.push(("bias".to_owned(), crate::OpaqueHostValue::Number(0.025_f64)));
            __flight_record.push((
                "intensity".to_owned(),
                crate::OpaqueHostValue::Number(1.0_f64),
            ));
            __flight_record.push(("radius".to_owned(), crate::OpaqueHostValue::Number(0.5_f64)));
            __flight_record.push((
                "samples".to_owned(),
                crate::OpaqueHostValue::Number(16.0_f64),
            ));
            __flight_record
        }));
        __flight_record.push(("SsrEffect".to_owned(), {
            let mut __flight_record = Vec::new();
            __flight_record.push((
                "maxDistance".to_owned(),
                crate::OpaqueHostValue::Number(100.0_f64),
            ));
            __flight_record.push((
                "maxSteps".to_owned(),
                crate::OpaqueHostValue::Number(64.0_f64),
            ));
            __flight_record.push((
                "resolution".to_owned(),
                crate::OpaqueHostValue::Number(0.5_f64),
            ));
            __flight_record.push(("steps".to_owned(), crate::OpaqueHostValue::Number(64.0_f64)));
            __flight_record.push((
                "thickness".to_owned(),
                crate::OpaqueHostValue::Number(0.1_f64),
            ));
            __flight_record
        }));
        __flight_record.push(("TaaEffect".to_owned(), {
            let mut __flight_record = Vec::new();
            __flight_record.push((
                "feedback".to_owned(),
                crate::OpaqueHostValue::Number(0.9_f64),
            ));
            __flight_record
        }));
        __flight_record.push(("TiltShiftEffect".to_owned(), {
            let mut __flight_record = Vec::new();
            __flight_record.push(("blur".to_owned(), crate::OpaqueHostValue::Number(1.0_f64)));
            __flight_record.push(("center".to_owned(), crate::OpaqueHostValue::Number(0.5_f64)));
            __flight_record.push(("width".to_owned(), crate::OpaqueHostValue::Number(0.2_f64)));
            __flight_record
        }));
        __flight_record.push(("ToneMapEffect".to_owned(), {
            let mut __flight_record = Vec::new();
            __flight_record.push((
                "exposure".to_owned(),
                crate::OpaqueHostValue::Number(0.0_f64),
            ));
            __flight_record.push((
                "operator".to_owned(),
                crate::OpaqueHostValue::String("aces".to_owned()),
            ));
            __flight_record
        }));
        __flight_record.push(("VignetteEffect".to_owned(), {
            let mut __flight_record = Vec::new();
            __flight_record.push((
                "color".to_owned(),
                crate::OpaqueHostValue::Number(255.0_f64),
            ));
            __flight_record.push((
                "intensity".to_owned(),
                crate::OpaqueHostValue::Number(0.5_f64),
            ));
            __flight_record.push(("radius".to_owned(), crate::OpaqueHostValue::Number(1.0_f64)));
            __flight_record.push((
                "softness".to_owned(),
                crate::OpaqueHostValue::Number(0.5_f64),
            ));
            __flight_record
        }));
        __flight_record.push(("VolumetricLightEffect".to_owned(), {
            let mut __flight_record = Vec::new();
            __flight_record.push((
                "density".to_owned(),
                crate::OpaqueHostValue::Number(0.5_f64),
            ));
            __flight_record.push((
                "lightColor".to_owned(),
                crate::OpaqueHostValue::Number(4294967295.0_f64),
            ));
            __flight_record.push(("lightX".to_owned(), crate::OpaqueHostValue::Number(0.5_f64)));
            __flight_record.push(("lightY".to_owned(), crate::OpaqueHostValue::Number(0.2_f64)));
            __flight_record.push((
                "samples".to_owned(),
                crate::OpaqueHostValue::Number(32.0_f64),
            ));
            __flight_record.push((
                "scattering".to_owned(),
                crate::OpaqueHostValue::Number(0.7_f64),
            ));
            __flight_record
        }));
        __flight_record.push(("WhiteBalanceEffect".to_owned(), {
            let mut __flight_record = Vec::new();
            __flight_record.push((
                "temperature".to_owned(),
                crate::OpaqueHostValue::Number(0.0_f64),
            ));
            __flight_record.push(("tint".to_owned(), crate::OpaqueHostValue::Number(0.0_f64)));
            __flight_record
        }));
        __flight_record
    });
