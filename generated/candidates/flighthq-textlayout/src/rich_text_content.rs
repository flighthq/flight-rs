// @generated from upstream/packages/textlayout/src/richTextContent.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::merge_text_format;
use flighthq_types::{RichTextContent, RichTextData, RichTextRuntime, TextFormat, TextFormatRange};

// Source: upstream/packages/textlayout/src/richTextContent.ts:5 (sha256:b40cc1fb78aa22db5607db40e1b7c99cb99c3ca95de8de4e18b6a365aa3d49af)
pub fn clear_rich_text_content(mut runtime: RichTextRuntime) -> () {
    {
        let __flight_runtime = runtime;
        let __flight_value = None;
        let mut __flight_storage = __flight_runtime.inner.lock().unwrap();
        __flight_storage.rich_text_content = __flight_value;
    };
}

// Source: upstream/packages/textlayout/src/richTextContent.ts:13 (sha256:30f88764e43644af3300a07a4de799bea9576c2b0322758b13e0835ed3a570c6)
pub fn compute_rich_text_content(
    out: &mut RichTextContent,
    data: &RichTextData,
    password_character: Option<String>,
) -> () {
    out.text = "".to_owned();
    out.format_ranges.clear();
    let base_format = create_base_format(data);
    let source = get_renderable_source(data, ((password_character).clone()).clone());
    if ((source.encode_utf16().count() as f64) == 0.0_f64) {
        return;
    }
    append_text(
        out,
        (source).clone(),
        &base_format,
        data.condense_white,
        data.max_chars,
    );
    {
        let __flight_argument_1 = (out.text.encode_utf16().count() as f64);
        clamp_ranges(&mut out.format_ranges, __flight_argument_1)
    };
    apply_text_format_ranges(out, &data.text_format_ranges);
}

// Source: upstream/packages/textlayout/src/richTextContent.ts:34 (sha256:05c5233b916c715e5b06c9150e295f1065149bffd63718972dfafa5eb2fea130)
pub fn create_rich_text_content() -> RichTextContent {
    return RichTextContent {
        __flight_identity: std::sync::Arc::new(()),
        format_ranges: vec![],
        text: "".to_owned(),
    };
}

// Source: upstream/packages/textlayout/src/richTextContent.ts:38 (sha256:56db16cad411b29c5bb8836c46fdca70a13980c29887fdf5c23d83786382e85a)
pub fn get_rich_text_content(mut runtime: RichTextRuntime) -> RichTextContent {
    if ((runtime.inner.lock().unwrap().rich_text_content).clone()).is_none() {
        {
            let __flight_runtime = runtime;
            let __flight_value = Some(create_rich_text_content());
            let mut __flight_storage = __flight_runtime.inner.lock().unwrap();
            __flight_storage.rich_text_content = __flight_value;
        };
    }
    return ((runtime.inner.lock().unwrap().rich_text_content).clone()).unwrap();
}

// Source: upstream/packages/textlayout/src/richTextContent.ts:45 (sha256:2032b25b2d9d1a3dc1eea76dcb23cb441fbd8a5351ad813759bec413992ed9a4)
fn append_text(
    out: &mut RichTextContent,
    text: String,
    format: &TextFormat,
    condense_white: bool,
    max_chars: f64,
) -> () {
    let mut value = decode_html_entities((text).clone());
    if condense_white {
        value = (regex::RegexBuilder::new("[ \\f\\n\\r\\t\\v]+")
            .case_insensitive(false)
            .multi_line(false)
            .dot_matches_new_line(false)
            .build()
            .expect("upstream TypeScript regular expression must be valid Rust regex syntax"))
        .replace_all(&(value), " ")
        .into_owned();
        if ((out.text.encode_utf16().count() as f64) == 0.0_f64) {
            value = (value.trim_start)();
        }
        if ((out.text).clone()).ends_with(" ") {
            value = (value.trim_start)();
        }
    }
    if ((value.encode_utf16().count() as f64) == 0.0_f64) {
        return;
    }
    let remaining = if (max_chars < 0.0_f64) {
        (value.encode_utf16().count() as f64)
    } else {
        (0.0_f64).max((max_chars - (out.text.encode_utf16().count() as f64)))
    };
    if (remaining == 0.0_f64) {
        return;
    }
    if ((value.encode_utf16().count() as f64) > remaining) {
        value = (value.slice)(0.0_f64, remaining);
    }
    let start = (out.text.encode_utf16().count() as f64);
    out.text += (value).clone();
    {
        let __flight_argument_3 = (out.text.encode_utf16().count() as f64);
        write_format_range(&mut out.format_ranges, format, start, __flight_argument_3)
    };
}

