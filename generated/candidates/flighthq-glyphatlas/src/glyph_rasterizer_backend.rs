// @generated from upstream/packages/glyphatlas/src/glyphRasterizerBackend.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::{GlyphRasterizeOptions, GlyphRasterizedBitmap, GlyphRasterizerBackend};

// Source: upstream/packages/glyphatlas/src/glyphRasterizerBackend.ts:9 (sha256:6ee88e601aef10e43a1242471ea89503124afb3ddb5c73ca6f3ad226e59c4898)
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
                let mut pixels = vec![0_u8; ((width * height) * 4.0_f64) as usize];
                pixels.fill((255.0_f64) as u8);
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
    };
}

// Source: upstream/packages/glyphatlas/src/glyphRasterizerBackend.ts:36 (sha256:9f5438b38ff38914dddb9dac489f85752c3d5ae8edb24c9e9973601892f9bda6)
pub fn create_web_glyph_rasterizer_backend() -> GlyphRasterizerBackend {
    return GlyphRasterizerBackend {
        __flight_identity: std::sync::Arc::new(()),
        rasterize: std::sync::Arc::new(std::sync::Mutex::new(Box::new(
            move |codepoint: f64,
                  options: GlyphRasterizeOptions|
                  -> Option<GlyphRasterizedBitmap> {
                let mut context = _acquire_glyph_raster_context();
                if (context).is_none() {
                    return None;
                }
                return _rasterize_glyph_on_context(
                    (context).clone().unwrap(),
                    codepoint,
                    &options,
                );
            },
        )
            as Box<
                dyn FnMut(f64, GlyphRasterizeOptions) -> Option<GlyphRasterizedBitmap>
                    + Send
                    + 'static,
            >)),
    };
}

// Source: upstream/packages/glyphatlas/src/glyphRasterizerBackend.ts:48 (sha256:c8604ceefc9ed680168177faa121bbaf51e3c2559caa29e33a50e7e23e6522f9)
pub fn get_glyph_rasterizer_backend() -> GlyphRasterizerBackend {
    if ((*_BACKEND.lock().unwrap()).clone()).is_none() {
        (*_BACKEND.lock().unwrap()) = Some(create_web_glyph_rasterizer_backend());
    }
    return ((*_BACKEND.lock().unwrap()).clone()).clone().unwrap();
}

// Source: upstream/packages/glyphatlas/src/glyphRasterizerBackend.ts:54 (sha256:db0a730806afd604b259ee162e309d2d45828b73b7fba5ab3c50e59bbe5cb1c4)
pub fn set_glyph_rasterizer_backend(backend: Option<GlyphRasterizerBackend>) -> () {
    (*_BACKEND.lock().unwrap()) = (backend).clone();
}

// Source: upstream/packages/glyphatlas/src/glyphRasterizerBackend.ts:58 (sha256:8a69294db958bdf5d0b47608c745de8183f7716c9cd557a4e86010f2977cce68)
static _BACKEND: std::sync::LazyLock<std::sync::Mutex<Option<GlyphRasterizerBackend>>> =
    std::sync::LazyLock::new(|| std::sync::Mutex::new(None));

// Source: upstream/packages/glyphatlas/src/glyphRasterizerBackend.ts:63 (sha256:84884a2867a5b8b22005dff255ba00c6f2093d1a478bf0e1d77777f6370467d8)
fn _acquire_glyph_raster_context() -> Option<crate::OpaqueHostValue> {
    let __flight_try_return: Option<Option<crate::OpaqueHostValue>> = match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| -> Option<Option<crate::OpaqueHostValue>> {
            {}
            None
        }),
    ) {
        Ok(value) => value,
        Err(_) => (|| -> Option<Option<crate::OpaqueHostValue>> {
            {
                return Some(None);
            }
            None
        })(),
    };
    if let Some(__flight_return) = __flight_try_return {
        return __flight_return;
    }
    return None;
}

// Source: upstream/packages/glyphatlas/src/glyphRasterizerBackend.ts:83 (sha256:506790e8fd97a9d199322077531c577d7c43aaca91d44ae006f0445bc37385a0)
fn _rasterize_glyph_on_context(
    context: crate::OpaqueHostValue,
    codepoint: f64,
    options: &GlyphRasterizeOptions,
) -> Option<GlyphRasterizedBitmap> {
    let text = (string.from_code_point)(codepoint);
    let font_style = ((options.font_style).clone()).unwrap_or("normal".to_owned());
    let font_weight = ((options.font_weight).clone())
        .unwrap_or(crate::FlightUnion2::<f64, String>::B("normal".to_owned()));
    crate::host_set(
        "host.font",
        format!(
            "{} {} {}px {}",
            font_style,
            font_weight,
            options.font_size,
            (options.font_family).clone()
        ),
    );
    crate::host_set("host.textBaseline", "alphabetic");
    crate::host_set("host.textAlign", "left");
    let metrics = crate::host_value::<()>("host.measureText");
    let advance = metrics.width;
    let left = (metrics.actual_bounding_box_left).unwrap_or(0.0_f64);
    let right = (metrics.actual_bounding_box_right).unwrap_or(advance);
    let ascent = (metrics.actual_bounding_box_ascent).unwrap_or(options.font_size);
    let descent = (metrics.actual_bounding_box_descent).unwrap_or(0.0_f64);
    let guard = 1.0_f64;
    let width = ((0.0_f64).max((left + right).ceil()) + (guard * 2.0_f64));
    let height = ((0.0_f64).max((ascent + descent).ceil()) + (guard * 2.0_f64));
    if ((width <= (guard * 2.0_f64)) || (height <= (guard * 2.0_f64))) {
        return None;
    }
    let mut canvas = crate::host_value::<crate::OpaqueHostValue>("host.canvas");
    crate::host_set("host.width", width);
    crate::host_set("host.height", height);
    crate::host_set(
        "host.font",
        format!(
            "{} {} {}px {}",
            font_style,
            font_weight,
            options.font_size,
            (options.font_family).clone()
        ),
    );
    crate::host_set("host.textBaseline", "alphabetic");
    crate::host_set("host.textAlign", "left");
    crate::host_value::<()>("host.clearRect");
    crate::host_set("host.fillStyle", "#ffffff");
    crate::host_value::<()>("host.fillText");
    let image = crate::host_value::<()>("host.getImageData");
    return Some(GlyphRasterizedBitmap {
        __flight_identity: std::sync::Arc::new(()),
        advance: advance,
        bearing_x: (-left),
        bearing_y: ascent,
        height: height,
        pixels: vec![0_u8; (image.data) as usize],
        width: width,
    });
}
