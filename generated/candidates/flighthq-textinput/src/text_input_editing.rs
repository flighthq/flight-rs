// @generated from upstream/packages/textinput/src/textInputEditing.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::get_text_input_state;
use flighthq_node::invalidate_node_appearance;
use flighthq_text::{set_rich_text_scroll_h, set_rich_text_scroll_v};
use flighthq_textlayout::{
    TEXT_BOUNDS_GUTTER as text_bounds_gutter_constant, get_rich_text_selection_rectangles,
};
use flighthq_types::{
    HandleTextInputKeyboardOptions, KeyCode, KeyboardEventData, ReplaceTextInputOptions, RichText,
    TextFormatRange, TextInputHistoryEntry, TextInputState, TextLayoutGroup, TextLayoutResult,
    TextSelectionRectangle,
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

#[inline]

fn __flight_string_repeat(value: &str, count: f64) -> String {
    assert!(
        count.is_finite() && count >= 0.0_f64,
        "String.repeat count must be finite and non-negative"
    );
    value.repeat(count.trunc() as usize)
}

#[derive(Clone, Default)]
pub struct SharedStructuralRecord1 {
    pub __flight_identity: std::sync::Arc<()>,
    pub accepted: String,
    pub declined: String,
}
impl PartialEq for SharedStructuralRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/textinput/src/textInputEditing.ts:22 (sha256:e326b2d75f2adb4313b56534b50f4798e0e36564f5ff874ba3c7b3e5dcf6275e)
const DESIRED_CARET_X_UNSET: f64 = -1.0_f64;

// Source: upstream/packages/textinput/src/textInputEditing.ts:24 (sha256:822ae4d1588ba3c539010c11d459e7b21b08b296f9c12a6adc83a440a65f57b3)
pub fn append_text_input(source: &mut RichText, text: String) -> () {
    {
        let __flight_argument_1 = (source.data.text.encode_utf16().count() as f64);
        let __flight_argument_2 = (source.data.text.encode_utf16().count() as f64);
        let __flight_result = replace_text_input(
            source,
            __flight_argument_1,
            __flight_argument_2,
            (text).clone(),
            None,
        );
        __flight_result
    };
}

// Source: upstream/packages/textinput/src/textInputEditing.ts:28 (sha256:2bcf55cee88568cda03c95371a3becad84d2b6c174a001685b1fc961891f3b60)
pub fn apply_text_input_restriction(
    source: &RichText,
    text: String,
    replace_length: Option<f64>,
) -> String {
    let replace_length = replace_length.unwrap_or(0.0_f64);
    let mut value = (text).clone();
    if (!source.data.multiline) {
        value = (regex::RegexBuilder::new("[\\n\\r]+")
            .case_insensitive(false)
            .multi_line(false)
            .dot_matches_new_line(false)
            .build()
            .expect("upstream TypeScript regular expression must be valid Rust regex syntax"))
        .replace_all(&(value), "".to_owned())
        .into_owned();
    }
    value = restrict_text_input((value).clone(), (get_input_state(source).restrict).clone());
    if (source.data.max_chars > 0.0_f64) {
        let max_length = ((source.data.max_chars
            - (source.data.text.encode_utf16().count() as f64))
            + replace_length);
        if (max_length <= 0.0_f64) {
            return "".to_owned();
        }
        if ((value.encode_utf16().count() as f64) > max_length) {
            value = __flight_string_slice(&(value), 0.0_f64, Some(max_length));
        }
    }
    return value;
}

// Source: upstream/packages/textinput/src/textInputEditing.ts:45 (sha256:b3e029e76f8082259d1b78518f18fab923268eabeaeecedb5132e5cf40c45797)
pub fn can_redo_text_input(source: &RichText) -> bool {
    let state = get_input_state(source);
    return (state.history_index < ((state.history.len() as f64) - 1.0_f64));
}

// Source: upstream/packages/textinput/src/textInputEditing.ts:52 (sha256:47f4ee7687de190cef3caf62d51934012476beb154efe188704a4f0119560f1f)
pub fn can_undo_text_input(source: &RichText) -> bool {
    return (get_input_state(source).history_index >= 0.0_f64);
}

// Source: upstream/packages/textinput/src/textInputEditing.ts:57 (sha256:f4ad016c548b9283debcc9949177179748b9bf070d94cb5c5cc39d1aa7335848)
pub fn clear_text_input_history(source: &RichText) -> () {
    let mut state = get_input_state(source);
    state.history = vec![];
    state.history_index = (-1.0_f64);
}

// Source: upstream/packages/textinput/src/textInputEditing.ts:63 (sha256:d6623c8320caac904bd8373651ce5688b04a9a9bc7f0a15b58651147e877569f)
pub fn delete_text_input_backward(source: &mut RichText) -> () {
    let mut state = get_input_state(source);
    let start = get_text_input_selection_begin_index(source);
    let end = get_text_input_selection_end_index(source);
    if (start != end) {
        replace_text_input(source, start, end, "".to_owned(), None);
    } else {
        if (start > 0.0_f64) {
            replace_text_input(source, (start - 1.0_f64), start, "".to_owned(), None);
        }
    }
    state.selection_index = state.caret_index;
}

// Source: upstream/packages/textinput/src/textInputEditing.ts:75 (sha256:5b82628e49ab8cccf75d4ec4c11269865016d320ff3b47c4ac16ebb5d733a346)
pub fn delete_text_input_forward(source: &mut RichText) -> () {
    let start = get_text_input_selection_begin_index(source);
    let end = get_text_input_selection_end_index(source);
    if (start != end) {
        replace_text_input(source, start, end, "".to_owned(), None);
    } else {
        if (start < (source.data.text.encode_utf16().count() as f64)) {
            replace_text_input(source, start, (start + 1.0_f64), "".to_owned(), None);
        }
    }
}

// Source: upstream/packages/textinput/src/textInputEditing.ts:88 (sha256:1ae34b532e2d0622bef4a5dd47ea22be616947491d5a509b19ec2d53c83a1b31)
pub fn delete_text_input_word_backward(source: &mut RichText) -> () {
    let start = get_text_input_selection_begin_index(source);
    let end = get_text_input_selection_end_index(source);
    if (start != end) {
        replace_text_input(source, start, end, "".to_owned(), None);
        return;
    }
    let word_start = find_word_start_before((source.data.text).clone(), start);
    if (word_start < start) {
        replace_text_input(source, word_start, start, "".to_owned(), None);
    }
}

// Source: upstream/packages/textinput/src/textInputEditing.ts:102 (sha256:26a2b10fe2a564477f8fc22e2b33a17e45f54f6b9fbd81527361ac8ff6d82122)
pub fn delete_text_input_word_forward(source: &mut RichText) -> () {
    let start = get_text_input_selection_begin_index(source);
    let end = get_text_input_selection_end_index(source);
    if (start != end) {
        replace_text_input(source, start, end, "".to_owned(), None);
        return;
    }
    let word_end = find_word_end_after((source.data.text).clone(), start);
    if (word_end > start) {
        replace_text_input(source, start, word_end, "".to_owned(), None);
    }
}

// Source: upstream/packages/textinput/src/textInputEditing.ts:113 (sha256:f9fe38d5d6e63701c2907e4ed3c7c583a615a480a7fede9b36806a826de59423)
pub fn get_text_input_caret_index(source: &RichText) -> f64 {
    return clamp_index(
        get_input_state(source).caret_index,
        (source.data.text.encode_utf16().count() as f64),
    );
}

// Source: upstream/packages/textinput/src/textInputEditing.ts:117 (sha256:aeae66cdba43974af192034ba3c2869c9210e846dd2f83122c8ddcc4c2b6628d)
pub fn get_text_input_caret_rectangle(
    out: &mut TextSelectionRectangle,
    source: &RichText,
    layout: &TextLayoutResult,
) -> () {
    let caret_index = get_text_input_caret_index(source);
    let group = get_text_layout_group_at_index(layout, caret_index);
    if (group).is_none() {
        out.x = text_bounds_gutter_constant;
        out.y = text_bounds_gutter_constant;
        out.width = 1.0_f64;
        out.height = get_fallback_line_height(layout);
        out.line_index = 0.0_f64;
        return;
    }
    out.x = get_text_layout_group_caret_x(&group.as_ref().unwrap(), caret_index);
    out.y = group.as_ref().unwrap().offset_y;
    out.width = 1.0_f64;
    out.height = group.as_ref().unwrap().height;
    out.line_index = group.as_ref().unwrap().line_index;
}

// Source: upstream/packages/textinput/src/textInputEditing.ts:140 (sha256:af0e0e34d8f8aea0296d741b1a519c90b90da997dd57397e4b3d21f1969b4d3f)
pub fn get_text_input_character_index_at_point(
    source: &RichText,
    layout: &TextLayoutResult,
    x: f64,
    y: f64,
) -> f64 {
    if ((layout.groups.len() as f64) == 0.0_f64) {
        return 0.0_f64;
    }
    let mut closest_line_index = 0.0_f64;
    let mut closest_line_distance = f64::INFINITY;
    {
        let mut i = 0.0_f64;
        while (i < (layout.line_heights.len() as f64)) {
            let line_top = get_line_offset_y(layout, i);
            let line_bottom = (line_top + layout.line_heights[i as usize].clone());
            let distance = if (y < line_top) {
                (line_top - y)
            } else {
                if (y > line_bottom) {
                    (y - line_bottom)
                } else {
                    0.0_f64
                }
            };
            if (distance < closest_line_distance) {
                closest_line_distance = distance;
                closest_line_index = i;
            }
            {
                i += 1.0;
                i
            };
        }
    }
    let mut line_start = (source.data.text.encode_utf16().count() as f64);
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
            return get_text_layout_group_character_index_at_x(&group, x);
        }
    }
    return if (line_end > 0.0_f64) {
        line_end
    } else {
        line_start
    };
}

