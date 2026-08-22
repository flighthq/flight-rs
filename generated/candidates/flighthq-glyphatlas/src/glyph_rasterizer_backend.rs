// @generated from upstream/packages/glyphatlas/src/glyphRasterizerBackend.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::{
    GlyphMetrics, GlyphRasterizeOptions, GlyphRasterizedBitmap, GlyphRasterizerBackend,
};

#[inline]

fn __flight_string_from_code_point(value: f64) -> String {
    assert!(
        value.is_finite()
            && value.fract() == 0.0_f64
            && (0.0_f64..=0x10FFFF_u32 as f64).contains(&value),
        "String.fromCodePoint received an invalid code point"
    );
    char::from_u32(value as u32)
        .expect("Rust strings cannot represent surrogate code points")
        .to_string()
}

// Source: upstream/packages/glyphatlas/src/glyphRasterizerBackend.ts:14 (sha256:6ee88e601aef10e43a1242471ea89503124afb3ddb5c73ca6f3ad226e59c4898)
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

// Source: upstream/packages/glyphatlas/src/glyphRasterizerBackend.ts:41 (sha256:481a85529405f503f9fe7bb5525b286681a4301df375bda6e97157cdd8bca3c7)
pub fn create_web_glyph_rasterizer_backend() -> GlyphRasterizerBackend {
    return GlyphRasterizerBackend {
        __flight_identity: std::sync::Arc::new(()),
        measure_metrics: Some(std::sync::Arc::new(std::sync::Mutex::new(Box::new(
            move |options: GlyphRasterizeOptions| -> Option<GlyphMetrics> {
                let mut context = _acquire_glyph_raster_context();
                if (context).is_none() {
                    return None;
                }
                _apply_glyph_raster_font((context.as_mut().unwrap()).clone(), &options);
                let metrics = crate::host_value::<()>("host.measureText");
                let ascent =
                    crate::host_value::<crate::OpaqueHostValue>("host.fontBoundingBoxAscent");
                let descent =
                    crate::host_value::<crate::OpaqueHostValue>("host.fontBoundingBoxDescent");
                if (!(ascent > 0.0_f64)) || (!(descent >= 0.0_f64)) {
                    return None;
                }
                return Some(GlyphMetrics {
                    __flight_identity: std::sync::Arc::new(()),
                    ascent: ascent,
                    descent: descent,
                    line_gap: 0.0_f64,
                });
            },
        )
            as Box<
                dyn FnMut(GlyphRasterizeOptions) -> Option<GlyphMetrics> + Send + 'static,
            >))),
        rasterize: std::sync::Arc::new(std::sync::Mutex::new(Box::new(
            move |codepoint: f64,
                  options: GlyphRasterizeOptions|
                  -> Option<GlyphRasterizedBitmap> {
                let mut context = _acquire_glyph_raster_context();
                if (context).is_none() {
                    return None;
                }
                return _rasterize_glyph_on_context(
                    (context.as_mut().unwrap()).clone(),
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

// Source: upstream/packages/glyphatlas/src/glyphRasterizerBackend.ts:75 (sha256:c8604ceefc9ed680168177faa121bbaf51e3c2559caa29e33a50e7e23e6522f9)
pub fn get_glyph_rasterizer_backend() -> GlyphRasterizerBackend {
    if ((*_BACKEND.lock().unwrap()).clone()).is_none() {
        (*_BACKEND.lock().unwrap()) = Some(create_web_glyph_rasterizer_backend());
    }
    return (((*_BACKEND.lock().unwrap()).clone()).clone().unwrap()).clone();
}

// Source: upstream/packages/glyphatlas/src/glyphRasterizerBackend.ts:81 (sha256:db0a730806afd604b259ee162e309d2d45828b73b7fba5ab3c50e59bbe5cb1c4)
pub fn set_glyph_rasterizer_backend(backend: &Option<GlyphRasterizerBackend>) -> () {
    (*_BACKEND.lock().unwrap()) = (*backend).clone();
}

// Source: upstream/packages/glyphatlas/src/glyphRasterizerBackend.ts:85 (sha256:8a69294db958bdf5d0b47608c745de8183f7716c9cd557a4e86010f2977cce68)
static _BACKEND: std::sync::LazyLock<std::sync::Mutex<Option<GlyphRasterizerBackend>>> =
    std::sync::LazyLock::new(|| std::sync::Mutex::new(None));

// Source: upstream/packages/glyphatlas/src/glyphRasterizerBackend.ts:90 (sha256:84884a2867a5b8b22005dff255ba00c6f2093d1a478bf0e1d77777f6370467d8)
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

// Source: upstream/packages/glyphatlas/src/glyphRasterizerBackend.ts:110 (sha256:72b8ce5a853694fa730d3b49ced2bccb98ef4406ca7f22eddb3bb272850375cd)
fn _rasterize_glyph_on_context(
    context: crate::OpaqueHostValue,
    codepoint: f64,
    options: &GlyphRasterizeOptions,
) -> Option<GlyphRasterizedBitmap> {
    let text = __flight_string_from_code_point(codepoint);
    _apply_glyph_raster_font((context).clone(), options);
    crate::host_set("host.textBaseline", "alphabetic");
    crate::host_set("host.textAlign", "left");
    let metrics = crate::host_value::<()>("host.measureText");
    let advance = crate::host_value::<crate::OpaqueHostValue>("host.width");
    let left = (crate::host_value::<Option<f64>>("host.actualBoundingBoxLeft")).unwrap_or(0.0_f64);
    let right =
        (crate::host_value::<Option<crate::OpaqueHostValue>>("host.actualBoundingBoxRight"))
            .unwrap_or((advance).clone());
    let ascent = (crate::host_value::<Option<f64>>("host.actualBoundingBoxAscent"))
        .unwrap_or(options.font_size);
    let descent =
        (crate::host_value::<Option<f64>>("host.actualBoundingBoxDescent")).unwrap_or(0.0_f64);
    let guard = 1.0_f64;
    let width = ((0.0_f64).max((left + right).ceil()) + (guard * 2.0_f64));
    let height = ((0.0_f64).max((ascent + descent).ceil()) + (guard * 2.0_f64));
    if (width <= (guard * 2.0_f64)) || (height <= (guard * 2.0_f64)) {
        return None;
    }
    let mut canvas = crate::host_value::<crate::OpaqueHostValue>("host.canvas");
    crate::host_set("host.width", width);
    crate::host_set("host.height", height);
    _apply_glyph_raster_font((context).clone(), options);
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
        pixels: vec![0_u8; (crate::host_value::<crate::OpaqueHostValue>("host.data")) as usize],
        width: width,
    });
}

// Source: upstream/packages/glyphatlas/src/glyphRasterizerBackend.ts:156 (sha256:4a6c1b83a3662ca60784461542ea6ab81a9e213d4dabc5c4faf29dca09e1687d)
fn _apply_glyph_raster_font(
    context: crate::OpaqueHostValue,
    options: &GlyphRasterizeOptions,
) -> () {
    let font_style = ((options.font_style).clone()).unwrap_or("normal".to_owned());
    let font_weight = ((options.font_weight).clone())
        .unwrap_or(crate::FlightUnion2::<f64, String>::B("normal".to_owned()));
    crate::host_set(
        "host.font",
        format!(
            "{} {} {}px {}",
            (font_style).clone(),
            (font_weight).clone(),
            options.font_size,
            (options.font_family).clone()
        ),
    );
}
