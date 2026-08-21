// @generated from upstream/packages/glyphatlas/src/glyphAtlasEntry.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_bitmap::{create_bitmap_region, write_bitmap_pixels};
use flighthq_types::{
    GlyphAtlas, GlyphAtlasRuntime, GlyphAtlasShelf, GlyphEntry, GlyphRasterizedBitmap,
};

#[derive(Clone, Default)]
pub struct SharedStructuralRecord1 {
    pub __flight_identity: std::sync::Arc<()>,
    pub x: f64,
    pub y: f64,
}
impl PartialEq for SharedStructuralRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/glyphatlas/src/glyphAtlasEntry.ts:9 (sha256:3e9498a708f83d15b746cc24aedafb87927466108876e5eeeb0f412b14c0a46a)
pub fn get_glyph_atlas_entry(atlas: &mut GlyphAtlas, codepoint: f64) -> Option<GlyphEntry> {
    let existing = atlas
        .runtime
        .entries
        .iter()
        .find(|(entry_key, _)| entry_key == &codepoint)
        .map(|(_, value)| value.clone());
    if (existing).is_some() {
        _touch_glyph_lru(&mut atlas.runtime, codepoint);
        return Some((existing.as_ref().unwrap()).clone());
    }
    let bitmap = {
        let __flight_callback = (atlas.runtime.rasterizer_backend.rasterize).clone();
        let __flight_result =
            __flight_callback.lock().unwrap()(codepoint, (atlas.runtime.rasterize_options).clone());
        __flight_result
    };
    if (bitmap).is_none() {
        {
            let __flight_callback = (*_ENTRY_GUARD.lock().unwrap()).clone();
            __flight_callback.as_ref().map(|callback| {
                callback.lock().unwrap()("rasterizer-returned-null".to_owned(), codepoint)
            })
        };
        return None;
    }
    let padding = atlas.runtime.padding;
    let usable_width = (atlas.runtime.bitmap.width - (2.0_f64 * padding));
    let usable_height = (atlas.runtime.bitmap.height - (2.0_f64 * padding));
    if (bitmap.as_ref().unwrap().width > usable_width)
        || (bitmap.as_ref().unwrap().height > usable_height)
    {
        {
            let __flight_callback = (*_ENTRY_GUARD.lock().unwrap()).clone();
            __flight_callback.as_ref().map(|callback| {
                callback.lock().unwrap()("glyph-larger-than-atlas".to_owned(), codepoint)
            })
        };
        return None;
    }
    let mut needs_repack = false;
    let incoming_bytes = bitmap.as_ref().unwrap().pixels.byte_length;
    let incoming_area = (bitmap.as_ref().unwrap().width * bitmap.as_ref().unwrap().height);
    while _is_glyph_atlas_over_budget(&atlas.runtime, incoming_bytes, incoming_area) {
        if (!_evict_least_recently_used_glyph(&mut atlas.runtime)) {
            break;
        }
        needs_repack = true;
    }
    let mut placement = _place_glyph_on_shelf(
        &mut atlas.runtime,
        bitmap.as_ref().unwrap().width,
        bitmap.as_ref().unwrap().height,
    );
    if ((placement).is_none()) && (needs_repack) {
        _repack_glyph_atlas((atlas.runtime).clone());
        placement = _place_glyph_on_shelf(
            &mut atlas.runtime,
            bitmap.as_ref().unwrap().width,
            bitmap.as_ref().unwrap().height,
        );
    }
    while (placement).is_none() {
        if ((atlas.runtime.entries.len() as f64) == 0.0_f64) {
            return None;
        }
        _evict_least_recently_used_glyph(&mut atlas.runtime);
        _repack_glyph_atlas((atlas.runtime).clone());
        placement = _place_glyph_on_shelf(
            &mut atlas.runtime,
            bitmap.as_ref().unwrap().width,
            bitmap.as_ref().unwrap().height,
        );
    }
    let entry: GlyphEntry = GlyphEntry {
        __flight_identity: std::sync::Arc::new(()),
        advance: bitmap.as_ref().unwrap().advance,
        bearing_x: bitmap.as_ref().unwrap().bearing_x,
        bearing_y: bitmap.as_ref().unwrap().bearing_y,
        height: bitmap.as_ref().unwrap().height,
        page: 0.0_f64,
        width: bitmap.as_ref().unwrap().width,
        x: placement.as_mut().unwrap().x,
        y: placement.as_mut().unwrap().y,
    };
    {
        let __flight_key = codepoint;
        let __flight_value = (entry).clone();
        if let Some((_, value)) = atlas
            .runtime
            .entries
            .iter_mut()
            .find(|(key, _)| key == &__flight_key)
        {
            *value = __flight_value;
        } else {
            atlas.runtime.entries.push((__flight_key, __flight_value));
        }
    };
    {
        let __flight_key = codepoint;
        let __flight_value = (bitmap.as_ref().unwrap()).clone();
        if let Some((_, value)) = atlas
            .runtime
            .bitmaps
            .iter_mut()
            .find(|(key, _)| key == &__flight_key)
        {
            *value = __flight_value;
        } else {
            atlas.runtime.bitmaps.push((__flight_key, __flight_value));
        }
    };
    atlas.runtime.retained_bytes += incoming_bytes;
    atlas.runtime.occupied_area += incoming_area;
    {
        let __flight_key = codepoint;
        let __flight_value = true;
        if let Some((_, value)) = atlas
            .runtime
            .lru
            .iter_mut()
            .find(|(key, _)| key == &__flight_key)
        {
            *value = __flight_value;
        } else {
            atlas.runtime.lru.push((__flight_key, __flight_value));
        }
    };
    _blit_glyph_into_atlas_bitmap(&mut atlas.runtime, &entry, &bitmap.as_ref().unwrap());
    return Some((entry).clone());
}

