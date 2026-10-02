// @generated from upstream/packages/textlayout/src/textLayoutMeasure.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_textshaper::measure_text;
use flighthq_types::{HostTextShaperCapability, TextFormat, TextMeasureFunction};

// Source: upstream/packages/textlayout/src/textLayoutMeasure.ts:9 (sha256:7dbbefae23749c0285db59644c79c6e190f7688c662a8f4f004f50c01b02c1a3)
pub fn get_text_layout_measure_provider(
    host_text_shaper: Option<HostTextShaperCapability>,
) -> Option<TextMeasureFunction> {
    if ((*_MEASURE_PROVIDER.lock().unwrap()).clone()).is_some() {
        return Some(((*_MEASURE_PROVIDER.lock().unwrap()).as_ref().unwrap()).clone());
    }
    if (host_text_shaper).is_some() {
        return Some(std::sync::Arc::new(std::sync::Mutex::new(Box::new({
            let host_text_shaper = host_text_shaper.clone();
            move |text: String, format: TextFormat| -> f64 {
                measure_text(&host_text_shaper.as_ref().unwrap(), (text).clone(), &format)
            }
        })
            as Box<dyn FnMut(String, TextFormat) -> f64 + Send + 'static>)));
    }
    return None;
}

// Source: upstream/packages/textlayout/src/textLayoutMeasure.ts:21 (sha256:893aea19bf40728505d0bab91bfacc2ada9ad0fd844f054437ffe52d1890b359)
pub fn set_text_layout_measure_provider(measure: &Option<TextMeasureFunction>) -> () {
    (*_MEASURE_PROVIDER.lock().unwrap()) = (*measure).clone();
}

// Source: upstream/packages/textlayout/src/textLayoutMeasure.ts:25 (sha256:d7a4d39f002ca30a2a6ed02d1e44c91d4402d6393525a6e11a3beef87c655c51)
static _MEASURE_PROVIDER: std::sync::LazyLock<std::sync::Mutex<Option<TextMeasureFunction>>> =
    std::sync::LazyLock::new(|| std::sync::Mutex::new(None));
