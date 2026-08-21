// @generated from upstream/packages/textinput/src/textInputManager.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{
    get_text_input_character_index_at_point, get_text_input_state, handle_text_input_keyboard,
    insert_text_input, move_text_input_caret, select_line_at_text_input_index,
    select_word_at_text_input_index,
};
use flighthq_signals::{connect_signal, disconnect_signal};
use flighthq_text::{get_rich_text_runtime, set_rich_text_scroll_v};
use flighthq_types::{
    HandleTextInputKeyboardOptions, InputKeyboardData, InputTextData, KeyboardEventData, RichText,
    TextInputManager, TextInputSource,
};

// Source: upstream/packages/textinput/src/textInputManager.ts:21 (sha256:9ab5cbd2cf4707069b7499d4fb910172de2721b5d2aca8db6fa8fe4d8808139f)
pub fn blur_text_input(manager: &mut TextInputManager) -> () {
    let target = (manager.focused).clone();
    if (target).is_some() {
        set_text_input_focused(&target.as_ref().unwrap(), false);
    }
    manager.focused = None;
}

// Source: upstream/packages/textinput/src/textInputManager.ts:27 (sha256:9b98672c105dc99b74504105b9e91c0445ddfc70734a9e97f851fa0546490b4e)
pub fn connect_input_to_text_input(
    mut input: TextInputSource,
    manager: TextInputManager,
) -> std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>> {
    let mut on_key_down: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut(InputKeyboardData) -> bool + Send + 'static>>,
    > = std::sync::Arc::new(std::sync::Mutex::new(Box::new({
        let manager = manager.clone();
        move |data: InputKeyboardData| -> bool {
            dispatch_text_input_key_down(&manager, &data, None, None)
        }
    })
        as Box<dyn FnMut(InputKeyboardData) -> bool + Send + 'static>));
    let mut on_text_input: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut(InputTextData) -> bool + Send + 'static>>,
    > = std::sync::Arc::new(std::sync::Mutex::new(Box::new({
        let manager = manager.clone();
        move |mut data: InputTextData| -> bool {
            dispatch_text_input(&manager, (data.text).clone())
        }
    })
        as Box<dyn FnMut(InputTextData) -> bool + Send + 'static>));
    connect_signal(&mut input.on_key_down, (on_key_down).clone(), None);
    connect_signal(&mut input.on_text_input, (on_text_input).clone(), None);
    return std::sync::Arc::new(std::sync::Mutex::new(Box::new({
        let mut input = input.clone();
        let on_key_down = on_key_down.clone();
        let on_text_input = on_text_input.clone();
        move || -> () {
            disconnect_signal(&mut input.on_key_down, (on_key_down).clone());
            disconnect_signal(&mut input.on_text_input, (on_text_input).clone());
        }
    })
        as Box<dyn FnMut() -> () + Send + 'static>));
}

// Source: upstream/packages/textinput/src/textInputManager.ts:40 (sha256:e9323686b2e65106cc0e08e36a88cf34d90cb0b9da2bc6b5da86c3a9ad49ed2e)
pub fn create_text_input_manager() -> TextInputManager {
    return TextInputManager {
        __flight_identity: std::sync::Arc::new(()),
        enabled: true,
        focused: None,
    };
}

// Source: upstream/packages/textinput/src/textInputManager.ts:47 (sha256:71866e07667c1624b5639beadcaa833b2deb09241a9cf49147e55c33cdf50948)
pub fn dispatch_text_input(manager: &TextInputManager, text: String) -> bool {
    let __flight_utf16_text: std::sync::Arc<Vec<u16>> =
        std::sync::Arc::new(text.encode_utf16().collect());
    let mut target = get_text_input_focus_target(manager);
    if ((target).is_none()) || ((__flight_utf16_text.len() as f64) == 0.0_f64) {
        return false;
    }
    insert_text_input(&mut target.as_mut().unwrap(), (text).clone());
    return true;
}

// Source: upstream/packages/textinput/src/textInputManager.ts:54 (sha256:2eccda4007d21a0cd9ff02edcac75e74d04229995733a87a73ed4d3cd05be3f3)
pub fn dispatch_text_input_key_down(
    manager: &TextInputManager,
    data: &InputKeyboardData,
    clipboard_text: Option<String>,
    on_copy: Option<
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(String) -> () + Send + 'static>>>,
    >,
) -> bool {
    let mut target = get_text_input_focus_target(manager);
    if (target).is_none() {
        return false;
    }
    let layout = (get_rich_text_runtime(&target.as_mut().unwrap())
        .inner
        .lock()
        .unwrap()
        .text_layout)
        .clone();
    return handle_text_input_keyboard(
        &mut target.as_mut().unwrap(),
        &{
            let __flight_source = &(data);
            KeyboardEventData {
                __flight_identity: std::sync::Arc::clone(&__flight_source.__flight_identity),
                alt_key: __flight_source.alt_key,
                ctrl_key: __flight_source.ctrl_key,
                key: (__flight_source.key).clone(),
                key_code: __flight_source.key_code,
                meta_key: __flight_source.meta_key,
                shift_key: __flight_source.shift_key,
            }
        },
        Some(HandleTextInputKeyboardOptions {
            __flight_identity: std::sync::Arc::new(()),
            clipboard_text: Some((clipboard_text).clone().unwrap()),
            layout: Some((layout).clone().unwrap()),
            on_copy: Some((on_copy).clone().unwrap()),
        }),
    );
}

// Source: upstream/packages/textinput/src/textInputManager.ts:66 (sha256:d2bb82836fcfe63014aa591312a50e3b835af4ff32b3e741d0293af0dd142ab2)
pub fn dispatch_text_input_pointer_down(
    manager: &mut TextInputManager,
    target: &mut RichText,
    x: f64,
    y: f64,
    extend: Option<bool>,
    click_count: Option<f64>,
) -> () {
    let extend = extend.unwrap_or(false);
    let click_count = click_count.unwrap_or(1.0_f64);
    focus_text_input(manager, target);
    let layout = (get_rich_text_runtime(target)
        .inner
        .lock()
        .unwrap()
        .text_layout)
        .clone();
    if (layout).is_none() {
        return;
    }
    let index = get_text_input_character_index_at_point(target, &layout.as_ref().unwrap(), x, y);
    if (click_count >= 3.0_f64) {
        select_line_at_text_input_index(target, index);
    } else {
        if (click_count == 2.0_f64) {
            select_word_at_text_input_index(target, index);
        } else {
            move_text_input_caret(target, index, Some(extend));
        }
    }
}

// Source: upstream/packages/textinput/src/textInputManager.ts:87 (sha256:b74eb4f68c9f6a4d16075fe1dabd01a0a866e258ad56c0ad8321a310fcb30166)
pub fn dispatch_text_input_pointer_move(manager: &mut TextInputManager, x: f64, y: f64) -> () {
    let mut target = (manager.focused).clone();
    if ((target).is_none()) || (!target.as_mut().unwrap().enabled) {
        return;
    }
    let layout = (get_rich_text_runtime(&target.as_mut().unwrap())
        .inner
        .lock()
        .unwrap()
        .text_layout)
        .clone();
    if (layout).is_none() {
        return;
    }
    let index = get_text_input_character_index_at_point(
        &mut target.as_mut().unwrap(),
        &layout.as_ref().unwrap(),
        x,
        y,
    );
    move_text_input_caret(&target.as_mut().unwrap(), index, Some(true));
}

// Source: upstream/packages/textinput/src/textInputManager.ts:96 (sha256:c437416723ce314df0ff774053b771d0ec3fc81d9105175f60a5e0f75dc14903)
pub fn dispatch_text_input_wheel(manager: &mut TextInputManager, delta_lines: f64) -> () {
    let mut target = (manager.focused).clone();
    if ((target).is_none()) || (!target.as_mut().unwrap().enabled) {
        return;
    }
    {
        let __flight_argument_1 = (target.as_mut().unwrap().data.scroll_v + (delta_lines).round());
        let __flight_result =
            set_rich_text_scroll_v(&mut target.as_mut().unwrap(), __flight_argument_1, None);
        __flight_result
    };
}

// Source: upstream/packages/textinput/src/textInputManager.ts:102 (sha256:c72725f0773ffbea1f408368e39a3fb3fa8938134c1876d1a234045048e8b509)
pub fn focus_text_input(manager: &mut TextInputManager, target: &RichText) -> () {
    if !(((manager.focused).clone()) == Some((*target).clone())) {
        let previous = (manager.focused).clone();
        if (previous).is_some() {
            set_text_input_focused(&previous.as_ref().unwrap(), false);
        }
    }
    manager.focused = Some((*target).clone());
    set_text_input_focused(target, true);
}

// Source: upstream/packages/textinput/src/textInputManager.ts:111 (sha256:3b3b62db95cee44187361455b968c5cc4a750a28916b201be39a8fba81d51160)
fn get_text_input_focus_target(manager: &TextInputManager) -> Option<RichText> {
    if (!manager.enabled) {
        return None;
    }
    let target = (manager.focused).clone();
    if ((target).is_none()) || (!target.as_ref().unwrap().enabled) {
        return None;
    }
    return Some((target.as_ref().unwrap()).clone());
}

// Source: upstream/packages/textinput/src/textInputManager.ts:120 (sha256:a6fecc46dca2fd97c36843bbfae467ac03bf9b80304811707ba2d4c9bf627789)
fn set_text_input_focused(target: &RichText, focused: bool) -> () {
    let mut state = get_text_input_state(target);
    if (state).is_some() {
        state.as_mut().unwrap().focused = focused;
    }
}