// Source: upstream/packages/textlayout/src/richTextContent.ts:69 (sha256:f1747b1c807432a75063b6faa16cd2854fba96b7541ac633455c123cf92d47d8)
fn apply_text_format_ranges(out: &mut RichTextContent, overrides: &Vec<TextFormatRange>) -> () {
    if ((overrides.len() as f64) == 0.0_f64)
        || ((out.text.encode_utf16().count() as f64) == 0.0_f64)
    {
        return;
    }
    for override_ in (overrides).iter().cloned() {
        let start = (0.0_f64).max((out.text.encode_utf16().count() as f64).min(override_.start));
        let end = (start).max((out.text.encode_utf16().count() as f64).min(override_.end));
        if (start == end) {
            continue;
        }
        let mut next: Vec<TextFormatRange> = vec![];
        for range in (out.format_ranges).iter().cloned() {
            if (range.end <= start) || (range.start >= end) {
                write_format_range(&mut next, &range.format, range.start, range.end);
                continue;
            }
            if (range.start < start) {
                write_format_range(&mut next, &range.format, range.start, start);
            }
            write_format_range(
                &mut next,
                &merge_text_format(&range.format, &override_.format),
                (range.start).max(start),
                (range.end).min(end),
            );
            if (range.end > end) {
                write_format_range(&mut next, &range.format, end, range.end);
            }
        }
        out.format_ranges = (next).clone();
    }
    out.format_ranges.clear();
    for range in (out.format_ranges).iter().cloned() {
        write_format_range(
            &mut out.format_ranges,
            &range.format,
            range.start,
            range.end,
        );
    }
}

// Source: upstream/packages/textlayout/src/richTextContent.ts:101 (sha256:ef80663a5b42c81c1363d1dcda13672236450a739f119a107d11fc44d0bf5b10)
fn clamp_ranges(ranges: &mut Vec<TextFormatRange>, length: f64) -> () {
    {
        let mut i = ((ranges.len() as f64) - 1.0_f64);
        while (i >= 0.0_f64) {
            let mut range = ranges[i as usize].clone();
            if (range.start >= length) {
                ranges.splice((i) as usize..((i) + (1.0_f64)) as usize, vec![]);
            } else {
                if (range.end > length) {
                    range.end = length;
                }
            }
            {
                i -= 1.0;
                i
            };
        }
    }
}

// Source: upstream/packages/textlayout/src/richTextContent.ts:112 (sha256:b593981411e261f5be687efabce2f1a0bd7b4c755db650fc3fbfa225dd4daa4a)
fn create_base_format(data: &RichTextData) -> TextFormat {
    let mut format = merge_text_format(&data.default_text_format, &data.text_format);
    if (format.color).is_none() {
        format.color = Some(data.text_color);
    }
    return format;
}

// Source: upstream/packages/textlayout/src/richTextContent.ts:118 (sha256:7f06abd12e25e9e61f75d50d23db46f1a9cdc500b91e99739865b7c18f05c259)
fn decode_html_entities(value: String) -> String {
    return {
        let mut __flight_replace = |_match: String, entity: String| -> String {
            let lower = (entity).to_lowercase();
            if (lower).starts_with("#x") {
                return (string.from_code_point)((number.parse_int)(
                    (lower.slice)(2.0_f64),
                    16.0_f64,
                ));
            }
            if (lower).starts_with("#") {
                return (string.from_code_point)((number.parse_int)(
                    (lower.slice)(1.0_f64),
                    10.0_f64,
                ));
            }
            return NAMED_ENTITIES
                .iter()
                .find(|(key, _)| key == &(lower).clone())
                .map(|(_, value)| value)
                .expect("TypeScript Record key was absent")
                .clone();
        };
        (regex::RegexBuilder::new("&(#x[0-9a-f]+|#[0-9]+|[a-z]+);")
            .case_insensitive(true)
            .multi_line(false)
            .dot_matches_new_line(false)
            .build()
            .expect("upstream TypeScript regular expression must be valid Rust regex syntax"))
        .replace_all(&(value), |captures: &regex::Captures<'_>| {
            __flight_replace(
                captures
                    .get(0)
                    .map_or("", |matched| matched.as_str())
                    .to_owned(),
                captures
                    .get(1)
                    .map_or("", |matched| matched.as_str())
                    .to_owned(),
            )
        })
        .into_owned()
    };
}