// Source: upstream/packages/textinput/src/textInputEditing.ts:173 (sha256:9a879d6139ef32775d6b5a52a414f52db8c30f2acc9963339c5137ca9c25c941)
pub fn get_text_input_display_text(source: &RichText) -> String {
    let state = get_input_state(source);
    if (!state.display_as_password) {
        return (source.data.text).clone();
    }
    let password_character = if ((state.password_character.encode_utf16().count() as f64) > 0.0_f64)
    {
        (state.password_character.char_at)(0.0_f64)
    } else {
        "•".to_owned()
    };
    return __flight_string_repeat(
        &(password_character),
        (source.data.text.encode_utf16().count() as f64),
    );
}

// Source: upstream/packages/textinput/src/textInputEditing.ts:180 (sha256:6b955d3b8c3ce332cbfce64fe58488f118427ef712bf4ef362d76ef72235ffec)
pub fn get_text_input_selection_begin_index(source: &RichText) -> f64 {
    let state = get_input_state(source);
    return (clamp_index(
        state.caret_index,
        (source.data.text.encode_utf16().count() as f64),
    ))
    .min(clamp_index(
        state.selection_index,
        (source.data.text.encode_utf16().count() as f64),
    ));
}

// Source: upstream/packages/textinput/src/textInputEditing.ts:188 (sha256:cb2082e4f1f56ce7e9038bf85b683daceb8c1549a9c806485fdb8293086e4ebd)
pub fn get_text_input_selection_end_index(source: &RichText) -> f64 {
    let state = get_input_state(source);
    return (clamp_index(
        state.caret_index,
        (source.data.text.encode_utf16().count() as f64),
    ))
    .max(clamp_index(
        state.selection_index,
        (source.data.text.encode_utf16().count() as f64),
    ));
}

// Source: upstream/packages/textinput/src/textInputEditing.ts:196 (sha256:056bda6bbd20a4f4697f235a8bf9c5052e05fa7b9c2411e5128778ac83f852ab)
pub fn get_text_input_selection_rectangles(
    out: &mut Vec<TextSelectionRectangle>,
    source: &RichText,
    layout: &TextLayoutResult,
) -> () {
    get_rich_text_selection_rectangles(
        out,
        get_text_input_selection_begin_index(source),
        get_text_input_selection_end_index(source),
        layout,
    );
}

// Source: upstream/packages/textinput/src/textInputEditing.ts:209 (sha256:e702e98a319e54cf229be449d6f3424d6f0a26495eb47a9b4e250379e7ac425e)
pub fn get_text_input_selection_text(source: &RichText) -> String {
    return __flight_string_slice(
        &((source.data.text).clone()),
        get_text_input_selection_begin_index(source),
        Some(get_text_input_selection_end_index(source)),
    );
}

// Source: upstream/packages/textinput/src/textInputEditing.ts:213 (sha256:48308fe26a2b823b466181b0a142b24b09bddfe03fb3d0a068694ef2147188f7)
pub fn handle_text_input_keyboard(
    source: &mut RichText,
    data: &KeyboardEventData,
    options: Option<HandleTextInputKeyboardOptions>,
) -> bool {
    let command = get_keyboard_command(data);
    if (command == "none") {
        return false;
    }
    {
        let __switch_value = command;
        let __flight_case = if __switch_value == "backspace" {
            0_usize
        } else if __switch_value == "copy" {
            1_usize
        } else if __switch_value == "cut" {
            2_usize
        } else if __switch_value == "delete" {
            3_usize
        } else if __switch_value == "deleteWordBackward" {
            4_usize
        } else if __switch_value == "deleteWordForward" {
            5_usize
        } else if __switch_value == "documentEnd" {
            6_usize
        } else if __switch_value == "documentStart" {
            7_usize
        } else if __switch_value == "down" {
            8_usize
        } else if __switch_value == "end" {
            9_usize
        } else if __switch_value == "home" {
            10_usize
        } else if __switch_value == "left" {
            11_usize
        } else if __switch_value == "paste" {
            12_usize
        } else if __switch_value == "return" {
            13_usize
        } else if __switch_value == "right" {
            14_usize
        } else if __switch_value == "selectAll" {
            15_usize
        } else if __switch_value == "up" {
            16_usize
        } else if __switch_value == "wordLeft" {
            17_usize
        } else if __switch_value == "wordRight" {
            18_usize
        } else {
            19_usize
        };
        '__flight_switch: {
            if __flight_case <= 0_usize {
                delete_text_input_backward(source);
                return true;
            }
            if __flight_case <= 1_usize {
                {
                    let copy_text = get_text_input_selection_text(source);
                    if ((copy_text.encode_utf16().count() as f64) > 0.0_f64) {
                        {
                            let __flight_callback =
                                options.as_ref().and_then(|value| (value.on_copy).clone());
                            __flight_callback
                                .as_ref()
                                .map(|callback| callback.lock().unwrap()((copy_text).clone()))
                        };
                    }
                    return true;
                }
            }
            if __flight_case <= 2_usize {
                {
                    let cut_text = get_text_input_selection_text(source);
                    if ((cut_text.encode_utf16().count() as f64) > 0.0_f64) {
                        {
                            {
                                let __flight_callback =
                                    options.as_ref().and_then(|value| (value.on_copy).clone());
                                __flight_callback
                                    .as_ref()
                                    .map(|callback| callback.lock().unwrap()((cut_text).clone()))
                            };
                            replace_selected_text_input(source, "".to_owned(), None);
                        }
                    }
                    return true;
                }
            }
            if __flight_case <= 3_usize {
                delete_text_input_forward(source);
                return true;
            }
            if __flight_case <= 4_usize {
                delete_text_input_word_backward(source);
                return true;
            }
            if __flight_case <= 5_usize {
                delete_text_input_word_forward(source);
                return true;
            }
            if __flight_case <= 6_usize {
                move_text_input_caret(
                    source,
                    (source.data.text.encode_utf16().count() as f64),
                    Some(data.shift_key),
                );
                return true;
            }
            if __flight_case <= 7_usize {
                move_text_input_caret(source, 0.0_f64, Some(data.shift_key));
                return true;
            }
            if __flight_case <= 8_usize {
                move_text_input_caret_down(
                    source,
                    &(options.as_ref().unwrap().layout),
                    Some(data.shift_key),
                );
                return true;
            }
            if __flight_case <= 9_usize {
                move_text_input_caret_to_line_end(
                    source,
                    &(options.as_ref().unwrap().layout),
                    Some(data.shift_key),
                );
                return true;
            }
            if __flight_case <= 10_usize {
                move_text_input_caret_to_line_start(
                    source,
                    &(options.as_ref().unwrap().layout),
                    Some(data.shift_key),
                );
                return true;
            }
            if __flight_case <= 11_usize {
                move_text_input_caret(
                    source,
                    (get_text_input_caret_index(source) - 1.0_f64),
                    Some(data.shift_key),
                );
                return true;
            }
            if __flight_case <= 12_usize {
                insert_text_input(
                    source,
                    (options
                        .as_ref()
                        .and_then(|value| (value.clipboard_text).clone()))
                    .clone()
                    .unwrap_or("".to_owned()),
                );
                return true;
            }
            if __flight_case <= 13_usize {
                if (!source.data.multiline) {
                    return false;
                }
                insert_text_input(source, "\n".to_owned());
                return true;
            }
            if __flight_case <= 14_usize {
                move_text_input_caret(
                    source,
                    (get_text_input_caret_index(source) + 1.0_f64),
                    Some(data.shift_key),
                );
                return true;
            }
            if __flight_case <= 15_usize {
                select_all_text_input(source);
                return true;
            }
            if __flight_case <= 16_usize {
                move_text_input_caret_up(
                    source,
                    &(options.as_ref().unwrap().layout),
                    Some(data.shift_key),
                );
                return true;
            }
            if __flight_case <= 17_usize {
                move_text_input_caret_by_word(source, (-1.0_f64), Some(data.shift_key));
                return true;
            }
            if __flight_case <= 18_usize {
                move_text_input_caret_by_word(source, 1.0_f64, Some(data.shift_key));
                return true;
            }
        }
    }
}

