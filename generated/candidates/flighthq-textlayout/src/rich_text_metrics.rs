// @generated from upstream/packages/textlayout/src/richTextMetrics.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{
    TEXT_BOUNDS_GUTTER as text_bounds_gutter_constant, compute_text_bounds_height,
    compute_text_bounds_width,
};
use flighthq_types::{RichTextData, TextLayoutResult};

// Source: upstream/packages/textlayout/src/richTextMetrics.ts:5 (sha256:b2900d13dd2754a4d8bac797e963c1203224c0c21cf21c5f67e7955afeb262c9)
pub fn compute_rich_text_bottom_scroll_v(data: &RichTextData, layout: &TextLayoutResult) -> f64 {
    return (layout.num_lines)
        .min(((data.scroll_v + get_visible_line_count(data, layout)) - 1.0_f64));
}

// Source: upstream/packages/textlayout/src/richTextMetrics.ts:9 (sha256:b4104f25844971df78faba0ce3e052c13e22c37926be193db318382f99baba5a)
pub fn compute_rich_text_line_count(layout: &TextLayoutResult) -> f64 {
    return layout.num_lines;
}

// Source: upstream/packages/textlayout/src/richTextMetrics.ts:13 (sha256:e078cd35aa3ea18bfa4282bfd9649a07535ff47de2277105988caf72968ecd42)
pub fn compute_rich_text_max_scroll_h(data: &RichTextData, layout: &TextLayoutResult) -> f64 {
    let visible_width = (0.0_f64)
        .max((compute_text_bounds_width(data, layout) - (text_bounds_gutter_constant * 2.0_f64)));
    return (0.0_f64).max((layout.text_width - visible_width).ceil());
}

// Source: upstream/packages/textlayout/src/richTextMetrics.ts:18 (sha256:c67264363342a08c053b70ec96582fe0c08425f68af38c7135a2e5bce4d3cae6)
pub fn compute_rich_text_max_scroll_v(data: &RichTextData, layout: &TextLayoutResult) -> f64 {
    if (layout.num_lines <= 1.0_f64) {
        return 1.0_f64;
    }
    return (1.0_f64).max(((layout.num_lines - get_visible_line_count(data, layout)) + 1.0_f64));
}

// Source: upstream/packages/textlayout/src/richTextMetrics.ts:23 (sha256:dc27eda0d96e9de5c23a05884ce03999131a7b4dcbb1f51abc00a7746019faa7)
pub fn compute_rich_text_text_height(layout: &TextLayoutResult) -> f64 {
    return (layout.text_height).ceil();
}

// Source: upstream/packages/textlayout/src/richTextMetrics.ts:27 (sha256:b8d5b111b53c9a31fe9d64352863bcbe144a7660d33ecaa476c408f90ea3069e)
pub fn compute_rich_text_text_width(layout: &TextLayoutResult) -> f64 {
    return (layout.text_width).ceil();
}

// Source: upstream/packages/textlayout/src/richTextMetrics.ts:31 (sha256:1664ac84fd15a34b258921ad214aff0ef2236771f4ecaf0ba81b55ae31c98e19)
pub fn get_rich_text_scroll_y_offset(line_heights: &Vec<f64>, first_visible_line: f64) -> f64 {
    let mut offset = 0.0_f64;
    let limit = (first_visible_line).min((line_heights.len() as f64));
    {
        let mut i = 0.0_f64;
        while (i < limit) {
            offset += line_heights[i as usize].clone();
            {
                i += 1.0;
                i
            };
        }
    }
    return offset;
}

// Source: upstream/packages/textlayout/src/richTextMetrics.ts:38 (sha256:c1f89faf9d4093caa3b15d6200f0d8c577669082e9d43234768d85141238f965)
fn get_visible_line_count(data: &RichTextData, layout: &TextLayoutResult) -> f64 {
    let visible_height = (0.0_f64)
        .max((compute_text_bounds_height(data, layout) - (text_bounds_gutter_constant * 2.0_f64)));
    if (visible_height == 0.0_f64) {
        return 1.0_f64;
    }
    let mut total = 0.0_f64;
    let mut count = 0.0_f64;
    for height in ((layout.line_heights).clone()).iter().cloned() {
        if (count > 0.0_f64) && ((total + height) > visible_height) {
            break;
        }
        total += height;
        {
            count += 1.0;
            count
        };
    }
    return (1.0_f64).max(count);
}
