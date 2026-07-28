// @generated from upstream/packages/textlayout/src/textLayoutMeasure.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_textshaper::{get_text_shaper_backend, measure_text};
use flighthq_types::TextMeasureFunction;

// Source: upstream/packages/textlayout/src/textLayoutMeasure.ts:11 (sha256:c9787b742cfed724dbb1d937ab2b1365577b4dac7bc6328ffe7393fbf143a2bd)
pub fn get_text_layout_measure_provider() -> Option<TextMeasureFunction> {
    if ((*_MEASURE_PROVIDER.lock().unwrap()).clone()).is_some() {
        return Some((_MEASURE_PROVIDER.as_ref().unwrap()).clone());
    }
    if (get_text_shaper_backend()).is_some() {
        return Some(measure_text);
    }
    return None;
}

// Source: upstream/packages/textlayout/src/textLayoutMeasure.ts:20 (sha256:893aea19bf40728505d0bab91bfacc2ada9ad0fd844f054437ffe52d1890b359)
pub fn set_text_layout_measure_provider(measure: Option<TextMeasureFunction>) -> () {
    (*_MEASURE_PROVIDER.lock().unwrap()) = (measure).clone();
}

// Source: upstream/packages/textlayout/src/textLayoutMeasure.ts:24 (sha256:d7a4d39f002ca30a2a6ed02d1e44c91d4402d6393525a6e11a3beef87c655c51)
static _MEASURE_PROVIDER: std::sync::LazyLock<std::sync::Mutex<Option<TextMeasureFunction>>> =
    std::sync::LazyLock::new(|| std::sync::Mutex::new(None));
