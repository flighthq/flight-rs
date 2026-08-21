// @generated from upstream/packages/textlayout/src/textLayout.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{
    create_text_layout_group, get_text_format_ascent, get_text_format_descent,
    get_text_format_leading, get_text_line_breaks, merge_text_format,
};
use flighthq_types::{
    TextDirection, TextFormat, TextFormatRange, TextJustification, TextLayoutGroup,
    TextLayoutParams, TextLayoutResult, TextMeasureFunction, TextVerticalAlign,
};

// Source: upstream/packages/textlayout/src/textLayout.ts:18 (sha256:7a3c85a761fcca811e5c5d5d387dafd74075ddb93b69ab5ab7e4baa514df6891)
pub const TEXT_LAYOUT_GUTTER: f64 = 2.0_f64;

// Source: upstream/packages/textlayout/src/textLayout.ts:20 (sha256:9b48a8b479bff422b9ca9c9d76cab39956bb0370c37fe4ad80cfb25cc4f471a9)
static _LINE_BREAKS: std::sync::LazyLock<std::sync::Mutex<Vec<f64>>> =
    std::sync::LazyLock::new(|| std::sync::Mutex::new(vec![]));

// Source: upstream/packages/textlayout/src/textLayout.ts:21 (sha256:0a7e890c25e999f1e491257d483e1bbf71e077fb59b5792cbf7f3bfefb56787d)
static _CHAR_ADVANCES: std::sync::LazyLock<std::sync::Mutex<Vec<f64>>> =
    std::sync::LazyLock::new(|| std::sync::Mutex::new(vec![]));

// Source: upstream/packages/textlayout/src/textLayout.ts:24 (sha256:556dd483602d4707bf523a13dd724f98f739e18fc66b2766ea79dbb492c56d43)
static _PARAGRAPH_LAST_LINES: std::sync::LazyLock<std::sync::Mutex<Vec<f64>>> =
    std::sync::LazyLock::new(|| std::sync::Mutex::new(Vec::new()));

// Source: upstream/packages/textlayout/src/textLayout.ts:26 (sha256:ca540e4b34f38ee2dc29926f251dc88b9f376ca4062df6f391af16e00fba8801)
pub fn compute_text_layout(out: &mut TextLayoutResult, params: &mut TextLayoutParams) -> () {
    let mut text = (params.text).clone();
    let width = params.width;
    let word_wrap = (params.word_wrap).unwrap_or(false);
    let multiline = (params.multiline).unwrap_or(false);
    let auto_size = ((params.auto_size).clone()).unwrap_or("none".to_owned());
    let border = (params.border).unwrap_or(false);
    let direction = ((params.direction).clone()).unwrap_or("LeftToRight".to_owned());
    let justification = ((params.justification).clone()).unwrap_or("interWord".to_owned());
    let max_lines = (params.max_lines).unwrap_or((-1.0_f64));
    let truncation_character = ((params.truncation_character).clone()).unwrap_or("…".to_owned());
    let vertical_align = ((params.vertical_align).clone()).unwrap_or("top".to_owned());
    if (!text) || ((params.format_ranges.len() as f64) == 0.0_f64) {
        out.groups.clear();
        out.line_ascents.clear();
        out.line_descents.clear();
        out.line_heights.clear();
        out.line_leadings.clear();
        out.line_widths.clear();
        out.num_lines = 1.0_f64;
        out.text_height = 0.0_f64;
        out.text_width = 0.0_f64;
        return;
    }
    get_text_line_breaks(&mut _LINE_BREAKS, (text).clone());
    (*_PARAGRAPH_LAST_LINES.lock().unwrap()).clear();
    build_groups(
        &mut out.groups,
        &mut (*_PARAGRAPH_LAST_LINES.lock().unwrap()),
        (text).clone(),
        (params.format_ranges).clone(),
        &_LINE_BREAKS,
        width,
        (params.measure).clone(),
        word_wrap,
        multiline,
        max_lines,
        (truncation_character).clone(),
    );
    {
        let __flight_argument_1 = (out.groups).clone();
        write_line_metrics(out, &__flight_argument_1)
    };
    apply_alignment(
        &out.groups,
        width,
        &out.line_widths,
        (direction).clone(),
        (justification).clone(),
        &(*_PARAGRAPH_LAST_LINES.lock().unwrap()),
        (text).clone(),
    );
    apply_vertical_alignment(
        &out.groups,
        params.height,
        out.text_height,
        (vertical_align).clone(),
    );
    {
        auto_size;
        ()
    };
    {
        border;
        ()
    };
}

// Source: upstream/packages/textlayout/src/textLayout.ts:90 (sha256:166c4f8ff0313c00bf36e2b11758417d971dd08eeb354097095b7eccbb2a9d73)
fn char_advances(
    out: &mut Vec<f64>,
    text: String,
    format: &TextFormat,
    start: f64,
    end: f64,
    measure: &mut impl FnMut(String, TextFormat) -> f64,
    start_x: Option<f64>,
) -> () {
    let start_x = start_x.unwrap_or(0.0_f64);
    let __flight_utf16_text: Vec<u16> = text.encode_utf16().collect();
    out.clear();
    let letter_spacing = (format.letter_spacing).unwrap_or(0.0_f64);
    let tab_stops = (format.tab_stops).clone();
    let kerning_enabled = !((format.kerning) == Some(false));
    let mut current_x = start_x;
    let mut i = start;
    while (i < end) {
        let cp = {
            let __flight_units: &[u16] = &__flight_utf16_text;
            let __flight_raw_index = i;
            let __flight_index = if __flight_raw_index.is_nan() {
                0_i64
            } else if __flight_raw_index.is_finite() {
                __flight_raw_index.trunc() as i64
            } else {
                -1_i64
            };
            if __flight_index < 0 {
                f64::NAN
            } else if let Some(&__flight_first) = __flight_units.get(__flight_index as usize) {
                let __flight_first = u32::from(__flight_first);
                if (0xD800_u32..=0xDBFF_u32).contains(&__flight_first) {
                    if let Some(&__flight_second) = __flight_units.get(__flight_index as usize + 1)
                    {
                        let __flight_second = u32::from(__flight_second);
                        if (0xDC00_u32..=0xDFFF_u32).contains(&__flight_second) {
                            (((__flight_first - 0xD800_u32) << 10)
                                + (__flight_second - 0xDC00_u32)
                                + 0x10000_u32) as f64
                        } else {
                            __flight_first as f64
                        }
                    } else {
                        __flight_first as f64
                    }
                } else {
                    __flight_first as f64
                }
            } else {
                f64::NAN
            }
        };
        let char_len = if (cp > 65535.0_f64) { 2.0_f64 } else { 1.0_f64 };
        let char = String::from_utf16_lossy(
            &(text)
                .encode_utf16()
                .skip((i) as usize)
                .take(((i + char_len) as usize).saturating_sub((i) as usize))
                .collect::<Vec<u16>>(),
        );
        let mut advance: f64;
        if (char == "\t") {
            advance = get_tab_advance(current_x, &(tab_stops), measure, format);
            out.push(advance);
            current_x += advance;
            i += char_len;
            continue;
        }
        let next_start = (i + char_len);
        if ((kerning_enabled) && (next_start < end)) && ((text.char_code_at)(next_start) != 9.0_f64)
        {
            let next_cp = {
                let __flight_units: &[u16] = &__flight_utf16_text;
                let __flight_raw_index = next_start;
                let __flight_index = if __flight_raw_index.is_nan() {
                    0_i64
                } else if __flight_raw_index.is_finite() {
                    __flight_raw_index.trunc() as i64
                } else {
                    -1_i64
                };
                if __flight_index < 0 {
                    f64::NAN
                } else if let Some(&__flight_first) = __flight_units.get(__flight_index as usize) {
                    let __flight_first = u32::from(__flight_first);
                    if (0xD800_u32..=0xDBFF_u32).contains(&__flight_first) {
                        if let Some(&__flight_second) =
                            __flight_units.get(__flight_index as usize + 1)
                        {
                            let __flight_second = u32::from(__flight_second);
                            if (0xDC00_u32..=0xDFFF_u32).contains(&__flight_second) {
                                (((__flight_first - 0xD800_u32) << 10)
                                    + (__flight_second - 0xDC00_u32)
                                    + 0x10000_u32) as f64
                            } else {
                                __flight_first as f64
                            }
                        } else {
                            __flight_first as f64
                        }
                    } else {
                        __flight_first as f64
                    }
                } else {
                    f64::NAN
                }
            };
            let next_len = if (next_cp > 65535.0_f64) {
                2.0_f64
            } else {
                1.0_f64
            };
            let next_char = String::from_utf16_lossy(
                &(text)
                    .encode_utf16()
                    .skip((next_start) as usize)
                    .take(((next_start + next_len) as usize).saturating_sub((next_start) as usize))
                    .collect::<Vec<u16>>(),
            );
            let next_w = measure(next_char, (*format).clone());
            let pair_w = measure((char + next_char), (*format).clone());
            advance = (pair_w - next_w);
        } else {
            advance = measure(char, (*format).clone());
        }
        out.push((advance + letter_spacing));
        current_x += (advance + letter_spacing);
        i += char_len;
    }
}