// Source: upstream/packages/glyphatlas/src/glyphAtlasEntry.ts:81 (sha256:8aff5c03aeadfb919b4068fc13d595582c781709d7718396d70bafcd9cf6b21f)
fn _blit_glyph_into_atlas_bitmap(
    runtime: &mut GlyphAtlasRuntime,
    entry: &GlyphEntry,
    bitmap: &GlyphRasterizedBitmap,
) -> () {
    let mut region = create_bitmap_region(
        &runtime.bitmap,
        Some(entry.x),
        Some(entry.y),
        Some(entry.width),
        Some(entry.height),
    );
    write_bitmap_pixels(&mut region, &bitmap.pixels);
    _mark_glyph_atlas_dirty_rect(runtime, entry.x, entry.y, entry.width, entry.height);
}

// Source: upstream/packages/glyphatlas/src/glyphAtlasEntry.ts:94 (sha256:ae7ce92d7f2ab063880a08c0c105bce750838a47ab6cd3e35d221adab916eaac)
fn _evict_least_recently_used_glyph(runtime: &mut GlyphAtlasRuntime) -> bool {
    let oldest = (runtime
        .lru
        .iter()
        .map(|(key, _)| key.clone())
        .collect::<Vec<_>>()
        .next)();
    if (oldest.done == true) {
        return false;
    }
    let codepoint = oldest.value;
    {
        let __flight_key = codepoint;
        if let Some(__flight_index) = runtime.lru.iter().position(|(key, _)| key == &__flight_key) {
            runtime.lru.remove(__flight_index);
            true
        } else {
            false
        }
    };
    _release_glyph_budget(runtime, codepoint);
    {
        let __flight_key = codepoint;
        if let Some(__flight_index) = runtime
            .entries
            .iter()
            .position(|(key, _)| key == &__flight_key)
        {
            runtime.entries.remove(__flight_index);
            true
        } else {
            false
        }
    };
    {
        let __flight_key = codepoint;
        if let Some(__flight_index) = runtime
            .bitmaps
            .iter()
            .position(|(key, _)| key == &__flight_key)
        {
            runtime.bitmaps.remove(__flight_index);
            true
        } else {
            false
        }
    };
    return true;
}

