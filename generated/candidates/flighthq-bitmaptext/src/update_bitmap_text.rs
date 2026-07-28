// @generated from upstream/packages/bitmaptext/src/updateBitmapText.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_adjustments::create_color_transform_adjustment;
use flighthq_displayobject::{get_display_object_runtime, set_display_object_color_adjustments};
use flighthq_geometry::create_rectangle;
use flighthq_materials::create_color_transform;
use flighthq_node::{add_node_child, invalidate_node_local_bounds};
use flighthq_sprite::{append_quad_batch_instance, clear_quad_batch, create_quad_batch};
use flighthq_textureatlas::{add_texture_atlas_region, create_texture_atlas};
use flighthq_types::{
    BitmapText, BitmapTextData, BitmapTextRuntime, ColorTransformLike, GlyphEntry, GlyphSource,
    QuadBatch, QuadBatchData, Rectangle, TextureAtlas,
};

#[inline]
fn __flight_js_to_u32(value: f64) -> u32 {
    if !value.is_finite() || value == 0.0 {
        return 0;
    }
    value.trunc().rem_euclid(4294967296.0_f64) as u32
}

#[inline]
fn __flight_js_to_i32(value: f64) -> i32 {
    __flight_js_to_u32(value) as i32
}

// Source: upstream/packages/bitmaptext/src/updateBitmapText.ts:20 (sha256:6092d0ea3715e74338136af9aad3fb871bc32e15dea783aa072633373219d2fe)
const BITMAP_TEXT_DEFAULT_COLOR: f64 = 4294967295.0_f64;

// Source: upstream/packages/bitmaptext/src/updateBitmapText.ts:21 (sha256:945bc25bcb4a4dbbb1555bb1828c3a1d08003f223c5b9e79cdc1cce606986637)
const CARRIAGE_RETURN: f64 = 13.0_f64;

// Source: upstream/packages/bitmaptext/src/updateBitmapText.ts:22 (sha256:111c1172e5ab8d948ad84285fdac7880dc972715f0b0a9c7da683887c231e300)
const SPACE: f64 = 32.0_f64;