// Source: upstream/packages/textlayout/src/textLayout.ts:139 (sha256:4d6743b273bf54531c3505f3fac05a94f517d46a9ab43d17b53e7b6ee7dbe4bd)
fn sum_advances(positions: &Vec<f64>) -> f64 {
    let mut total = 0.0_f64;
    for p in (positions).iter().cloned() {
        total += p;
    }
    return total;
}

// Source: upstream/packages/textlayout/src/textLayout.ts:145 (sha256:3a91afce7bc4b223eb4e6b65375f3fdae16e5594f0e7efc32e780af892dbb314)
fn get_tab_advance(
    current_x: f64,
    tab_stops: &Option<Vec<f64>>,
    measure: &mut impl FnMut(String, TextFormat) -> f64,
    format: &TextFormat,
) -> f64 {
    if ((tab_stops).is_some()) && ((tab_stops.as_ref().unwrap().len() as f64) > 0.0_f64) {
        for stop in (tab_stops)
            .as_ref()
            .expect("TypeScript nullable iterable was not narrowed")
            .iter()
            .cloned()
        {
            if (stop > current_x) {
                return (stop - current_x);
            }
        }
    }
    let space_w = (measure("    ".to_owned(), (*format).clone()) / 4.0_f64);
    let tab_w = ((space_w).max(1.0_f64) * 4.0_f64);
    return (tab_w - (current_x % tab_w));
}

// Source: upstream/packages/textlayout/src/textLayout.ts:166 (sha256:c3d8c03964f30c8e521a40add816ab8f7ed143b4731d6e95211160aeed283508)
#[derive(Clone, Default)]
struct BuildGroupsRecord1 {
    __flight_identity: std::sync::Arc<()>,
    positions: Vec<f64>,
    width: f64,
}
impl PartialEq for BuildGroupsRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