// Source: upstream/packages/textinput/src/textInputEditing.ts:290 (sha256:88cdbbf595a3131dbdf070e9075b980cd0b01b5bdcf2eb03c01ee4c92d4da133)
#[derive(Clone, Default)]
struct InsertTextInputRecord5 {
    __flight_identity: std::sync::Arc<()>,
    apply_input_rules: bool,
}
impl PartialEq for InsertTextInputRecord5 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn insert_text_input(source: &mut RichText, text: String) -> () {
    replace_selected_text_input(
        source,
        (text).clone(),
        Some(ReplaceTextInputOptions {
            __flight_identity: std::sync::Arc::new(()),
            apply_input_rules: Some(true),
            merge_kind: None,
            skip_history: None,
        }),
    );
}

// Source: upstream/packages/textinput/src/textInputEditing.ts:294 (sha256:88bf3add461c3f4d5a8be11cc63942ed1e5fc19ae9eec849509e97c5a4dd73a6)
pub fn move_text_input_caret(source: &RichText, index: f64, extend_selection: Option<bool>) -> () {
    let extend_selection = extend_selection.unwrap_or(false);
    let caret = clamp_index(index, (source.data.text.encode_utf16().count() as f64));
    let mut state = get_input_state(source);
    state.caret_index = caret;
    if (!extend_selection) {
        state.selection_index = caret;
    }
    state.desired_caret_x = DESIRED_CARET_X_UNSET;
    invalidate_node_appearance(source);
}

// Source: upstream/packages/textinput/src/textInputEditing.ts:307 (sha256:d43d7e9993b000f9edae9a1b198ed818d51bf38a9c9314ab7c42ce671e1bafce)
pub fn move_text_input_caret_by_word(
    source: &RichText,
    direction: f64,
    extend_selection: Option<bool>,
) -> () {
    let extend_selection = extend_selection.unwrap_or(false);
    let caret_index = get_text_input_caret_index(source);
    let text = (source.data.text).clone();
    let mut target: f64;
    if (direction < 0.0_f64) {
        target = find_word_start_before((text).clone(), caret_index);
    } else {
        target = find_word_end_after((text).clone(), caret_index);
    }
    move_text_input_caret(source, target, Some(extend_selection));
}

// Source: upstream/packages/textinput/src/textInputEditing.ts:322 (sha256:c4c887171789522899c4c567b4a05a684afceb74c4fef116c46a1d3f8329ccb5)
pub fn move_text_input_caret_down(
    source: &RichText,
    layout: &Option<TextLayoutResult>,
    extend_selection: Option<bool>,
) -> () {
    let extend_selection = extend_selection.unwrap_or(false);
    if ((layout).is_none()) || ((layout).is_none()) {
        move_text_input_caret(
            source,
            (source.data.text.encode_utf16().count() as f64),
            Some(extend_selection),
        );
        return;
    }
    let mut state = get_input_state(source);
    (|| -> () {
        let caret_index = get_text_input_caret_index(&source);
        let group = get_text_layout_group_at_index(&layout.as_ref().unwrap(), caret_index);
        if (group).is_none() {
            out.x = text_bounds_gutter_constant;
            out.y = text_bounds_gutter_constant;
            out.width = 1.0_f64;
            out.height = get_fallback_line_height(&layout.as_ref().unwrap());
            out.line_index = 0.0_f64;
            return;
        }
        out.x = get_text_layout_group_caret_x(&group.as_ref().unwrap(), caret_index);
        out.y = group.as_ref().unwrap().offset_y;
        out.width = 1.0_f64;
        out.height = group.as_ref().unwrap().height;
        out.line_index = group.as_ref().unwrap().line_index;
    })();
    if (state.desired_caret_x == DESIRED_CARET_X_UNSET) {
        state.desired_caret_x = (*SCRATCH_RECT.lock().unwrap()).x;
    }
    let target_line_index = ((*SCRATCH_RECT.lock().unwrap()).line_index + 1.0_f64);
    if (target_line_index >= layout.as_ref().unwrap().num_lines) {
        move_text_input_caret(
            source,
            (source.data.text.encode_utf16().count() as f64),
            Some(extend_selection),
        );
        return;
    }
    let target_y = (get_line_offset_y(layout.as_ref().unwrap(), target_line_index)
        + (layout.as_ref().unwrap().line_heights[target_line_index as usize].clone() / 2.0_f64));
    let target_index = get_text_input_character_index_at_point(
        source,
        layout.as_ref().unwrap(),
        state.desired_caret_x,
        target_y,
    );
    let new_caret = clamp_index(
        target_index,
        (source.data.text.encode_utf16().count() as f64),
    );
    state.caret_index = new_caret;
    if (!extend_selection) {
        state.selection_index = new_caret;
    }
    invalidate_node_appearance(source);
}

// Source: upstream/packages/textinput/src/textInputEditing.ts:351 (sha256:1509412c2465dcc2b45e54eff96328a2d2c8922551280dceb60f5199928774c1)
pub fn move_text_input_caret_to_line_end(
    source: &RichText,
    layout: &Option<TextLayoutResult>,
    extend_selection: Option<bool>,
) -> () {
    let extend_selection = extend_selection.unwrap_or(false);
    if ((layout).is_none()) || ((layout).is_none()) {
        move_text_input_caret(
            source,
            (source.data.text.encode_utf16().count() as f64),
            Some(extend_selection),
        );
        return;
    }
    let line_index = get_caret_line_index(source, layout.as_ref().unwrap());
    let line_end = get_line_end_index(
        layout.as_ref().unwrap(),
        line_index,
        (source.data.text.encode_utf16().count() as f64),
    );
    move_text_input_caret(source, line_end, Some(extend_selection));
}

// Source: upstream/packages/textinput/src/textInputEditing.ts:367 (sha256:f891c6a9baa7d8db48f8f7a6a3fa359b76e7532b3459d9c5a206c8304ab2e573)
pub fn move_text_input_caret_to_line_start(
    source: &RichText,
    layout: &Option<TextLayoutResult>,
    extend_selection: Option<bool>,
) -> () {
    let extend_selection = extend_selection.unwrap_or(false);
    if ((layout).is_none()) || ((layout).is_none()) {
        move_text_input_caret(source, 0.0_f64, Some(extend_selection));
        return;
    }
    let line_index = get_caret_line_index(source, layout.as_ref().unwrap());
    let line_start = get_line_start_index(layout.as_ref().unwrap(), line_index);
    move_text_input_caret(source, line_start, Some(extend_selection));
}

