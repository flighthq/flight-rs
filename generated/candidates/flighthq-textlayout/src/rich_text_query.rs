// @generated from upstream/packages/textlayout/src/richTextQuery.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::TEXT_LAYOUT_GUTTER as text_layout_gutter_constant;
use flighthq_types::{
    Rectangle, TextLayoutGroup, TextLayoutResult, TextLineMetrics, TextSelectionRectangle,
};

// Source: upstream/packages/textlayout/src/richTextQuery.ts:11 (sha256:b0ec9b594c94c81e786bbab75f4f8e9c2961db2321d263120e08be8d4cc5f593)
pub fn compute_rich_text_char_index_at_point(layout: &TextLayoutResult, x: f64, y: f64) -> f64 {
    if ((layout.groups.len() as f64) == 0.0_f64) {
        return 0.0_f64;
    }
    let mut closest_line_index = 0.0_f64;
    let mut closest_dist = f64::INFINITY;
    let mut closest_line_bottom = 0.0_f64;
    {
        let mut i = 0.0_f64;
        while (i < (layout.line_heights.len() as f64)) {
            let line_top = get_line_offset_y(layout, i);
            let line_bottom = (line_top + layout.line_heights[i as usize].clone());
            let dist = if (y < line_top) {
                (line_top - y)
            } else {
                if (y > line_bottom) {
                    (y - line_bottom)
                } else {
                    0.0_f64
                }
            };
            if (dist < closest_dist) {
                closest_dist = dist;
                closest_line_index = i;
                closest_line_bottom = line_bottom;
            }
            {
                i += 1.0;
                i
            };
        }
    }
    if (y > closest_line_bottom) {
        let mut line_end = 0.0_f64;
        for group in ((layout.groups).clone()).iter().cloned() {
            if (group.line_index == closest_line_index) {
                line_end = (line_end).max(group.end_index);
            }
        }
        return line_end;
    }
    let mut line_start = if ((layout.groups.len() as f64) > 0.0_f64) {
        (layout.groups[((layout.groups.len() as f64) - 1.0_f64) as usize].end_index)
            .unwrap_or(0.0_f64)
    } else {
        0.0_f64
    };
    let mut line_end = 0.0_f64;
    for group in ((layout.groups).clone()).iter().cloned() {
        if (group.line_index != closest_line_index) {
            continue;
        }
        line_start = (line_start).min(group.start_index);
        line_end = (line_end).max(group.end_index);
        if (x <= group.offset_x) {
            return group.start_index;
        }
        if (x <= (group.offset_x + group.width)) {
            let mut gx = group.offset_x;
            {
                let mut i = 0.0_f64;
                while (i < (group.positions.len() as f64)) {
                    let advance = group.positions[i as usize].clone();
                    if (x <= (gx + (advance / 2.0_f64))) {
                        return (group.start_index + i);
                    }
                    gx += advance;
                    {
                        i += 1.0;
                        i
                    };
                }
            }
            return group.end_index;
        }
    }
    return if (line_end > 0.0_f64) {
        line_end
    } else {
        line_start
    };
}

// Source: upstream/packages/textlayout/src/richTextQuery.ts:57 (sha256:6582d2788a8c23fcad529a500a32ee9add0d5b68e8605cdbbbd6dbfaf06d8427)
pub fn compute_rich_text_line_metrics(
    layout: &TextLayoutResult,
    line_index: f64,
) -> Option<TextLineMetrics> {
    let mut ascent = 0.0_f64;
    let mut descent = 0.0_f64;
    let mut leading = 0.0_f64;
    let mut x = f64::INFINITY;
    let mut right = 0.0_f64;
    let mut found = false;
    for group in ((layout.groups).clone()).iter().cloned() {
        if (group.line_index != line_index) {
            continue;
        }
        found = true;
        ascent = (ascent).max(group.ascent);
        descent = (descent).max(group.descent);
        leading = (leading).max(group.leading);
        x = (x).min(group.offset_x);
        right = (right).max((group.offset_x + group.width));
    }
    if (!found) {
        return None;
    }
    return Some(TextLineMetrics {
        __flight_identity: std::sync::Arc::new(()),
        ascent: ascent,
        descent: descent,
        height: layout.line_heights[line_index as usize].clone(),
        leading: leading,
        width: (right - x),
        x: if (x == f64::INFINITY) { 0.0_f64 } else { x },
    });
}