fn build_groups(
    out: &mut Vec<TextLayoutGroup>,
    paragraph_last_lines: &mut Vec<f64>,
    text: String,
    mut format_ranges: Vec<TextFormatRange>,
    line_breaks: &Vec<f64>,
    container_width: f64,
    measure: TextMeasureFunction,
    word_wrap: bool,
    multiline: bool,
    max_lines: f64,
    truncation_character: String,
) -> () {
    let __flight_utf16_text: Vec<u16> = text.encode_utf16().collect();
    out.clear();
    let range_index: std::sync::Arc<std::sync::Mutex<f64>> =
        std::sync::Arc::new(std::sync::Mutex::new(0.0_f64));
    let format_range: std::sync::Arc<std::sync::Mutex<TextFormatRange>> = std::sync::Arc::new(
        std::sync::Mutex::new(format_ranges[0.0_f64 as usize].clone()),
    );
    let current_format: std::sync::Arc<std::sync::Mutex<TextFormat>> = std::sync::Arc::new(
        std::sync::Mutex::new((((*format_range.lock().unwrap()).format).clone()).clone()),
    );
    let left_margin: std::sync::Arc<std::sync::Mutex<f64>> = std::sync::Arc::new(
        std::sync::Mutex::new(((*current_format.lock().unwrap()).left_margin).unwrap_or(0.0_f64)),
    );
    let right_margin: std::sync::Arc<std::sync::Mutex<f64>> = std::sync::Arc::new(
        std::sync::Mutex::new(((*current_format.lock().unwrap()).right_margin).unwrap_or(0.0_f64)),
    );
    let block_indent: std::sync::Arc<std::sync::Mutex<f64>> = std::sync::Arc::new(
        std::sync::Mutex::new(((*current_format.lock().unwrap()).block_indent).unwrap_or(0.0_f64)),
    );
    let indent: std::sync::Arc<std::sync::Mutex<f64>> = std::sync::Arc::new(std::sync::Mutex::new(
        ((*current_format.lock().unwrap()).indent).unwrap_or(0.0_f64),
    ));
    let first_line_of_paragraph: std::sync::Arc<std::sync::Mutex<bool>> =
        std::sync::Arc::new(std::sync::Mutex::new(true));
    let bullet_pending: std::sync::Arc<std::sync::Mutex<bool>> =
        std::sync::Arc::new(std::sync::Mutex::new(false));
    let bullet_char = "•";
    let ascent: std::sync::Arc<std::sync::Mutex<f64>> = std::sync::Arc::new(std::sync::Mutex::new(
        get_text_format_ascent(&(*current_format.lock().unwrap())),
    ));
    let descent: std::sync::Arc<std::sync::Mutex<f64>> = std::sync::Arc::new(
        std::sync::Mutex::new(get_text_format_descent(&(*current_format.lock().unwrap()))),
    );
    let leading: std::sync::Arc<std::sync::Mutex<f64>> = std::sync::Arc::new(
        std::sync::Mutex::new(get_text_format_leading(&(*current_format.lock().unwrap()))),
    );
    let mut line_height = (((*ascent.lock().unwrap()).clone()
        + (*descent.lock().unwrap()).clone())
        + (*leading.lock().unwrap()).clone())
    .ceil();
    let max_ascent: std::sync::Arc<std::sync::Mutex<f64>> =
        std::sync::Arc::new(std::sync::Mutex::new((*ascent.lock().unwrap()).clone()));
    let mut max_line_height = (*line_height.lock().unwrap());
    let text_index: std::sync::Arc<std::sync::Mutex<f64>> =
        std::sync::Arc::new(std::sync::Mutex::new(0.0_f64));
    let line_index: std::sync::Arc<std::sync::Mutex<f64>> =
        std::sync::Arc::new(std::sync::Mutex::new(0.0_f64));
    let offset_x: std::sync::Arc<std::sync::Mutex<f64>> =
        std::sync::Arc::new(std::sync::Mutex::new(0.0_f64));
    let offset_y: std::sync::Arc<std::sync::Mutex<f64>> =
        std::sync::Arc::new(std::sync::Mutex::new(0.0_f64));
    let truncated: std::sync::Arc<std::sync::Mutex<bool>> =
        std::sync::Arc::new(std::sync::Mutex::new(false));
    let mut break_count = 0.0_f64;
    let mut break_index = if ((line_breaks.len() as f64) > 0.0_f64) {
        line_breaks[0.0_f64 as usize].clone()
    } else {
        (-1.0_f64)
    };
    let mut space_index = (text.index_of)(" ");
    let active_group: std::sync::Arc<std::sync::Mutex<Option<TextLayoutGroup>>> =
        std::sync::Arc::new(std::sync::Mutex::new(None));
    let mut base_x: std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> f64 + Send + 'static>>> =
        std::sync::Arc::new(std::sync::Mutex::new(Box::new({
            let mut block_indent = block_indent.clone();
            let mut first_line_of_paragraph = first_line_of_paragraph.clone();
            let mut indent = indent.clone();
            let mut left_margin = left_margin.clone();
            move || -> f64 {
                return (((TEXT_LAYOUT_GUTTER + (*left_margin.lock().unwrap()).clone())
                    + (*block_indent.lock().unwrap()).clone())
                    + if (*first_line_of_paragraph.lock().unwrap()).clone() {
                        (*indent.lock().unwrap()).clone()
                    } else {
                        0.0_f64
                    });
            }
        })
            as Box<dyn FnMut() -> f64 + Send + 'static>));
    let mut wrap_width: std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> f64 + Send + 'static>>> =
        std::sync::Arc::new(std::sync::Mutex::new(Box::new({
            let base_x = base_x.clone();
            let mut right_margin = right_margin.clone();
            move || -> f64 {
                return (((container_width - TEXT_LAYOUT_GUTTER)
                    - (*right_margin.lock().unwrap()).clone())
                    - {
                        let __flight_callback = (base_x).clone();
                        let __flight_result = __flight_callback.lock().unwrap()();
                        __flight_result
                    });
            }
        })
            as Box<dyn FnMut() -> f64 + Send + 'static>));
    let mut update_line_metrics: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>,
    > = std::sync::Arc::new(std::sync::Mutex::new(Box::new({
        let mut ascent = ascent.clone();
        let mut current_format = current_format.clone();
        let mut descent = descent.clone();
        let mut leading = leading.clone();
        let mut max_ascent = max_ascent.clone();
        move || -> () {
            (*ascent.lock().unwrap()) = get_text_format_ascent(&(*current_format.lock().unwrap()));
            (*descent.lock().unwrap()) =
                get_text_format_descent(&(*current_format.lock().unwrap()));
            (*leading.lock().unwrap()) =
                get_text_format_leading(&(*current_format.lock().unwrap()));
            (*line_height.lock().unwrap()) = (((*ascent.lock().unwrap()).clone()
                + (*descent.lock().unwrap()).clone())
                + (*leading.lock().unwrap()).clone())
            .ceil();
            if ((*line_height.lock().unwrap()) > (*max_line_height.lock().unwrap())) {
                (*max_line_height.lock().unwrap()) = (*line_height.lock().unwrap());
            }
            if ((*ascent.lock().unwrap()).clone() > (*max_ascent.lock().unwrap()).clone()) {
                (*max_ascent.lock().unwrap()) = (*ascent.lock().unwrap()).clone();
            }
        }
    })
        as Box<dyn FnMut() -> () + Send + 'static>));
    let mut update_paragraph_metrics: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>,
    > = std::sync::Arc::new(std::sync::Mutex::new(Box::new({
        let mut block_indent = block_indent.clone();
        let mut bullet_pending = bullet_pending.clone();
        let mut current_format = current_format.clone();
        let mut first_line_of_paragraph = first_line_of_paragraph.clone();
        let mut indent = indent.clone();
        let mut left_margin = left_margin.clone();
        let mut right_margin = right_margin.clone();
        move || -> () {
            (*first_line_of_paragraph.lock().unwrap()) = true;
            (*left_margin.lock().unwrap()) =
                ((*current_format.lock().unwrap()).left_margin).unwrap_or(0.0_f64);
            (*right_margin.lock().unwrap()) =
                ((*current_format.lock().unwrap()).right_margin).unwrap_or(0.0_f64);
            (*block_indent.lock().unwrap()) =
                ((*current_format.lock().unwrap()).block_indent).unwrap_or(0.0_f64);
            (*indent.lock().unwrap()) =
                ((*current_format.lock().unwrap()).indent).unwrap_or(0.0_f64);
            (*bullet_pending.lock().unwrap()) =
                ((*current_format.lock().unwrap()).bullet) == Some(true);
        }
    })
        as Box<dyn FnMut() -> () + Send + 'static>));
    let mut advance_format_range: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut() -> bool + Send + 'static>>,
    > = std::sync::Arc::new(std::sync::Mutex::new(Box::new({
        let mut current_format = current_format.clone();
        let mut format_range = format_range.clone();
        let mut format_ranges = format_ranges.clone();
        let mut range_index = range_index.clone();
        move || -> bool {
            if ((*range_index.lock().unwrap()).clone() < ((format_ranges.len() as f64) - 1.0_f64)) {
                {
                    (*range_index.lock().unwrap()) += 1.0;
                    (*range_index.lock().unwrap())
                };
                (*format_range.lock().unwrap()) =
                    format_ranges[(*range_index.lock().unwrap()).clone() as usize].clone();
                (*current_format.lock().unwrap()) = merge_text_format(
                    &(*current_format.lock().unwrap()),
                    &(*format_range.lock().unwrap()).format,
                );
                return true;
            }
            return false;
        }
    })
        as Box<dyn FnMut() -> bool + Send + 'static>));
    let mut commit_line: std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>> =
        std::sync::Arc::new(std::sync::Mutex::new(Box::new({
            let mut active_group = active_group.clone();
            let mut first_line_of_paragraph = first_line_of_paragraph.clone();
            let mut groups = groups.clone();
            let mut line_index = line_index.clone();
            let mut max_ascent = max_ascent.clone();
            let mut offset_x = offset_x.clone();
            let mut offset_y = offset_y.clone();
            let update_line_metrics = update_line_metrics.clone();
            move || -> () {
                {
                    let mut i = (((*groups.lock().unwrap()).len() as f64) - 1.0_f64);
                    while (i >= 0.0_f64) {
                        let g: std::sync::Arc<std::sync::Mutex<TextLayoutGroup>> =
                            std::sync::Arc::new(std::sync::Mutex::new(
                                (*groups.lock().unwrap())[i as usize].clone(),
                            ));
                        if ((*g.lock().unwrap()).line_index < (*line_index.lock().unwrap()).clone())
                        {
                            break;
                        }
                        (*g.lock().unwrap()).ascent = (*max_ascent.lock().unwrap()).clone();
                        (*g.lock().unwrap()).height = (*max_line_height.lock().unwrap()).clone();
                        {
                            i -= 1.0;
                            i
                        };
                    }
                }
                (*offset_y.lock().unwrap()) += (*max_line_height.lock().unwrap()).clone();
                (*max_ascent.lock().unwrap()) = 0.0_f64;
                (*max_line_height.lock().unwrap()) = 0.0_f64;
                {
                    (*line_index.lock().unwrap()) += 1.0;
                    (*line_index.lock().unwrap())
                };
                (*offset_x.lock().unwrap()) = 0.0_f64;
                (*first_line_of_paragraph.lock().unwrap()) = false;
                (*active_group.lock().unwrap()) = None;
                {
                    let __flight_callback = (update_line_metrics).clone();
                    let __flight_result = __flight_callback.lock().unwrap()();
                    __flight_result
                };
            }
        })
            as Box<dyn FnMut() -> () + Send + 'static>));
    let mut check_truncation: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut() -> bool + Send + 'static>>,
    > = std::sync::Arc::new(std::sync::Mutex::new(Box::new({
        let mut groups = groups.clone();
        let mut line_index = line_index.clone();
        let measure = measure.clone();
        let mut right_margin = right_margin.clone();
        let mut truncated = truncated.clone();
        move || -> bool {
            if (max_lines < 0.0_f64) || ((*line_index.lock().unwrap()).clone() < max_lines) {
                return false;
            }
            let last_line_index = ((*line_index.lock().unwrap()).clone() - 1.0_f64);
            if ((truncation_character.encode_utf16().count() as f64) > 0.0_f64)
                && (((*groups.lock().unwrap()).len() as f64) > 0.0_f64)
            {
                let mut last_group: Option<TextLayoutGroup> = None;
                {
                    let mut i = (((*groups.lock().unwrap()).len() as f64) - 1.0_f64);
                    while (i >= 0.0_f64) {
                        if ((*groups.lock().unwrap())[i as usize].line_index == last_line_index) {
                            last_group = Some((*groups.lock().unwrap())[i as usize].clone());
                            break;
                        }
                        {
                            i -= 1.0;
                            i
                        };
                    }
                }
                if (last_group).is_some() {
                    let ellipsis_w = {
                        let __flight_callback = (measure).clone();
                        let __flight_result = __flight_callback.lock().unwrap()(
                            (truncation_character).clone(),
                            (last_group.as_mut().unwrap().format).clone(),
                        );
                        __flight_result
                    };
                    let available = (((container_width - TEXT_LAYOUT_GUTTER)
                        - (*right_margin.lock().unwrap()).clone())
                        - last_group.as_mut().unwrap().offset_x);
                    while ((last_group.as_mut().unwrap().positions.len() as f64) > 0.0_f64) {
                        let used_w = sum_advances(&last_group.as_mut().unwrap().positions);
                        if ((used_w + ellipsis_w) <= available) {
                            break;
                        }
                        let trimmed = (last_group
                            .as_mut()
                            .unwrap()
                            .positions
                            .pop()
                            .expect("TypeScript Array.pop returned undefined"))
                        .unwrap_or(0.0_f64);
                        last_group.as_mut().unwrap().width -= trimmed;
                        {
                            last_group.as_mut().unwrap().end_index -= 1.0;
                            last_group.as_mut().unwrap().end_index
                        };
                        if (last_group.as_mut().unwrap().end_index
                            <= last_group.as_mut().unwrap().start_index)
                        {
                            break;
                        }
                    }
                    let mut ellipsis_group = create_text_layout_group(
                        &last_group.as_mut().unwrap().format,
                        last_group.as_mut().unwrap().end_index,
                        last_group.as_mut().unwrap().end_index,
                    );
                    let ellipsis_offset_x = (last_group.as_mut().unwrap().offset_x
                        + last_group.as_mut().unwrap().width);
                    ellipsis_group.positions = vec![ellipsis_w];
                    ellipsis_group.width = ellipsis_w;
                    ellipsis_group.offset_x = ellipsis_offset_x;
                    ellipsis_group.ascent = last_group.as_mut().unwrap().ascent;
                    ellipsis_group.descent = last_group.as_mut().unwrap().descent;
                    ellipsis_group.leading = last_group.as_mut().unwrap().leading;
                    ellipsis_group.line_index = last_line_index;
                    ellipsis_group.offset_y = last_group.as_mut().unwrap().offset_y;
                    ellipsis_group.height = last_group.as_mut().unwrap().height;
                    (*groups.lock().unwrap()).push(((ellipsis_group).clone()).clone());
                }
            }
            (*truncated.lock().unwrap()) = true;
            return true;
        }
    })
        as Box<dyn FnMut() -> bool + Send + 'static>));
    let mut emit_bullet: std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>> =
        std::sync::Arc::new(std::sync::Mutex::new(Box::new({
            let mut ascent = ascent.clone();
            let mut block_indent = block_indent.clone();
            let mut bullet_pending = bullet_pending.clone();
            let mut current_format = current_format.clone();
            let mut descent = descent.clone();
            let mut groups = groups.clone();
            let mut indent = indent.clone();
            let mut leading = leading.clone();
            let mut left_margin = left_margin.clone();
            let mut line_index = line_index.clone();
            let measure = measure.clone();
            let mut offset_y = offset_y.clone();
            let mut text_index = text_index.clone();
            move || -> () {
                if (!(*bullet_pending.lock().unwrap()).clone()) {
                    return;
                }
                (*bullet_pending.lock().unwrap()) = false;
                if (((*current_format.lock().unwrap()).list_marker).clone() == "none") {
                    if ((*indent.lock().unwrap()).clone() <= 0.0_f64) {
                        (*indent.lock().unwrap()) = (({
                            let __flight_callback = (measure).clone();
                            let __flight_result = __flight_callback.lock().unwrap()(
                                (bullet_char).clone(),
                                (*current_format.lock().unwrap()).clone(),
                            );
                            __flight_result
                        })
                        .ceil()
                            + 2.0_f64);
                    }
                    return;
                }
                let bullet_w = {
                    let __flight_callback = (measure).clone();
                    let __flight_result = __flight_callback.lock().unwrap()(
                        (bullet_char).clone(),
                        (*current_format.lock().unwrap()).clone(),
                    );
                    __flight_result
                };
                let mut bullet_group = create_text_layout_group(
                    &(*current_format.lock().unwrap()),
                    (*text_index.lock().unwrap()).clone(),
                    (*text_index.lock().unwrap()).clone(),
                );
                bullet_group.positions = vec![bullet_w];
                bullet_group.width = bullet_w;
                bullet_group.offset_x = ((TEXT_LAYOUT_GUTTER
                    + (*left_margin.lock().unwrap()).clone())
                    + (*block_indent.lock().unwrap()).clone());
                bullet_group.ascent = (*ascent.lock().unwrap()).clone();
                bullet_group.descent = (*descent.lock().unwrap()).clone();
                bullet_group.leading = (*leading.lock().unwrap()).clone();
                bullet_group.line_index = (*line_index.lock().unwrap()).clone();
                bullet_group.offset_y = ((*offset_y.lock().unwrap()).clone() + TEXT_LAYOUT_GUTTER);
                bullet_group.height = (*line_height.lock().unwrap());
                (*groups.lock().unwrap()).push(((bullet_group).clone()).clone());
                if ((*indent.lock().unwrap()).clone() <= 0.0_f64) {
                    (*indent.lock().unwrap()) = ((bullet_w).ceil() + 2.0_f64);
                }
            }
        })
            as Box<dyn FnMut() -> () + Send + 'static>));
    let mut place_span: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut(f64, f64) -> () + Send + 'static>>,
    > = std::sync::Arc::new(std::sync::Mutex::new(Box::new({
        let mut active_group = active_group.clone();
        let advance_format_range = advance_format_range.clone();
        let mut ascent = ascent.clone();
        let base_x = base_x.clone();
        let mut current_format = current_format.clone();
        let mut descent = descent.clone();
        let mut format_range = format_range.clone();
        let mut format_ranges = format_ranges.clone();
        let mut groups = groups.clone();
        let mut leading = leading.clone();
        let mut line_index = line_index.clone();
        let measure = measure.clone();
        let mut offset_x = offset_x.clone();
        let mut offset_y = offset_y.clone();
        let mut range_index = range_index.clone();
        let mut text_index = text_index.clone();
        let update_line_metrics = update_line_metrics.clone();
        move |start: f64, end: f64| -> () {
            let mut idx = start;
            while (idx < end) {
                let range_end = (end).min((*format_range.lock().unwrap()).end);
                if (idx < range_end) {
                    if (((*active_group.lock().unwrap()).clone()).is_none())
                        || ((*active_group.lock().unwrap())
                            .as_mut()
                            .unwrap()
                            .start_index
                            != (*active_group.lock().unwrap()).as_mut().unwrap().end_index)
                    {
                        (*active_group.lock().unwrap()) = Some(create_text_layout_group(
                            &(*format_range.lock().unwrap()).format,
                            idx,
                            range_end,
                        ));
                        (*groups.lock().unwrap()).push(
                            (((*active_group.lock().unwrap()).clone()).clone().unwrap()).clone(),
                        );
                    } else {
                        (*active_group.lock().unwrap()).as_mut().unwrap().format =
                            ((*format_range.lock().unwrap()).format).clone();
                        (*active_group.lock().unwrap())
                            .as_mut()
                            .unwrap()
                            .start_index = idx;
                        (*active_group.lock().unwrap()).as_mut().unwrap().end_index = range_end;
                    }
                    char_advances(
                        &mut (*active_group.lock().unwrap()).as_mut().unwrap().positions,
                        (text).clone(),
                        &(*current_format.lock().unwrap()),
                        idx,
                        range_end,
                        &mut |__flight_callback_argument_0: String,
                              __flight_callback_argument_1: TextFormat|
                         -> f64 {
                            let __flight_callback = (measure).clone();
                            __flight_callback.lock().unwrap()(
                                __flight_callback_argument_0,
                                __flight_callback_argument_1,
                            )
                        },
                        Some(
                            ((*offset_x.lock().unwrap()).clone() + {
                                let __flight_callback = (base_x).clone();
                                let __flight_result = __flight_callback.lock().unwrap()();
                                __flight_result
                            }),
                        ),
                    );
                    let span_width =
                        sum_advances(&(*active_group.lock().unwrap()).as_mut().unwrap().positions);
                    (*active_group.lock().unwrap()).as_mut().unwrap().offset_x =
                        ((*offset_x.lock().unwrap()).clone() + {
                            let __flight_callback = (base_x).clone();
                            let __flight_result = __flight_callback.lock().unwrap()();
                            __flight_result
                        });
                    (*active_group.lock().unwrap()).as_mut().unwrap().ascent =
                        (*ascent.lock().unwrap()).clone();
                    (*active_group.lock().unwrap()).as_mut().unwrap().descent =
                        (*descent.lock().unwrap()).clone();
                    (*active_group.lock().unwrap()).as_mut().unwrap().leading =
                        (*leading.lock().unwrap()).clone();
                    (*active_group.lock().unwrap()).as_mut().unwrap().line_index =
                        (*line_index.lock().unwrap()).clone();
                    (*active_group.lock().unwrap()).as_mut().unwrap().offset_y =
                        ((*offset_y.lock().unwrap()).clone() + TEXT_LAYOUT_GUTTER);
                    (*active_group.lock().unwrap()).as_mut().unwrap().width = span_width;
                    (*active_group.lock().unwrap()).as_mut().unwrap().height =
                        (*line_height.lock().unwrap());
                    (*offset_x.lock().unwrap()) += span_width;
                    idx = range_end;
                }
                if (idx >= end) {
                    break;
                }
                if (!{
                    let __flight_callback = (advance_format_range).clone();
                    let __flight_result = __flight_callback.lock().unwrap()();
                    __flight_result
                }) {
                    break;
                }
                {
                    let __flight_callback = (update_line_metrics).clone();
                    let __flight_result = __flight_callback.lock().unwrap()();
                    __flight_result
                };
            }
            (*text_index.lock().unwrap()) = end;
            while ((*text_index.lock().unwrap()).clone() >= (*format_range.lock().unwrap()).end)
                && ((*range_index.lock().unwrap()).clone()
                    < ((format_ranges.len() as f64) - 1.0_f64))
            {
                {
                    let __flight_callback = (advance_format_range).clone();
                    let __flight_result = __flight_callback.lock().unwrap()();
                    __flight_result
                };
                {
                    let __flight_callback = (update_line_metrics).clone();
                    let __flight_result = __flight_callback.lock().unwrap()();
                    __flight_result
                };
            }
        }
    })
        as Box<dyn FnMut(f64, f64) -> () + Send + 'static>));
    let mut measure_span: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut(f64, f64) -> BuildGroupsRecord1 + Send + 'static>>,
    > = std::sync::Arc::new(std::sync::Mutex::new(Box::new({
        let advance_format_range = advance_format_range.clone();
        let base_x = base_x.clone();
        let mut current_format = current_format.clone();
        let mut format_range = format_range.clone();
        let measure = measure.clone();
        let mut offset_x = offset_x.clone();
        let mut range_index = range_index.clone();
        move |start: f64, end: f64| -> BuildGroupsRecord1 {
            if (start >= end) {
                return BuildGroupsRecord1 {
                    __flight_identity: std::sync::Arc::new(()),
                    positions: vec![],
                    width: 0.0_f64,
                };
            }
            let saved_range_index = (*range_index.lock().unwrap()).clone();
            let saved_format = ((*current_format.lock().unwrap()).clone()).clone();
            let mut idx = start;
            let mut all_positions: Vec<f64> = vec![];
            while (idx < end) {
                let range_end = (end).min((*format_range.lock().unwrap()).end);
                if (idx < range_end) {
                    char_advances(
                        &mut _CHAR_ADVANCES,
                        (text).clone(),
                        &(*current_format.lock().unwrap()),
                        idx,
                        range_end,
                        &mut |__flight_callback_argument_0: String,
                              __flight_callback_argument_1: TextFormat|
                         -> f64 {
                            let __flight_callback = (measure).clone();
                            __flight_callback.lock().unwrap()(
                                __flight_callback_argument_0,
                                __flight_callback_argument_1,
                            )
                        },
                        Some(
                            (((*offset_x.lock().unwrap()).clone() + {
                                let __flight_callback = (base_x).clone();
                                let __flight_result = __flight_callback.lock().unwrap()();
                                __flight_result
                            }) + sum_advances(&all_positions)),
                        ),
                    );
                    for p in ((_CHAR_ADVANCES).clone()).iter().cloned() {
                        all_positions.push(p);
                    }
                    idx = range_end;
                }
                if (idx >= end) {
                    break;
                }
                if (!{
                    let __flight_callback = (advance_format_range).clone();
                    let __flight_result = __flight_callback.lock().unwrap()();
                    __flight_result
                }) {
                    break;
                }
            }
            (*range_index.lock().unwrap()) = saved_range_index;
            (*format_range.lock().unwrap()) = (*format_range.lock().unwrap()).clone();
            (*current_format.lock().unwrap()) = (saved_format).clone();
            return BuildGroupsRecord1 {
                __flight_identity: std::sync::Arc::new(()),
                positions: (all_positions).clone(),
                width: sum_advances(&all_positions),
            };
        }
    })
        as Box<dyn FnMut(f64, f64) -> BuildGroupsRecord1 + Send + 'static>));
    let mut break_long_word: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut(f64) -> () + Send + 'static>>,
    > = std::sync::Arc::new(std::sync::Mutex::new(Box::new({
        let base_x = base_x.clone();
        let check_truncation = check_truncation.clone();
        let commit_line = commit_line.clone();
        let mut current_format = current_format.clone();
        let measure = measure.clone();
        let mut offset_x = offset_x.clone();
        let place_span = place_span.clone();
        let mut text_index = text_index.clone();
        let mut truncated = truncated.clone();
        let wrap_width = wrap_width.clone();
        move |end: f64| -> () {
            let mut remaining = (*text_index.lock().unwrap()).clone();
            while (remaining < end) {
                if (*truncated.lock().unwrap()).clone() {
                    return;
                }
                char_advances(
                    &mut _CHAR_ADVANCES,
                    (text).clone(),
                    &(*current_format.lock().unwrap()),
                    remaining,
                    end,
                    &mut |__flight_callback_argument_0: String,
                          __flight_callback_argument_1: TextFormat|
                     -> f64 {
                        let __flight_callback = (measure).clone();
                        __flight_callback.lock().unwrap()(
                            __flight_callback_argument_0,
                            __flight_callback_argument_1,
                        )
                    },
                    Some(
                        ((*offset_x.lock().unwrap()).clone() + {
                            let __flight_callback = (base_x).clone();
                            let __flight_result = __flight_callback.lock().unwrap()();
                            __flight_result
                        }),
                    ),
                );
                let total_w = sum_advances(&_CHAR_ADVANCES);
                if (((*offset_x.lock().unwrap()).clone() + total_w) <= {
                    let __flight_callback = (wrap_width).clone();
                    let __flight_result = __flight_callback.lock().unwrap()();
                    __flight_result
                }) {
                    {
                        let __flight_callback = (place_span).clone();
                        let __flight_result = __flight_callback.lock().unwrap()(remaining, end);
                        __flight_result
                    };
                    return;
                }
                let mut count = 0.0_f64;
                let mut char_count = 0.0_f64;
                let mut w = 0.0_f64;
                let mut i = remaining;
                while (i < end) && (count < (_CHAR_ADVANCES.lock().unwrap().len() as f64)) {
                    let cp = {
                        let __flight_units: &[u16] = &__flight_utf16_text;
                        let __flight_raw_index = i;
                        let __flight_index = if __flight_raw_index.is_nan() {
                            0_i64
                        } else if __flight_raw_index.is_finite() {
                            __flight_raw_index.trunc() as i64
                        } else {
                            -1_i64
                        };
                        if __flight_index < 0 {
                            f64::NAN
                        } else if let Some(&__flight_first) =
                            __flight_units.get(__flight_index as usize)
                        {
                            let __flight_first = u32::from(__flight_first);
                            if (0xD800_u32..=0xDBFF_u32).contains(&__flight_first) {
                                if let Some(&__flight_second) =
                                    __flight_units.get(__flight_index as usize + 1)
                                {
                                    let __flight_second = u32::from(__flight_second);
                                    if (0xDC00_u32..=0xDFFF_u32).contains(&__flight_second) {
                                        (((__flight_first - 0xD800_u32) << 10)
                                            + (__flight_second - 0xDC00_u32)
                                            + 0x10000_u32)
                                            as f64
                                    } else {
                                        __flight_first as f64
                                    }
                                } else {
                                    __flight_first as f64
                                }
                            } else {
                                __flight_first as f64
                            }
                        } else {
                            f64::NAN
                        }
                    };
                    let cp_len = if (cp > 65535.0_f64) { 2.0_f64 } else { 1.0_f64 };
                    if ((((*offset_x.lock().unwrap()).clone() + w)
                        + _CHAR_ADVANCES[count as usize].clone())
                        > {
                            let __flight_callback = (wrap_width).clone();
                            let __flight_result = __flight_callback.lock().unwrap()();
                            __flight_result
                        })
                    {
                        break;
                    }
                    w += _CHAR_ADVANCES[count as usize].clone();
                    {
                        count += 1.0;
                        count
                    };
                    char_count += cp_len;
                    i += cp_len;
                }
                if (char_count == 0.0_f64) {
                    let cp = {
                        let __flight_units: &[u16] = &__flight_utf16_text;
                        let __flight_raw_index = remaining;
                        let __flight_index = if __flight_raw_index.is_nan() {
                            0_i64
                        } else if __flight_raw_index.is_finite() {
                            __flight_raw_index.trunc() as i64
                        } else {
                            -1_i64
                        };
                        if __flight_index < 0 {
                            f64::NAN
                        } else if let Some(&__flight_first) =
                            __flight_units.get(__flight_index as usize)
                        {
                            let __flight_first = u32::from(__flight_first);
                            if (0xD800_u32..=0xDBFF_u32).contains(&__flight_first) {
                                if let Some(&__flight_second) =
                                    __flight_units.get(__flight_index as usize + 1)
                                {
                                    let __flight_second = u32::from(__flight_second);
                                    if (0xDC00_u32..=0xDFFF_u32).contains(&__flight_second) {
                                        (((__flight_first - 0xD800_u32) << 10)
                                            + (__flight_second - 0xDC00_u32)
                                            + 0x10000_u32)
                                            as f64
                                    } else {
                                        __flight_first as f64
                                    }
                                } else {
                                    __flight_first as f64
                                }
                            } else {
                                __flight_first as f64
                            }
                        } else {
                            f64::NAN
                        }
                    };
                    char_count = if (cp > 65535.0_f64) { 2.0_f64 } else { 1.0_f64 };
                }
                {
                    let __flight_callback = (place_span).clone();
                    let __flight_result =
                        __flight_callback.lock().unwrap()(remaining, (remaining + char_count));
                    __flight_result
                };
                {
                    let __flight_callback = (commit_line).clone();
                    let __flight_result = __flight_callback.lock().unwrap()();
                    __flight_result
                };
                if {
                    let __flight_callback = (check_truncation).clone();
                    let __flight_result = __flight_callback.lock().unwrap()();
                    __flight_result
                } {
                    return;
                }
                remaining += char_count;
            }
        }
    })
        as Box<dyn FnMut(f64) -> () + Send + 'static>));
    {
        let __flight_callback = (update_line_metrics).clone();
        let __flight_result = __flight_callback.lock().unwrap()();
        __flight_result
    };
    {
        let __flight_callback = (update_paragraph_metrics).clone();
        let __flight_result = __flight_callback.lock().unwrap()();
        __flight_result
    };
    while ((*text_index.lock().unwrap()).clone() <= (__flight_utf16_text.len() as f64)) {
        if (*truncated.lock().unwrap()).clone() {
            break;
        }
        {
            let __flight_callback = (emit_bullet).clone();
            let __flight_result = __flight_callback.lock().unwrap()();
            __flight_result
        };
        let has_break = (break_index != (-1.0_f64));
        let break_before_space =
            (has_break) && ((space_index == (-1.0_f64)) || (break_index <= space_index));
        if break_before_space {
            if ((*text_index.lock().unwrap()).clone() <= break_index) {
                {
                    let __flight_callback = (place_span).clone();
                    let __flight_result = __flight_callback.lock().unwrap()(
                        (*text_index.lock().unwrap()).clone(),
                        break_index,
                    );
                    __flight_result
                };
                (*active_group.lock().unwrap()) = None;
            }
            {
                let __flight_callback = (commit_line).clone();
                let __flight_result = __flight_callback.lock().unwrap()();
                __flight_result
            };
            {
                let __flight_value = ((*line_index.lock().unwrap()).clone() - 1.0_f64);
                if !paragraph_last_lines.contains(&__flight_value) {
                    paragraph_last_lines.push(__flight_value);
                }
            };
            if {
                let __flight_callback = (check_truncation).clone();
                let __flight_result = __flight_callback.lock().unwrap()();
                __flight_result
            } {
                break;
            }
            if (!multiline) {
                break;
            }
            (*text_index.lock().unwrap()) = (break_index + 1.0_f64);
            {
                break_count += 1.0;
                break_count
            };
            break_index = if (break_count < (line_breaks.len() as f64)) {
                line_breaks[break_count as usize].clone()
            } else {
                (-1.0_f64)
            };
            space_index = (text.index_of)(" ", (*text_index.lock().unwrap()).clone());
            {
                let __flight_callback = (update_paragraph_metrics).clone();
                let __flight_result = __flight_callback.lock().unwrap()();
                __flight_result
            };
            {
                let __flight_callback = (update_line_metrics).clone();
                let __flight_result = __flight_callback.lock().unwrap()();
                __flight_result
            };
        } else {
            if (space_index != (-1.0_f64)) {
                let word_end = (space_index + 1.0_f64);
                let seg_end = if (has_break) && (break_index < word_end) {
                    break_index
                } else {
                    word_end
                };
                let __destructure1 = {
                    let __flight_callback = (measure_span).clone();
                    let __flight_result = __flight_callback.lock().unwrap()(
                        (*text_index.lock().unwrap()).clone(),
                        seg_end,
                    );
                    __flight_result
                };
                let seg_width = __destructure1.width;
                let mut should_wrap = ((word_wrap)
                    && (container_width >= (TEXT_LAYOUT_GUTTER * 2.0_f64)))
                    && (((*offset_x.lock().unwrap()).clone() + seg_width) > {
                        let __flight_callback = (wrap_width).clone();
                        let __flight_result = __flight_callback.lock().unwrap()();
                        __flight_result
                    });
                if ((should_wrap) && (seg_end == word_end))
                    && ((__destructure1.positions.len() as f64) > 0.0_f64)
                {
                    let trailing_space = __destructure1.positions
                        [((__destructure1.positions.len() as f64) - 1.0_f64) as usize]
                        .clone();
                    if ((((*offset_x.lock().unwrap()).clone() + seg_width) - trailing_space) <= {
                        let __flight_callback = (wrap_width).clone();
                        let __flight_result = __flight_callback.lock().unwrap()();
                        __flight_result
                    }) {
                        should_wrap = false;
                    }
                }
                if should_wrap {
                    let mut trim_target = ((*active_group.lock().unwrap()).clone()).unwrap_or(
                        if ((out.len() as f64) > 0.0_f64) {
                            out[((out.len() as f64) - 1.0_f64) as usize].clone()
                        } else {
                            None
                        },
                    );
                    if ((true) && ((trim_target.positions.len() as f64) > 0.0_f64))
                        && (trim_target.line_index == (*line_index.lock().unwrap()).clone())
                    {
                        let trailing_w = trim_target.positions
                            [((trim_target.positions.len() as f64) - 1.0_f64) as usize]
                            .clone();
                        trim_target.width -= trailing_w;
                        {
                            trim_target.end_index -= 1.0;
                            trim_target.end_index
                        };
                    }
                    {
                        let __flight_callback = (commit_line).clone();
                        let __flight_result = __flight_callback.lock().unwrap()();
                        __flight_result
                    };
                    if {
                        let __flight_callback = (check_truncation).clone();
                        let __flight_result = __flight_callback.lock().unwrap()();
                        __flight_result
                    } {
                        break;
                    }
                    if ((text.char_code_at)((*text_index.lock().unwrap()).clone()) == 32.0_f64) {
                        {
                            (*text_index.lock().unwrap()) += 1.0;
                            (*text_index.lock().unwrap())
                        };
                    }
                }
                {
                    let __flight_callback = (place_span).clone();
                    let __flight_result = __flight_callback.lock().unwrap()(
                        (*text_index.lock().unwrap()).clone(),
                        seg_end,
                    );
                    __flight_result
                };
                space_index = (text.index_of)(" ", word_end);
            } else {
                if ((*text_index.lock().unwrap()).clone() >= (__flight_utf16_text.len() as f64)) {
                    break;
                }
                if (word_wrap) && (container_width >= (TEXT_LAYOUT_GUTTER * 2.0_f64)) {
                    {
                        let __flight_callback = (break_long_word).clone();
                        let __flight_result =
                            __flight_callback.lock().unwrap()((__flight_utf16_text.len() as f64));
                        __flight_result
                    };
                } else {
                    {
                        let __flight_callback = (place_span).clone();
                        let __flight_result = __flight_callback.lock().unwrap()(
                            (*text_index.lock().unwrap()).clone(),
                            (__flight_utf16_text.len() as f64),
                        );
                        __flight_result
                    };
                }
                break;
            }
        }
    }
    {
        let mut i = ((out.len() as f64) - 1.0_f64);
        while (i >= 0.0_f64) {
            let g: std::sync::Arc<std::sync::Mutex<TextLayoutGroup>> =
                std::sync::Arc::new(std::sync::Mutex::new(out[i as usize].clone()));
            if ((*g.lock().unwrap()).line_index < (*line_index.lock().unwrap()).clone()) {
                break;
            }
            (*g.lock().unwrap()).ascent = if ((*max_ascent.lock().unwrap()).clone()) != 0.0_f64 {
                (*max_ascent.lock().unwrap()).clone()
            } else {
                (*g.lock().unwrap()).ascent
            };
            (*g.lock().unwrap()).height = if (*max_line_height.lock().unwrap()) != 0.0_f64 {
                (*max_line_height.lock().unwrap())
            } else {
                (*g.lock().unwrap()).height
            };
            {
                i -= 1.0;
                i
            };
        }
    }
    {
        let __flight_value = (*line_index.lock().unwrap()).clone();
        if !paragraph_last_lines.contains(&__flight_value) {
            paragraph_last_lines.push(__flight_value);
        }
    };
}

// Source: upstream/packages/textlayout/src/textLayout.ts:588 (sha256:27f547a87228a401d0c4bc5c5025318a4f77a84b3d5aab9cbca5f4af1df33577)
fn apply_alignment(
    groups: &Vec<TextLayoutGroup>,
    container_width: f64,
    line_widths: &Vec<f64>,
    direction: TextDirection,
    justification: TextJustification,
    paragraph_last_lines: &Vec<f64>,
    text: String,
) -> () {
    for g in (groups).iter().cloned() {
        let line_w = line_widths[g.line_index as usize].clone();
        let align = ((g.format.align).clone()).unwrap_or("left".to_owned());
        let mut shift = 0.0_f64;
        let resolved = if ((align).clone() == "start") {
            if (direction == "RightToLeft") {
                "right".to_owned()
            } else {
                "left".to_owned()
            }
        } else {
            if ((align).clone() == "end") {
                if (direction == "RightToLeft") {
                    "left".to_owned()
                } else {
                    "right".to_owned()
                }
            } else {
                (align).clone()
            }
        };
        if (resolved == "right") {
            shift = ((container_width - line_w) - (TEXT_LAYOUT_GUTTER * 2.0_f64));
        } else {
            if (resolved == "center") {
                shift = (((container_width - line_w) - (TEXT_LAYOUT_GUTTER * 2.0_f64)) / 2.0_f64);
            } else {
                if (resolved == "justify") && (justification != "none") {}
            }
        }
        if (shift != 0.0_f64) {
            g.offset_x += shift;
        }
    }
    justify_lines(
        groups,
        container_width,
        line_widths,
        (justification).clone(),
        paragraph_last_lines,
        (text).clone(),
    );
}

// Source: upstream/packages/textlayout/src/textLayout.ts:638 (sha256:5f5d390dd91c859cedcdf98d95f686e35ac2230c3976cb95620dc0324aef9c29)
fn apply_vertical_alignment(
    groups: &Vec<TextLayoutGroup>,
    container_height: f64,
    content_height: f64,
    vertical_align: TextVerticalAlign,
) -> () {
    if (vertical_align == "top") {
        return;
    }
    let slack = (container_height - (content_height + (TEXT_LAYOUT_GUTTER * 2.0_f64)));
    if (slack <= 0.0_f64) {
        return;
    }
    let shift = if (vertical_align == "middle") {
        (slack / 2.0_f64)
    } else {
        slack
    };
    for g in (groups).iter().cloned() {
        g.offset_y += shift;
    }
}

// Source: upstream/packages/textlayout/src/textLayout.ts:651 (sha256:2d6a21da78d4cf05054b043cb2424b51c38e532b49aecb73969723a63d4b2d58)
fn justify_lines(
    groups: &Vec<TextLayoutGroup>,
    container_width: f64,
    line_widths: &Vec<f64>,
    justification: TextJustification,
    paragraph_last_lines: &Vec<f64>,
    text: String,
) -> () {
    let __flight_utf16_text: Vec<u16> = text.encode_utf16().collect();
    if (justification == "none") {
        return;
    }
    let line_count = (line_widths.len() as f64);
    {
        let mut li = 0.0_f64;
        while (li < line_count) {
            if paragraph_last_lines.iter().any(|item| item == &li) {
                {
                    li += 1.0;
                    li
                };
                continue;
            }
            let mut line_groups: Vec<TextLayoutGroup> = vec![];
            for g in (groups).iter().cloned() {
                if (g.line_index == li) && ((g.format.align).clone() == "justify") {
                    line_groups.push(((g).clone()).clone());
                }
            }
            if ((line_groups.len() as f64) == 0.0_f64) {
                {
                    li += 1.0;
                    li
                };
                continue;
            }
            let line_w = line_widths[li as usize].clone();
            let available = (container_width - (TEXT_LAYOUT_GUTTER * 2.0_f64));
            let residual = (available - line_w);
            if (residual <= 0.0_f64) {
                {
                    li += 1.0;
                    li
                };
                continue;
            }
            if (justification == "interCharacter") {
                let mut char_count = 0.0_f64;
                for g in (line_groups).iter().cloned() {
                    char_count += (g.positions.len() as f64);
                }
                let gap_count = (0.0_f64).max((char_count - 1.0_f64));
                if (gap_count == 0.0_f64) {
                    {
                        li += 1.0;
                        li
                    };
                    continue;
                }
                let extra_per_gap = (residual / gap_count);
                let mut accumulated = 0.0_f64;
                let last_group =
                    line_groups[((line_groups.len() as f64) - 1.0_f64) as usize].clone();
                for g in (line_groups).iter().cloned() {
                    g.offset_x += accumulated;
                    let mut group_extra = 0.0_f64;
                    let last_pos = if ((g).clone() == last_group) {
                        ((g.positions.len() as f64) - 1.0_f64)
                    } else {
                        (g.positions.len() as f64)
                    };
                    {
                        let mut ci = 0.0_f64;
                        while (ci < last_pos) {
                            g.positions[ci as usize] += extra_per_gap;
                            accumulated += extra_per_gap;
                            group_extra += extra_per_gap;
                            {
                                ci += 1.0;
                                ci
                            };
                        }
                    }
                    g.width += group_extra;
                }
            } else {
                let mut space_count = 0.0_f64;
                for g in (line_groups).iter().cloned() {
                    let mut text_index = g.start_index;
                    {
                        let mut ci = 0.0_f64;
                        while (ci < (g.positions.len() as f64)) && (text_index < g.end_index) {
                            let codepoint = {
                                let __flight_units: &[u16] = &__flight_utf16_text;
                                let __flight_raw_index = text_index;
                                let __flight_index = if __flight_raw_index.is_nan() {
                                    0_i64
                                } else if __flight_raw_index.is_finite() {
                                    __flight_raw_index.trunc() as i64
                                } else {
                                    -1_i64
                                };
                                if __flight_index < 0 {
                                    f64::NAN
                                } else if let Some(&__flight_first) =
                                    __flight_units.get(__flight_index as usize)
                                {
                                    let __flight_first = u32::from(__flight_first);
                                    if (0xD800_u32..=0xDBFF_u32).contains(&__flight_first) {
                                        if let Some(&__flight_second) =
                                            __flight_units.get(__flight_index as usize + 1)
                                        {
                                            let __flight_second = u32::from(__flight_second);
                                            if (0xDC00_u32..=0xDFFF_u32).contains(&__flight_second)
                                            {
                                                (((__flight_first - 0xD800_u32) << 10)
                                                    + (__flight_second - 0xDC00_u32)
                                                    + 0x10000_u32)
                                                    as f64
                                            } else {
                                                __flight_first as f64
                                            }
                                        } else {
                                            __flight_first as f64
                                        }
                                    } else {
                                        __flight_first as f64
                                    }
                                } else {
                                    f64::NAN
                                }
                            };
                            if (codepoint == 32.0_f64) {
                                {
                                    space_count += 1.0;
                                    space_count
                                };
                            }
                            text_index += if (codepoint > 65535.0_f64) {
                                2.0_f64
                            } else {
                                1.0_f64
                            };
                            {
                                ci += 1.0;
                                ci
                            };
                        }
                    }
                }
                if (space_count == 0.0_f64) {
                    {
                        li += 1.0;
                        li
                    };
                    continue;
                }
                let extra_per_space = (residual / space_count);
                let mut accumulated = 0.0_f64;
                for g in (line_groups).iter().cloned() {
                    g.offset_x += accumulated;
                    let mut group_extra = 0.0_f64;
                    let mut text_index = g.start_index;
                    {
                        let mut ci = 0.0_f64;
                        while (ci < (g.positions.len() as f64)) && (text_index < g.end_index) {
                            let codepoint = {
                                let __flight_units: &[u16] = &__flight_utf16_text;
                                let __flight_raw_index = text_index;
                                let __flight_index = if __flight_raw_index.is_nan() {
                                    0_i64
                                } else if __flight_raw_index.is_finite() {
                                    __flight_raw_index.trunc() as i64
                                } else {
                                    -1_i64
                                };
                                if __flight_index < 0 {
                                    f64::NAN
                                } else if let Some(&__flight_first) =
                                    __flight_units.get(__flight_index as usize)
                                {
                                    let __flight_first = u32::from(__flight_first);
                                    if (0xD800_u32..=0xDBFF_u32).contains(&__flight_first) {
                                        if let Some(&__flight_second) =
                                            __flight_units.get(__flight_index as usize + 1)
                                        {
                                            let __flight_second = u32::from(__flight_second);
                                            if (0xDC00_u32..=0xDFFF_u32).contains(&__flight_second)
                                            {
                                                (((__flight_first - 0xD800_u32) << 10)
                                                    + (__flight_second - 0xDC00_u32)
                                                    + 0x10000_u32)
                                                    as f64
                                            } else {
                                                __flight_first as f64
                                            }
                                        } else {
                                            __flight_first as f64
                                        }
                                    } else {
                                        __flight_first as f64
                                    }
                                } else {
                                    f64::NAN
                                }
                            };
                            if (codepoint == 32.0_f64) {
                                g.positions[ci as usize] += extra_per_space;
                                accumulated += extra_per_space;
                                group_extra += extra_per_space;
                            }
                            text_index += if (codepoint > 65535.0_f64) {
                                2.0_f64
                            } else {
                                1.0_f64
                            };
                            {
                                ci += 1.0;
                                ci
                            };
                        }
                    }
                    g.width += group_extra;
                }
            }
            {
                li += 1.0;
                li
            };
        }
    }
}

// Source: upstream/packages/textlayout/src/textLayout.ts:734 (sha256:91963c0726ef60424115ef85615d94ece3fe37852fc3c7fb1167989749ccd27b)
fn write_line_metrics(out: &mut TextLayoutResult, groups: &Vec<TextLayoutGroup>) -> () {
    out.line_ascents.clear();
    out.line_descents.clear();
    out.line_heights.clear();
    out.line_leadings.clear();
    out.line_widths.clear();
    out.text_width = 0.0_f64;
    out.text_height = 0.0_f64;
    out.num_lines = 0.0_f64;
    for g in (groups).iter().cloned() {
        while (g.line_index >= out.num_lines) {
            out.line_ascents.push(0.0_f64);
            out.line_descents.push(0.0_f64);
            out.line_heights.push(0.0_f64);
            out.line_leadings.push(0.0_f64);
            out.line_widths.push(0.0_f64);
            {
                out.num_lines += 1.0;
                out.num_lines
            };
        }
        let li = g.line_index;
        {
            let __flight_index = (li) as usize;
            let __flight_value = (out.line_ascents[li as usize].clone()).max(g.ascent);
            if __flight_index == out.line_ascents.len() {
                out.line_ascents.push(__flight_value);
            } else {
                out.line_ascents[__flight_index] = __flight_value;
            }
        };
        {
            let __flight_index = (li) as usize;
            let __flight_value = (out.line_descents[li as usize].clone()).max(g.descent);
            if __flight_index == out.line_descents.len() {
                out.line_descents.push(__flight_value);
            } else {
                out.line_descents[__flight_index] = __flight_value;
            }
        };
        {
            let __flight_index = (li) as usize;
            let __flight_value = (out.line_heights[li as usize].clone()).max(g.height);
            if __flight_index == out.line_heights.len() {
                out.line_heights.push(__flight_value);
            } else {
                out.line_heights[__flight_index] = __flight_value;
            }
        };
        if (g.leading > out.line_leadings[li as usize].clone()) {
            {
                let __flight_index = (li) as usize;
                let __flight_value = g.leading;
                if __flight_index == out.line_leadings.len() {
                    out.line_leadings.push(__flight_value);
                } else {
                    out.line_leadings[__flight_index] = __flight_value;
                }
            };
        }
        let right_edge = ((g.offset_x - TEXT_LAYOUT_GUTTER) + g.width);
        if (right_edge > out.line_widths[li as usize].clone()) {
            {
                let __flight_index = (li) as usize;
                let __flight_value = right_edge;
                if __flight_index == out.line_widths.len() {
                    out.line_widths.push(__flight_value);
                } else {
                    out.line_widths[__flight_index] = __flight_value;
                }
            };
        }
        if (right_edge > out.text_width) {
            out.text_width = right_edge;
        }
        let bottom = (((g.offset_y - TEXT_LAYOUT_GUTTER) + g.ascent) + g.descent).ceil();
        if (bottom > out.text_height) {
            out.text_height = bottom;
        }
    }
    if (out.num_lines == 0.0_f64) {
        out.num_lines = 1.0_f64;
    }
}

// Source: upstream/packages/textlayout/src/textLayout.ts:775 (sha256:ef9aee364d575119a00353d0e0fa467b68ced5b306de669536c8eb98c8beae77)
pub fn create_text_layout_result() -> TextLayoutResult {
    return TextLayoutResult {
        __flight_identity: std::sync::Arc::new(()),
        groups: vec![],
        line_ascents: vec![],
        line_descents: vec![],
        line_heights: vec![],
        line_leadings: vec![],
        line_widths: vec![],
        num_lines: 0.0_f64,
        text_height: 0.0_f64,
        text_width: 0.0_f64,
    };
}

// Source: upstream/packages/textlayout/src/textLayout.ts:789 (sha256:29eef70d4b82cb8c4cafec5bfa991e6143190a4f2930489fe2dd028bec3c0193)
pub fn is_text_layout_truncated(layout: &TextLayoutResult, params: &TextLayoutParams) -> bool {
    if ((params.max_lines).is_none())
        || ((params.max_lines)
            .as_ref()
            .is_some_and(|value| *value < 0.0_f64))
    {
        return false;
    }
    return (layout.num_lines >= params.max_lines) && ((layout.groups.len() as f64) > 0.0_f64);
}