// Source: upstream/packages/textinput/src/textInputEditing.ts:384 (sha256:231d5d2e12edbb8073adeb938af7d421b54bf085c93920fb512a9d0c211fe4ea)
pub fn move_text_input_caret_up(
    source: &RichText,
    layout: &Option<TextLayoutResult>,
    extend_selection: Option<bool>,
) -> () {
    let extend_selection = extend_selection.unwrap_or(false);
    if ((layout).is_none()) || ((layout).is_none()) {
        move_text_input_caret(source, 0.0_f64, Some(extend_selection));
        return;
    }
    let mut state = get_input_state(source);
    (|| -> () {
        let caret_index = get_text_input_caret_index(&source);
        let group = get_text_layout_group_at_index(&layout.as_ref().unwrap(), caret_index);
        if (group).is_none() {
            out.x = text_bounds_gutter_constant;
            out.y = text_bounds_gutter_constant;
            out.width = 1.0_f64;
            out.height = get_fallback_line_height(&layout.as_ref().unwrap());
            out.line_index = 0.0_f64;
            return;
        }
        out.x = get_text_layout_group_caret_x(&group.as_ref().unwrap(), caret_index);
        out.y = group.as_ref().unwrap().offset_y;
        out.width = 1.0_f64;
        out.height = group.as_ref().unwrap().height;
        out.line_index = group.as_ref().unwrap().line_index;
    })();
    if (state.desired_caret_x == DESIRED_CARET_X_UNSET) {
        state.desired_caret_x = (*SCRATCH_RECT.lock().unwrap()).x;
    }
    let target_line_index = ((*SCRATCH_RECT.lock().unwrap()).line_index - 1.0_f64);
    if (target_line_index < 0.0_f64) {
        move_text_input_caret(source, 0.0_f64, Some(extend_selection));
        return;
    }
    let target_y = (get_line_offset_y(layout.as_ref().unwrap(), target_line_index)
        + (layout.as_ref().unwrap().line_heights[target_line_index as usize].clone() / 2.0_f64));
    let target_index = get_text_input_character_index_at_point(
        source,
        layout.as_ref().unwrap(),
        state.desired_caret_x,
        target_y,
    );
    let new_caret = clamp_index(
        target_index,
        (source.data.text.encode_utf16().count() as f64),
    );
    state.caret_index = new_caret;
    if (!extend_selection) {
        state.selection_index = new_caret;
    }
    invalidate_node_appearance(source);
}

// Source: upstream/packages/textinput/src/textInputEditing.ts:413 (sha256:dab606cc4915822a4db327af96edcf10983917cb1548c16faa1783d46511942d)
pub fn redo_text_input(source: &mut RichText) -> () {
    let mut state = get_input_state(source);
    if (!can_redo_text_input(source)) {
        return;
    }
    {
        state.history_index += 1.0;
        state.history_index
    };
    let record = state.history[state.history_index as usize].clone();
    apply_history_record(
        source,
        &mut state,
        (record.text_after).clone(),
        record.caret_index_after,
        record.selection_index_after,
    );
}

// Source: upstream/packages/textinput/src/textInputEditing.ts:421 (sha256:c0ef8d8deb35a2463049e212244310d297978816cdae0b83901a183768c8b862)
pub fn replace_selected_text_input(
    source: &mut RichText,
    text: String,
    options: Option<ReplaceTextInputOptions>,
) -> () {
    {
        let __flight_argument_1 = get_text_input_selection_begin_index(source);
        let __flight_argument_2 = get_text_input_selection_end_index(source);
        let __flight_result = replace_text_input(
            source,
            __flight_argument_1,
            __flight_argument_2,
            (text).clone(),
            ((options).clone()).clone(),
        );
        __flight_result
    };
}

// Source: upstream/packages/textinput/src/textInputEditing.ts:435 (sha256:002b130522137d99169e98e9e4013aeb5ac3f118aa7002ffe92dbb426354a182)
pub fn replace_text_input(
    source: &mut RichText,
    begin_index: f64,
    end_index: f64,
    text: String,
    options: Option<ReplaceTextInputOptions>,
) -> () {
    let mut start = clamp_index(
        begin_index,
        (source.data.text.encode_utf16().count() as f64),
    );
    let mut end = clamp_index(end_index, (source.data.text.encode_utf16().count() as f64));
    if (end < start) {
        let swap = start;
        start = end;
        end = swap;
    }
    let value = if (options.as_ref().and_then(|value| value.apply_input_rules)) == Some(true) {
        apply_text_input_restriction(source, (text).clone(), Some((end - start)))
    } else {
        (text).clone()
    };
    if ((value.encode_utf16().count() as f64) == 0.0_f64) && (start == end) {
        return;
    }
    let mut state = get_input_state(source);
    let text_before = (source.data.text).clone();
    let caret_before = clamp_index(
        state.caret_index,
        (text_before.encode_utf16().count() as f64),
    );
    let selection_before = clamp_index(
        state.selection_index,
        (text_before.encode_utf16().count() as f64),
    );
    source.data.text = format!(
        "{}{}",
        format!(
            "{}{}",
            __flight_string_slice(&(text_before), 0.0_f64, Some(start)),
            value
        ),
        __flight_string_slice(&(text_before), end, None)
    );
    {
        let __flight_argument_1 = {
            let __flight_portable_source = (source.data.default_text_format).clone();
            crate::FlightValue::Record({
                let mut __flight_record = Vec::new();
                if let Some(value) = (&__flight_portable_source).align.as_ref() {
                    __flight_record.push((
                        "align".to_owned(),
                        crate::FlightValue::String((value).clone()),
                    ));
                }
                if let Some(value) = (&__flight_portable_source).block_indent.as_ref() {
                    __flight_record.push((
                        "blockIndent".to_owned(),
                        crate::FlightValue::Number(*(value) as f64),
                    ));
                }
                if let Some(value) = (&__flight_portable_source).bold.as_ref() {
                    __flight_record.push(("bold".to_owned(), crate::FlightValue::Bool(*(value))));
                }
                if let Some(value) = (&__flight_portable_source).bullet.as_ref() {
                    __flight_record.push(("bullet".to_owned(), crate::FlightValue::Bool(*(value))));
                }
                if let Some(value) = (&__flight_portable_source).color.as_ref() {
                    __flight_record.push((
                        "color".to_owned(),
                        crate::FlightValue::Number(*(value) as f64),
                    ));
                }
                if let Some(value) = (&__flight_portable_source).font.as_ref() {
                    __flight_record.push((
                        "font".to_owned(),
                        crate::FlightValue::String((value).clone()),
                    ));
                }
                if let Some(value) = (&__flight_portable_source).indent.as_ref() {
                    __flight_record.push((
                        "indent".to_owned(),
                        crate::FlightValue::Number(*(value) as f64),
                    ));
                }
                if let Some(value) = (&__flight_portable_source).italic.as_ref() {
                    __flight_record.push(("italic".to_owned(), crate::FlightValue::Bool(*(value))));
                }
                if let Some(value) = (&__flight_portable_source).kerning.as_ref() {
                    __flight_record
                        .push(("kerning".to_owned(), crate::FlightValue::Bool(*(value))));
                }
                if let Some(value) = (&__flight_portable_source).leading.as_ref() {
                    __flight_record.push((
                        "leading".to_owned(),
                        crate::FlightValue::Number(*(value) as f64),
                    ));
                }
                if let Some(value) = (&__flight_portable_source).left_margin.as_ref() {
                    __flight_record.push((
                        "leftMargin".to_owned(),
                        crate::FlightValue::Number(*(value) as f64),
                    ));
                }
                if let Some(value) = (&__flight_portable_source).letter_spacing.as_ref() {
                    __flight_record.push((
                        "letterSpacing".to_owned(),
                        crate::FlightValue::Number(*(value) as f64),
                    ));
                }
                if let Some(value) = (&__flight_portable_source).list_marker.as_ref() {
                    __flight_record.push((
                        "listMarker".to_owned(),
                        crate::FlightValue::String((value).clone()),
                    ));
                }
                if let Some(value) = (&__flight_portable_source).right_margin.as_ref() {
                    __flight_record.push((
                        "rightMargin".to_owned(),
                        crate::FlightValue::Number(*(value) as f64),
                    ));
                }
                if let Some(value) = (&__flight_portable_source).size.as_ref() {
                    __flight_record.push((
                        "size".to_owned(),
                        crate::FlightValue::Number(*(value) as f64),
                    ));
                }
                if let Some(value) = (&__flight_portable_source).strikethrough.as_ref() {
                    __flight_record.push((
                        "strikethrough".to_owned(),
                        crate::FlightValue::Bool(*(value)),
                    ));
                }
                if let Some(value) = (&__flight_portable_source).tab_stops.as_ref() {
                    __flight_record.push((
                        "tabStops".to_owned(),
                        crate::FlightValue::Array(
                            (value)
                                .iter()
                                .map(|value| crate::FlightValue::Number(*(value) as f64))
                                .collect(),
                        ),
                    ));
                }
                if let Some(value) = (&__flight_portable_source).target.as_ref() {
                    __flight_record.push((
                        "target".to_owned(),
                        crate::FlightValue::String((value).clone()),
                    ));
                }
                if let Some(value) = (&__flight_portable_source).underline.as_ref() {
                    __flight_record
                        .push(("underline".to_owned(), crate::FlightValue::Bool(*(value))));
                }
                if let Some(value) = (&__flight_portable_source).url.as_ref() {
                    __flight_record.push((
                        "url".to_owned(),
                        crate::FlightValue::String((value).clone()),
                    ));
                }
                if let Some(value) = (&__flight_portable_source).variations.as_ref() {
                    __flight_record.push((
                        "variations".to_owned(),
                        crate::FlightValue::Array(
                            (value)
                                .iter()
                                .map(|value| {
                                    crate::FlightValue::Record({
                                        let mut __flight_record = Vec::new();
                                        __flight_record.push((
                                            "axis".to_owned(),
                                            crate::FlightValue::String((&((value).axis)).clone()),
                                        ));
                                        __flight_record.push((
                                            "value".to_owned(),
                                            crate::FlightValue::Number(*(&((value).value)) as f64),
                                        ));
                                        __flight_record
                                    })
                                })
                                .collect(),
                        ),
                    ));
                }
                __flight_record
            })
        };
        let __flight_result = adjust_text_format_ranges(
            &mut source.data.text_format_ranges,
            __flight_argument_1,
            start,
            end,
            (value.encode_utf16().count() as f64),
        );
        __flight_result
    };
    state.desired_caret_x = DESIRED_CARET_X_UNSET;
    set_text_input_selection(
        source,
        (start + (value.encode_utf16().count() as f64)),
        (start + (value.encode_utf16().count() as f64)),
    );
    if (!((options.as_ref().and_then(|value| value.skip_history)) == Some(true)))
        && (state.history_limit > 0.0_f64)
    {
        record_text_input_edit(
            &mut state,
            (text_before).clone(),
            (source.data.text).clone(),
            caret_before,
            selection_before,
            (options
                .as_ref()
                .and_then(|value| (value.merge_kind).clone()))
            .clone(),
        );
    }
    invalidate_node_appearance(source);
}