// Source: upstream/packages/glyphatlas/src/glyphAtlasEntry.ts:108 (sha256:da70939e3a39108695dfbbc0366138b81433df961a4993f85386bb040897b78d)
fn _mark_glyph_atlas_dirty_rect(
    runtime: &mut GlyphAtlasRuntime,
    x: f64,
    y: f64,
    width: f64,
    height: f64,
) -> () {
    let max_x = (x + width);
    let max_y = (y + height);
    if (!runtime.dirty) {
        runtime.dirty = true;
        runtime.dirty_min_x = x;
        runtime.dirty_min_y = y;
        runtime.dirty_max_x = max_x;
        runtime.dirty_max_y = max_y;
        return;
    }
    runtime.dirty_min_x = (runtime.dirty_min_x).min(x);
    runtime.dirty_min_y = (runtime.dirty_min_y).min(y);
    runtime.dirty_max_x = (runtime.dirty_max_x).max(max_x);
    runtime.dirty_max_y = (runtime.dirty_max_y).max(max_y);
}

// Source: upstream/packages/glyphatlas/src/glyphAtlasEntry.ts:135 (sha256:99e88a8b7357b7b3a4091cfadba263e3f42671daecd8c8bd071aaae2c48dd243)
fn _place_glyph_on_shelf(
    runtime: &mut GlyphAtlasRuntime,
    width: f64,
    height: f64,
) -> Option<SharedStructuralRecord1> {
    let padding = runtime.padding;
    let right_limit = (runtime.bitmap.width - padding);
    let mut best: Option<crate::OpaqueHostValue> = None;
    let mut best_slack = f64::INFINITY;
    for shelf in ((runtime.shelves).clone()).iter().cloned() {
        if (shelf.height < height) {
            continue;
        }
        if ((shelf.cursor_x + width) > right_limit) {
            continue;
        }
        let slack = (shelf.height - height);
        if (slack < best_slack) {
            best = Some({
                let __flight_portable_source = (shelf).clone();
                crate::FlightValue::Record({
                    let mut __flight_record = Vec::new();
                    __flight_record.push((
                        "cursorX".to_owned(),
                        crate::FlightValue::Number(
                            *(&((&__flight_portable_source).cursor_x)) as f64,
                        ),
                    ));
                    __flight_record.push((
                        "height".to_owned(),
                        crate::FlightValue::Number(*(&((&__flight_portable_source).height)) as f64),
                    ));
                    __flight_record.push((
                        "y".to_owned(),
                        crate::FlightValue::Number(*(&((&__flight_portable_source).y)) as f64),
                    ));
                    __flight_record
                })
            });
            best_slack = slack;
        }
    }
    if (best).is_some() {
        let x = crate::host_value::<crate::OpaqueHostValue>("host.cursorX");
        crate::host_set("host.cursorX", ((x + width) + padding));
        return Some(SharedStructuralRecord1 {
            __flight_identity: std::sync::Arc::new(()),
            x: x,
            y: crate::host_value::<f64>("host.y"),
        });
    }
    let y = runtime.pack_bottom;
    if ((y + height) > (runtime.bitmap.height - padding)) {
        return None;
    }
    if ((padding + width) > right_limit) {
        return None;
    }
    runtime.shelves.push(GlyphAtlasShelf {
        __flight_identity: std::sync::Arc::new(()),
        cursor_x: ((padding + width) + padding),
        height: height,
        y: y,
    });
    runtime.pack_bottom = ((y + height) + padding);
    return Some(SharedStructuralRecord1 {
        __flight_identity: std::sync::Arc::new(()),
        x: padding,
        y: y,
    });
}