// Source: upstream/packages/bitmaptext/src/updateBitmapText.ts:34 (sha256:50064fa2c76e353881909c6c12cf4b2525bf2e44ac6204da17ec036199a6aa95)
pub fn update_bitmap_text(bitmap_text: &BitmapText) -> () {
    let mut runtime = get_display_object_runtime(bitmap_text);
    let mut bounds = ensure_bounds_rectangle(&mut runtime);
    for quad_batch in ((runtime.quad_batches).clone()).iter().cloned() {
        clear_quad_batch(&mut quad_batch);
        if ((quad_batch.data.atlas).clone()).is_some() {
            quad_batch.data.atlas.as_mut().unwrap().regions.clear();
        }
        apply_bitmap_text_color(&quad_batch, bitmap_text.data.color);
    }
    let glyph_source = (bitmap_text.data.glyph_source).clone();
    if ((glyph_source).is_none() || (bitmap_text.data.text.length == 0.0_f64)) {
        set_empty_rectangle(&mut bounds);
        invalidate_node_local_bounds(bitmap_text);
        return;
    }
    let metrics = ((glyph_source.as_ref().unwrap().get_glyph_metrics).clone())
        .lock()
        .unwrap()();
    let line_advance =
        (((metrics.ascent + metrics.descent) + metrics.line_gap) * bitmap_text.data.line_height);
    let lines = layout_bitmap_text_lines(glyph_source.as_ref().unwrap(), &bitmap_text.data);
    let ref_width = (bitmap_text.data.wrap_width).unwrap_or(max_line_width(&lines));
    let mut pages = Vec::new();
    let mut min_x = f64::INFINITY;
    let mut min_y = f64::INFINITY;
    let mut max_x = (-f64::INFINITY);
    let mut max_y = (-f64::INFINITY);
    {
        let mut li = 0.0_f64;
        while (li < (lines.len() as f64)) {
            let line = lines[li as usize].clone();
            let baseline_y = (metrics.ascent + (li * line_advance));
            let mut start_x = 0.0_f64;
            let mut gap_extra = 0.0_f64;
            if ((bitmap_text.data.align).clone() == "center") {
                start_x = ((ref_width - line.width) / 2.0_f64);
            } else {
                if ((bitmap_text.data.align).clone() == "right") {
                    start_x = (ref_width - line.width);
                } else {
                    if (((((bitmap_text.data.align).clone() == "justify")
                        && (bitmap_text.data.wrap_width).is_some())
                        && (!line.paragraph_end))
                        && ((line.gaps.len() as f64) > 0.0_f64))
                    {
                        gap_extra =
                            ((bitmap_text.data.wrap_width - line.width) / (line.gaps.len() as f64));
                    }
                }
            }
            let mut pen_x = start_x;
            {
                let mut wi = 0.0_f64;
                while (wi < (line.words.len() as f64)) {
                    if (wi > 0.0_f64) {
                        pen_x += (line.gaps[(wi - 1.0_f64) as usize].clone() + gap_extra);
                    }
                    let word = line.words[wi as usize].clone();
                    for glyph in ((word.glyphs).clone()).iter().cloned() {
                        let mut page = ensure_bitmap_text_page_batch(
                            bitmap_text,
                            &mut runtime,
                            glyph_source.as_ref().unwrap(),
                            bitmap_text.data.color,
                            &mut pages,
                            glyph.entry.page,
                        );
                        if (page).is_none() {
                            continue;
                        }
                        let quad_x = ((pen_x + glyph.pen_within_word) + glyph.entry.bearing_x);
                        let quad_y = (baseline_y - glyph.entry.bearing_y);
                        let mut region_id = page
                            .as_mut()
                            .unwrap()
                            .region_by_codepoint
                            .iter()
                            .find(|(key, _)| key == &glyph.codepoint)
                            .map(|(_, value)| value.clone());
                        if (region_id).is_none() {
                            add_texture_atlas_region(
                                &mut page.as_mut().unwrap().atlas,
                                glyph.entry.x,
                                glyph.entry.y,
                                glyph.entry.width,
                                glyph.entry.height,
                                None,
                                None,
                                None,
                            );
                            region_id = Some(
                                ((page.as_mut().unwrap().atlas.regions.len() as f64) - 1.0_f64),
                            );
                            {
                                let __flight_key = glyph.codepoint;
                                let __flight_value = (region_id).clone().unwrap();
                                if let Some((_, value)) = page
                                    .as_mut()
                                    .unwrap()
                                    .region_by_codepoint
                                    .iter_mut()
                                    .find(|(key, _)| key == &__flight_key)
                                {
                                    *value = __flight_value;
                                } else {
                                    page.as_mut()
                                        .unwrap()
                                        .region_by_codepoint
                                        .push((__flight_key, __flight_value));
                                }
                            };
                        }
                        append_quad_batch_instance(
                            &mut page.as_mut().unwrap().quad_batch,
                            (region_id).clone().unwrap(),
                            quad_x,
                            quad_y,
                        );
                        if (quad_x < min_x) {
                            min_x = quad_x;
                        }
                        if (quad_y < min_y) {
                            min_y = quad_y;
                        }
                        if ((quad_x + glyph.entry.width) > max_x) {
                            max_x = (quad_x + glyph.entry.width);
                        }
                        if ((quad_y + glyph.entry.height) > max_y) {
                            max_y = (quad_y + glyph.entry.height);
                        }
                    }
                    pen_x += word.width;
                    {
                        wi += 1.0;
                        wi
                    };
                }
            }
            {
                li += 1.0;
                li
            };
        }
    }
    if (min_x == f64::INFINITY) {
        set_empty_rectangle(&mut bounds);
    } else {
        bounds.x = min_x;
        bounds.y = min_y;
        bounds.width = (max_x - min_x);
        bounds.height = (max_y - min_y);
    }
    invalidate_node_local_bounds(bitmap_text);
}

