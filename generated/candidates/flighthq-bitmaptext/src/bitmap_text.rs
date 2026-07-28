// @generated from upstream/packages/bitmaptext/src/bitmapText.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_displayobject::{
    create_display_object_generic, create_display_object_runtime, get_display_object_runtime,
};
use flighthq_geometry::{copy_rectangle, create_rectangle};
use flighthq_node::add_node_child;
use flighthq_sprite::{create_quad_batch, reserve_quad_batch};
use flighthq_textureatlas::create_texture_atlas;
use flighthq_types::{
    BITMAP_TEXT_KIND as bitmap_text_kind_constant, BitmapText, BitmapTextAlign, BitmapTextData,
    BitmapTextOptions, BitmapTextRuntime, BoundsNodeAny, GlyphSource, Node, QuadBatch,
    QuadBatchData, Rectangle,
};

// Source: upstream/packages/bitmaptext/src/bitmapText.ts:25 (sha256:6092d0ea3715e74338136af9aad3fb871bc32e15dea783aa072633373219d2fe)
const BITMAP_TEXT_DEFAULT_COLOR: f64 = 4294967295.0_f64;

// Source: upstream/packages/bitmaptext/src/bitmapText.ts:29 (sha256:5a66105373f19ad8ce0c004a477bc8541626e945b9dd4b6f872c70be422a7635)
pub fn compute_bitmap_text_local_bounds_rectangle(out: &mut Rectangle, source: &BitmapText) -> () {
    let runtime = get_display_object_runtime(source);
    let bounds = (runtime.local_bounds_rectangle).clone();
    if (bounds).is_none() {
        out.x = 0.0_f64;
        out.y = 0.0_f64;
        out.width = 0.0_f64;
        out.height = 0.0_f64;
        return;
    }
    copy_rectangle(out, &bounds);
}