// Source: upstream/packages/textlayout/src/richTextQuery.ts:89 (sha256:dae6ca957331d8e6f233baecded31d66991e860c1cd4e6ced1092d1144d2dc7b)
pub fn get_rich_text_char_boundaries(
    out: &mut Rectangle,
    layout: &TextLayoutResult,
    char_index: f64,
) -> bool {
    let mut group = get_group_containing_index(layout, char_index);
    if (group).is_none() {
        return false;
    }
    let mut x = crate::host_value::<crate::OpaqueHostValue>("host.offsetX");
    let limit = (char_index - crate::host_value::<crate::OpaqueHostValue>("host.startIndex"))
        .min(crate::host_value::<f64>("host.length"));
    {
        let mut i = 0.0_f64;
        while (i < limit) {
            x += (crate::host_value::<Option<f64>>("host.index")).unwrap_or(0.0_f64);
            {
                i += 1.0;
                i
            };
        }
    }
    let char_width = (crate::host_value::<Option<f64>>("host.index")).unwrap_or(0.0_f64);
    out.x = x;
    out.y = crate::host_value::<f64>("host.offsetY");
    out.width = char_width;
    out.height = crate::host_value::<f64>("host.height");
    return true;
}

// Source: upstream/packages/textlayout/src/richTextQuery.ts:109 (sha256:0298d7186f72dc2b349928f8c31fc22aac7dce5366ecbc2061dd1567d0ae3484)
pub fn get_rich_text_first_char_in_paragraph(text: String, char_index: f64) -> f64 {
    let clamped = (0.0_f64).max((text.encode_utf16().count() as f64).min(char_index));
    {
        let mut i = (clamped - 1.0_f64);
        while (i >= 0.0_f64) {
            if (text[i as usize].clone() == "\n") {
                return (i + 1.0_f64);
            }
            {
                i -= 1.0;
                i
            };
        }
    }
    return 0.0_f64;
}

// Source: upstream/packages/textlayout/src/richTextQuery.ts:117 (sha256:173eb9ec6fb2ef89b26d615067c424cdc517d47b11c5bd0da33192bb617d5e05)
pub fn get_rich_text_line_index_at_point(layout: &TextLayoutResult, y: f64) -> f64 {
    let mut closest_line_index = 0.0_f64;
    let mut closest_dist = f64::INFINITY;
    {
        let mut i = 0.0_f64;
        while (i < (layout.line_heights.len() as f64)) {
            let line_top = get_line_offset_y(layout, i);
            let line_bottom = (line_top + layout.line_heights[i as usize].clone());
            let dist = if (y < line_top) {
                (line_top - y)
            } else {
                if (y > line_bottom) {
                    (y - line_bottom)
                } else {
                    0.0_f64
                }
            };
            if (dist < closest_dist) {
                closest_dist = dist;
                closest_line_index = i;
            }
            {
                i += 1.0;
                i
            };
        }
    }
    return closest_line_index;
}

// Source: upstream/packages/textlayout/src/richTextQuery.ts:132 (sha256:b3313ad08b40bb441598ac753c053328e87bb3aefc9df22db3a8c3a7b1ab6c3f)
pub fn get_rich_text_line_index_of_char(layout: &TextLayoutResult, char_index: f64) -> f64 {
    let group = get_group_containing_index(layout, char_index);
    return (crate::host_value::<crate::OpaqueHostValue>("host.lineIndex")).unwrap_or(0.0_f64);
}

// Source: upstream/packages/textlayout/src/richTextQuery.ts:137 (sha256:c6d996feb6316a64eb2c5f1b8ce0f215b5d0a29d7b519c089a7284ac956f6764)
pub fn get_rich_text_line_length(layout: &TextLayoutResult, line_index: f64) -> f64 {
    let mut start = f64::INFINITY;
    let mut end = 0.0_f64;
    for group in ((layout.groups).clone()).iter().cloned() {
        if (group.line_index != line_index) {
            continue;
        }
        start = (start).min(group.start_index);
        end = (end).max(group.end_index);
    }
    return if (start == f64::INFINITY) {
        0.0_f64
    } else {
        (end - start)
    };
}

// Source: upstream/packages/textlayout/src/richTextQuery.ts:148 (sha256:ad9defe314b84d24d829a584f788770725513c4ed1c1355149a8ec21f786303c)
pub fn get_rich_text_line_offset(layout: &TextLayoutResult, line_index: f64) -> f64 {
    for group in ((layout.groups).clone()).iter().cloned() {
        if (group.line_index == line_index) {
            return group.start_index;
        }
    }
    return 0.0_f64;
}