// Source: upstream/packages/bitmaptext/src/updateBitmapText.ts:116 (sha256:3ae1df5b1514f0d988011929cfbab2cccae11bc3e085653db3e58a62480dc212)
fn apply_bitmap_text_color(quad_batch: &QuadBatch, color: f64) -> () {
    if (color == BITMAP_TEXT_DEFAULT_COLOR) {
        set_display_object_color_adjustments(quad_batch, None);
        return;
    }
    let color_transform = create_color_transform(Some(ColorTransformLike {
        __flight_identity: std::sync::Arc::new(()),
        red_multiplier: ((__flight_js_to_i32(
            (__flight_js_to_u32(color) >> (__flight_js_to_u32(24.0_f64) & 31)) as f64,
        ) & __flight_js_to_i32(255.0_f64)) as f64
            / 255.0_f64),
        green_multiplier: ((__flight_js_to_i32(
            (__flight_js_to_u32(color) >> (__flight_js_to_u32(16.0_f64) & 31)) as f64,
        ) & __flight_js_to_i32(255.0_f64)) as f64
            / 255.0_f64),
        blue_multiplier: ((__flight_js_to_i32(
            (__flight_js_to_u32(color) >> (__flight_js_to_u32(8.0_f64) & 31)) as f64,
        ) & __flight_js_to_i32(255.0_f64)) as f64
            / 255.0_f64),
        alpha_multiplier: ((__flight_js_to_i32(color) & __flight_js_to_i32(255.0_f64)) as f64
            / 255.0_f64),
    }));
    set_display_object_color_adjustments(
        quad_batch,
        Some((vec![create_color_transform_adjustment(&color_transform)]).clone()),
    );
}