// Source: upstream/packages/glyphatlas/src/glyphAtlasEntry.ts:173 (sha256:c933d1b0b03c6bc81663b09c38a7a753ffb70733ba260a2d692c46b9c0f36745)
fn _repack_glyph_atlas(mut runtime: GlyphAtlasRuntime) -> () {
    runtime.shelves.clear();
    runtime.pack_bottom = runtime.padding;
    {
        let __flight_value = (0.0_f64) as u8;
        let __flight_collection = &mut runtime.bitmap.data;
        __flight_collection.fill(__flight_value);
        __flight_collection.clone()
    };
    let codepoints = {
        let mut __flight_values = {
            let mut __flight_array = Vec::new();
            __flight_array.extend(
                (runtime
                    .entries
                    .iter()
                    .map(|(key, _)| key.clone())
                    .collect::<Vec<_>>())
                .iter()
                .cloned(),
            );
            __flight_array
        };
        __flight_values.sort_by(|left, right| {
            let __flight_order = (|a: crate::OpaqueHostValue, b: crate::OpaqueHostValue| -> f64 {
                let height_delta = (runtime
                    .entries
                    .iter()
                    .find(|(entry_key, _)| entry_key == &(b).clone())
                    .map(|(_, value)| value.clone())
                    .as_ref()
                    .unwrap()
                    .height
                    - runtime
                        .entries
                        .iter()
                        .find(|(entry_key, _)| entry_key == &(a).clone())
                        .map(|(_, value)| value.clone())
                        .as_ref()
                        .unwrap()
                        .height);
                return if (height_delta != 0.0_f64) {
                    height_delta
                } else {
                    (a - b)
                };
            })(left.clone(), right.clone());
            __flight_order
                .partial_cmp(&0.0_f64)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        __flight_values
    };
    for codepoint in (codepoints).iter().cloned() {
        let mut entry = runtime
            .entries
            .iter()
            .find(|(entry_key, _)| entry_key == &(codepoint).clone())
            .map(|(_, value)| value.clone());
        let bitmap = runtime
            .bitmaps
            .iter()
            .find(|(entry_key, _)| entry_key == &(codepoint).clone())
            .map(|(_, value)| value.clone());
        let placement = _place_glyph_on_shelf(
            &mut runtime,
            bitmap.as_ref().unwrap().width,
            bitmap.as_ref().unwrap().height,
        );
        if (placement).is_none() {
            {
                let __flight_callback = (*_ENTRY_GUARD.lock().unwrap()).clone();
                __flight_callback.as_ref().map(|callback| {
                    callback.lock().unwrap()("repack-dropped".to_owned(), (codepoint).clone())
                })
            };
            _release_glyph_budget(&mut runtime, (codepoint).clone());
            {
                let __flight_key = (codepoint).clone();
                if let Some(__flight_index) = runtime
                    .entries
                    .iter()
                    .position(|(key, _)| key == &__flight_key)
                {
                    runtime.entries.remove(__flight_index);
                    true
                } else {
                    false
                }
            };
            {
                let __flight_key = (codepoint).clone();
                if let Some(__flight_index) = runtime
                    .bitmaps
                    .iter()
                    .position(|(key, _)| key == &__flight_key)
                {
                    runtime.bitmaps.remove(__flight_index);
                    true
                } else {
                    false
                }
            };
            {
                let __flight_key = (codepoint).clone();
                if let Some(__flight_index) =
                    runtime.lru.iter().position(|(key, _)| key == &__flight_key)
                {
                    runtime.lru.remove(__flight_index);
                    true
                } else {
                    false
                }
            };
            continue;
        }
        entry.as_mut().unwrap().x = placement.as_ref().unwrap().x;
        entry.as_mut().unwrap().y = placement.as_ref().unwrap().y;
        let mut region = create_bitmap_region(
            &runtime.bitmap,
            Some(entry.as_mut().unwrap().x),
            Some(entry.as_mut().unwrap().y),
            Some(entry.as_mut().unwrap().width),
            Some(entry.as_mut().unwrap().height),
        );
        write_bitmap_pixels(&mut region, &bitmap.as_ref().unwrap().pixels);
    }
    {
        let __flight_argument_3 = runtime.bitmap.width;
        let __flight_argument_4 = runtime.bitmap.height;
        let __flight_result = _mark_glyph_atlas_dirty_rect(
            &mut runtime,
            0.0_f64,
            0.0_f64,
            __flight_argument_3,
            __flight_argument_4,
        );
        __flight_result
    };
}

// Source: upstream/packages/glyphatlas/src/glyphAtlasEntry.ts:203 (sha256:099edbb6014bfbca56b586c344c7245950e45e5af330003fc1c30247147847fa)
fn _touch_glyph_lru(runtime: &mut GlyphAtlasRuntime, codepoint: f64) -> () {
    {
        let __flight_key = codepoint;
        if let Some(__flight_index) = runtime.lru.iter().position(|(key, _)| key == &__flight_key) {
            runtime.lru.remove(__flight_index);
            true
        } else {
            false
        }
    };
    {
        let __flight_key = codepoint;
        let __flight_value = true;
        if let Some((_, value)) = runtime.lru.iter_mut().find(|(key, _)| key == &__flight_key) {
            *value = __flight_value;
        } else {
            runtime.lru.push((__flight_key, __flight_value));
        }
    };
}

// Source: upstream/packages/glyphatlas/src/glyphAtlasEntry.ts:214 (sha256:79acd9cd620b379255b3a5d8cdb23d5c738bd444a8d0c15c2ea0802c5137d664)
fn _is_glyph_atlas_over_budget(
    runtime: &GlyphAtlasRuntime,
    incoming_bytes: f64,
    incoming_area: f64,
) -> bool {
    if ((runtime.entries.len() as f64) == 0.0_f64) {
        return false;
    }
    if (runtime.max_glyphs > 0.0_f64) && ((runtime.entries.len() as f64) >= runtime.max_glyphs) {
        return true;
    }
    if (runtime.max_bytes > 0.0_f64)
        && ((runtime.retained_bytes + incoming_bytes) > runtime.max_bytes)
    {
        return true;
    }
    if (runtime.max_area > 0.0_f64) && ((runtime.occupied_area + incoming_area) > runtime.max_area)
    {
        return true;
    }
    return false;
}

// Source: upstream/packages/glyphatlas/src/glyphAtlasEntry.ts:225 (sha256:79933cd4446589a8b18816e70a892223c8d3bfa611524132bda6078c46f43fcc)
fn _release_glyph_budget(runtime: &mut GlyphAtlasRuntime, codepoint: f64) -> () {
    let bitmap = runtime
        .bitmaps
        .iter()
        .find(|(entry_key, _)| entry_key == &codepoint)
        .map(|(_, value)| value.clone());
    if (bitmap).is_none() {
        return;
    }
    runtime.retained_bytes -= bitmap.as_ref().unwrap().pixels.byte_length;
    runtime.occupied_area -= (bitmap.as_ref().unwrap().width * bitmap.as_ref().unwrap().height);
}

// Source: upstream/packages/glyphatlas/src/glyphAtlasEntry.ts:235 (sha256:b34f8b630bf4df95b98b8749311cacf338f50269a5682e722cf0ad793d6bfc18)
pub fn set_glyph_atlas_entry_guard(
    guard: &Option<
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(String, f64) -> () + Send + 'static>>>,
    >,
) -> () {
    (*_ENTRY_GUARD.lock().unwrap()) = (*guard).clone();
}

// Source: upstream/packages/glyphatlas/src/glyphAtlasEntry.ts:239 (sha256:0a859771828160938f49e3e2fc23f48aca34b73e3ddb1b25791578b5387f4f32)
static _ENTRY_GUARD: std::sync::LazyLock<
    std::sync::Mutex<
        Option<
            std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(String, f64) -> () + Send + 'static>>>,
        >,
    >,
> = std::sync::LazyLock::new(|| std::sync::Mutex::new(None));