// Source: upstream/packages/textlayout/src/richTextQuery.ts:155 (sha256:7b288db0f92270185f52c5d0e053f882f4a927b89bac642decfd0cd31749e3d5)
pub fn get_rich_text_line_text(text: String, layout: &TextLayoutResult, line_index: f64) -> String {
    let mut start = f64::INFINITY;
    let mut end = 0.0_f64;
    for group in ((layout.groups).clone()).iter().cloned() {
        if (group.line_index != line_index) {
            continue;
        }
        start = (start).min(group.start_index);
        end = (end).max(group.end_index);
    }
    return if (start == f64::INFINITY) {
        "".to_owned()
    } else {
        (text.slice)(start, end)
    };
}

// Source: upstream/packages/textlayout/src/richTextQuery.ts:166 (sha256:5117064cd2bf44c90995796f6def5812b8a8bf6a0f52ca07d5955c6d387252d0)
pub fn get_rich_text_link_at_point(layout: &TextLayoutResult, x: f64, y: f64) -> Option<String> {
    for group in ((layout.groups).clone()).iter().cloned() {
        if ((group.format.url).clone()).is_none() {
            continue;
        }
        if (((x >= group.offset_x) && (x <= (group.offset_x + group.width)))
            && (y >= group.offset_y))
            && (y <= (group.offset_y + group.height))
        {
            return (group.format.url).clone();
        }
    }
    return None;
}

// Source: upstream/packages/textlayout/src/richTextQuery.ts:181 (sha256:cef0a11727296f4dc4f8de3196eb01f49fd68c7e8500a6711291396728d6e545)
pub fn get_rich_text_paragraph_length(text: String, char_index: f64) -> f64 {
    let start = get_rich_text_first_char_in_paragraph((text).clone(), char_index);
    let newline = (text.index_of)("\n", start);
    let end = if (newline == (-1.0_f64)) {
        (text.encode_utf16().count() as f64)
    } else {
        (newline + 1.0_f64)
    };
    return (end - start);
}

// Source: upstream/packages/textlayout/src/richTextQuery.ts:188 (sha256:5e40ca0b3e462bc43511dc698b412b3a1b097be2a691a4052f16525e2a06e6bd)
pub fn get_rich_text_selection_rectangles(
    out: &mut Vec<TextSelectionRectangle>,
    begin_index: f64,
    end_index: f64,
    layout: &TextLayoutResult,
) -> () {
    out.clear();
    if (begin_index == end_index) {
        return;
    }
    let start = (begin_index).min(end_index);
    let end = (begin_index).max(end_index);
    for group in ((layout.groups).clone()).iter().cloned() {
        let group_start = (start).max(group.start_index);
        let group_end = (end).min(group.end_index);
        if (group_start >= group_end) {
            continue;
        }
        let x = get_caret_x(&mut group, group_start);
        let right = get_caret_x(&mut group, group_end);
        out.push(TextSelectionRectangle {
            __flight_identity: std::sync::Arc::new(()),
            height: group.height,
            line_index: group.line_index,
            width: (right - x),
            x: x,
            y: group.offset_y,
        });
    }
}

// Source: upstream/packages/textlayout/src/richTextQuery.ts:210 (sha256:cb64de90d636e8527eaa224ad7158abadc5bf99863faf8a80d8f340b78304d1b)
fn get_caret_x(group: &mut TextLayoutGroup, index: f64) -> f64 {
    let mut x = group.offset_x;
    let limit = (0.0_f64).max(((index).min(group.end_index) - group.start_index));
    {
        let mut i = 0.0_f64;
        while (i < limit) {
            x += group.positions[i as usize].clone();
            {
                i += 1.0;
                i
            };
        }
    }
    return x;
}

// Source: upstream/packages/textlayout/src/richTextQuery.ts:217 (sha256:1de20ad3e47739ff87117a16b3be90142d7905c1172aec52d966a19e9597d534)
fn get_group_containing_index(
    layout: &TextLayoutResult,
    char_index: f64,
) -> crate::OpaqueHostValue {
    for group in ((layout.groups).clone()).iter().cloned() {
        if (char_index >= group.start_index) && (char_index < group.end_index) {
            return group;
        }
    }
    return crate::OpaqueHostValue::Null;
}

// Source: upstream/packages/textlayout/src/richTextQuery.ts:224 (sha256:c64f745710b65cd48ca87616137b9a6d6c0476b40d8b76c1c0e283b9a87c21c5)
fn get_line_offset_y(layout: &TextLayoutResult, line_index: f64) -> f64 {
    for group in ((layout.groups).clone()).iter().cloned() {
        if (group.line_index == line_index) {
            return group.offset_y;
        }
    }
    let mut y = text_layout_gutter_constant;
    {
        let mut i = 0.0_f64;
        while (i < line_index) {
            y += layout.line_heights[i as usize].clone();
            {
                i += 1.0;
                i
            };
        }
    }
    return y;
}
