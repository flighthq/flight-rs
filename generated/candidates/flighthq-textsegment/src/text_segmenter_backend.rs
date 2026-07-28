// @generated from upstream/packages/textsegment/src/textSegmenterBackend.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::{TextSegment, TextSegmentGranularity, TextSegmenterBackend};

// Source: upstream/packages/textsegment/src/textSegmenterBackend.ts:9 (sha256:d2f9cc3e2e977902024921b5c76115fb5ac3a4a2150c23546310644ac5cd0584)
pub fn create_web_text_segmenter_backend() -> TextSegmenterBackend {
    return TextSegmenterBackend {
        __flight_identity: std::sync::Arc::new(()),
        segment: std::sync::Arc::new(std::sync::Mutex::new(Box::new(
            move |__flight_argument_0: String,
                  __flight_argument_1: TextSegmentGranularity,
                  __flight_argument_2: Option<String>|
                  -> Vec<TextSegment> {
                segment_with_intl_segmenter(
                    (__flight_argument_0).clone(),
                    (__flight_argument_1).clone(),
                    Some(((__flight_argument_2).clone().unwrap()).clone()),
                )
            },
        )
            as Box<
                dyn FnMut(String, TextSegmentGranularity, Option<String>) -> Vec<TextSegment>
                    + Send
                    + 'static,
            >)),
    };
}

// Source: upstream/packages/textsegment/src/textSegmenterBackend.ts:16 (sha256:2fca361e3de77ce792b2814ae5ef3fe0f920230977f5046103b089b5c933d85e)
pub fn get_text_segmenter_backend() -> TextSegmenterBackend {
    if ((*_BACKEND.lock().unwrap()).clone()).is_none() {
        (*_BACKEND.lock().unwrap()) = Some(create_web_text_segmenter_backend());
    }
    return ((*_BACKEND.lock().unwrap()).clone()).clone().unwrap();
}

// Source: upstream/packages/textsegment/src/textSegmenterBackend.ts:24 (sha256:598a62c1dae3193fd76072bf9a00672a2d17ed2ce2914fde26ba57bdc0e40bda)
pub fn set_text_segmenter_backend(backend: Option<TextSegmenterBackend>) -> () {
    (*_BACKEND.lock().unwrap()) = (backend).clone();
}

// Source: upstream/packages/textsegment/src/textSegmenterBackend.ts:28 (sha256:14f21ae90a84f4f8ea4fe7406c771f684b6ab654bfd6b52172a5fc790409bbf1)
static _BACKEND: std::sync::LazyLock<std::sync::Mutex<Option<TextSegmenterBackend>>> =
    std::sync::LazyLock::new(|| std::sync::Mutex::new(None));

// Source: upstream/packages/textsegment/src/textSegmenterBackend.ts:33 (sha256:e705626cc57df6bd19aa087beb254d37fb5010654f8570ec3bc442f648daabc3)
static _SEGMENTER_CACHE: std::sync::LazyLock<
    std::sync::Mutex<Vec<(String, crate::OpaqueHostValue)>>,
> = std::sync::LazyLock::new(|| std::sync::Mutex::new(Vec::new()));

// Source: upstream/packages/textsegment/src/textSegmenterBackend.ts:34 (sha256:d65d9ef7cac07d1e8c6724d81f0ee32f6d00f230815bb2db7054518b0bf92f2f)
const _SEGMENTER_CACHE_CAPACITY: f64 = 64.0_f64;

// Source: upstream/packages/textsegment/src/textSegmenterBackend.ts:36 (sha256:f8fafb29fafca4e5aca2a532f1bc26eb83d09d8e7ccfe64cc8746eeb8123ae3d)
fn get_cached_segmenter(
    locale: Option<String>,
    granularity: TextSegmentGranularity,
) -> Option<crate::OpaqueHostValue> {
    return None;
}

// Source: upstream/packages/textsegment/src/textSegmenterBackend.ts:51 (sha256:5a0bb5677e13da18563a19423d8989d8c6551e557defe2e6faaae7a695d34d8a)
fn segment_with_intl_segmenter(
    text: String,
    granularity: TextSegmentGranularity,
    locale: Option<String>,
) -> Vec<TextSegment> {
    let segmenter = get_cached_segmenter(((locale).clone()).clone(), (granularity).clone());
    if (segmenter).is_none() {
        return vec![];
    }
    let mut out: Vec<TextSegment> = vec![];
    let is_word_granularity = (granularity == "word");
    for data in (crate::host_value::<Vec<crate::OpaqueHostValue>>("host.call"))
        .iter()
        .cloned()
    {
        let start = crate::host_value::<f64>("host.index");
        let mut record: TextSegment = TextSegment {
            __flight_identity: std::sync::Arc::new(()),
            start: start,
            end: (start
                + (crate::host_value::<String>("host.segment")
                    .encode_utf16()
                    .count() as f64)),
            text: crate::host_value::<String>("host.segment"),
            is_word_like: None,
        };
        if is_word_granularity {
            record.is_word_like = Some(crate::host_value::<bool>("host.isWordLike"));
        }
        out.push(((record).clone()).clone());
    }
    return out;
}
