// @generated from upstream/packages/textlayout/src/textLayoutRuntime.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::create_text_layout_result;
use flighthq_types::{TextLabelRuntime, TextLayoutResult};

// Source: upstream/packages/textlayout/src/textLayoutRuntime.ts:5 (sha256:f30d6ef17bfe5f0f44a4d1053b0d15d28a22e4121651b6a198087ccb8bb41546)
pub fn clear_text_layout_result(mut runtime: TextLabelRuntime) -> () {
    {
        let __flight_runtime = runtime;
        let __flight_value = None;
        let mut __flight_storage = __flight_runtime.inner.lock().unwrap();
        __flight_storage.text_layout = __flight_value;
    };
}

// Source: upstream/packages/textlayout/src/textLayoutRuntime.ts:9 (sha256:b64039a94ae938c5f98957171fe7db14bcdb51c5f415e46fa7b5ebcc373ea6c5)
pub fn get_text_layout_result(mut runtime: TextLabelRuntime) -> TextLayoutResult {
    if ((runtime.inner.lock().unwrap().text_layout).clone()).is_none() {
        {
            let __flight_runtime = runtime;
            let __flight_value = Some(create_text_layout_result());
            let mut __flight_storage = __flight_runtime.inner.lock().unwrap();
            __flight_storage.text_layout = __flight_value;
        };
    }
    return ((runtime.inner.lock().unwrap().text_layout).clone()).unwrap();
}