// Source: upstream/packages/bitmaptext/src/updateBitmapText.ts:133 (sha256:4f3667584f9d957b98c2e77b16859c5a22f92185621203b3ac759aa1ee2e5efb)
fn build_bitmap_text_words(
    glyph_source: &GlyphSource,
    paragraph: String,
    letter_spacing: f64,
) -> Vec<BitmapTextToken> {
    let tokens: std::sync::Arc<std::sync::Mutex<Vec<BitmapTextToken>>> =
        std::sync::Arc::new(std::sync::Mutex::new(vec![]));
    let pending_gap: std::sync::Arc<std::sync::Mutex<f64>> =
        std::sync::Arc::new(std::sync::Mutex::new(0.0_f64));
    let glyphs: std::sync::Arc<std::sync::Mutex<Vec<BitmapTextGlyph>>> =
        std::sync::Arc::new(std::sync::Mutex::new(vec![]));
    let pen: std::sync::Arc<std::sync::Mutex<f64>> =
        std::sync::Arc::new(std::sync::Mutex::new(0.0_f64));
    let previous_codepoint: std::sync::Arc<std::sync::Mutex<f64>> =
        std::sync::Arc::new(std::sync::Mutex::new((-1.0_f64)));
    let in_word: std::sync::Arc<std::sync::Mutex<bool>> =
        std::sync::Arc::new(std::sync::Mutex::new(false));
    let mut flush: std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>> =
        std::sync::Arc::new(std::sync::Mutex::new(Box::new({
            let mut glyphs = glyphs.clone();
            let mut in_word = in_word.clone();
            let mut pen = pen.clone();
            let mut pending_gap = pending_gap.clone();
            let mut previous_codepoint = previous_codepoint.clone();
            let mut tokens = tokens.clone();
            move || -> () {
                if (!(*in_word.lock().unwrap()).clone()) {
                    return;
                }
                (*tokens.lock().unwrap()).push(BitmapTextToken {
                    __flight_identity: std::sync::Arc::new(()),
                    gap: (*pending_gap.lock().unwrap()).clone(),
                    word: BitmapTextWord {
                        __flight_identity: std::sync::Arc::new(()),
                        glyphs: (*glyphs.lock().unwrap()).clone(),
                        width: (*pen.lock().unwrap()).clone(),
                    },
                });
                (*pending_gap.lock().unwrap()) = 0.0_f64;
                (*glyphs.lock().unwrap()) = vec![];
                (*pen.lock().unwrap()) = 0.0_f64;
                (*previous_codepoint.lock().unwrap()) = (-1.0_f64);
                (*in_word.lock().unwrap()) = false;
            }
        })
            as Box<dyn FnMut() -> () + Send + 'static>));
    for character in (paragraph).iter().cloned() {
        let codepoint = (character.code_point_at)(0.0_f64);
        if ((codepoint).is_none() || (codepoint == CARRIAGE_RETURN)) {
            continue;
        }
        if (codepoint == SPACE) {
            ((flush).clone()).lock().unwrap()();
            let space_entry = ((glyph_source.get_glyph_entry).clone()).lock().unwrap()(SPACE);
            (*pending_gap.lock().unwrap()) += (if (space_entry).is_some() {
                space_entry.as_ref().unwrap().advance
            } else {
                0.0_f64
            } + letter_spacing);
            continue;
        }
        let entry = ((glyph_source.get_glyph_entry).clone()).lock().unwrap()(codepoint);
        if (entry).is_none() {
            continue;
        }
        if ((*previous_codepoint.lock().unwrap()).clone() >= 0.0_f64) {
            (*pen.lock().unwrap()) += ((glyph_source.get_glyph_kerning).clone()).lock().unwrap()(
                (*previous_codepoint.lock().unwrap()).clone(),
                codepoint,
            );
        }
        if ((entry.as_ref().unwrap().width > 0.0_f64) && (entry.as_ref().unwrap().height > 0.0_f64))
        {
            (*glyphs.lock().unwrap()).push(BitmapTextGlyph {
                __flight_identity: std::sync::Arc::new(()),
                codepoint: codepoint,
                entry: (entry).clone().unwrap(),
                pen_within_word: (*pen.lock().unwrap()).clone(),
            });
        }
        (*pen.lock().unwrap()) += (entry.as_ref().unwrap().advance + letter_spacing);
        (*previous_codepoint.lock().unwrap()) = codepoint;
        (*in_word.lock().unwrap()) = true;
    }
    ((flush).clone()).lock().unwrap()();
    return (*tokens.lock().unwrap()).clone();
}

// Source: upstream/packages/bitmaptext/src/updateBitmapText.ts:177 (sha256:6204caf477405079ed4f0f65be3ee86fe97d7be8e1103f06325b4fe3027cd8b5)
fn ensure_bitmap_text_page_batch(
    bitmap_text: &BitmapText,
    runtime: &mut BitmapTextRuntime,
    glyph_source: &GlyphSource,
    color: f64,
    pages: &mut Vec<(f64, BitmapTextPageBatch)>,
    page: f64,
) -> Option<BitmapTextPageBatch> {
    let cached = pages
        .iter()
        .find(|(key, _)| key == &page)
        .map(|(_, value)| value.clone());
    if (cached).is_some() {
        return Some((cached.as_ref().unwrap()).clone());
    }
    let image = ((glyph_source.get_glyph_atlas_image).clone())
        .lock()
        .unwrap()(page);
    if (image).is_none() {
        return None;
    }
    while ((runtime.quad_batches.len() as f64) <= page) {
        let created = create_quad_batch(Some(QuadBatch {
            __flight_identity: std::sync::Arc::new(()),
            data: QuadBatchData {
                __flight_identity: std::sync::Arc::new(()),
                atlas: create_texture_atlas(None),
            },
        }));
        apply_bitmap_text_color(&created, color);
        runtime.quad_batches.push(((created).clone()).clone());
        add_node_child(bitmap_text, &created);
    }
    let mut quad_batch = runtime.quad_batches[page as usize].clone();
    let mut atlas = (quad_batch.data.atlas).clone();
    atlas.as_mut().unwrap().image = (image).clone();
    let mut page_batch: BitmapTextPageBatch = BitmapTextPageBatch {
        __flight_identity: std::sync::Arc::new(()),
        atlas: (atlas).clone().unwrap(),
        quad_batch: (quad_batch).clone(),
        region_by_codepoint: Vec::new(),
    };
    {
        let __flight_key = page;
        let __flight_value = (page_batch).clone();
        if let Some((_, value)) = pages.iter_mut().find(|(key, _)| key == &__flight_key) {
            *value = __flight_value;
        } else {
            pages.push((__flight_key, __flight_value));
        }
    };
    return Some((page_batch).clone());
}

// Source: upstream/packages/bitmaptext/src/updateBitmapText.ts:205 (sha256:3a3b63bd630db2bd5b2173e069198940411e4fa8618ab60eed8210d1ef0855cb)
fn ensure_bounds_rectangle(runtime: &mut BitmapTextRuntime) -> Rectangle {
    if ((runtime.local_bounds_rectangle).clone()).is_none() {
        runtime.local_bounds_rectangle = Some(create_rectangle(None, None, None, None));
    }
    return ((runtime.local_bounds_rectangle).clone()).unwrap();
}

// Source: upstream/packages/bitmaptext/src/updateBitmapText.ts:213 (sha256:37a28d4828e8076d45bbf780947328d6faebfad02cf933c4140e5b2eed58534d)
fn layout_bitmap_text_lines(
    glyph_source: &GlyphSource,
    data: &BitmapTextData,
) -> Vec<BitmapTextLine> {
    let mut lines: Vec<BitmapTextLine> = vec![];
    let paragraphs = (data.text.split)("\n");
    {
        let mut pi = 0.0_f64;
        while (pi < (paragraphs.len() as f64)) {
            let tokens = build_bitmap_text_words(
                glyph_source,
                paragraphs[pi as usize].clone(),
                data.letter_spacing,
            );
            let mut current: BitmapTextLine = BitmapTextLine {
                __flight_identity: std::sync::Arc::new(()),
                words: vec![],
                gaps: vec![],
                width: 0.0_f64,
                paragraph_end: false,
            };
            for token in (tokens).iter().cloned() {
                let wraps = (((data.wrap_width).is_some()
                    && ((current.words.len() as f64) > 0.0_f64))
                    && (((current.width + token.gap) + token.word.width) > data.wrap_width));
                if wraps {
                    lines.push(((current).clone()).clone());
                    current = BitmapTextLine {
                        __flight_identity: std::sync::Arc::new(()),
                        words: vec![(token.word).clone()],
                        gaps: vec![],
                        width: token.word.width,
                        paragraph_end: false,
                    };
                } else {
                    if ((current.words.len() as f64) > 0.0_f64) {
                        current.gaps.push(token.gap);
                        current.width += token.gap;
                    }
                    current.words.push((token.word).clone());
                    current.width += token.word.width;
                }
            }
            current.paragraph_end = true;
            lines.push(((current).clone()).clone());
            {
                pi += 1.0;
                pi
            };
        }
    }
    return (lines).clone();
}

// Source: upstream/packages/bitmaptext/src/updateBitmapText.ts:242 (sha256:d2a73c20d95a1a1512ff1f0278c6bd97e0108f4d220b7013c3ed2c4ffbff3066)
fn max_line_width(lines: &Vec<BitmapTextLine>) -> f64 {
    let mut max = 0.0_f64;
    for line in (lines).iter().cloned() {
        if (line.width > max) {
            max = line.width;
        }
    }
    return max;
}

// Source: upstream/packages/bitmaptext/src/updateBitmapText.ts:248 (sha256:a394c69d44085bed7f8938b68dae56d8b1073fef074f7cca0b7872f173cda766)
fn set_empty_rectangle(out: &mut Rectangle) -> () {
    out.x = 0.0_f64;
    out.y = 0.0_f64;
    out.width = 0.0_f64;
    out.height = 0.0_f64;
}

// Source: upstream/packages/bitmaptext/src/updateBitmapText.ts:257 (sha256:addde9091e717fe715e2337f95e4c64b535673b79412c3fccda76136eb05f90a)
#[derive(Clone)]
struct BitmapTextGlyph {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub codepoint: f64,
    pub entry: GlyphEntry,
    pub pen_within_word: f64,
}
impl PartialEq for BitmapTextGlyph {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/bitmaptext/src/updateBitmapText.ts:265 (sha256:f6f1145192267bf5628b287995e2a28d9e834b5c7c96a7c9d16b90e38ccf38eb)
#[derive(Clone)]
struct BitmapTextPageBatch {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub atlas: TextureAtlas,
    pub quad_batch: QuadBatch,
    pub region_by_codepoint: Vec<(f64, f64)>,
}
impl PartialEq for BitmapTextPageBatch {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/bitmaptext/src/updateBitmapText.ts:273 (sha256:e16d053098a238a17b8aff009c25ee0710c6f3819a3a825092cf06b4ed9783c0)
#[derive(Clone)]
struct BitmapTextLine {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub gaps: Vec<f64>,
    pub paragraph_end: bool,
    pub width: f64,
    pub words: Vec<BitmapTextWord>,
}
impl PartialEq for BitmapTextLine {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/bitmaptext/src/updateBitmapText.ts:281 (sha256:2dafecf5ba6b88df3930aa9c085040985a8191db73398e385aacc144cc6698c4)
#[derive(Clone)]
struct BitmapTextToken {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub gap: f64,
    pub word: BitmapTextWord,
}
impl PartialEq for BitmapTextToken {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/bitmaptext/src/updateBitmapText.ts:287 (sha256:3401bd7270b995d95bd0883e9b0104675fac441bbb5d1b368e2ff25dc110b1b9)
#[derive(Clone)]
struct BitmapTextWord {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub glyphs: Vec<BitmapTextGlyph>,
    pub width: f64,
}
impl PartialEq for BitmapTextWord {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