// Source: upstream/packages/textinput/src/textInputEditing.ts:476 (sha256:cca2ef0f83eee72cca02f10736892f7f070bd25a7c7fcbf0765981f222bad398)
pub fn scroll_text_input_caret_into_view(
    source: &mut RichText,
    layout: &TextLayoutResult,
    viewport_width: f64,
    viewport_height: f64,
) -> () {
    (|| -> () {
        let caret_index = get_text_input_caret_index(&source);
        let group = get_text_layout_group_at_index(&layout, caret_index);
        if (group).is_none() {
            out.x = text_bounds_gutter_constant;
            out.y = text_bounds_gutter_constant;
            out.width = 1.0_f64;
            out.height = get_fallback_line_height(&layout);
            out.line_index = 0.0_f64;
            return;
        }
        out.x = get_text_layout_group_caret_x(&group.as_ref().unwrap(), caret_index);
        out.y = group.as_ref().unwrap().offset_y;
        out.width = 1.0_f64;
        out.height = group.as_ref().unwrap().height;
        out.line_index = group.as_ref().unwrap().line_index;
    })();
    let caret_top = (*SCRATCH_RECT.lock().unwrap()).y;
    let caret_bottom = ((*SCRATCH_RECT.lock().unwrap()).y + (*SCRATCH_RECT.lock().unwrap()).height);
    let scroll_v_line = (source.data.scroll_v - 1.0_f64);
    let mut view_top = 0.0_f64;
    {
        let mut i = 0.0_f64;
        while (i < scroll_v_line) {
            view_top += layout.line_heights[i as usize].clone();
            {
                i += 1.0;
                i
            };
        }
    }
    let view_bottom = (view_top + viewport_height);
    if (caret_top < view_top) {
        set_rich_text_scroll_v(
            source,
            ((*SCRATCH_RECT.lock().unwrap()).line_index + 1.0_f64),
            Some(((*layout).clone()).clone()),
        );
    } else {
        if (caret_bottom > view_bottom) {
            let mut pixel_offset = 0.0_f64;
            let mut first_visible_line = 0.0_f64;
            {
                let mut i = 0.0_f64;
                while (i < layout.num_lines) {
                    if ((pixel_offset + layout.line_heights[i as usize].clone())
                        > (caret_bottom - viewport_height))
                    {
                        first_visible_line = i;
                        break;
                    }
                    pixel_offset += layout.line_heights[i as usize].clone();
                    {
                        i += 1.0;
                        i
                    };
                }
            }
            set_rich_text_scroll_v(
                source,
                (first_visible_line + 1.0_f64),
                Some(((*layout).clone()).clone()),
            );
        }
    }
    let caret_scroll_margin = 8.0_f64;
    let scroll_h = source.data.scroll_h;
    let caret_left = ((*SCRATCH_RECT.lock().unwrap()).x - scroll_h);
    let caret_right = (caret_left + (*SCRATCH_RECT.lock().unwrap()).width);
    if (caret_left < 0.0_f64) {
        set_rich_text_scroll_h(
            source,
            (0.0_f64).max(((*SCRATCH_RECT.lock().unwrap()).x - caret_scroll_margin)),
            Some(((*layout).clone()).clone()),
        );
    } else {
        if ((caret_right + caret_scroll_margin) > viewport_width) {
            set_rich_text_scroll_h(
                source,
                ((((*SCRATCH_RECT.lock().unwrap()).x + (*SCRATCH_RECT.lock().unwrap()).width)
                    + caret_scroll_margin)
                    - viewport_width),
                Some(((*layout).clone()).clone()),
            );
        }
    }
}

// Source: upstream/packages/textinput/src/textInputEditing.ts:523 (sha256:a94512389b36f72031c31c9c2261138fca7672614a4cbd035d7d4b7351110c98)
pub fn select_all_text_input(source: &RichText) -> () {
    set_text_input_selection(
        source,
        0.0_f64,
        (source.data.text.encode_utf16().count() as f64),
    );
}

// Source: upstream/packages/textinput/src/textInputEditing.ts:527 (sha256:f1ec87a198f30c2104bbbcd69dc33c92c3d93487931e103f7ecbc1aedef3895e)
pub fn select_line_at_text_input_index(source: &RichText, index: f64) -> () {
    let text = (source.data.text).clone();
    let clamped = (0.0_f64).max((text.encode_utf16().count() as f64).min(index));
    let mut start = clamped;
    let mut end = clamped;
    while (start > 0.0_f64) && ((text.char_at)((start - 1.0_f64)) != "\n") {
        {
            start -= 1.0;
            start
        };
    }
    while (end < (text.encode_utf16().count() as f64)) && ((text.char_at)(end) != "\n") {
        {
            end += 1.0;
            end
        };
    }
    set_text_input_selection(source, start, end);
}

// Source: upstream/packages/textinput/src/textInputEditing.ts:537 (sha256:a2d982c0f9f31526ed53ad92848dff3fe8e5cc10e8367d3b0d60d3323f3fc3ef)
pub fn select_word_at_text_input_index(source: &RichText, index: f64) -> () {
    let text = (source.data.text).clone();
    let clamped = (0.0_f64).max((text.encode_utf16().count() as f64).min(index));
    let mut start = clamped;
    let mut end = clamped;
    while (start > 0.0_f64) && (is_word_char((text.char_at)((start - 1.0_f64)))) {
        {
            start -= 1.0;
            start
        };
    }
    while (end < (text.encode_utf16().count() as f64)) && (is_word_char((text.char_at)(end))) {
        {
            end += 1.0;
            end
        };
    }
    if (start == end) {
        while (start > 0.0_f64) && (!is_word_char((text.char_at)((start - 1.0_f64)))) {
            {
                start -= 1.0;
                start
            };
        }
        while (end < (text.encode_utf16().count() as f64)) && (!is_word_char((text.char_at)(end))) {
            {
                end += 1.0;
                end
            };
        }
    }
    set_text_input_selection(source, start, end);
}

