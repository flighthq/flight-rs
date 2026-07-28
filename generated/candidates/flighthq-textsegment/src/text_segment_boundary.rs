// @generated from upstream/packages/textsegment/src/textSegmentBoundary.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{segment_graphemes, segment_words};
use flighthq_types::{TextSegment, TextSegmentRange};

// Source: upstream/packages/textsegment/src/textSegmentBoundary.ts:8 (sha256:9b261149a3b16b1c07aa338d8f15b6ec1c8296c9a5550e7f6742efc0defad87e)
pub fn get_next_grapheme_boundary(text: String, index: f64, locale: Option<String>) -> f64 {
    return next_segment_boundary(
        &segment_graphemes((text).clone(), Some(((locale).clone().unwrap()).clone())),
        index,
        text.length,
    );
}

// Source: upstream/packages/textsegment/src/textSegmentBoundary.ts:15 (sha256:187329cb86ebec5e2203fcf8c28f5c57477922e46b8ebff0722630023f9eff13)
pub fn get_next_word_boundary(text: String, index: f64, locale: Option<String>) -> f64 {
    return next_segment_boundary(
        &segment_words((text).clone(), Some(((locale).clone().unwrap()).clone())),
        index,
        text.length,
    );
}

// Source: upstream/packages/textsegment/src/textSegmentBoundary.ts:22 (sha256:b27ae99940226f7908e4ef1290216d470eceae33b7e7a155359155f98fe58b15)
pub fn get_previous_grapheme_boundary(text: String, index: f64, locale: Option<String>) -> f64 {
    return previous_segment_boundary(
        &segment_graphemes((text).clone(), Some(((locale).clone().unwrap()).clone())),
        index,
    );
}

// Source: upstream/packages/textsegment/src/textSegmentBoundary.ts:28 (sha256:f609c082d8e7b78f4dafa99bb0aaf11c20d4852ff70dfc1bb008e73311b7e6c3)
pub fn get_previous_word_boundary(text: String, index: f64, locale: Option<String>) -> f64 {
    return previous_segment_boundary(
        &segment_words((text).clone(), Some(((locale).clone().unwrap()).clone())),
        index,
    );
}

// Source: upstream/packages/textsegment/src/textSegmentBoundary.ts:36 (sha256:edd5b770a0c436a12a1408366e65bedf6e0ba28995047af52c5ae91787631ac4)
pub fn get_word_range_at(
    text: String,
    index: f64,
    locale: Option<String>,
) -> Option<TextSegmentRange> {
    if (text.length == 0.0_f64) {
        return None;
    }
    let clamped = clamp_index(index, text.length);
    let lookup = if (clamped == text.length) {
        (text.length - 1.0_f64)
    } else {
        clamped
    };
    let segments = segment_words((text).clone(), Some(((locale).clone().unwrap()).clone()));
    for segment in (segments).iter().cloned() {
        if ((lookup >= segment.start) && (lookup < segment.end)) {
            return if (segment.is_word_like) == Some(true) {
                Some(TextSegmentRange {
                    __flight_identity: std::sync::Arc::new(()),
                    start: segment.start,
                    end: segment.end,
                })
            } else {
                None
            };
        }
    }
    return None;
}

// Source: upstream/packages/textsegment/src/textSegmentBoundary.ts:53 (sha256:1d33354808837763bd0329465d868445ef61f7cc2020ecd7bcc99d39fea2a21f)
fn clamp_index(index: f64, length: f64) -> f64 {
    if (index < 0.0_f64) {
        return 0.0_f64;
    }
    if (index > length) {
        return length;
    }
    return index;
}

// Source: upstream/packages/textsegment/src/textSegmentBoundary.ts:59 (sha256:b9c0b2bfef0909b6fd3e3eb8b7c29ed784962b0e0aca23684513cab13abac598)
fn next_segment_boundary(segments: &Vec<TextSegment>, index: f64, length: f64) -> f64 {
    let from = clamp_index(index, length);
    if (from >= length) {
        return length;
    }
    for segment in (segments).iter().cloned() {
        if (segment.start > from) {
            return segment.start;
        }
    }
    return length;
}

// Source: upstream/packages/textsegment/src/textSegmentBoundary.ts:68 (sha256:bc782e446780d7ed66af433dfcf3551ac9d0e10c93ca13ad4e789c9a474dcc32)
fn previous_segment_boundary(segments: &Vec<TextSegment>, index: f64) -> f64 {
    let length = if ((segments.len() as f64) == 0.0_f64) {
        0.0_f64
    } else {
        segments[((segments.len() as f64) - 1.0_f64) as usize].end
    };
    let from = clamp_index(index, length);
    if (from <= 0.0_f64) {
        return 0.0_f64;
    }
    let mut previous = 0.0_f64;
    for segment in (segments).iter().cloned() {
        if (segment.start >= from) {
            break;
        }
        previous = segment.start;
    }
    return previous;
}
