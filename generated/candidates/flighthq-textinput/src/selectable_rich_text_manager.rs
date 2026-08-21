// @generated from upstream/packages/textinput/src/selectableRichTextManager.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_text::{get_rich_text_runtime, set_rich_text_scroll_v};
use flighthq_textlayout::compute_rich_text_char_index_at_point;
use flighthq_types::{
    InputKeyboardData, KeyCode, RichText, RichTextRuntime, SelectableRichTextManager,
};

#[inline]

fn __flight_string_slice(value: &str, start: f64, end: Option<f64>) -> String {
    let value: Vec<u16> = value.encode_utf16().collect();
    let length = value.len();
    let relative = |index: f64| -> usize {
        if index.is_nan() {
            0
        } else if index < 0.0_f64 {
            length.saturating_sub((-index.trunc()) as usize)
        } else {
            (index.trunc() as usize).min(length)
        }
    };
    let start = relative(start);
    let end = end.map_or(length, relative);
    String::from_utf16_lossy(&value[start..end.max(start)])
}

// Source: upstream/packages/textinput/src/selectableRichTextManager.ts:6 (sha256:dc0cfcd96ce6e431b84590d05050cb5b21117b511f5aaa41b5950090906a8ffe)
pub fn blur_selectable_rich_text(manager: &mut SelectableRichTextManager) -> () {
    if ((manager.focused).clone()).is_some() {
        let mut runtime = get_mutable_runtime(manager.focused.as_ref().unwrap());
        {
            let __flight_runtime = runtime;
            let __flight_value = 0.0_f64;
            let mut __flight_storage = __flight_runtime.inner.lock().unwrap();
            __flight_storage.selection_begin_index = __flight_value;
        };
        {
            let __flight_runtime = runtime;
            let __flight_value = 0.0_f64;
            let mut __flight_storage = __flight_runtime.inner.lock().unwrap();
            __flight_storage.selection_end_index = __flight_value;
        };
    }
    manager.focused = None;
}

// Source: upstream/packages/textinput/src/selectableRichTextManager.ts:15 (sha256:b036116604472a36d40849b18c2d80a3b12fd8884d12553fc3301d15461afa57)
pub fn create_selectable_rich_text_manager() -> SelectableRichTextManager {
    return SelectableRichTextManager {
        __flight_identity: std::sync::Arc::new(()),
        focused: None,
    };
}

// Source: upstream/packages/textinput/src/selectableRichTextManager.ts:19 (sha256:2c301c56502f40a2a6cfc48af4ed32e6a294b1fc26f3f4e92059e1fbc27eda4c)
pub fn dispatch_selectable_rich_text_key_down(
    manager: &SelectableRichTextManager,
    data: &InputKeyboardData,
    on_copy: Option<
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(String) -> () + Send + 'static>>>,
    >,
) -> bool {
    let target = (manager.focused).clone();
    if (target).is_none() {
        return false;
    }
    if ((data.ctrl_key) || (data.meta_key))
        && ((((data.key).clone()).to_lowercase() == "a") || (data.key_code == KeyCode::A))
    {
        let mut runtime = get_mutable_runtime(&target.as_ref().unwrap());
        {
            let __flight_runtime = runtime;
            let __flight_value = 0.0_f64;
            let mut __flight_storage = __flight_runtime.inner.lock().unwrap();
            __flight_storage.selection_begin_index = __flight_value;
        };
        {
            let __flight_runtime = runtime;
            let __flight_value = (target.as_ref().unwrap().data.text.encode_utf16().count() as f64);
            let mut __flight_storage = __flight_runtime.inner.lock().unwrap();
            __flight_storage.selection_end_index = __flight_value;
        };
        return true;
    }
    if ((data.ctrl_key) || (data.meta_key))
        && ((((data.key).clone()).to_lowercase() == "c") || (data.key_code == KeyCode::C))
    {
        let mut runtime = get_mutable_runtime(&target.as_ref().unwrap());
        let start = (runtime.inner.lock().unwrap().selection_begin_index)
            .min(runtime.inner.lock().unwrap().selection_end_index);
        let end = (runtime.inner.lock().unwrap().selection_begin_index)
            .max(runtime.inner.lock().unwrap().selection_end_index);
        let selected = __flight_string_slice(
            &((target.as_ref().unwrap().data.text).clone()),
            start,
            Some(end),
        );
        if ((selected.encode_utf16().count() as f64) > 0.0_f64) {
            {
                let __flight_callback = on_copy;
                __flight_callback
                    .as_ref()
                    .map(|callback| callback.lock().unwrap()((selected).clone()))
            };
        }
        return true;
    }
    return false;
}