// Source: upstream/packages/textinput/src/textInputEditing.ts:551 (sha256:b558229e1bb188cad26e6a65e43c9f813fa6ce65f99100f53e0f698a36fd9593)
pub fn set_text_input_selection(source: &RichText, begin_index: f64, end_index: f64) -> () {
    let mut state = get_input_state(source);
    state.selection_index = clamp_index(
        begin_index,
        (source.data.text.encode_utf16().count() as f64),
    );
    state.caret_index = clamp_index(end_index, (source.data.text.encode_utf16().count() as f64));
    invalidate_node_appearance(source);
}

// Source: upstream/packages/textinput/src/textInputEditing.ts:560 (sha256:d3ebe80a72c277594e0e8700a1f8a362245bac81edcebf416f37eab6adeb7528)
pub fn undo_text_input(source: &mut RichText) -> () {
    let mut state = get_input_state(source);
    if (!can_undo_text_input(source)) {
        return;
    }
    let record = state.history[state.history_index as usize].clone();
    {
        state.history_index -= 1.0;
        state.history_index
    };
    apply_history_record(
        source,
        &mut state,
        (record.text_before).clone(),
        record.caret_index_before,
        record.selection_index_before,
    );
}

// Source: upstream/packages/textinput/src/textInputEditing.ts:568 (sha256:0d449368501740a90589be55056512dba6699118161cc4a2d70c1fb9c53f6a88)
fn adjust_text_format_ranges(
    ranges: &mut Vec<TextFormatRange>,
    default_format: crate::OpaqueHostValue,
    begin_index: f64,
    end_index: f64,
    insert_length: f64,
) -> () {
    let remove_length = (end_index - begin_index);
    let offset = (insert_length - remove_length);
    {
        let mut i = 0.0_f64;
        while (i < (ranges.len() as f64)) {
            let mut range = ranges[i as usize].clone();
            if (begin_index == end_index) {
                if (range.end < begin_index) {
                    {
                        i += 1.0;
                        i
                    };
                    continue;
                } else {
                    if (range.start >= begin_index) {
                        range.start += offset;
                        range.end += offset;
                    } else {
                        if (range.start < begin_index) && (range.end >= begin_index) {
                            range.end += offset;
                        }
                    }
                }
            } else {
                if (range.end <= begin_index) {
                    {
                        i += 1.0;
                        i
                    };
                    continue;
                } else {
                    if (range.start > end_index) {
                        range.start += offset;
                        range.end += offset;
                    } else {
                        if (range.start <= begin_index) && (range.end > end_index) {
                            range.end += offset;
                        } else {
                            if (range.start >= begin_index) && (range.end <= end_index) {
                                {
                                    let __flight_start = ({
                                        i -= 1.0;
                                        i
                                    });
                                    let __flight_count = (1.0_f64);
                                    ranges
                                        .splice(
                                            (__flight_start) as usize
                                                ..(__flight_start + __flight_count) as usize,
                                            vec![],
                                        )
                                        .collect::<Vec<_>>()
                                };
                            } else {
                                if ((range.end > end_index) && (range.start > begin_index))
                                    && (range.start <= end_index)
                                {
                                    range.start = begin_index;
                                    range.end += offset;
                                } else {
                                    if ((range.start < begin_index) && (range.end > begin_index))
                                        && (range.end <= end_index)
                                    {
                                        range.end = begin_index;
                                    }
                                }
                            }
                        }
                    }
                }
            }
            {
                i += 1.0;
                i
            };
        }
    }
    {
        let mut i = ((ranges.len() as f64) - 1.0_f64);
        while (i >= 0.0_f64) {
            if (ranges[i as usize].start >= ranges[i as usize].end) {
                {
                    let __flight_start = (i);
                    let __flight_count = (1.0_f64);
                    ranges
                        .splice(
                            (__flight_start) as usize..(__flight_start + __flight_count) as usize,
                            vec![],
                        )
                        .collect::<Vec<_>>()
                };
            }
            {
                i -= 1.0;
                i
            };
        }
    }
    if ((ranges.len() as f64) == 0.0_f64) && (insert_length > 0.0_f64) {
        ranges.push(TextFormatRange {
            __flight_identity: std::sync::Arc::new(()),
            end: (begin_index + insert_length),
            format: (default_format).clone(),
            start: begin_index,
        });
    }
}

// Source: upstream/packages/textinput/src/textInputEditing.ts:618 (sha256:297a701669f30ba2dc47d492cac1e006a761cf5fe12aa441f67fbd5dda53f69d)
fn apply_history_record(
    source: &mut RichText,
    state: &mut TextInputState,
    text: String,
    caret_index: f64,
    selection_index: f64,
) -> () {
    let __flight_utf16_text: std::sync::Arc<Vec<u16>> =
        std::sync::Arc::new(text.encode_utf16().collect());
    source.data.text = (text).clone();
    state.caret_index = clamp_index(caret_index, (__flight_utf16_text.len() as f64));
    state.selection_index = clamp_index(selection_index, (__flight_utf16_text.len() as f64));
    state.desired_caret_x = DESIRED_CARET_X_UNSET;
    invalidate_node_appearance(source);
}

// Source: upstream/packages/textinput/src/textInputEditing.ts:632 (sha256:0b764cf844894884927d1f7b88e07a85e0b069b7be78938f86370d48c1938486)
fn clamp_index(value: f64, length: f64) -> f64 {
    if (!(value).is_finite()) {
        return 0.0_f64;
    }
    return (0.0_f64).max((length).min((value).trunc()));
}

// Source: upstream/packages/textinput/src/textInputEditing.ts:639 (sha256:0b4306825609746f5ef1e7e8ef8a03d4a13bd52bb3da6edbbc21ca7096c4c02a)
fn get_caret_line_index(source: &RichText, layout: &TextLayoutResult) -> f64 {
    (|| -> () {
        let caret_index = get_text_input_caret_index(&source);
        let group = get_text_layout_group_at_index(&layout, caret_index);
        if (group).is_none() {
            out.x = text_bounds_gutter_constant;
            out.y = text_bounds_gutter_constant;
            out.width = 1.0_f64;
            out.height = get_fallback_line_height(&layout);
            out.line_index = 0.0_f64;
            return;
        }
        out.x = get_text_layout_group_caret_x(&group.as_ref().unwrap(), caret_index);
        out.y = group.as_ref().unwrap().offset_y;
        out.width = 1.0_f64;
        out.height = group.as_ref().unwrap().height;
        out.line_index = group.as_ref().unwrap().line_index;
    })();
    return (*SCRATCH_RECT.lock().unwrap()).line_index;
}

// Source: upstream/packages/textinput/src/textInputEditing.ts:645 (sha256:eb161c6f6ab166b1769dc6be2bde0ad05e6adcd63bd8a4ee1d7b41cba66e2310)
fn get_fallback_line_height(layout: &TextLayoutResult) -> f64 {
    return layout.line_heights[0.0_f64 as usize].clone();
}

// Source: upstream/packages/textinput/src/textInputEditing.ts:651 (sha256:65c34441612ab225520e48a928b9ce2e0c5f35afc2dafb0a88e06cc0546d6ae5)
fn get_input_state(source: &RichText) -> TextInputState {
    let state = get_text_input_state(source);
    if (state).is_none() {
        panic!(
            "{}",
            "text input is not enabled on this RichText; call enableTextInput first"
        );
    }
    return ((state.as_ref().unwrap()).clone()).clone();
}

