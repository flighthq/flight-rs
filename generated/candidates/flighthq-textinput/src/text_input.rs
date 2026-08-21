// @generated from upstream/packages/textinput/src/textInput.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_text::get_rich_text_runtime;
use flighthq_types::{RichText, TextInputOptions, TextInputState};

// Source: upstream/packages/textinput/src/textInput.ts:11 (sha256:76507ce67861b1a1347fd0f6f76ba2aaf8e24ade6e4786c42de05ab486ed74ef)
pub fn disable_text_input(node: &RichText) -> () {
    {
        let __flight_runtime = get_rich_text_runtime(node);
        let __flight_value = None;
        let mut __flight_storage = __flight_runtime.inner.lock().unwrap();
        __flight_storage.input = __flight_value;
    };
}

// Source: upstream/packages/textinput/src/textInput.ts:17 (sha256:10757c4ab40395f45a82484fd513ac85646153be488963c65eaa449fc4d69d79)
pub fn enable_text_input(node: &RichText, options: Option<TextInputOptions>) -> TextInputState {
    let mut runtime = get_rich_text_runtime(node);
    let mut state = (runtime.inner.lock().unwrap().input).clone();
    if ((state).clone()).is_none() {
        state = Some(create_text_input_state(((options).clone()).clone()));
        {
            let __flight_runtime = runtime;
            let __flight_value = (state).clone();
            let mut __flight_storage = __flight_runtime.inner.lock().unwrap();
            __flight_storage.input = __flight_value;
        };
    } else {
        if (options).is_some() {
            apply_text_input_options(&mut state.as_mut().unwrap(), &options.as_ref().unwrap());
        }
    }
    return ((state).clone().unwrap()).clone();
}

// Source: upstream/packages/textinput/src/textInput.ts:29 (sha256:084146c3fd6190c26e0e9272eb6a74f0d6f6fe71ee806877f6f79961cb6453df)
pub fn get_text_input_state(node: &RichText) -> Option<TextInputState> {
    return (get_rich_text_runtime(node).inner.lock().unwrap().input).clone();
}

// Source: upstream/packages/textinput/src/textInput.ts:33 (sha256:4d09410f18f70184c1df6c5d062ec13892748efccb38eae75fb6ba1a14b365b9)
pub fn has_text_input(node: &RichText) -> bool {
    return ((get_rich_text_runtime(node).inner.lock().unwrap().input).clone()).is_some();
}

// Source: upstream/packages/textinput/src/textInput.ts:37 (sha256:40a03bb092e4c944038299c20684effb630f70a035c57fdd89d26b0be6c01168)
fn apply_text_input_options(state: &mut TextInputState, options: &TextInputOptions) -> () {
    if (options.always_show_selection).is_some() {
        state.always_show_selection = (options.always_show_selection).unwrap();
    }
    if (options.caret_color).is_some() {
        state.caret_color = (options.caret_color).unwrap();
    }
    if (options.caret_width).is_some() {
        state.caret_width = (options.caret_width).unwrap();
    }
    if (options.display_as_password).is_some() {
        state.display_as_password = (options.display_as_password).unwrap();
    }
    if (options.history_limit).is_some() {
        state.history_limit = (0.0_f64).max(options.history_limit);
    }
    if ((options.password_character).clone()).is_some() {
        state.password_character = ((options.password_character).clone()).unwrap();
    }
    if ((options.restrict).clone()).is_some() {
        state.restrict = ((options.restrict).clone()).unwrap();
    }
    if (options.selection_alpha).is_some() {
        state.selection_alpha = (options.selection_alpha).unwrap();
    }
    if (options.selection_color).is_some() {
        state.selection_color = (options.selection_color).unwrap();
    }
}

// Source: upstream/packages/textinput/src/textInput.ts:52 (sha256:b2557c022962c91d047b2efa411f2ae2e2b5e91fb350e3ead4901608ea3103a8)
fn create_text_input_state(options: Option<TextInputOptions>) -> TextInputState {
    return TextInputState {
        __flight_identity: std::sync::Arc::new(()),
        always_show_selection: (options
            .as_ref()
            .and_then(|value| value.always_show_selection))
        .clone()
        .unwrap_or(false),
        caret_color: (options.as_ref().and_then(|value| value.caret_color))
            .clone()
            .unwrap_or(0.0_f64),
        caret_index: 0.0_f64,
        caret_width: (options.as_ref().and_then(|value| value.caret_width))
            .clone()
            .unwrap_or(1.0_f64),
        desired_caret_x: (-1.0_f64),
        display_as_password: (options.as_ref().and_then(|value| value.display_as_password))
            .clone()
            .unwrap_or(false),
        focused: false,
        history: vec![],
        history_index: (-1.0_f64),
        history_limit: if (options.as_ref().and_then(|value| value.history_limit)).is_some() {
            (0.0_f64).max(options.as_ref().unwrap().history_limit)
        } else {
            100.0_f64
        },
        password_character: (options
            .as_ref()
            .and_then(|value| (value.password_character).clone()))
        .clone()
        .unwrap_or("•".to_owned()),
        restrict: (options.as_ref().and_then(|value| (value.restrict).clone()))
            .clone()
            .unwrap_or("".to_owned()),
        selection_alpha: (options.as_ref().and_then(|value| value.selection_alpha))
            .clone()
            .unwrap_or(0.35_f64),
        selection_color: (options.as_ref().and_then(|value| value.selection_color))
            .clone()
            .unwrap_or(30935.0_f64),
        selection_index: 0.0_f64,
    };
}
