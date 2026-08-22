// @generated from upstream/packages/glyphatlas/src/glyphRasterizerBackend.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::{
    BackendExplanation, GlyphRasterizeOptions, GlyphRasterizedBitmap, GlyphRasterizerBackend,
};

// Source: upstream/packages/glyphatlas/src/glyphRasterizerBackend.ts:3 (sha256:6ee88e601aef10e43a1242471ea89503124afb3ddb5c73ca6f3ad226e59c4898)
pub fn create_stub_glyph_rasterizer_backend() -> GlyphRasterizerBackend {
    return GlyphRasterizerBackend {
        __flight_identity: std::sync::Arc::new(()),
        rasterize: std::sync::Arc::new(std::sync::Mutex::new(Box::new(
            move |_codepoint: f64,
                  options: GlyphRasterizeOptions|
                  -> Option<GlyphRasterizedBitmap> {
                let size = (1.0_f64).max((options.font_size).round());
                let width = (1.0_f64).max((size * 0.6_f64).round());
                let height = (1.0_f64).max((size * 0.7_f64).round());
                let mut pixels: Vec<u8> = vec![0_u8; ((width * height) * 4.0_f64) as usize];
                {
                    let __flight_value = (255.0_f64) as u8;
                    let __flight_collection = &mut pixels;
                    __flight_collection.fill(__flight_value);
                    __flight_collection.clone()
                };
                return Some(GlyphRasterizedBitmap {
                    __flight_identity: std::sync::Arc::new(()),
                    advance: (width + (1.0_f64).max((size * 0.1_f64).round())),
                    bearing_x: 0.0_f64,
                    bearing_y: height,
                    height: height,
                    pixels: (pixels).clone(),
                    width: width,
                });
            },
        )
            as Box<
                dyn FnMut(f64, GlyphRasterizeOptions) -> Option<GlyphRasterizedBitmap>
                    + Send
                    + 'static,
            >)),
        measure_metrics: None,
    };
}

// Source: upstream/packages/glyphatlas/src/glyphRasterizerBackend.ts:23 (sha256:6a81538f3d3ae743e4ae747ba62c354f6b529c1558427fc2e175502208396f03)
#[derive(Clone, Default)]
struct ExplainGlyphRasterizerBackendRecord1 {
    __flight_identity: std::sync::Arc<()>,
    layer: String,
    viability: String,
}
impl PartialEq for ExplainGlyphRasterizerBackendRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn explain_glyph_rasterizer_backend() -> BackendExplanation {
    if ((*_CUSTOM.lock().unwrap()).clone()).is_some() {
        return BackendExplanation {
            __flight_identity: std::sync::Arc::new(()),
            layer: "custom".to_owned(),
            viability: "available".to_owned(),
        };
    }
    if ((*_HOST.lock().unwrap()).clone()).is_some() {
        if _HOST_CONFLICT.load(std::sync::atomic::Ordering::Relaxed) {
            return BackendExplanation {
                __flight_identity: std::sync::Arc::new(()),
                layer: "host".to_owned(),
                viability: "provider-conflict".to_owned(),
            };
        }
        return BackendExplanation {
            __flight_identity: std::sync::Arc::new(()),
            layer: "host".to_owned(),
            viability: if _HOST_VIABLE.load(std::sync::atomic::Ordering::Relaxed) {
                "available".to_owned()
            } else {
                "runtime-api-unavailable".to_owned()
            },
        };
    }
    return BackendExplanation {
        __flight_identity: std::sync::Arc::new(()),
        layer: "host-not-enabled".to_owned(),
        viability: "available".to_owned(),
    };
}

// Source: upstream/packages/glyphatlas/src/glyphRasterizerBackend.ts:32 (sha256:e65d931617a37668f0563eb9e13ae8da6f3b9b99d7009a9f3f0f1fa7972c2364)
pub fn get_glyph_rasterizer_backend() -> GlyphRasterizerBackend {
    return (((*_CUSTOM.lock().unwrap()).clone()).or((*_HOST.lock().unwrap()).clone()))
        .unwrap_or(((*_SENTINEL).clone()).clone());
}