// Source: upstream/packages/textinput/src/textInputEditing.ts:657 (sha256:b3ebba0e62bbc63048faf9a23e621cc28707b9164f0c9fe2ce6071ffc677d87d)
fn get_keyboard_command(data: &KeyboardEventData) -> KeyboardCommand {
    if (data.ctrl_key) || (data.meta_key) {
        let key = ((data.key).clone()).to_lowercase();
        if (key == "a") || (data.key_code == KeyCode::A) {
            return "selectAll".to_owned();
        }
        if (key == "c") || (data.key_code == KeyCode::C) {
            return "copy".to_owned();
        }
        if (key == "v") || (data.key_code == KeyCode::V) {
            return "paste".to_owned();
        }
        if (key == "x") || (data.key_code == KeyCode::X) {
            return "cut".to_owned();
        }
        if (data.key_code == KeyCode::LEFT) || ((data.key).clone() == "ArrowLeft") {
            return "wordLeft".to_owned();
        }
        if (data.key_code == KeyCode::RIGHT) || ((data.key).clone() == "ArrowRight") {
            return "wordRight".to_owned();
        }
        if (data.key_code == KeyCode::BACKSPACE) || ((data.key).clone() == "Backspace") {
            return "deleteWordBackward".to_owned();
        }
        if (data.key_code == KeyCode::DELETE) || ((data.key).clone() == "Delete") {
            return "deleteWordForward".to_owned();
        }
        if (data.key_code == KeyCode::HOME) || ((data.key).clone() == "Home") {
            return "documentStart".to_owned();
        }
        if (data.key_code == KeyCode::END) || ((data.key).clone() == "End") {
            return "documentEnd".to_owned();
        }
        return "none".to_owned();
    }
    if data.alt_key {
        if (data.key_code == KeyCode::LEFT) || ((data.key).clone() == "ArrowLeft") {
            return "wordLeft".to_owned();
        }
        if (data.key_code == KeyCode::RIGHT) || ((data.key).clone() == "ArrowRight") {
            return "wordRight".to_owned();
        }
        if (data.key_code == KeyCode::BACKSPACE) || ((data.key).clone() == "Backspace") {
            return "deleteWordBackward".to_owned();
        }
        if (data.key_code == KeyCode::DELETE) || ((data.key).clone() == "Delete") {
            return "deleteWordForward".to_owned();
        }
    }
    if (data.key_code == KeyCode::BACKSPACE) || ((data.key).clone() == "Backspace") {
        return "backspace".to_owned();
    }
    if (data.key_code == KeyCode::DELETE) || ((data.key).clone() == "Delete") {
        return "delete".to_owned();
    }
    if (data.key_code == KeyCode::DOWN) || ((data.key).clone() == "ArrowDown") {
        return "down".to_owned();
    }
    if (data.key_code == KeyCode::END) || ((data.key).clone() == "End") {
        return "end".to_owned();
    }
    if (data.key_code == KeyCode::HOME) || ((data.key).clone() == "Home") {
        return "home".to_owned();
    }
    if (data.key_code == KeyCode::LEFT) || ((data.key).clone() == "ArrowLeft") {
        return "left".to_owned();
    }
    if (data.key_code == KeyCode::RETURN) || ((data.key).clone() == "Enter") {
        return "return".to_owned();
    }
    if (data.key_code == KeyCode::RIGHT) || ((data.key).clone() == "ArrowRight") {
        return "right".to_owned();
    }
    if (data.key_code == KeyCode::UP) || ((data.key).clone() == "ArrowUp") {
        return "up".to_owned();
    }
    return "none".to_owned();
}

// Source: upstream/packages/textinput/src/textInputEditing.ts:696 (sha256:57109af3ee76c15113f0c33ac98572d8f9b9fa9e82daeed8a761ccc5cd299641)
fn get_line_end_index(layout: &TextLayoutResult, line_index: f64, text_length: f64) -> f64 {
    let mut end = (-1.0_f64);
    for group in ((layout.groups).clone()).iter().cloned() {
        if (group.line_index == line_index) && (group.end_index > end) {
            end = group.end_index;
        }
    }
    return if (end < 0.0_f64) { text_length } else { end };
}

