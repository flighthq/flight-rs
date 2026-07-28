// @generated from upstream/packages/textshaper-canvas/src/canvasTextShaper.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_text::compute_text_format_font_string;
use flighthq_types::{
    FontMetrics, GlyphExtents, ShapeRunOptions, ShapedRun, TextFormat, TextMeasureFunction,
};

// Source: upstream/packages/textshaper-canvas/src/canvasTextShaper.ts:8 (sha256:557072965dcccd1613f98f320372f780382b2ea829124dfb36ac3a23cb78a658)
pub fn clear_canvas_text_shaper_backend_cache(backend: crate::OpaqueHostValue) -> () {
    crate::host_value::<()>("host.clearCache");
}

// Source: upstream/packages/textshaper-canvas/src/canvasTextShaper.ts:30 (sha256:68588e035d46e30f2a6fbbadcbc2078cdca2fea98e06a87aaa2d6eb55947f8f1)
pub fn create_canvas_text_shaper_backend() -> crate::OpaqueHostValue {
    let ctx: std::sync::Arc<std::sync::Mutex<Option<crate::OpaqueHostValue>>> =
        std::sync::Arc::new(std::sync::Mutex::new(_create_context()));
    if ((*ctx.lock().unwrap()).clone()).is_none() {
        return _create_sentinel_backend();
    }
    let supports_letter_spacing = false;
    let supports_word_spacing = false;
    let supports_direction = false;
    let cache: std::sync::Arc<std::sync::Mutex<Vec<(String, f64)>>> =
        std::sync::Arc::new(std::sync::Mutex::new(Vec::new()));
    let backend = CanvasTextShaperBackend {
        __flight_identity: std::sync::Arc::new(()),
        clear_cache: std::sync::Arc::new(std::sync::Mutex::new(Box::new({
            let mut cache = cache.clone();
            move || -> () {
                (*cache.lock().unwrap()).clear();
            }
        })
            as Box<dyn FnMut() -> () + Send + 'static>)),
        get_font_metrics: Some(std::sync::Arc::new(std::sync::Mutex::new(Box::new({
            let mut ctx = ctx.clone();
            move |format: TextFormat| -> Option<FontMetrics> {
                let font_string = compute_text_format_font_string(&format);
                crate::host_set("host.font", font_string);
                let cap_measure = crate::host_value::<()>("host.measureText");
                let x_measure = crate::host_value::<()>("host.measureText");
                let descender_measure = crate::host_value::<()>("host.measureText");
                let ascent = (crate::host_value::<Option<crate::OpaqueHostValue>>(
                    "host.fontBoundingBoxAscent",
                ))
                .unwrap_or(crate::host_value::<crate::OpaqueHostValue>(
                    "host.actualBoundingBoxAscent",
                ));
                let descent = (crate::host_value::<Option<crate::OpaqueHostValue>>(
                    "host.fontBoundingBoxDescent",
                ))
                .unwrap_or(crate::host_value::<crate::OpaqueHostValue>(
                    "host.actualBoundingBoxDescent",
                ));
                let size = (format.size).unwrap_or(12.0_f64);
                return Some(FontMetrics {
                    __flight_identity: std::sync::Arc::new(()),
                    ascent: ascent,
                    cap_height: crate::host_value::<f64>("host.actualBoundingBoxAscent"),
                    descent: descent,
                    line_gap: 0.0_f64,
                    underline_position: (-(size * 0.1_f64)),
                    underline_thickness: (1.0_f64).max((size * 0.05_f64)),
                    units_per_em: size,
                    x_height: crate::host_value::<f64>("host.actualBoundingBoxAscent"),
                });
            }
        })
            as Box<dyn FnMut(TextFormat) -> Option<FontMetrics> + Send + 'static>))),
        measure_text: std::sync::Arc::new(std::sync::Mutex::new(Box::new({
            let mut cache = cache.clone();
            let mut ctx = ctx.clone();
            move |text: String, format: TextFormat| -> f64 {
                let font_string = compute_text_format_font_string(&format);
                let cache_key = format!(
                    "{}\u{0000}{}\u{0000}{}",
                    font_string,
                    (format.letter_spacing).unwrap_or(0.0_f64),
                    text
                );
                let cached = (*cache.lock().unwrap())
                    .iter()
                    .find(|(key, _)| key == &cache_key)
                    .map(|(_, value)| value.clone());
                if (cached).is_some() {
                    return *(cached.as_ref().unwrap());
                }
                crate::host_set("host.font", font_string);
                if supports_letter_spacing {
                    crate::host_value::<Vec<(String, crate::OpaqueHostValue)>>("host.cast")
                        .iter()
                        .find(|(key, _)| key == &"letterSpacing".to_owned())
                        .map(|(_, value)| value)
                        .expect("TypeScript Record key was absent") =
                        format!("{}px", (format.letter_spacing).unwrap_or(0.0_f64));
                }
                if supports_word_spacing {
                    crate::host_value::<Vec<(String, crate::OpaqueHostValue)>>("host.cast")
                        .iter()
                        .find(|(key, _)| key == &"wordSpacing".to_owned())
                        .map(|(_, value)| value)
                        .expect("TypeScript Record key was absent") =
                        crate::OpaqueHostValue::String("0px".to_owned());
                }
                if supports_direction {
                    crate::host_value::<Vec<(String, crate::OpaqueHostValue)>>("host.cast")
                        .iter()
                        .find(|(key, _)| key == &"direction".to_owned())
                        .map(|(_, value)| value)
                        .expect("TypeScript Record key was absent") =
                        crate::OpaqueHostValue::String("ltr".to_owned());
                }
                let width = crate::host_value::<crate::OpaqueHostValue>("host.width");
                if (((*cache.lock().unwrap()).len() as f64) >= _CACHE_MAX_SIZE) {
                    {
                        let __flight_key = (((*cache.lock().unwrap())
                            .iter()
                            .map(|(key, _)| key.clone())
                            .collect::<Vec<_>>()
                            .next)()
                        .value)
                            .clone();
                        if let Some(__flight_index) = (*cache.lock().unwrap())
                            .iter()
                            .position(|(key, _)| key == &__flight_key)
                        {
                            (*cache.lock().unwrap()).remove(__flight_index);
                            true
                        } else {
                            false
                        }
                    };
                }
                {
                    let __flight_key = cache_key;
                    let __flight_value = width;
                    if let Some((_, value)) = (*cache.lock().unwrap())
                        .iter_mut()
                        .find(|(key, _)| key == &__flight_key)
                    {
                        *value = __flight_value;
                    } else {
                        (*cache.lock().unwrap()).push((__flight_key, __flight_value));
                    }
                };
                return width;
            }
        })
            as Box<dyn FnMut(String, TextFormat) -> f64 + Send + 'static>)),
    };
    return backend;
}

// Source: upstream/packages/textshaper-canvas/src/canvasTextShaper.ts:137 (sha256:ec2a8d741282e07b096884af35aee292291e75a93866bb04de5340201244a43b)
#[derive(Clone)]
pub struct CanvasTextShaperBackend {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub get_code_point_for_glyph:
        Option<std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(f64) -> f64 + Send + 'static>>>>,
    pub get_font_metrics: Option<
        std::sync::Arc<
            std::sync::Mutex<Box<dyn FnMut(TextFormat) -> Option<FontMetrics> + Send + 'static>>,
        >,
    >,
    pub get_glyph_extents: Option<
        std::sync::Arc<
            std::sync::Mutex<Box<dyn FnMut(f64) -> Option<GlyphExtents> + Send + 'static>>,
        >,
    >,
    pub get_glyph_index_for_code_point:
        Option<std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(f64) -> f64 + Send + 'static>>>>,
    pub get_glyph_name:
        Option<std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(f64) -> String + Send + 'static>>>>,
    pub measure_text: TextMeasureFunction,
    pub shape_run: Option<
        std::sync::Arc<
            std::sync::Mutex<
                Box<
                    dyn FnMut(String, TextFormat, Option<ShapeRunOptions>) -> ShapedRun
                        + Send
                        + 'static,
                >,
            >,
        >,
    >,
    pub clear_cache: std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>,
}
impl PartialEq for CanvasTextShaperBackend {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/textshaper-canvas/src/canvasTextShaper.ts:147 (sha256:ebaf12403f6fd8bd27cdb698faaf46149bca12720a2d9b28e65b71ef3d36c2d3)
const _CACHE_MAX_SIZE: f64 = 512.0_f64;

// Source: upstream/packages/textshaper-canvas/src/canvasTextShaper.ts:152 (sha256:fd74a1590efbac3b577f590046f79072aeb18c2cca863829b8b83e205fc31c52)
fn _create_context() -> Option<crate::OpaqueHostValue> {
    return None;
}

// Source: upstream/packages/textshaper-canvas/src/canvasTextShaper.ts:179 (sha256:38b021866b9acc7793383ebf1f7f76034cf8a3d37b6c47b063f64cefdacf3c70)
fn _create_sentinel_backend() -> crate::OpaqueHostValue {
    return CanvasTextShaperBackend {
        __flight_identity: std::sync::Arc::new(()),
        clear_cache: std::sync::Arc::new(std::sync::Mutex::new(
            Box::new(move || -> () {}) as Box<dyn FnMut() -> () + Send + 'static>
        )),
        get_font_metrics: Some(std::sync::Arc::new(std::sync::Mutex::new(Box::new(
            move |_format: TextFormat| -> Option<FontMetrics> {
                return None;
            },
        )
            as Box<dyn FnMut(TextFormat) -> Option<FontMetrics> + Send + 'static>))),
        measure_text: std::sync::Arc::new(std::sync::Mutex::new(Box::new(
            move |_text: String, _format: TextFormat| -> f64 {
                return (-1.0_f64);
            },
        )
            as Box<dyn FnMut(String, TextFormat) -> f64 + Send + 'static>)),
    };
}