// Source: upstream/packages/glyphatlas/src/glyphRasterizerBackend.ts:40 (sha256:a5b14d8fc6dd3169fb71f4b345556f205603e416e855ea8bbae681983b6d4a6f)
pub fn install_glyph_rasterizer_host_backend(backend: &GlyphRasterizerBackend, viable: bool) -> () {
    if ((*_HOST.lock().unwrap()).clone()).is_some() {
        if (((*_HOST.lock().unwrap()).as_mut().unwrap()).clone() != backend) {
            _HOST_CONFLICT.store(true, std::sync::atomic::Ordering::Relaxed);
        }
        return;
    }
    (*_HOST.lock().unwrap()) = Some((*backend).clone());
    _HOST_VIABLE.store(viable, std::sync::atomic::Ordering::Relaxed);
}

// Source: upstream/packages/glyphatlas/src/glyphRasterizerBackend.ts:49 (sha256:9e2faec833d09af35d7a1eb16e0e760f5473e79306c06309e5e57c697fe7726a)
pub fn reset_glyph_rasterizer_backend_for_test() -> () {
    (*_CUSTOM.lock().unwrap()) = None;
    (*_HOST.lock().unwrap()) = None;
    _HOST_VIABLE.store(false, std::sync::atomic::Ordering::Relaxed);
    _HOST_CONFLICT.store(false, std::sync::atomic::Ordering::Relaxed);
}

// Source: upstream/packages/glyphatlas/src/glyphRasterizerBackend.ts:56 (sha256:0295ab0c84bb832d5d3fd610fa39222f8cbc073daf186f636b667ce0a3d44a0a)
pub fn set_glyph_rasterizer_backend(backend: &Option<GlyphRasterizerBackend>) -> () {
    (*_CUSTOM.lock().unwrap()) = (*backend).clone();
}

// Source: upstream/packages/glyphatlas/src/glyphRasterizerBackend.ts:60 (sha256:74178d9f09bf58daa54b31b12937a29ecbb85d082ca92128d4b92cc7f8d449e2)
static _CUSTOM: std::sync::LazyLock<std::sync::Mutex<Option<GlyphRasterizerBackend>>> =
    std::sync::LazyLock::new(|| std::sync::Mutex::new(None));

// Source: upstream/packages/glyphatlas/src/glyphRasterizerBackend.ts:61 (sha256:45e6b59d0d9c6fb463e793a6e3a250e746b4fa015cb087d4413ba1d175a23713)
static _HOST: std::sync::LazyLock<std::sync::Mutex<Option<GlyphRasterizerBackend>>> =
    std::sync::LazyLock::new(|| std::sync::Mutex::new(None));

// Source: upstream/packages/glyphatlas/src/glyphRasterizerBackend.ts:62 (sha256:147e6b11c92bec6fa647fd3fd33b97808cf3a54bfe0a4871efc518eb3df50139)
static _HOST_VIABLE: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

// Source: upstream/packages/glyphatlas/src/glyphRasterizerBackend.ts:63 (sha256:e44a9f5048c4d9c76906d58e325f2bbeb07ee3c67eebdef66e54c0149df217d1)
static _HOST_CONFLICT: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

// Source: upstream/packages/glyphatlas/src/glyphRasterizerBackend.ts:67 (sha256:4b22d0184974bc5f3185c441384fa5fd035a1977094268078cf4c02ab3d2ff7d)
static _SENTINEL: std::sync::LazyLock<GlyphRasterizerBackend> =
    std::sync::LazyLock::new(|| GlyphRasterizerBackend {
        __flight_identity: std::sync::Arc::new(()),
        rasterize: std::sync::Arc::new(std::sync::Mutex::new(Box::new(
            move |__flight_unused_0: f64,
                  __flight_unused_1: GlyphRasterizeOptions|
                  -> Option<GlyphRasterizedBitmap> {
                return None;
            },
        )
            as Box<
                dyn FnMut(f64, GlyphRasterizeOptions) -> Option<GlyphRasterizedBitmap>
                    + Send
                    + 'static,
            >)),
        measure_metrics: None,
    });