// Source: upstream/packages/textinput/src/textInputEditing.ts:704 (sha256:4933f618f6def32d3cde1fe3656ca3a04b68d4c74aa48f0cc4e82682731353e9)
fn get_line_offset_y(layout: &TextLayoutResult, line_index: f64) -> f64 {
    for group in ((layout.groups).clone()).iter().cloned() {
        if (group.line_index == line_index) {
            return group.offset_y;
        }
    }
    let mut y = text_bounds_gutter_constant;
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

// Source: upstream/packages/textinput/src/textInputEditing.ts:715 (sha256:b060e975bd7d4c2e1715db4f89fd5097010365cd554749ec6ff7e67ba06db378)
fn get_line_start_index(layout: &TextLayoutResult, line_index: f64) -> f64 {
    let mut start = (-1.0_f64);
    for group in ((layout.groups).clone()).iter().cloned() {
        if (group.line_index == line_index) && ((start < 0.0_f64) || (group.start_index < start)) {
            start = group.start_index;
        }
    }
    return if (start < 0.0_f64) { 0.0_f64 } else { start };
}

// Source: upstream/packages/textinput/src/textInputEditing.ts:723 (sha256:cf395338835ed4f057b8d31a23cf6651624f35254b9c443ce62ba87c184c4282)
fn get_text_layout_group_at_index(
    layout: &TextLayoutResult,
    index: f64,
) -> Option<TextLayoutGroup> {
    for group in ((layout.groups).clone()).iter().cloned() {
        if (index >= group.start_index) && (index <= group.end_index) {
            return Some((group).clone());
        }
    }
    return Some(layout.groups[((layout.groups.len() as f64) - 1.0_f64) as usize].clone());
}

// Source: upstream/packages/textinput/src/textInputEditing.ts:733 (sha256:f10829642510b105e7a6afe4e4ffcc19b3938cbf88343db7486babef07b94dca)
fn get_text_layout_group_caret_x(group: &TextLayoutGroup, index: f64) -> f64 {
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

// Source: upstream/packages/textinput/src/textInputEditing.ts:740 (sha256:f2a2b80414b786299703b32d7520f092c7e5384ef4f64b0348950bf457362e97)
fn get_text_layout_group_character_index_at_x(group: &TextLayoutGroup, x: f64) -> f64 {
    let mut current_x = group.offset_x;
    {
        let mut i = 0.0_f64;
        while (i < (group.positions.len() as f64)) {
            let advance = group.positions[i as usize].clone();
            if (x < (current_x + (advance / 2.0_f64))) {
                return (group.start_index + i);
            }
            current_x += advance;
            {
                i += 1.0;
                i
            };
        }
    }
    return group.end_index;
}

// Source: upstream/packages/textinput/src/textInputEditing.ts:754 (sha256:4006c76eb119a6ed41ae4503a80c0e49a1d26700b27b30181938f01015bb6863)
fn record_text_input_edit(
    state: &mut TextInputState,
    text_before: String,
    text_after: String,
    caret_index_before: f64,
    selection_index_before: f64,
    merge_kind: Option<String>,
) -> () {
    if (state.history_index < ((state.history.len() as f64) - 1.0_f64)) {
        state
            .history
            .truncate((state.history_index + 1.0_f64) as usize);
    }
    let mut previous = if (state.history_index >= 0.0_f64) {
        Some(state.history[state.history_index as usize].clone())
    } else {
        None
    };
    if (((previous).is_some()) && ((merge_kind).is_some()))
        && ((previous.as_mut().unwrap().merge_kind).clone() == merge_kind)
    {
        previous.as_mut().unwrap().text_after = (text_after).clone();
        previous.as_mut().unwrap().caret_index_after = state.caret_index;
        previous.as_mut().unwrap().selection_index_after = state.selection_index;
        return;
    }
    state.history.push(TextInputHistoryEntry {
        __flight_identity: std::sync::Arc::new(()),
        caret_index_after: state.caret_index,
        caret_index_before: caret_index_before,
        merge_kind: (merge_kind).clone(),
        selection_index_after: state.selection_index,
        selection_index_before: selection_index_before,
        text_after: (text_after).clone(),
        text_before: (text_before).clone(),
    });
    state.history_index = ((state.history.len() as f64) - 1.0_f64);
    if ((state.history.len() as f64) > state.history_limit) {
        let overflow = ((state.history.len() as f64) - state.history_limit);
        {
            let __flight_start = (0.0_f64);
            let __flight_count = (overflow);
            state
                .history
                .splice(
                    (__flight_start) as usize..(__flight_start + __flight_count) as usize,
                    vec![],
                )
                .collect::<Vec<_>>()
        };
        state.history_index = ((state.history.len() as f64) - 1.0_f64);
    }
}

// Source: upstream/packages/textinput/src/textInputEditing.ts:794 (sha256:3f0b23bb62c98647447578fd8f6e1a9c3b2d87f69c2e9aeb2b77405cb829cd5a)
fn restrict_text_input(text: String, restrict: String) -> String {
    let __flight_utf16_text: std::sync::Arc<Vec<u16>> =
        std::sync::Arc::new(text.encode_utf16().collect());
    let __flight_utf16_restrict: std::sync::Arc<Vec<u16>> =
        std::sync::Arc::new(restrict.encode_utf16().collect());
    if ((__flight_utf16_restrict.len() as f64) == 0.0_f64)
        || ((__flight_utf16_text.len() as f64) == 0.0_f64)
    {
        return text;
    }
    let __destructure0 = split_restrict_ranges((restrict).clone());
    let accepted = (__destructure0.accepted).clone();
    let declined = (__destructure0.declined).clone();
    let mut out = "".to_owned();
    for char in (text).iter().cloned() {
        let accepted_match =
            ((accepted).clone() == "") || (matches_restrict_ranges(char, (accepted).clone()));
        let declined_match =
            ((declined).clone() != "") && (matches_restrict_ranges(char, (declined).clone()));
        if (accepted_match) && (!declined_match) {
            out.push_str(&(char));
        }
    }
    return out;
}

// Source: upstream/packages/textinput/src/textInputEditing.ts:807 (sha256:5ec18eb9e2021e2b4c9df750ff90a2c0a31b4f64ed62f5e82484dd1303753774)
fn matches_restrict_ranges(char: String, ranges: String) -> bool {
    let __flight_utf16_char: std::sync::Arc<Vec<u16>> =
        std::sync::Arc::new(char.encode_utf16().collect());
    let __flight_utf16_ranges: std::sync::Arc<Vec<u16>> =
        std::sync::Arc::new(ranges.encode_utf16().collect());
    {
        let mut i = 0.0_f64;
        while (i < (__flight_utf16_ranges.len() as f64)) {
            let current = (ranges.char_at)(i);
            if (current == "\\") && ((i + 1.0_f64) < (__flight_utf16_ranges.len() as f64)) {
                if (char == (ranges.char_at)((i + 1.0_f64))) {
                    return true;
                }
                {
                    i += 1.0;
                    i
                };
            } else {
                if ((i + 2.0_f64) < (__flight_utf16_ranges.len() as f64))
                    && ((ranges.char_at)((i + 1.0_f64)) == "-")
                {
                    let end = (ranges.char_at)((i + 2.0_f64));
                    let code = {
                        let __flight_units: &[u16] = &__flight_utf16_char;
                        let __flight_raw_index = 0.0_f64;
                        let __flight_index = if __flight_raw_index.is_nan() {
                            0_i64
                        } else if __flight_raw_index.is_finite() {
                            __flight_raw_index.trunc() as i64
                        } else {
                            -1_i64
                        };
                        if __flight_index < 0 {
                            f64::NAN
                        } else {
                            __flight_units
                                .get(__flight_index as usize)
                                .map_or(f64::NAN, |unit| f64::from(*unit))
                        }
                    };
                    if (code >= (current.char_code_at)(0.0_f64))
                        && (code <= (end.char_code_at)(0.0_f64))
                    {
                        return true;
                    }
                    i += 2.0_f64;
                } else {
                    if (char == current) {
                        return true;
                    }
                }
            }
            {
                i += 1.0;
                i
            };
        }
    }
    return false;
}

// Source: upstream/packages/textinput/src/textInputEditing.ts:825 (sha256:d2a615ef67c355856c20d433446309bf66a747a0bc9b5c1c7c6a46c0a0227ee1)
fn split_restrict_ranges(restrict: String) -> SharedStructuralRecord1 {
    let __flight_utf16_restrict: std::sync::Arc<Vec<u16>> =
        std::sync::Arc::new(restrict.encode_utf16().collect());
    let mut accepted = "".to_owned();
    let mut declined = "".to_owned();
    let mut declining = false;
    {
        let mut i = 0.0_f64;
        while (i < (__flight_utf16_restrict.len() as f64)) {
            let char = (restrict.char_at)(i);
            if (char == "\\") && ((i + 1.0_f64) < (__flight_utf16_restrict.len() as f64)) {
                let escaped = (char + (restrict.char_at)((i + 1.0_f64)));
                if declining {
                    declined.push_str(&(escaped));
                } else {
                    accepted.push_str(&(escaped));
                }
                {
                    i += 1.0;
                    i
                };
            } else {
                if (char == "^") {
                    declining = (!declining);
                } else {
                    if declining {
                        declined.push_str(&(char));
                    } else {
                        accepted.push_str(&(char));
                    }
                }
            }
            {
                i += 1.0;
                i
            };
        }
    }
    return SharedStructuralRecord1 {
        __flight_identity: std::sync::Arc::new(()),
        accepted: (accepted).clone(),
        declined: (declined).clone(),
    };
}

// Source: upstream/packages/textinput/src/textInputEditing.ts:849 (sha256:720f2b25f91ba85604ad37408e8482ab435fcd5b0ce3d9ddaf5762c9110b59ec)
pub(crate) type KeyboardCommand = String;

// Source: upstream/packages/textinput/src/textInputEditing.ts:874 (sha256:050b27a35b02df39a3fc90980f133783be4f9664f3b949d4b83db163e23e8b91)
fn find_word_start_before(text: String, index: f64) -> f64 {
    let mut i = index;
    while (i > 0.0_f64) && (!is_word_char((text.char_at)((i - 1.0_f64)))) {
        {
            i -= 1.0;
            i
        };
    }
    while (i > 0.0_f64) && (is_word_char((text.char_at)((i - 1.0_f64)))) {
        {
            i -= 1.0;
            i
        };
    }
    return i;
}

// Source: upstream/packages/textinput/src/textInputEditing.ts:883 (sha256:de59b7e56679d4df27ddc24ed1632145cb5e86dc12179b53206cc627e13b832e)
fn find_word_end_after(text: String, index: f64) -> f64 {
    let __flight_utf16_text: std::sync::Arc<Vec<u16>> =
        std::sync::Arc::new(text.encode_utf16().collect());
    let mut i = index;
    while (i < (__flight_utf16_text.len() as f64)) && (!is_word_char((text.char_at)(i))) {
        {
            i += 1.0;
            i
        };
    }
    while (i < (__flight_utf16_text.len() as f64)) && (is_word_char((text.char_at)(i))) {
        {
            i += 1.0;
            i
        };
    }
    return i;
}

// Source: upstream/packages/textinput/src/textInputEditing.ts:890 (sha256:fde0c21474c9deb12a3b07d9f774d8db35e0d78b44b745eccd6c2cccba313dcf)
fn is_word_char(char: String) -> bool {
    return (regex::RegexBuilder::new("\\w")
        .case_insensitive(false)
        .multi_line(false)
        .dot_matches_new_line(false)
        .build()
        .expect("upstream TypeScript regular expression must be valid Rust regex syntax"))
    .is_match(&(char));
}

// Source: upstream/packages/textinput/src/textInputEditing.ts:897 (sha256:7351f720a26190716f486bc006c1865a3cf502a56fdb176ccb12b37771be5e07)
#[derive(Clone, Default)]
pub(crate) struct ScratchRect {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub height: f64,
    pub line_index: f64,
    pub width: f64,
    pub x: f64,
    pub y: f64,
}
impl PartialEq for ScratchRect {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

static SCRATCH_RECT: std::sync::LazyLock<std::sync::Mutex<ScratchRect>> =
    std::sync::LazyLock::new(|| {
        std::sync::Mutex::new(ScratchRect {
            __flight_identity: std::sync::Arc::new(()),
            height: 0.0_f64,
            line_index: 0.0_f64,
            width: 0.0_f64,
            x: 0.0_f64,
            y: 0.0_f64,
        })
    });
