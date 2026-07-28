// @generated from upstream/packages/glyphatlas/src/glyphAtlasEntry.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::get_glyph_rasterizer_backend;
use flighthq_surface::{create_surface_region, write_surface_pixels};
use flighthq_types::{
    GlyphAtlas, GlyphAtlasRuntime, GlyphAtlasShelf, GlyphEntry, GlyphRasterizedBitmap,
};

// Source: upstream/packages/glyphatlas/src/glyphAtlasEntry.ts:11 (sha256:373222fdd72b51dac4115ab2eab87a0c412c7559b36131aae5bfeb33a49a755a)
pub fn get_glyph_atlas_entry(atlas: &mut GlyphAtlas, codepoint: f64) -> Option<GlyphEntry> {
    let existing = atlas
        .runtime
        .entries
        .iter()
        .find(|(key, _)| key == &codepoint)
        .map(|(_, value)| value.clone());
    if (existing).is_some() {
        _touch_glyph_lru(&mut atlas.runtime, codepoint);
        return Some((existing.as_ref().unwrap()).clone());
    }
    let bitmap = ((get_glyph_rasterizer_backend().rasterize).clone())
        .lock()
        .unwrap()(codepoint, (atlas.runtime.rasterize_options).clone());
    if (bitmap).is_none() {
        return None;
    }
    let padding = atlas.runtime.padding;
    let usable_width = (atlas.runtime.surface.width - (2.0_f64 * padding));
    let usable_height = (atlas.runtime.surface.height - (2.0_f64 * padding));
    if ((bitmap.as_ref().unwrap().width > usable_width)
        || (bitmap.as_ref().unwrap().height > usable_height))
    {
        return None;
    }
    let mut needs_repack = false;
    while ((atlas.runtime.max_glyphs > 0.0_f64)
        && (atlas
            .runtime
            .entries
            .iter()
            .find(|(key, _)| key == &"size")
            .map(|(_, value)| value.clone())
            .expect("TypeScript Record key was absent")
            >= atlas.runtime.max_glyphs))
    {
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
    if ((placement).is_none() && needs_repack) {
        _repack_glyph_atlas(&mut atlas.runtime);
        placement = _place_glyph_on_shelf(
            &mut atlas.runtime,
            bitmap.as_ref().unwrap().width,
            bitmap.as_ref().unwrap().height,
        );
    }
    while (placement).is_none() {
        if (atlas
            .runtime
            .entries
            .iter()
            .find(|(key, _)| key == &"size")
            .map(|(_, value)| value.clone())
            .expect("TypeScript Record key was absent")
            == 0.0_f64)
        {
            return None;
        }
        _evict_least_recently_used_glyph(&mut atlas.runtime);
        _repack_glyph_atlas(&mut atlas.runtime);
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
        let __flight_value = (bitmap).clone().unwrap();
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
    atlas.runtime.lru.push(codepoint);
    _blit_glyph_into_atlas_surface(&mut atlas.runtime, &entry, bitmap.as_ref().unwrap());
    return Some((entry).clone());
}

// Source: upstream/packages/glyphatlas/src/glyphAtlasEntry.ts:69 (sha256:0ef6ee115396a3e982f3cb9474f0823f2719347811807f13784fb5e29d1425a1)
fn _blit_glyph_into_atlas_surface(
    runtime: &mut GlyphAtlasRuntime,
    entry: &GlyphEntry,
    bitmap: &GlyphRasterizedBitmap,
) -> () {
    let mut region = create_surface_region(
        &runtime.surface,
        Some(entry.x),
        Some(entry.y),
        Some(entry.width),
        Some(entry.height),
    );
    write_surface_pixels(&mut region, &bitmap.pixels);
    _mark_glyph_atlas_dirty_rect(runtime, entry.x, entry.y, entry.width, entry.height);
}

// Source: upstream/packages/glyphatlas/src/glyphAtlasEntry.ts:82 (sha256:31bd60cf5bdbc22630ab9bee26605da714fa1f2d09585beb7e30eedd554582d9)
fn _evict_least_recently_used_glyph(runtime: &mut GlyphAtlasRuntime) -> bool {
    let codepoint = (runtime.lru.shift)();
    if (codepoint).is_none() {
        return false;
    }
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

// Source: upstream/packages/glyphatlas/src/glyphAtlasEntry.ts:92 (sha256:da70939e3a39108695dfbbc0366138b81433df961a4993f85386bb040897b78d)
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

// Source: upstream/packages/glyphatlas/src/glyphAtlasEntry.ts:119 (sha256:d6bf407e0f838a0a0c58b177fbab592d3abe3157c606abdcfb7ecc9099de0ef4)
#[derive(Clone)]
struct PlaceGlyphOnShelfRecord1 {
    __flight_identity: std::sync::Arc<()>,
    x: f64,
    y: f64,
}
impl PartialEq for PlaceGlyphOnShelfRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

fn _place_glyph_on_shelf(
    runtime: &mut GlyphAtlasRuntime,
    width: f64,
    height: f64,
) -> Option<PlaceGlyphOnShelfRecord1> {
    let padding = runtime.padding;
    let right_limit = (runtime.surface.width - padding);
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
            best = Some(shelf);
            best_slack = slack;
        }
    }
    if (best).is_some() {
        let x = crate::host_value::<crate::OpaqueHostValue>("host.cursorX");
        crate::host_set("host.cursorX", ((x + width) + padding));
        return Some(PlaceGlyphOnShelfRecord1 {
            __flight_identity: std::sync::Arc::new(()),
            x: x,
            y: crate::host_value::<f64>("host.y"),
        });
    }
    let y = runtime.pack_bottom;
    if ((y + height) > (runtime.surface.height - padding)) {
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
    return Some(PlaceGlyphOnShelfRecord1 {
        __flight_identity: std::sync::Arc::new(()),
        x: padding,
        y: y,
    });
}

// Source: upstream/packages/glyphatlas/src/glyphAtlasEntry.ts:157 (sha256:ccf03c6c637ae88c04c11c2c7d2ace70ed1b43a7521607e1ce0cc81ecf0343d5)
fn _repack_glyph_atlas(mut runtime: GlyphAtlasRuntime) -> () {
    runtime.shelves.clear();
    runtime.pack_bottom = runtime.padding;
    runtime.surface.data.fill((0.0_f64) as u8);
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
                    .find(|(key, _)| key == &b)
                    .map(|(_, value)| value.clone())
                    .as_ref()
                    .unwrap()
                    .height
                    - runtime
                        .entries
                        .iter()
                        .find(|(key, _)| key == &a)
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
            .find(|(key, _)| key == &codepoint)
            .map(|(_, value)| value.clone());
        let bitmap = runtime
            .bitmaps
            .iter()
            .find(|(key, _)| key == &codepoint)
            .map(|(_, value)| value.clone());
        let placement = _place_glyph_on_shelf(
            &mut runtime,
            bitmap.as_ref().unwrap().width,
            bitmap.as_ref().unwrap().height,
        );
        if (placement).is_none() {
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
            let lru_index = {
                let __flight_value = codepoint;
                ((runtime.lru).clone())
                    .iter()
                    .position(|item| item == &__flight_value)
                    .map_or(-1.0_f64, |index| index as f64)
            };
            if (lru_index != (-1.0_f64)) {
                runtime.lru.splice(
                    (lru_index) as usize..((lru_index) + (1.0_f64)) as usize,
                    vec![],
                );
            }
            continue;
        }
        entry.as_mut().unwrap().x = placement.as_ref().unwrap().x;
        entry.as_mut().unwrap().y = placement.as_ref().unwrap().y;
        let mut region = create_surface_region(
            &runtime.surface,
            Some(entry.as_mut().unwrap().x),
            Some(entry.as_mut().unwrap().y),
            Some(entry.as_mut().unwrap().width),
            Some(entry.as_mut().unwrap().height),
        );
        write_surface_pixels(&mut region, &bitmap.as_ref().unwrap().pixels);
    }
    _mark_glyph_atlas_dirty_rect(
        &mut runtime,
        0.0_f64,
        0.0_f64,
        runtime.surface.width,
        runtime.surface.height,
    );
}

// Source: upstream/packages/glyphatlas/src/glyphAtlasEntry.ts:186 (sha256:58d1b855596a9310033c2b5671dbc900c5d6f09d865a7a1e71a7e46ccfa742ce)
fn _touch_glyph_lru(runtime: &mut GlyphAtlasRuntime, codepoint: f64) -> () {
    let index = {
        let __flight_value = codepoint;
        ((runtime.lru).clone())
            .iter()
            .position(|item| item == &__flight_value)
            .map_or(-1.0_f64, |index| index as f64)
    };
    if (index != (-1.0_f64)) {
        runtime
            .lru
            .splice((index) as usize..((index) + (1.0_f64)) as usize, vec![]);
    }
    runtime.lru.push(codepoint);
}