// Source: upstream/packages/textinput/src/selectableRichTextManager.ts:43 (sha256:17858d41ee8a97578a5354aacea20b206f7f6857151967f3a44d82e7eeb600ac)
pub fn dispatch_selectable_rich_text_pointer_down(
    manager: &mut SelectableRichTextManager,
    target: &RichText,
    x: f64,
    y: f64,
    extend: Option<bool>,
) -> () {
    let extend = extend.unwrap_or(false);
    manager.focused = Some((*target).clone());
    let mut runtime = get_mutable_runtime(target);
    let layout = (runtime.inner.lock().unwrap().text_layout).clone();
    if (layout).is_none() {
        if (!extend) {
            {
                let __flight_runtime = runtime;
                let __flight_value = 0.0_f64;
                let mut __flight_storage = __flight_runtime.inner.lock().unwrap();
                __flight_storage.selection_begin_index = __flight_value;
            };
            {
                let __flight_runtime = runtime;
                let __flight_value = 0.0_f64;
                let mut __flight_storage = __flight_runtime.inner.lock().unwrap();
                __flight_storage.selection_end_index = __flight_value;
            };
        }
        return;
    }
    let index = compute_rich_text_char_index_at_point(&layout.as_ref().unwrap(), x, y);
    if extend {
        {
            let __flight_runtime = runtime;
            let __flight_value = index;
            let mut __flight_storage = __flight_runtime.inner.lock().unwrap();
            __flight_storage.selection_end_index = __flight_value;
        };
    } else {
        {
            let __flight_runtime = runtime;
            let __flight_value = index;
            let mut __flight_storage = __flight_runtime.inner.lock().unwrap();
            __flight_storage.selection_begin_index = __flight_value;
        };
        {
            let __flight_runtime = runtime;
            let __flight_value = index;
            let mut __flight_storage = __flight_runtime.inner.lock().unwrap();
            __flight_storage.selection_end_index = __flight_value;
        };
    }
}

// Source: upstream/packages/textinput/src/selectableRichTextManager.ts:69 (sha256:925955e6cded27a704a7ccaa61f1b831647530e6dcb4262c8eccdc86e0a59140)
pub fn dispatch_selectable_rich_text_pointer_move(
    manager: &SelectableRichTextManager,
    x: f64,
    y: f64,
) -> () {
    let target = (manager.focused).clone();
    if (target).is_none() {
        return;
    }
    let mut runtime = get_mutable_runtime(&target.as_ref().unwrap());
    let layout = (runtime.inner.lock().unwrap().text_layout).clone();
    if (layout).is_none() {
        return;
    }
    {
        let __flight_runtime = runtime;
        let __flight_value = compute_rich_text_char_index_at_point(&layout.as_ref().unwrap(), x, y);
        let mut __flight_storage = __flight_runtime.inner.lock().unwrap();
        __flight_storage.selection_end_index = __flight_value;
    };
}

// Source: upstream/packages/textinput/src/selectableRichTextManager.ts:78 (sha256:d80e14dfbb500694e6db9f91e6bd26d8389c8660dbf4aac9583e49b096dd3f09)
pub fn dispatch_selectable_rich_text_wheel(
    manager: &mut SelectableRichTextManager,
    delta_lines: f64,
) -> () {
    let mut target = (manager.focused).clone();
    if (target).is_none() {
        return;
    }
    {
        let __flight_argument_1 = (target.as_mut().unwrap().data.scroll_v + (delta_lines).round());
        let __flight_result =
            set_rich_text_scroll_v(&mut target.as_mut().unwrap(), __flight_argument_1, None);
        __flight_result
    };
}

// Source: upstream/packages/textinput/src/selectableRichTextManager.ts:84 (sha256:3e078a74724577332e280401524f14b2c6933d282bb12963a07776987cd3d6a8)
pub fn focus_selectable_rich_text(
    manager: &mut SelectableRichTextManager,
    target: &RichText,
) -> () {
    manager.focused = Some((*target).clone());
}

// Source: upstream/packages/textinput/src/selectableRichTextManager.ts:88 (sha256:1e8629eb066c102b653229a10c178f41d234f81659d87fd1aa6a98bbefcc1180)
pub fn get_selectable_rich_text_selection_text(manager: &SelectableRichTextManager) -> String {
    let target = (manager.focused).clone();
    if (target).is_none() {
        return "".to_owned();
    }
    let runtime = get_mutable_runtime(&target.as_ref().unwrap());
    let start = (runtime.inner.lock().unwrap().selection_begin_index)
        .min(runtime.inner.lock().unwrap().selection_end_index);
    let end = (runtime.inner.lock().unwrap().selection_begin_index)
        .max(runtime.inner.lock().unwrap().selection_end_index);
    return __flight_string_slice(
        &((target.as_ref().unwrap().data.text).clone()),
        start,
        Some(end),
    );
}

// Source: upstream/packages/textinput/src/selectableRichTextManager.ts:97 (sha256:733e60d658d9c7706305e948bd891d19dd23aaca31961e89d0f62f726b7ddd45)
fn get_mutable_runtime(source: &RichText) -> RichTextRuntime {
    return get_rich_text_runtime(source);
}
