// @generated from upstream/packages/textsegment/src/textSegment.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::get_text_segmenter_backend;
use flighthq_types::TextSegment;

// Source: upstream/packages/textsegment/src/textSegment.ts:9 (sha256:0d818661bccf843975721eecb7793cadbe76ddb0e705b581a3d882da18082afb)
pub fn segment_graphemes(text: String, locale: Option<String>) -> Vec<TextSegment> {
    return ((get_text_segmenter_backend().segment).clone())
        .lock()
        .unwrap()(
        (text).clone(),
        "grapheme".to_owned(),
        (locale).clone().unwrap(),
    );
}

// Source: upstream/packages/textsegment/src/textSegment.ts:15 (sha256:ca722ccb732deb10e1a7d324a9cc437a9e9202e8e328f27bf6ea5ec00bd85a6f)
pub fn segment_sentences(text: String, locale: Option<String>) -> Vec<TextSegment> {
    return ((get_text_segmenter_backend().segment).clone())
        .lock()
        .unwrap()(
        (text).clone(),
        "sentence".to_owned(),
        (locale).clone().unwrap(),
    );
}

// Source: upstream/packages/textsegment/src/textSegment.ts:22 (sha256:97dda7c23fcd8c54267854d607297f313ea6b36e5e2a73d1bdccec15eb2445d1)
pub fn segment_words(text: String, locale: Option<String>) -> Vec<TextSegment> {
    return ((get_text_segmenter_backend().segment).clone())
        .lock()
        .unwrap()((text).clone(), "word".to_owned(), (locale).clone().unwrap());
}