// Source: upstream/packages/bitmaptext/src/bitmapText.ts:48 (sha256:9ae99b607504937177e17558023ec62201669c98fc99d2efc27270f6a21924ca)
pub fn create_bitmap_text(
    glyph_source: Option<GlyphSource>,
    options: Option<BitmapTextOptions>,
) -> BitmapText {
    let mut bitmap_text = create_display_object_generic(
        (bitmap_text_kind_constant).to_owned(),
        Some(undefined),
        Some(std::sync::Arc::new(std::sync::Mutex::new(Box::new(
            move |__flight_argument_0: Option<D>| -> D {
                create_bitmap_text_data(Some(((__flight_argument_0).clone().unwrap()).clone()))
            },
        )
            as Box<dyn FnMut(Option<D>) -> D + Send + 'static>))),
        Some(std::sync::Arc::new(std::sync::Mutex::new(Box::new(
            move |__flight_argument_0: Option<R>| -> R { create_bitmap_text_runtime() },
        )
            as Box<dyn FnMut(Option<R>) -> R + Send + 'static>))),
    );
    bitmap_text.data.glyph_source = (glyph_source).clone();
    if (options).is_some() {
        apply_bitmap_text_options(&mut bitmap_text.data, &options.as_ref().unwrap());
    }
    let quad_batch = create_quad_batch(Some(QuadBatch {
        __flight_identity: std::sync::Arc::new(()),
        data: QuadBatchData {
            __flight_identity: std::sync::Arc::new(()),
            atlas: Some(create_texture_atlas(None)),
        },
    }));
    let mut runtime = get_display_object_runtime(&bitmap_text);
    runtime.quad_batches.push(((quad_batch).clone()).clone());
    add_node_child(&bitmap_text, &quad_batch);
    return bitmap_text;
}

// Source: upstream/packages/bitmaptext/src/bitmapText.ts:65 (sha256:652b9b9decde0da5b6ec262d3fa215cdbabc38279b594e72639ef8075e765e5d)
pub fn create_bitmap_text_data(data: Option<BitmapTextData>) -> BitmapTextData {
    return BitmapTextData {
        __flight_identity: std::sync::Arc::new(()),
        align: (data.as_ref().map(|value| (value.align).clone())).unwrap_or("left".to_owned()),
        color: (data.as_ref().map(|value| value.color)).unwrap_or(BITMAP_TEXT_DEFAULT_COLOR),
        glyph_source: data.as_ref().and_then(|value| (value.glyph_source).clone()),
        letter_spacing: (data.as_ref().map(|value| value.letter_spacing)).unwrap_or(0.0_f64),
        line_height: (data.as_ref().map(|value| value.line_height)).unwrap_or(1.0_f64),
        text: (data.as_ref().map(|value| (value.text).clone())).unwrap_or("".to_owned()),
        wrap_width: data.as_ref().and_then(|value| value.wrap_width),
    };
}

// Source: upstream/packages/bitmaptext/src/bitmapText.ts:77 (sha256:99eea2265fed47ad3c08dceb9dd1d2ff08555854102b821b8465dc89f02a12ad)
pub fn create_bitmap_text_runtime() -> BitmapTextRuntime {
    let mut runtime = create_display_object_runtime(Some(((*DEFAULT_METHODS).clone()).clone()));
    runtime.local_bounds_rectangle = None;
    runtime.quad_batches = vec![];
    return runtime;
}

// Source: upstream/packages/bitmaptext/src/bitmapText.ts:86 (sha256:beb2063df494c0cf5820839f49241ce13a95548f488d2e11f0cd9133bea028f2)
pub fn get_bitmap_text_bounds(source: &BitmapText) -> Rectangle {
    let mut out = create_rectangle(None, None, None, None);
    compute_bitmap_text_local_bounds_rectangle(&mut out, source);
    return out;
}

// Source: upstream/packages/bitmaptext/src/bitmapText.ts:94 (sha256:9e24e8e7989101781f47989d6ec78853ea00bae1ca1c2488985d26f552e208da)
pub fn get_bitmap_text_quad_batches(source: &BitmapText) -> Vec<QuadBatch> {
    return (get_display_object_runtime(source).quad_batches).clone();
}

// Source: upstream/packages/bitmaptext/src/bitmapText.ts:101 (sha256:dd245b6880d4bebde7f723053d5c9bc0018a35a4f0bb5885b484c0919249847d)
pub fn reserve_bitmap_text(target: &BitmapText, glyph_capacity: f64) -> () {
    let runtime = get_display_object_runtime(target);
    for quad_batch in ((runtime.quad_batches).clone()).iter().cloned() {
        reserve_quad_batch(&mut quad_batch, glyph_capacity);
    }
}

// Source: upstream/packages/bitmaptext/src/bitmapText.ts:107 (sha256:86964ba65f3f4396e7ffef7642d63385d310454ca142fe20064743c04b44f7fc)
pub fn set_bitmap_text_align(target: &mut BitmapText, align: BitmapTextAlign) -> () {
    target.data.align = (align).clone();
}

// Source: upstream/packages/bitmaptext/src/bitmapText.ts:111 (sha256:0ce08cafc8e5bf48ba7bb97a82beddb955ea2f3b4e3bc5e0cce033864701df6d)
pub fn set_bitmap_text_color(target: &mut BitmapText, color: f64) -> () {
    target.data.color = color;
}

// Source: upstream/packages/bitmaptext/src/bitmapText.ts:115 (sha256:f3979369c293d3771fc2e96a8d8dfb6ba5251701b4afc9a02b5e1fe11752ae7e)
pub fn set_bitmap_text_glyph_source(
    target: &mut BitmapText,
    glyph_source: Option<GlyphSource>,
) -> () {
    target.data.glyph_source = (glyph_source).clone();
}

// Source: upstream/packages/bitmaptext/src/bitmapText.ts:119 (sha256:2248db62f27e90cd43fff4588f3775db6d8cac400b6addd2ae87914f04ce47c7)
pub fn set_bitmap_text_letter_spacing(target: &mut BitmapText, letter_spacing: f64) -> () {
    target.data.letter_spacing = letter_spacing;
}

// Source: upstream/packages/bitmaptext/src/bitmapText.ts:123 (sha256:eba236685e316a1bd57865a42ab1ce5479dafd9e2964cb1309691861e538bb9e)
pub fn set_bitmap_text_line_height(target: &mut BitmapText, line_height: f64) -> () {
    target.data.line_height = line_height;
}

// Source: upstream/packages/bitmaptext/src/bitmapText.ts:127 (sha256:72271f9f1f7e6213b541cc3731589dabb4a1674ee6c7472c0a249f2230b25f1a)
pub fn set_bitmap_text_text(target: &mut BitmapText, text: String) -> () {
    target.data.text = (text).clone();
}

// Source: upstream/packages/bitmaptext/src/bitmapText.ts:131 (sha256:909454d4aa245693104ad943c8572177af39c646c872ef9c17a8044509b922ea)
pub fn set_bitmap_text_wrap_width(target: &mut BitmapText, wrap_width: Option<f64>) -> () {
    target.data.wrap_width = wrap_width;
}

// Source: upstream/packages/bitmaptext/src/bitmapText.ts:135 (sha256:814a6a2d91cf6fc5683ef3b949c7e76c423d15c39af9c02939e45696c55d0e39)
fn apply_bitmap_text_options(data: &mut BitmapTextData, options: &BitmapTextOptions) -> () {
    if ((options.align).clone()).is_some() {
        data.align = ((options.align).clone()).unwrap();
    }
    if (options.color).is_some() {
        data.color = (options.color).unwrap();
    }
    if (options.letter_spacing).is_some() {
        data.letter_spacing = (options.letter_spacing).unwrap();
    }
    if (options.line_height).is_some() {
        data.line_height = (options.line_height).unwrap();
    }
    if ((options.text).clone()).is_some() {
        data.text = ((options.text).clone()).unwrap();
    }
    if (options.wrap_width).is_some() {
        data.wrap_width = options.wrap_width;
    }
}

// Source: upstream/packages/bitmaptext/src/bitmapText.ts:144 (sha256:b5db92c66924ee72ef0e4f17d590cc803773c9681c5ac1c7cef2a33dd5481a1f)
fn copy_local_bounds_rectangle(out: &mut Rectangle, source: &Node) -> () {
    let runtime = get_display_object_runtime(&source);
    if ((runtime.local_bounds_rectangle).clone()).is_some() {
        copy_rectangle(out, &runtime.local_bounds_rectangle);
    }
}

// Source: upstream/packages/bitmaptext/src/bitmapText.ts:149 (sha256:8a9b089bd51cc37c07cebbbfddf583d54c250e80e084a008dab75e82e397a0d0)
static DEFAULT_METHODS: std::sync::LazyLock<BitmapTextRuntime> =
    std::sync::LazyLock::new(|| BitmapTextRuntime {
        __flight_identity: std::sync::Arc::new(()),
        compute_local_bounds_rectangle: std::sync::Arc::new(std::sync::Mutex::new(Box::new(
            move |mut __flight_argument_0: Rectangle, __flight_argument_1: BoundsNodeAny| -> () {
                copy_local_bounds_rectangle(&mut __flight_argument_0, &__flight_argument_1)
            },
        )
            as Box<dyn FnMut(Rectangle, BoundsNodeAny) -> () + Send + 'static>)),
    });