// Source: upstream/packages/textlayout/src/richTextContent.ts:127 (sha256:b047f25f7ca82be989a273e4d199a15190e34ba1694f90ce1dcf24b185ce9fd0)
fn get_renderable_source(data: &RichTextData, password_character: Option<String>) -> String {
    if (password_character).is_none() {
        return (data.text).clone();
    }
    let mask = if ((password_character.as_ref().unwrap().encode_utf16().count() as f64) > 0.0_f64) {
        (password_character.as_ref().unwrap().char_at)(0.0_f64)
    } else {
        "•".to_owned()
    };
    return (mask.repeat)((data.text.encode_utf16().count() as f64));
}

// Source: upstream/packages/textlayout/src/richTextContent.ts:133 (sha256:29e6bd354a194abe1d36111a87bc0a8e9728d443c9b9c2b05e169be762ffb718)
fn write_format_range(
    ranges: &mut Vec<TextFormatRange>,
    format: &TextFormat,
    start: f64,
    end: f64,
) -> () {
    if (start == end) {
        return;
    }
    let mut previous = ranges[((ranges.len() as f64) - 1.0_f64) as usize].clone();
    if (((previous).is_some()) && (previous.end == start))
        && (text_format_equals(&previous.format, format))
    {
        previous.end = end;
    } else {
        ranges.push(TextFormatRange {
            __flight_identity: std::sync::Arc::new(()),
            end: end,
            format: (format).clone(),
            start: start,
        });
    }
}

// Source: upstream/packages/textlayout/src/richTextContent.ts:143 (sha256:8fd21605927746f23b9de316987d4358bd6fa1e6040a447988bb6f6191cdb340)
fn text_format_equals(a: &TextFormat, b: &TextFormat) -> bool {
    let a_keys = crate::host_value::<Vec<TextFormat>>("host.keys");
    let b_keys = crate::host_value::<Vec<TextFormat>>("host.keys");
    if ((a_keys.len() as f64) != (b_keys.len() as f64)) {
        return false;
    }
    for key in (a_keys).iter().cloned() {
        let a_value = a[key as usize].clone();
        let b_value = b[key as usize].clone();
        if (false) && (false) {
            if (a_value.length != b_value.length) {
                return false;
            }
            {
                let mut i = 0.0_f64;
                while (i < a_value.length) {
                    if (a_value[i as usize].clone() != b_value[i as usize].clone()) {
                        return false;
                    }
                    {
                        i += 1.0;
                        i
                    };
                }
            }
        } else {
            if (a_value != b_value) {
                return false;
            }
        }
    }
    return true;
}

// Source: upstream/packages/textlayout/src/richTextContent.ts:162 (sha256:b81d577e3d17a6837d68a7d0f19b37e0a6d993712c67071c0a8ddc495d096a2b)
static NAMED_ENTITIES: std::sync::LazyLock<Vec<(String, String)>> =
    std::sync::LazyLock::new(|| {
        let mut __flight_record = Vec::new();
        __flight_record.push(("amp".to_owned(), "&".to_owned()));
        __flight_record.push(("apos".to_owned(), "'".to_owned()));
        __flight_record.push(("gt".to_owned(), ">".to_owned()));
        __flight_record.push(("lt".to_owned(), "<".to_owned()));
        __flight_record.push(("nbsp".to_owned(), " ".to_owned()));
        __flight_record.push(("quot".to_owned(), "\"".to_owned()));
        __flight_record
    });
