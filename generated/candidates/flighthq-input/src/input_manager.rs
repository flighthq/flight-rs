// @generated from upstream/packages/input/src/inputManager.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_signals::{connect_signal, create_signal, disconnect_signal, emit_signal};
use flighthq_types::{
    AttachInputOptions, GAMEPAD_AXIS_KIND as gamepad_axis_kind_values_constant,
    GAMEPAD_BUTTON_KIND as gamepad_button_kind_values_constant, GamepadAxisKind, GamepadButtonKind,
    GamepadMappingKind, InputGamepadAxisData, InputGamepadButtonData, InputGamepadConnectData,
    InputKeyRepeatOptions, InputKeyRepeatTimer, InputKeyboardData, InputManager, InputPointerData,
    InputSignals, InputState, InputTextData, KeyCode, KeyModifier, MouseWheelMode,
};

#[inline]
fn __flight_js_to_u32(value: f64) -> u32 {
    if !value.is_finite() || value == 0.0 {
        return 0;
    }
    value.trunc().rem_euclid(4294967296.0_f64) as u32
}

#[inline]
fn __flight_js_to_i32(value: f64) -> i32 {
    __flight_js_to_u32(value) as i32
}

#[derive(Clone, Default)]
pub struct SharedStructuralRecord1 {
    pub __flight_identity: std::sync::Arc<()>,
    pub x: f64,
    pub y: f64,
}
impl PartialEq for SharedStructuralRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/input/src/inputManager.ts:30 (sha256:4a7b3c7dce7eb29f8dee9e960ad0cbb80e6f554168eee1663e2da52aa3ba5dfd)
const MAX_GAMEPAD_AXES: f64 = 32.0_f64;

// Source: upstream/packages/input/src/inputManager.ts:31 (sha256:5dfdf4fefadc07a13c6091c66b2ff842f0a7d4b1ad77e192bc2773cd39cf1982)
const MAX_GAMEPAD_BUTTONS: f64 = 64.0_f64;

// Source: upstream/packages/input/src/inputManager.ts:39 (sha256:51129b696f0f63d20d02e53c97e4259d9b397cab47fb34b81ccea602322a60e3)
pub fn apply_gamepad_axis_dead_zone(value: f64, dead_zone: f64) -> f64 {
    if (dead_zone <= 0.0_f64) {
        return value;
    }
    let abs = if (value < 0.0_f64) { (-value) } else { value };
    if (abs <= dead_zone) {
        return 0.0_f64;
    }
    let sign = if (value < 0.0_f64) {
        (-1.0_f64)
    } else {
        1.0_f64
    };
    return (sign * ((abs - dead_zone) / (1.0_f64 - dead_zone)));
}

// Source: upstream/packages/input/src/inputManager.ts:58 (sha256:cc1bb9388bcde5ec54cce54c4c88a99913560d9ca5ceeb0c36088a56a0e051c7)
pub fn apply_gamepad_stick_dead_zone(
    out: &mut SharedStructuralRecord1,
    x: f64,
    y: f64,
    dead_zone: f64,
) -> () {
    if (dead_zone <= 0.0_f64) {
        out.x = x;
        out.y = y;
        return;
    }
    let mag = ((x * x) + (y * y)).sqrt();
    if (mag <= dead_zone) {
        out.x = 0.0_f64;
        out.y = 0.0_f64;
        return;
    }
    let scale = ((mag - dead_zone) / ((1.0_f64 - dead_zone) * mag));
    out.x = (x * scale);
    out.y = (y * scale);
}

// Source: upstream/packages/input/src/inputManager.ts:75 (sha256:9aaedf5e7f6f760a3b824294cea61f5c404d2b272bfb306075d934f8165f26ab)
pub fn attach_gamepad_input(
    manager: InputManager,
    target: crate::OpaqueHostValue,
    options: Option<AttachInputOptions>,
) -> () {
    let mut on_gamepad_connected: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut(crate::OpaqueHostValue) -> () + Send + 'static>>,
    > = std::sync::Arc::new(std::sync::Mutex::new(Box::new({
        let manager = manager.clone();
        move |e: crate::OpaqueHostValue| -> () {
            if (!manager.enabled) {
                return;
            }
            let pad = crate::host_value::<crate::OpaqueHostValue>("host.gamepad");
            let mut prev = get_or_create_gamepad_poll_state(&manager);
            {
                let __flight_key = crate::host_value::<f64>("host.index");
                let __flight_value = crate::host_value::<Vec<f64>>("host.Array.from");
                if let Some((_, value)) = prev.axes.iter_mut().find(|(key, _)| key == &__flight_key)
                {
                    *value = __flight_value;
                } else {
                    prev.axes.push((__flight_key, __flight_value));
                }
            };
            {
                let __flight_key = crate::host_value::<f64>("host.index");
                let __flight_value = crate::host_value::<Vec<bool>>("host.Array.from");
                if let Some((_, value)) = prev
                    .buttons
                    .iter_mut()
                    .find(|(key, _)| key == &__flight_key)
                {
                    *value = __flight_value;
                } else {
                    prev.buttons.push((__flight_key, __flight_value));
                }
            };
            (*_CONNECT_DATA.lock().unwrap()).gamepad = crate::host_value::<f64>("host.index");
            (*_CONNECT_DATA.lock().unwrap()).id = crate::host_value::<String>("host.id");
            (*_CONNECT_DATA.lock().unwrap()).mapping =
                if (crate::host_value::<String>("host.mapping") == "standard") {
                    "standard".to_owned()
                } else {
                    if (crate::host_value::<String>("host.mapping") == "") {
                        "".to_owned()
                    } else {
                        "raw".to_owned()
                    }
                };
            emit_signal(
                (manager.on_gamepad_connect).clone(),
                ((*_CONNECT_DATA.lock().unwrap()).clone(),),
            );
        }
    })
        as Box<dyn FnMut(crate::OpaqueHostValue) -> () + Send + 'static>));
    let mut on_gamepad_disconnected: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut(crate::OpaqueHostValue) -> () + Send + 'static>>,
    > = std::sync::Arc::new(std::sync::Mutex::new(Box::new({
        let manager = manager.clone();
        move |e: crate::OpaqueHostValue| -> () {
            if (!manager.enabled) {
                return;
            }
            let pad = crate::host_value::<crate::OpaqueHostValue>("host.gamepad");
            let mut prev = get_or_create_gamepad_poll_state(&manager);
            {
                let __flight_key = crate::host_value::<f64>("host.index");
                if let Some(__flight_index) =
                    prev.axes.iter().position(|(key, _)| key == &__flight_key)
                {
                    prev.axes.remove(__flight_index);
                    true
                } else {
                    false
                }
            };
            {
                let __flight_key = crate::host_value::<f64>("host.index");
                if let Some(__flight_index) = prev
                    .buttons
                    .iter()
                    .position(|(key, _)| key == &__flight_key)
                {
                    prev.buttons.remove(__flight_index);
                    true
                } else {
                    false
                }
            };
            (*_CONNECT_DATA.lock().unwrap()).gamepad = crate::host_value::<f64>("host.index");
            (*_CONNECT_DATA.lock().unwrap()).id = crate::host_value::<String>("host.id");
            (*_CONNECT_DATA.lock().unwrap()).mapping =
                if (crate::host_value::<String>("host.mapping") == "standard") {
                    "standard".to_owned()
                } else {
                    if (crate::host_value::<String>("host.mapping") == "") {
                        "".to_owned()
                    } else {
                        "raw".to_owned()
                    }
                };
            emit_signal(
                (manager.on_gamepad_disconnect).clone(),
                ((*_CONNECT_DATA.lock().unwrap()).clone(),),
            );
        }
    })
        as Box<dyn FnMut(crate::OpaqueHostValue) -> () + Send + 'static>));
    let raf_id: std::sync::Arc<std::sync::Mutex<f64>> =
        std::sync::Arc::new(std::sync::Mutex::new(0.0_f64));
    let __flight_recursive_loop_: std::sync::Arc<
        std::sync::Mutex<
            Option<std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>>,
        >,
    > = std::sync::Arc::new(std::sync::Mutex::new(None));
    let mut loop_: std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>> =
        std::sync::Arc::new(std::sync::Mutex::new(Box::new({
            let __flight_recursive_loop_ = __flight_recursive_loop_.clone();
            let manager = manager.clone();
            let mut raf_id = raf_id.clone();
            move || -> () {
                poll_gamepad_input(&manager);
                (*raf_id.lock().unwrap()) = crate::host_value::<f64>("host.call");
            }
        })
            as Box<dyn FnMut() -> () + Send + 'static>));
    *__flight_recursive_loop_.lock().unwrap() = Some(loop_.clone());
    crate::host_value::<()>("host.addEventListener");
    crate::host_value::<()>("host.addEventListener");
    (*raf_id.lock().unwrap()) = crate::host_value::<f64>("host.call");
    set_input_binding(
        &manager,
        (target).clone(),
        *K_GAMEPAD_INPUT,
        std::sync::Arc::new(std::sync::Mutex::new(Box::new({
            let on_gamepad_connected = on_gamepad_connected.clone();
            let on_gamepad_disconnected = on_gamepad_disconnected.clone();
            let mut raf_id = raf_id.clone();
            move || -> () {
                crate::host_value::<()>("host.removeEventListener");
                crate::host_value::<()>("host.removeEventListener");
                crate::host_value::<()>("host.call");
            }
        })
            as Box<dyn FnMut() -> () + Send + 'static>)),
    );
    {
        options;
        ()
    };
}

// Source: upstream/packages/input/src/inputManager.ts:127 (sha256:47ef75d01c95091199cff3ba428d91f3c7bf0c2cc36f387ee4a2c5b1df2da4c8)
pub fn attach_keyboard_input(
    manager: InputManager,
    target: crate::OpaqueHostValue,
    options: Option<AttachInputOptions>,
) -> () {
    let prevent_default = (options.as_ref().and_then(|value| value.prevent_default))
        .clone()
        .unwrap_or(true);
    let mut on_key_down: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut(crate::OpaqueHostValue) -> () + Send + 'static>>,
    > = std::sync::Arc::new(std::sync::Mutex::new(Box::new({
        let manager = manager.clone();
        move |e: crate::OpaqueHostValue| -> () {
            if (!manager.enabled) {
                return;
            }
            let ke = (e).clone();
            if prevent_default {
                crate::host_value::<()>("host.preventDefault");
            }
            set_input_keyboard_data(&mut (*_KEYBOARD_DATA.lock().unwrap()), (ke).clone());
            emit_signal(
                (manager.on_key_down).clone(),
                ((*_KEYBOARD_DATA.lock().unwrap()).clone(),),
            );
        }
    })
        as Box<dyn FnMut(crate::OpaqueHostValue) -> () + Send + 'static>));
    let mut on_key_up: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut(crate::OpaqueHostValue) -> () + Send + 'static>>,
    > = std::sync::Arc::new(std::sync::Mutex::new(Box::new({
        let manager = manager.clone();
        move |e: crate::OpaqueHostValue| -> () {
            if (!manager.enabled) {
                return;
            }
            let ke = (e).clone();
            if prevent_default {
                crate::host_value::<()>("host.preventDefault");
            }
            set_input_keyboard_data(&mut (*_KEYBOARD_DATA.lock().unwrap()), (ke).clone());
            emit_signal(
                (manager.on_key_up).clone(),
                ((*_KEYBOARD_DATA.lock().unwrap()).clone(),),
            );
        }
    })
        as Box<dyn FnMut(crate::OpaqueHostValue) -> () + Send + 'static>));
    crate::host_value::<()>("host.addEventListener");
    crate::host_value::<()>("host.addEventListener");
    set_input_binding(
        &manager,
        (target).clone(),
        *K_KEYBOARD_INPUT,
        std::sync::Arc::new(std::sync::Mutex::new(Box::new({
            let on_key_down = on_key_down.clone();
            let on_key_up = on_key_up.clone();
            move || -> () {
                crate::host_value::<()>("host.removeEventListener");
                crate::host_value::<()>("host.removeEventListener");
            }
        })
            as Box<dyn FnMut() -> () + Send + 'static>)),
    );
}

// Source: upstream/packages/input/src/inputManager.ts:157 (sha256:f0241c1c79ae7bb17a999431677414e40dd8891a26c0e2b6c66c75cca5be5775)
pub fn attach_pointer_input(
    manager: InputManager,
    element: crate::OpaqueHostValue,
    options: Option<AttachInputOptions>,
) -> () {
    let prevent_default = (options.as_ref().and_then(|value| value.prevent_default))
        .clone()
        .unwrap_or(true);
    let mut on_context_menu: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut(crate::OpaqueHostValue) -> () + Send + 'static>>,
    > = std::sync::Arc::new(std::sync::Mutex::new(
        Box::new(move |e: crate::OpaqueHostValue| -> () {
            if prevent_default {
                crate::host_value::<()>("host.preventDefault");
            }
        }) as Box<dyn FnMut(crate::OpaqueHostValue) -> () + Send + 'static>,
    ));
    let mut on_pointer_cancel: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut(crate::OpaqueHostValue) -> () + Send + 'static>>,
    > = std::sync::Arc::new(std::sync::Mutex::new(Box::new({
        let manager = manager.clone();
        move |e: crate::OpaqueHostValue| -> () {
            if (!manager.enabled) {
                return;
            }
            if prevent_default {
                crate::host_value::<()>("host.preventDefault");
            }
            set_input_pointer_data(
                &mut (*_POINTER_DATA.lock().unwrap()),
                (e).clone(),
                0.0_f64,
                0.0_f64,
            );
            emit_signal(
                (manager.on_pointer_cancel).clone(),
                ((*_POINTER_DATA.lock().unwrap()).clone(),),
            );
        }
    })
        as Box<dyn FnMut(crate::OpaqueHostValue) -> () + Send + 'static>));
    let mut on_pointer_down: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut(crate::OpaqueHostValue) -> () + Send + 'static>>,
    > = std::sync::Arc::new(std::sync::Mutex::new(Box::new({
        let manager = manager.clone();
        move |e: crate::OpaqueHostValue| -> () {
            if (!manager.enabled) {
                return;
            }
            if prevent_default {
                crate::host_value::<()>("host.preventDefault");
            }
            set_input_pointer_data(
                &mut (*_POINTER_DATA.lock().unwrap()),
                (e).clone(),
                0.0_f64,
                0.0_f64,
            );
            emit_signal(
                (manager.on_pointer_down).clone(),
                ((*_POINTER_DATA.lock().unwrap()).clone(),),
            );
        }
    })
        as Box<dyn FnMut(crate::OpaqueHostValue) -> () + Send + 'static>));
    let mut on_pointer_move: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut(crate::OpaqueHostValue) -> () + Send + 'static>>,
    > = std::sync::Arc::new(std::sync::Mutex::new(Box::new({
        let manager = manager.clone();
        move |e: crate::OpaqueHostValue| -> () {
            if (!manager.enabled) {
                return;
            }
            if prevent_default {
                crate::host_value::<()>("host.preventDefault");
            }
            set_input_pointer_data(
                &mut (*_POINTER_DATA.lock().unwrap()),
                (e).clone(),
                0.0_f64,
                0.0_f64,
            );
            emit_signal(
                (manager.on_pointer_move).clone(),
                ((*_POINTER_DATA.lock().unwrap()).clone(),),
            );
        }
    })
        as Box<dyn FnMut(crate::OpaqueHostValue) -> () + Send + 'static>));
    let mut on_pointer_up: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut(crate::OpaqueHostValue) -> () + Send + 'static>>,
    > = std::sync::Arc::new(std::sync::Mutex::new(Box::new({
        let manager = manager.clone();
        move |e: crate::OpaqueHostValue| -> () {
            if (!manager.enabled) {
                return;
            }
            if prevent_default {
                crate::host_value::<()>("host.preventDefault");
            }
            set_input_pointer_data(
                &mut (*_POINTER_DATA.lock().unwrap()),
                (e).clone(),
                0.0_f64,
                0.0_f64,
            );
            emit_signal(
                (manager.on_pointer_up).clone(),
                ((*_POINTER_DATA.lock().unwrap()).clone(),),
            );
        }
    })
        as Box<dyn FnMut(crate::OpaqueHostValue) -> () + Send + 'static>));
    crate::host_value::<()>("host.addEventListener");
    crate::host_value::<()>("host.addEventListener");
    crate::host_value::<()>("host.addEventListener");
    crate::host_value::<()>("host.addEventListener");
    crate::host_value::<()>("host.addEventListener");
    set_input_binding(
        &manager,
        (element).clone(),
        *K_POINTER_INPUT,
        std::sync::Arc::new(std::sync::Mutex::new(Box::new({
            let on_context_menu = on_context_menu.clone();
            let on_pointer_cancel = on_pointer_cancel.clone();
            let on_pointer_down = on_pointer_down.clone();
            let on_pointer_move = on_pointer_move.clone();
            let on_pointer_up = on_pointer_up.clone();
            move || -> () {
                crate::host_value::<()>("host.removeEventListener");
                crate::host_value::<()>("host.removeEventListener");
                crate::host_value::<()>("host.removeEventListener");
                crate::host_value::<()>("host.removeEventListener");
                crate::host_value::<()>("host.removeEventListener");
            }
        })
            as Box<dyn FnMut() -> () + Send + 'static>)),
    );
}

// Source: upstream/packages/input/src/inputManager.ts:207 (sha256:20a4780a87e37aa069a79581f83d719611f9e6b2123f8cb1bb435233bfe7f4c4)
pub fn attach_relative_pointer_input(
    manager: InputManager,
    element: crate::OpaqueHostValue,
    options: Option<AttachInputOptions>,
) -> () {
    let prevent_default = (options.as_ref().and_then(|value| value.prevent_default))
        .clone()
        .unwrap_or(true);
    let target = crate::host_value::<crate::OpaqueHostValue>("host.ownerDocument");
    let mut handler: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut(crate::OpaqueHostValue) -> () + Send + 'static>>,
    > = std::sync::Arc::new(std::sync::Mutex::new(Box::new({
        let manager = manager.clone();
        move |e: crate::OpaqueHostValue| -> () {
            if (!manager.enabled) {
                return;
            }
            let me = (e).clone();
            if prevent_default {
                crate::host_value::<()>("host.preventDefault");
            }
            set_input_pointer_data(
                &mut (*_POINTER_DATA.lock().unwrap()),
                (me).clone(),
                crate::host_value::<f64>("host.movementX"),
                crate::host_value::<f64>("host.movementY"),
            );
            emit_signal(
                (manager.on_pointer_move_relative).clone(),
                ((*_POINTER_DATA.lock().unwrap()).clone(),),
            );
        }
    })
        as Box<dyn FnMut(crate::OpaqueHostValue) -> () + Send + 'static>));
    crate::host_value::<()>("host.addEventListener");
    set_input_binding(
        &manager,
        (element).clone(),
        *K_RELATIVE_POINTER_INPUT,
        std::sync::Arc::new(std::sync::Mutex::new(Box::new({
            let handler = handler.clone();
            move || -> () { crate::host_value::<()>("host.removeEventListener") }
        })
            as Box<dyn FnMut() -> () + Send + 'static>)),
    );
}

// Source: upstream/packages/input/src/inputManager.ts:225 (sha256:92721ed5ebb0604e065655850903ac64037ff43ef0251908fd39009972ef9e54)
pub fn attach_text_input(
    manager: InputManager,
    element: crate::OpaqueHostValue,
    options: Option<AttachInputOptions>,
) -> () {
    let mut on_before_input: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut(crate::OpaqueHostValue) -> () + Send + 'static>>,
    > = std::sync::Arc::new(std::sync::Mutex::new(Box::new({
        let manager = manager.clone();
        move |e: crate::OpaqueHostValue| -> () {
            if (!manager.enabled) {
                return;
            }
            let ie = (e).clone();
            let text = (crate::host_value::<Option<String>>("host.data"))
                .clone()
                .unwrap_or("".to_owned());
            (*_TEXT_DATA.lock().unwrap()).is_composing =
                crate::host_value::<bool>("host.isComposing");
            (*_TEXT_DATA.lock().unwrap()).text = text;
            emit_signal(
                (manager.on_text_input).clone(),
                ((*_TEXT_DATA.lock().unwrap()).clone(),),
            );
        }
    })
        as Box<dyn FnMut(crate::OpaqueHostValue) -> () + Send + 'static>));
    let mut on_composition_update: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut(crate::OpaqueHostValue) -> () + Send + 'static>>,
    > = std::sync::Arc::new(std::sync::Mutex::new(Box::new({
        let manager = manager.clone();
        move |e: crate::OpaqueHostValue| -> () {
            if (!manager.enabled) {
                return;
            }
            let ce = (e).clone();
            let text = (crate::host_value::<Option<String>>("host.data"))
                .clone()
                .unwrap_or("".to_owned());
            (*_TEXT_DATA.lock().unwrap()).is_composing = true;
            (*_TEXT_DATA.lock().unwrap()).text = text;
            emit_signal(
                (manager.on_text_edit).clone(),
                ((*_TEXT_DATA.lock().unwrap()).clone(),),
            );
        }
    })
        as Box<dyn FnMut(crate::OpaqueHostValue) -> () + Send + 'static>));
    crate::host_value::<()>("host.addEventListener");
    crate::host_value::<()>("host.addEventListener");
    set_input_binding(
        &manager,
        (element).clone(),
        *K_TEXT_INPUT,
        std::sync::Arc::new(std::sync::Mutex::new(Box::new({
            let on_before_input = on_before_input.clone();
            let on_composition_update = on_composition_update.clone();
            move || -> () {
                crate::host_value::<()>("host.removeEventListener");
                crate::host_value::<()>("host.removeEventListener");
            }
        })
            as Box<dyn FnMut() -> () + Send + 'static>)),
    );
    {
        options;
        ()
    };
}

// Source: upstream/packages/input/src/inputManager.ts:258 (sha256:513ffb107d07d9857dd4f46e44b67703d14d913038b19d8fb88c7a6fd2dfbd74)
#[derive(Clone, Default)]
struct AttachWheelInputRecord2 {
    __flight_identity: std::sync::Arc<()>,
    passive: bool,
}
impl PartialEq for AttachWheelInputRecord2 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn attach_wheel_input(
    manager: InputManager,
    element: crate::OpaqueHostValue,
    options: Option<AttachInputOptions>,
) -> () {
    let prevent_default = (options.as_ref().and_then(|value| value.prevent_default))
        .clone()
        .unwrap_or(true);
    let mut handler: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut(crate::OpaqueHostValue) -> () + Send + 'static>>,
    > = std::sync::Arc::new(std::sync::Mutex::new(Box::new({
        let manager = manager.clone();
        move |e: crate::OpaqueHostValue| -> () {
            if (!manager.enabled) {
                return;
            }
            let we = (e).clone();
            if prevent_default {
                crate::host_value::<()>("host.preventDefault");
            }
            set_input_pointer_data(
                &mut (*_POINTER_DATA.lock().unwrap()),
                (we).clone(),
                crate::host_value::<f64>("host.deltaX"),
                crate::host_value::<f64>("host.deltaY"),
            );
            (*_POINTER_DATA.lock().unwrap()).wheel_mode =
                get_mouse_wheel_mode_from_dom_wheel_event((we).clone());
            emit_signal(
                (manager.on_wheel).clone(),
                ((*_POINTER_DATA.lock().unwrap()).clone(),),
            );
        }
    })
        as Box<dyn FnMut(crate::OpaqueHostValue) -> () + Send + 'static>));
    crate::host_value::<()>("host.addEventListener");
    set_input_binding(
        &manager,
        (element).clone(),
        *K_WHEEL_INPUT,
        std::sync::Arc::new(std::sync::Mutex::new(Box::new({
            let handler = handler.clone();
            move || -> () { crate::host_value::<()>("host.removeEventListener") }
        })
            as Box<dyn FnMut() -> () + Send + 'static>)),
    );
}

// Source: upstream/packages/input/src/inputManager.ts:283 (sha256:ada32cffd3993edc7dec0adecd235e8ca8b614fb9721c12a6d28fbc7482563d3)
pub fn connect_input_state_to_input_manager(
    mut state: InputState,
    mut manager: InputManager,
) -> std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>> {
    let mut on_key_down: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut(InputKeyboardData) -> () + Send + 'static>>,
    > = std::sync::Arc::new(std::sync::Mutex::new(Box::new({
        let mut state = state.clone();
        move |data: InputKeyboardData| -> () {
            if (!state.keys_down.iter().any(|item| item == &data.key_code)) {
                {
                    let __flight_value = data.key_code;
                    if !state.just_pressed_keys.contains(&__flight_value) {
                        state.just_pressed_keys.push(__flight_value);
                    }
                };
            }
            {
                let __flight_value = data.key_code;
                if !state.keys_down.contains(&__flight_value) {
                    state.keys_down.push(__flight_value);
                }
            };
        }
    })
        as Box<dyn FnMut(InputKeyboardData) -> () + Send + 'static>));
    let mut on_key_up: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut(InputKeyboardData) -> () + Send + 'static>>,
    > = std::sync::Arc::new(std::sync::Mutex::new(Box::new({
        let mut state = state.clone();
        move |data: InputKeyboardData| -> () {
            {
                let __flight_value = data.key_code;
                if let Some(__flight_index) = state
                    .keys_down
                    .iter()
                    .position(|item| item == &__flight_value)
                {
                    state.keys_down.remove(__flight_index);
                    true
                } else {
                    false
                }
            };
            {
                let __flight_value = data.key_code;
                if !state.just_released_keys.contains(&__flight_value) {
                    state.just_released_keys.push(__flight_value);
                }
            };
        }
    })
        as Box<dyn FnMut(InputKeyboardData) -> () + Send + 'static>));
    let mut on_pointer_down: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut(InputPointerData) -> () + Send + 'static>>,
    > = std::sync::Arc::new(std::sync::Mutex::new(Box::new({
        let mut state = state.clone();
        move |data: InputPointerData| -> () {
            let prev = (state
                .pointer_buttons_down
                .iter()
                .find(|(entry_key, _)| entry_key == &data.pointer_id)
                .map(|(_, value)| value.clone()))
            .clone()
            .unwrap_or(0.0_f64);
            {
                let __flight_key = data.pointer_id;
                let __flight_value = (__flight_js_to_i32(prev)
                    | __flight_js_to_i32(
                        __flight_js_to_i32(1.0_f64)
                            .wrapping_shl((__flight_js_to_u32(data.button) & 31))
                            as f64,
                    )) as f64;
                if let Some((_, value)) = state
                    .pointer_buttons_down
                    .iter_mut()
                    .find(|(key, _)| key == &__flight_key)
                {
                    *value = __flight_value;
                } else {
                    state
                        .pointer_buttons_down
                        .push((__flight_key, __flight_value));
                }
            };
        }
    })
        as Box<dyn FnMut(InputPointerData) -> () + Send + 'static>));
    let mut on_pointer_up: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut(InputPointerData) -> () + Send + 'static>>,
    > = std::sync::Arc::new(std::sync::Mutex::new(Box::new({
        let mut state = state.clone();
        move |data: InputPointerData| -> () {
            let prev = (state
                .pointer_buttons_down
                .iter()
                .find(|(entry_key, _)| entry_key == &data.pointer_id)
                .map(|(_, value)| value.clone()))
            .clone()
            .unwrap_or(0.0_f64);
            let next = (__flight_js_to_i32(prev)
                & __flight_js_to_i32(
                    (!__flight_js_to_i32(
                        __flight_js_to_i32(1.0_f64)
                            .wrapping_shl((__flight_js_to_u32(data.button) & 31))
                            as f64,
                    )) as f64,
                )) as f64;
            if (next == 0.0_f64) {
                {
                    let __flight_key = data.pointer_id;
                    if let Some(__flight_index) = state
                        .pointer_buttons_down
                        .iter()
                        .position(|(key, _)| key == &__flight_key)
                    {
                        state.pointer_buttons_down.remove(__flight_index);
                        true
                    } else {
                        false
                    }
                };
            } else {
                {
                    let __flight_key = data.pointer_id;
                    let __flight_value = next;
                    if let Some((_, value)) = state
                        .pointer_buttons_down
                        .iter_mut()
                        .find(|(key, _)| key == &__flight_key)
                    {
                        *value = __flight_value;
                    } else {
                        state
                            .pointer_buttons_down
                            .push((__flight_key, __flight_value));
                    }
                };
            }
        }
    })
        as Box<dyn FnMut(InputPointerData) -> () + Send + 'static>));
    let mut on_pointer_cancel: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut(InputPointerData) -> () + Send + 'static>>,
    > = std::sync::Arc::new(std::sync::Mutex::new(Box::new({
        let mut state = state.clone();
        move |data: InputPointerData| -> () {
            {
                let __flight_key = data.pointer_id;
                if let Some(__flight_index) = state
                    .pointer_buttons_down
                    .iter()
                    .position(|(key, _)| key == &__flight_key)
                {
                    state.pointer_buttons_down.remove(__flight_index);
                    true
                } else {
                    false
                }
            };
        }
    })
        as Box<dyn FnMut(InputPointerData) -> () + Send + 'static>));
    let mut on_gamepad_button_down: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut(InputGamepadButtonData) -> () + Send + 'static>>,
    > = std::sync::Arc::new(std::sync::Mutex::new(Box::new({
        let mut state = state.clone();
        move |data: InputGamepadButtonData| -> () {
            let key = ((data.gamepad * MAX_GAMEPAD_BUTTONS) + data.button);
            if (!state.gamepad_buttons_down.iter().any(|item| item == &key)) {
                {
                    let __flight_value = key;
                    if !state.just_pressed_gamepad_buttons.contains(&__flight_value) {
                        state.just_pressed_gamepad_buttons.push(__flight_value);
                    }
                };
            }
            {
                let __flight_value = key;
                if !state.gamepad_buttons_down.contains(&__flight_value) {
                    state.gamepad_buttons_down.push(__flight_value);
                }
            };
        }
    })
        as Box<dyn FnMut(InputGamepadButtonData) -> () + Send + 'static>));
    let mut on_gamepad_button_up: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut(InputGamepadButtonData) -> () + Send + 'static>>,
    > = std::sync::Arc::new(std::sync::Mutex::new(Box::new({
        let mut state = state.clone();
        move |data: InputGamepadButtonData| -> () {
            let key = ((data.gamepad * MAX_GAMEPAD_BUTTONS) + data.button);
            {
                let __flight_value = key;
                if let Some(__flight_index) = state
                    .gamepad_buttons_down
                    .iter()
                    .position(|item| item == &__flight_value)
                {
                    state.gamepad_buttons_down.remove(__flight_index);
                    true
                } else {
                    false
                }
            };
            {
                let __flight_value = key;
                if !state
                    .just_released_gamepad_buttons
                    .contains(&__flight_value)
                {
                    state.just_released_gamepad_buttons.push(__flight_value);
                }
            };
        }
    })
        as Box<dyn FnMut(InputGamepadButtonData) -> () + Send + 'static>));
    let mut on_gamepad_axis_move: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut(InputGamepadAxisData) -> () + Send + 'static>>,
    > = std::sync::Arc::new(std::sync::Mutex::new(Box::new({
        let mut state = state.clone();
        move |data: InputGamepadAxisData| -> () {
            {
                let __flight_key = ((data.gamepad * MAX_GAMEPAD_AXES) + data.axis);
                let __flight_value = data.value;
                if let Some((_, value)) = state
                    .axis_values
                    .iter_mut()
                    .find(|(key, _)| key == &__flight_key)
                {
                    *value = __flight_value;
                } else {
                    state.axis_values.push((__flight_key, __flight_value));
                }
            };
        }
    })
        as Box<dyn FnMut(InputGamepadAxisData) -> () + Send + 'static>));
    let mut on_gamepad_connect: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut(InputGamepadConnectData) -> () + Send + 'static>>,
    > = std::sync::Arc::new(std::sync::Mutex::new(Box::new({
        let mut state = state.clone();
        move |data: InputGamepadConnectData| -> () {
            {
                let mut b = 0.0_f64;
                while (b < MAX_GAMEPAD_BUTTONS) {
                    let key = ((data.gamepad * MAX_GAMEPAD_BUTTONS) + b);
                    {
                        let __flight_value = key;
                        if let Some(__flight_index) = state
                            .gamepad_buttons_down
                            .iter()
                            .position(|item| item == &__flight_value)
                        {
                            state.gamepad_buttons_down.remove(__flight_index);
                            true
                        } else {
                            false
                        }
                    };
                    {
                        let __flight_value = key;
                        if let Some(__flight_index) = state
                            .just_pressed_gamepad_buttons
                            .iter()
                            .position(|item| item == &__flight_value)
                        {
                            state.just_pressed_gamepad_buttons.remove(__flight_index);
                            true
                        } else {
                            false
                        }
                    };
                    {
                        let __flight_value = key;
                        if let Some(__flight_index) = state
                            .just_released_gamepad_buttons
                            .iter()
                            .position(|item| item == &__flight_value)
                        {
                            state.just_released_gamepad_buttons.remove(__flight_index);
                            true
                        } else {
                            false
                        }
                    };
                    {
                        b += 1.0;
                        b
                    };
                }
            }
            {
                let mut a = 0.0_f64;
                while (a < MAX_GAMEPAD_AXES) {
                    {
                        let __flight_key = ((data.gamepad * MAX_GAMEPAD_AXES) + a);
                        if let Some(__flight_index) = state
                            .axis_values
                            .iter()
                            .position(|(key, _)| key == &__flight_key)
                        {
                            state.axis_values.remove(__flight_index);
                            true
                        } else {
                            false
                        }
                    };
                    {
                        a += 1.0;
                        a
                    };
                }
            }
        }
    })
        as Box<dyn FnMut(InputGamepadConnectData) -> () + Send + 'static>));
    let mut on_gamepad_disconnect: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut(InputGamepadConnectData) -> () + Send + 'static>>,
    > = std::sync::Arc::new(std::sync::Mutex::new(Box::new({
        let mut state = state.clone();
        move |data: InputGamepadConnectData| -> () {
            {
                let mut b = 0.0_f64;
                while (b < MAX_GAMEPAD_BUTTONS) {
                    let key = ((data.gamepad * MAX_GAMEPAD_BUTTONS) + b);
                    {
                        let __flight_value = key;
                        if let Some(__flight_index) = state
                            .gamepad_buttons_down
                            .iter()
                            .position(|item| item == &__flight_value)
                        {
                            state.gamepad_buttons_down.remove(__flight_index);
                            true
                        } else {
                            false
                        }
                    };
                    {
                        let __flight_value = key;
                        if let Some(__flight_index) = state
                            .just_pressed_gamepad_buttons
                            .iter()
                            .position(|item| item == &__flight_value)
                        {
                            state.just_pressed_gamepad_buttons.remove(__flight_index);
                            true
                        } else {
                            false
                        }
                    };
                    {
                        let __flight_value = key;
                        if let Some(__flight_index) = state
                            .just_released_gamepad_buttons
                            .iter()
                            .position(|item| item == &__flight_value)
                        {
                            state.just_released_gamepad_buttons.remove(__flight_index);
                            true
                        } else {
                            false
                        }
                    };
                    {
                        b += 1.0;
                        b
                    };
                }
            }
            {
                let mut a = 0.0_f64;
                while (a < MAX_GAMEPAD_AXES) {
                    {
                        let __flight_key = ((data.gamepad * MAX_GAMEPAD_AXES) + a);
                        if let Some(__flight_index) = state
                            .axis_values
                            .iter()
                            .position(|(key, _)| key == &__flight_key)
                        {
                            state.axis_values.remove(__flight_index);
                            true
                        } else {
                            false
                        }
                    };
                    {
                        a += 1.0;
                        a
                    };
                }
            }
        }
    })
        as Box<dyn FnMut(InputGamepadConnectData) -> () + Send + 'static>));
    connect_signal(&mut manager.on_key_down, (on_key_down).clone(), None);
    connect_signal(&mut manager.on_key_up, (on_key_up).clone(), None);
    connect_signal(
        &mut manager.on_pointer_down,
        (on_pointer_down).clone(),
        None,
    );
    connect_signal(&mut manager.on_pointer_up, (on_pointer_up).clone(), None);
    connect_signal(
        &mut manager.on_pointer_cancel,
        (on_pointer_cancel).clone(),
        None,
    );
    connect_signal(
        &mut manager.on_gamepad_button_down,
        (on_gamepad_button_down).clone(),
        None,
    );
    connect_signal(
        &mut manager.on_gamepad_button_up,
        (on_gamepad_button_up).clone(),
        None,
    );
    connect_signal(
        &mut manager.on_gamepad_axis_move,
        (on_gamepad_axis_move).clone(),
        None,
    );
    connect_signal(
        &mut manager.on_gamepad_connect,
        (on_gamepad_connect).clone(),
        None,
    );
    connect_signal(
        &mut manager.on_gamepad_disconnect,
        (on_gamepad_disconnect).clone(),
        None,
    );
    return std::sync::Arc::new(std::sync::Mutex::new(Box::new({
        let mut manager = manager.clone();
        let on_gamepad_axis_move = on_gamepad_axis_move.clone();
        let on_gamepad_button_down = on_gamepad_button_down.clone();
        let on_gamepad_button_up = on_gamepad_button_up.clone();
        let on_gamepad_connect = on_gamepad_connect.clone();
        let on_gamepad_disconnect = on_gamepad_disconnect.clone();
        let on_key_down = on_key_down.clone();
        let on_key_up = on_key_up.clone();
        let on_pointer_cancel = on_pointer_cancel.clone();
        let on_pointer_down = on_pointer_down.clone();
        let on_pointer_up = on_pointer_up.clone();
        move || -> () {
            disconnect_signal(&mut manager.on_key_down, (on_key_down).clone());
            disconnect_signal(&mut manager.on_key_up, (on_key_up).clone());
            disconnect_signal(&mut manager.on_pointer_down, (on_pointer_down).clone());
            disconnect_signal(&mut manager.on_pointer_up, (on_pointer_up).clone());
            disconnect_signal(&mut manager.on_pointer_cancel, (on_pointer_cancel).clone());
            disconnect_signal(
                &mut manager.on_gamepad_button_down,
                (on_gamepad_button_down).clone(),
            );
            disconnect_signal(
                &mut manager.on_gamepad_button_up,
                (on_gamepad_button_up).clone(),
            );
            disconnect_signal(
                &mut manager.on_gamepad_axis_move,
                (on_gamepad_axis_move).clone(),
            );
            disconnect_signal(
                &mut manager.on_gamepad_connect,
                (on_gamepad_connect).clone(),
            );
            disconnect_signal(
                &mut manager.on_gamepad_disconnect,
                (on_gamepad_disconnect).clone(),
            );
        }
    })
        as Box<dyn FnMut() -> () + Send + 'static>));
}

// Source: upstream/packages/input/src/inputManager.ts:404 (sha256:90c42c4afc94ec852b083a75587f12dc5d9b51cc83c4171a632049535b3957e4)
pub fn create_input_key_repeat_timer(options: InputKeyRepeatOptions) -> InputKeyRepeatTimer {
    let delay_id: std::sync::Arc<std::sync::Mutex<Option<crate::FlightTimeout>>> =
        std::sync::Arc::new(std::sync::Mutex::new(None));
    let interval_id: std::sync::Arc<std::sync::Mutex<Option<crate::FlightTimeout>>> =
        std::sync::Arc::new(std::sync::Mutex::new(None));
    let mut stop: std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>> =
        std::sync::Arc::new(std::sync::Mutex::new(Box::new({
            let mut delay_id = delay_id.clone();
            let mut interval_id = interval_id.clone();
            move || -> () {
                if let Some(__flight_timer) = ((*delay_id.lock().unwrap()).clone()).clone() {
                    crate::clear_timeout(__flight_timer);
                };
                if let Some(__flight_timer) = ((*interval_id.lock().unwrap()).clone()).clone() {
                    crate::clear_interval(__flight_timer);
                };
                (*delay_id.lock().unwrap()) = None;
                (*interval_id.lock().unwrap()) = None;
            }
        })
            as Box<dyn FnMut() -> () + Send + 'static>));
    let mut start: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(
                        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>,
                    ) -> ()
                    + Send
                    + 'static,
            >,
        >,
    > = std::sync::Arc::new(std::sync::Mutex::new(Box::new({
        let mut delay_id = delay_id.clone();
        let mut interval_id = interval_id.clone();
        let options = options.clone();
        let stop = stop.clone();
        move |callback: std::sync::Arc<
            std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>,
        >|
              -> () {
            {
                let __flight_callback = (stop).clone();
                let __flight_result = __flight_callback.lock().unwrap()();
                __flight_result
            };
            {
                let __flight_callback = (callback).clone();
                let __flight_result = __flight_callback.lock().unwrap()();
                __flight_result
            };
            (*delay_id.lock().unwrap()) = Some(crate::set_timeout(
                {
                    let callback = callback.clone();
                    let mut interval_id = interval_id.clone();
                    let options = options.clone();
                    move || -> () {
                        {
                            let __flight_callback = (callback).clone();
                            let __flight_result = __flight_callback.lock().unwrap()();
                            __flight_result
                        };
                        (*interval_id.lock().unwrap()) = Some(crate::set_interval(
                            {
                                let __flight_callback = (callback).clone();
                                move || __flight_callback.lock().unwrap()()
                            },
                            options.interval,
                        ));
                    }
                },
                options.delay,
            ));
        }
    })
        as Box<
            dyn FnMut(
                    std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>,
                ) -> ()
                + Send
                + 'static,
        >));
    return InputKeyRepeatTimer {
        __flight_identity: std::sync::Arc::new(()),
        start: (start).clone(),
        stop: (stop).clone(),
    };
}

// Source: upstream/packages/input/src/inputManager.ts:427 (sha256:0aa752968160d69181a564725ef4c7db5551e0f7e5230cbdc0104a5d2081bd0f)
pub fn create_input_manager() -> InputManager {
    return {
        let __flight_spread_0 = create_input_signals();
        InputManager {
            __flight_identity: std::sync::Arc::new(()),
            on_gamepad_axis_move: (__flight_spread_0.on_gamepad_axis_move).clone(),
            on_gamepad_button_down: (__flight_spread_0.on_gamepad_button_down).clone(),
            on_gamepad_button_up: (__flight_spread_0.on_gamepad_button_up).clone(),
            on_gamepad_connect: (__flight_spread_0.on_gamepad_connect).clone(),
            on_gamepad_disconnect: (__flight_spread_0.on_gamepad_disconnect).clone(),
            on_key_down: (__flight_spread_0.on_key_down).clone(),
            on_key_up: (__flight_spread_0.on_key_up).clone(),
            on_pointer_cancel: (__flight_spread_0.on_pointer_cancel).clone(),
            on_pointer_down: (__flight_spread_0.on_pointer_down).clone(),
            on_pointer_move: (__flight_spread_0.on_pointer_move).clone(),
            on_pointer_move_relative: (__flight_spread_0.on_pointer_move_relative).clone(),
            on_pointer_up: (__flight_spread_0.on_pointer_up).clone(),
            on_text_edit: (__flight_spread_0.on_text_edit).clone(),
            on_text_input: (__flight_spread_0.on_text_input).clone(),
            on_wheel: (__flight_spread_0.on_wheel).clone(),
            enabled: true,
        }
    };
}

// Source: upstream/packages/input/src/inputManager.ts:434 (sha256:28fd446928eacd2fe3821deb30d07e652b9a1bf7539a9c5964dddcad0f5dfa6a)
pub fn create_input_signals() -> InputSignals {
    return InputSignals {
        __flight_identity: std::sync::Arc::new(()),
        on_gamepad_axis_move: create_signal(),
        on_gamepad_button_down: create_signal(),
        on_gamepad_button_up: create_signal(),
        on_gamepad_connect: create_signal(),
        on_gamepad_disconnect: create_signal(),
        on_key_down: create_signal(),
        on_key_up: create_signal(),
        on_pointer_cancel: create_signal(),
        on_pointer_down: create_signal(),
        on_pointer_move: create_signal(),
        on_pointer_move_relative: create_signal(),
        on_pointer_up: create_signal(),
        on_text_edit: create_signal(),
        on_text_input: create_signal(),
        on_wheel: create_signal(),
    };
}

// Source: upstream/packages/input/src/inputManager.ts:460 (sha256:441cf45f7800fc6117250e0f7ecdd82823a66096c43474e0588da5d3d635799e)
#[derive(Clone, Default)]
struct CreateInputStateRecord2 {
    __flight_identity: std::sync::Arc<()>,
    axis_values: Vec<(crate::OpaqueHostValue, crate::OpaqueHostValue)>,
    gamepad_buttons_down: Vec<crate::OpaqueHostValue>,
    just_pressed_gamepad_buttons: Vec<crate::OpaqueHostValue>,
    just_pressed_keys: Vec<crate::OpaqueHostValue>,
    just_released_gamepad_buttons: Vec<crate::OpaqueHostValue>,
    just_released_keys: Vec<crate::OpaqueHostValue>,
    keys_down: Vec<crate::OpaqueHostValue>,
    pointer_buttons_down: Vec<(crate::OpaqueHostValue, crate::OpaqueHostValue)>,
}
impl PartialEq for CreateInputStateRecord2 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn create_input_state() -> InputState {
    return InputState {
        __flight_identity: std::sync::Arc::new(()),
        axis_values: Vec::new(),
        gamepad_buttons_down: Vec::new(),
        just_pressed_gamepad_buttons: Vec::new(),
        just_pressed_keys: Vec::new(),
        just_released_gamepad_buttons: Vec::new(),
        just_released_keys: Vec::new(),
        keys_down: Vec::new(),
        pointer_buttons_down: Vec::new(),
    };
}

// Source: upstream/packages/input/src/inputManager.ts:473 (sha256:66b36f89561316a3951d6c87fe7f6c2ae7073a236622a5c68b8d21305ace13e1)
pub fn detach_gamepad_input(manager: &InputManager, target: crate::OpaqueHostValue) -> () {
    clear_input_binding(manager, (target).clone(), *K_GAMEPAD_INPUT);
}

// Source: upstream/packages/input/src/inputManager.ts:477 (sha256:d5b8c3900d26a5eb7d6ca5eb7457f25a81741bd75f2934058f9d1c32d51721e7)
pub fn detach_keyboard_input(manager: &InputManager, target: crate::OpaqueHostValue) -> () {
    clear_input_binding(manager, (target).clone(), *K_KEYBOARD_INPUT);
}

// Source: upstream/packages/input/src/inputManager.ts:481 (sha256:ced7ac3c2f9c07f910292178baa85ecf195dede98d0343a97e1dbcf6769a118a)
pub fn detach_pointer_input(manager: &InputManager, element: crate::OpaqueHostValue) -> () {
    clear_input_binding(manager, (element).clone(), *K_POINTER_INPUT);
}

// Source: upstream/packages/input/src/inputManager.ts:485 (sha256:c4068d67cb43ca3ff3b50edf82848ff9e99b018b69d5f9a88453cdde81577fd6)
pub fn detach_relative_pointer_input(
    manager: &InputManager,
    element: crate::OpaqueHostValue,
) -> () {
    clear_input_binding(manager, (element).clone(), *K_RELATIVE_POINTER_INPUT);
}

// Source: upstream/packages/input/src/inputManager.ts:489 (sha256:1083ecadaa9bf72e80f626a5d8ecbb8d5c8fc0a64ae288f34c20cf5c7e3eec05)
pub fn detach_text_input(manager: &InputManager, element: crate::OpaqueHostValue) -> () {
    clear_input_binding(manager, (element).clone(), *K_TEXT_INPUT);
}

// Source: upstream/packages/input/src/inputManager.ts:493 (sha256:4510e29911583595c35c77710497969fc3d24856a627d66e56a4dc5e80b43dc0)
pub fn detach_wheel_input(manager: &InputManager, element: crate::OpaqueHostValue) -> () {
    clear_input_binding(manager, (element).clone(), *K_WHEEL_INPUT);
}

// Source: upstream/packages/input/src/inputManager.ts:503 (sha256:62c3cee4a9a441a44a6e05892e37eb56d9f281561d6940b39564e6ea9ac768f3)
pub fn end_input_state_frame(state: &mut InputState) -> () {
    state.just_pressed_keys.clear();
    state.just_released_keys.clear();
    state.just_pressed_gamepad_buttons.clear();
    state.just_released_gamepad_buttons.clear();
}

// Source: upstream/packages/input/src/inputManager.ts:514 (sha256:b0e8e54a187375456c63f06547c688a9b96c1d871f166fa3b77adc1320bea706)
pub fn exit_input_pointer_lock() -> () {
    if match &(crate::host_value::<crate::OpaqueHostValue>("host.exitPointerLock")) {
        crate::OpaqueHostValue::Undefined | crate::OpaqueHostValue::Null => false,
        crate::OpaqueHostValue::Bool(value) => *value,
        crate::OpaqueHostValue::Number(value) => *value != 0.0_f64 && !value.is_nan(),
        crate::OpaqueHostValue::String(value) => !value.is_empty(),
        crate::OpaqueHostValue::Array(_)
        | crate::OpaqueHostValue::Record(_)
        | crate::OpaqueHostValue::Error { .. }
        | crate::OpaqueHostValue::Function
        | crate::OpaqueHostValue::Symbol
        | crate::OpaqueHostValue::Object => true,
    } {
        crate::host_value::<()>("host.exitPointerLock");
    }
}

// Source: upstream/packages/input/src/inputManager.ts:529 (sha256:ea357580bbf34d829d260e34eef2cf6061080fc037b231ca5b8912db5f601758)
pub fn get_coalesced_input_pointer_events(
    event: crate::OpaqueHostValue,
    callback: &mut impl FnMut(InputPointerData) -> (),
) -> () {
    let coalesced: Option<Vec<crate::OpaqueHostValue>> = None;
    {
        set_input_pointer_data(
            &mut (*_POINTER_DATA.lock().unwrap()),
            (event).clone(),
            0.0_f64,
            0.0_f64,
        );
        callback((*_POINTER_DATA.lock().unwrap()).clone());
    }
}

// Source: upstream/packages/input/src/inputManager.ts:550 (sha256:e8d36ceb5c77b295dc1490db8b25584a70321ab0b26e492acfc90a29740c3c51)
pub fn get_gamepad_axis_name(mapping: GamepadMappingKind, index: f64) -> Option<GamepadAxisKind> {
    if (mapping != "standard") {
        return None;
    }
    return _STANDARD_AXIS_NAMES[index as usize].clone();
}

// Source: upstream/packages/input/src/inputManager.ts:560 (sha256:12c52e4b463da3aa621c12dc6bb73400de93c1fe3bc9857c7474cb1becf4aa3b)
pub fn get_gamepad_button_name(
    mapping: GamepadMappingKind,
    index: f64,
) -> Option<GamepadButtonKind> {
    if (mapping != "standard") {
        return None;
    }
    return _STANDARD_BUTTON_NAMES[index as usize].clone();
}

// Source: upstream/packages/input/src/inputManager.ts:569 (sha256:3238dbf90b348d67d363c9e668848ead648deab6796beafa80490a64cf88efad)
pub fn get_input_gamepad_axis(state: &InputState, gamepad: f64, axis: f64) -> f64 {
    return (state
        .axis_values
        .iter()
        .find(|(entry_key, _)| entry_key == &((gamepad * MAX_GAMEPAD_AXES) + axis))
        .map(|(_, value)| value.clone()))
    .clone()
    .unwrap_or(0.0_f64);
}

// Source: upstream/packages/input/src/inputManager.ts:573 (sha256:6322b4324c1ed1744f5965e54719abaa3da6a7f2d8a1ee72cb5d30f6f39c532f)
pub fn get_key_code_from_dom_keyboard_event(event: crate::OpaqueHostValue) -> f64 {
    let code = get_key_code_from_dom_keyboard_code(
        crate::host_value::<String>("host.code"),
        crate::host_value::<f64>("host.location"),
    );
    if (code != KeyCode::UNKNOWN) {
        return code;
    }
    if (crate::host_value::<f64>("host.length") == 1.0_f64) {
        return crate::host_value::<f64>("host.call");
    }
    return (KEY_CODES_BY_KEY
        .iter()
        .find(|(entry_key, _)| entry_key == &crate::host_value::<String>("host.key"))
        .map(|(_, value)| value.clone())
        .clone())
    .clone()
    .unwrap_or(KeyCode::UNKNOWN);
}

// Source: upstream/packages/input/src/inputManager.ts:580 (sha256:8bb4b63cf36a75fad481d7886f69db5e02d1de022cc2ca3580802c252dee0474)
pub fn get_key_modifier_from_dom_keyboard_event(event: crate::OpaqueHostValue) -> f64 {
    let mut modifier = KeyModifier::NONE;
    if match &(crate::host_value::<crate::OpaqueHostValue>("host.altKey")) {
        crate::OpaqueHostValue::Undefined | crate::OpaqueHostValue::Null => false,
        crate::OpaqueHostValue::Bool(value) => *value,
        crate::OpaqueHostValue::Number(value) => *value != 0.0_f64 && !value.is_nan(),
        crate::OpaqueHostValue::String(value) => !value.is_empty(),
        crate::OpaqueHostValue::Array(_)
        | crate::OpaqueHostValue::Record(_)
        | crate::OpaqueHostValue::Error { .. }
        | crate::OpaqueHostValue::Function
        | crate::OpaqueHostValue::Symbol
        | crate::OpaqueHostValue::Object => true,
    } {
        modifier = (__flight_js_to_i32(modifier)
            | __flight_js_to_i32(
                if (crate::host_value::<crate::OpaqueHostValue>("host.location")
                    == crate::host_value::<crate::OpaqueHostValue>("host.DOM_KEY_LOCATION_RIGHT"))
                {
                    KeyModifier::RIGHT_ALT
                } else {
                    KeyModifier::LEFT_ALT
                },
            )) as f64;
    }
    if match &(crate::host_value::<crate::OpaqueHostValue>("host.ctrlKey")) {
        crate::OpaqueHostValue::Undefined | crate::OpaqueHostValue::Null => false,
        crate::OpaqueHostValue::Bool(value) => *value,
        crate::OpaqueHostValue::Number(value) => *value != 0.0_f64 && !value.is_nan(),
        crate::OpaqueHostValue::String(value) => !value.is_empty(),
        crate::OpaqueHostValue::Array(_)
        | crate::OpaqueHostValue::Record(_)
        | crate::OpaqueHostValue::Error { .. }
        | crate::OpaqueHostValue::Function
        | crate::OpaqueHostValue::Symbol
        | crate::OpaqueHostValue::Object => true,
    } {
        modifier = (__flight_js_to_i32(modifier)
            | __flight_js_to_i32(
                if (crate::host_value::<crate::OpaqueHostValue>("host.location")
                    == crate::host_value::<crate::OpaqueHostValue>("host.DOM_KEY_LOCATION_RIGHT"))
                {
                    KeyModifier::RIGHT_CTRL
                } else {
                    KeyModifier::LEFT_CTRL
                },
            )) as f64;
    }
    if match &(crate::host_value::<crate::OpaqueHostValue>("host.metaKey")) {
        crate::OpaqueHostValue::Undefined | crate::OpaqueHostValue::Null => false,
        crate::OpaqueHostValue::Bool(value) => *value,
        crate::OpaqueHostValue::Number(value) => *value != 0.0_f64 && !value.is_nan(),
        crate::OpaqueHostValue::String(value) => !value.is_empty(),
        crate::OpaqueHostValue::Array(_)
        | crate::OpaqueHostValue::Record(_)
        | crate::OpaqueHostValue::Error { .. }
        | crate::OpaqueHostValue::Function
        | crate::OpaqueHostValue::Symbol
        | crate::OpaqueHostValue::Object => true,
    } {
        modifier = (__flight_js_to_i32(modifier)
            | __flight_js_to_i32(
                if (crate::host_value::<crate::OpaqueHostValue>("host.location")
                    == crate::host_value::<crate::OpaqueHostValue>("host.DOM_KEY_LOCATION_RIGHT"))
                {
                    KeyModifier::RIGHT_META
                } else {
                    KeyModifier::LEFT_META
                },
            )) as f64;
    }
    if match &(crate::host_value::<crate::OpaqueHostValue>("host.shiftKey")) {
        crate::OpaqueHostValue::Undefined | crate::OpaqueHostValue::Null => false,
        crate::OpaqueHostValue::Bool(value) => *value,
        crate::OpaqueHostValue::Number(value) => *value != 0.0_f64 && !value.is_nan(),
        crate::OpaqueHostValue::String(value) => !value.is_empty(),
        crate::OpaqueHostValue::Array(_)
        | crate::OpaqueHostValue::Record(_)
        | crate::OpaqueHostValue::Error { .. }
        | crate::OpaqueHostValue::Function
        | crate::OpaqueHostValue::Symbol
        | crate::OpaqueHostValue::Object => true,
    } {
        modifier = (__flight_js_to_i32(modifier)
            | __flight_js_to_i32(
                if (crate::host_value::<crate::OpaqueHostValue>("host.location")
                    == crate::host_value::<crate::OpaqueHostValue>("host.DOM_KEY_LOCATION_RIGHT"))
                {
                    KeyModifier::RIGHT_SHIFT
                } else {
                    KeyModifier::LEFT_SHIFT
                },
            )) as f64;
    }
    if (Some(false)) == Some(true) {
        modifier =
            (__flight_js_to_i32(modifier) | __flight_js_to_i32(KeyModifier::CAPS_LOCK)) as f64;
    }
    if (Some(false)) == Some(true) {
        modifier =
            (__flight_js_to_i32(modifier) | __flight_js_to_i32(KeyModifier::NUM_LOCK)) as f64;
    }
    return modifier;
}

// Source: upstream/packages/input/src/inputManager.ts:598 (sha256:090ca68d3668c36765ea7dd8180d9a3c7c8f2f10784d2b641c49ae8948515a54)
pub fn get_mouse_wheel_mode_from_dom_wheel_event(event: crate::OpaqueHostValue) -> MouseWheelMode {
    if (crate::host_value::<crate::OpaqueHostValue>("host.deltaMode")
        == crate::host_value::<crate::OpaqueHostValue>("host.DOM_DELTA_PIXEL"))
    {
        return "pixels".to_owned();
    }
    if (crate::host_value::<crate::OpaqueHostValue>("host.deltaMode")
        == crate::host_value::<crate::OpaqueHostValue>("host.DOM_DELTA_LINE"))
    {
        return "lines".to_owned();
    }
    if (crate::host_value::<crate::OpaqueHostValue>("host.deltaMode")
        == crate::host_value::<crate::OpaqueHostValue>("host.DOM_DELTA_PAGE"))
    {
        return "pages".to_owned();
    }
    return "unknown".to_owned();
}

// Source: upstream/packages/input/src/inputManager.ts:608 (sha256:5ad2cbeaa23397e4827f4ba47fc0aaf28eb27a85617e7cd6e86e8984ef32acf7)
pub fn has_input_pointer_lock() -> bool {
    return (crate::host_value::<Option<crate::OpaqueHostValue>>("host.pointerLockElement"))
        .is_some();
}

// Source: upstream/packages/input/src/inputManager.ts:616 (sha256:d927dcd51a3a70e13037227050648a1592dcf23ea1964f3017b0c965da8a6cae)
pub fn is_input_gamepad_button_down(state: &InputState, gamepad: f64, button: f64) -> bool {
    return state
        .gamepad_buttons_down
        .iter()
        .any(|item| item == &((gamepad * MAX_GAMEPAD_BUTTONS) + button));
}

// Source: upstream/packages/input/src/inputManager.ts:623 (sha256:6d7f88fbc380d7172c07cb7191aaf013b0052c41e1211f27ce9055ad2ce38f7c)
pub fn is_input_key_down(state: &InputState, key_code: f64) -> bool {
    return state.keys_down.iter().any(|item| item == &key_code);
}

// Source: upstream/packages/input/src/inputManager.ts:631 (sha256:7185bc6004109ad1c07574ab6d5f53e475c20a75ef4e6524cad9e345265891bf)
pub fn is_input_pointer_button_down(state: &InputState, pointer_id: f64, button: f64) -> bool {
    return ((__flight_js_to_i32(
        (state
            .pointer_buttons_down
            .iter()
            .find(|(entry_key, _)| entry_key == &pointer_id)
            .map(|(_, value)| value.clone()))
        .clone()
        .unwrap_or(0.0_f64),
    ) & __flight_js_to_i32(
        __flight_js_to_i32(1.0_f64).wrapping_shl((__flight_js_to_u32(button) & 31)) as f64,
    )) as f64
        != 0.0_f64);
}

// Source: upstream/packages/input/src/inputManager.ts:635 (sha256:d597a9a5a2cb16b6ff0e355fb6b92d80b1ca6acb8bb1154b2a48a5964e9120c3)
pub fn poll_gamepad_input(manager: &InputManager) -> () {
    if (!manager.enabled) || ("function".to_owned() != "function") {
        return;
    }
    let now = crate::host_value::<f64>("host.call");
    let mut prev = get_or_create_gamepad_poll_state(manager);
    let gamepads = crate::host_value::<Vec<Option<crate::OpaqueHostValue>>>("host.call");
    for pad in (gamepads).iter().cloned() {
        if (pad).is_none() {
            continue;
        }
        let mut prev_axes = (prev
            .axes
            .iter()
            .find(|(entry_key, _)| entry_key == &crate::host_value::<f64>("host.index"))
            .map(|(_, value)| value.clone()))
        .clone()
        .unwrap_or(vec![]);
        let mut prev_buttons = (prev
            .buttons
            .iter()
            .find(|(entry_key, _)| entry_key == &crate::host_value::<f64>("host.index"))
            .map(|(_, value)| value.clone()))
        .clone()
        .unwrap_or(vec![]);
        {
            let mut i = 0.0_f64;
            while (i < crate::host_value::<f64>("host.length")) {
                let value = crate::host_value::<f64>("host.index");
                if (value != prev_axes[i as usize].clone()) {
                    {
                        let __flight_index = (i) as usize;
                        let __flight_value = value;
                        if __flight_index == prev_axes.len() {
                            prev_axes.push(__flight_value);
                        } else {
                            prev_axes[__flight_index] = __flight_value;
                        }
                    };
                    (*_AXIS_DATA.lock().unwrap()).axis = i;
                    (*_AXIS_DATA.lock().unwrap()).gamepad = crate::host_value::<f64>("host.index");
                    (*_AXIS_DATA.lock().unwrap()).time_stamp = now;
                    (*_AXIS_DATA.lock().unwrap()).value = value;
                    emit_signal(
                        (manager.on_gamepad_axis_move).clone(),
                        ((*_AXIS_DATA.lock().unwrap()).clone(),),
                    );
                }
                {
                    i += 1.0;
                    i
                };
            }
        }
        {
            let mut i = 0.0_f64;
            while (i < crate::host_value::<f64>("host.length")) {
                let btn = crate::host_value::<crate::OpaqueHostValue>("host.index");
                let was_pressed = prev_buttons[i as usize].clone();
                if (crate::host_value::<bool>("host.pressed") != was_pressed) {
                    {
                        let __flight_index = (i) as usize;
                        let __flight_value = crate::host_value::<bool>("host.pressed");
                        if __flight_index == prev_buttons.len() {
                            prev_buttons.push(__flight_value);
                        } else {
                            prev_buttons[__flight_index] = __flight_value;
                        }
                    };
                    (*_BUTTON_DATA.lock().unwrap()).button = i;
                    (*_BUTTON_DATA.lock().unwrap()).gamepad =
                        crate::host_value::<f64>("host.index");
                    (*_BUTTON_DATA.lock().unwrap()).time_stamp = now;
                    (*_BUTTON_DATA.lock().unwrap()).value = crate::host_value::<f64>("host.value");
                    if match &(crate::host_value::<crate::OpaqueHostValue>("host.pressed")) {
                        crate::OpaqueHostValue::Undefined | crate::OpaqueHostValue::Null => false,
                        crate::OpaqueHostValue::Bool(value) => *value,
                        crate::OpaqueHostValue::Number(value) => {
                            *value != 0.0_f64 && !value.is_nan()
                        }
                        crate::OpaqueHostValue::String(value) => !value.is_empty(),
                        crate::OpaqueHostValue::Array(_)
                        | crate::OpaqueHostValue::Record(_)
                        | crate::OpaqueHostValue::Error { .. }
                        | crate::OpaqueHostValue::Function
                        | crate::OpaqueHostValue::Symbol
                        | crate::OpaqueHostValue::Object => true,
                    } {
                        emit_signal(
                            (manager.on_gamepad_button_down).clone(),
                            ((*_BUTTON_DATA.lock().unwrap()).clone(),),
                        );
                    } else {
                        emit_signal(
                            (manager.on_gamepad_button_up).clone(),
                            ((*_BUTTON_DATA.lock().unwrap()).clone(),),
                        );
                    }
                }
                {
                    i += 1.0;
                    i
                };
            }
        }
        {
            let __flight_key = crate::host_value::<f64>("host.index");
            let __flight_value = (prev_axes).clone();
            if let Some((_, value)) = prev.axes.iter_mut().find(|(key, _)| key == &__flight_key) {
                *value = __flight_value;
            } else {
                prev.axes.push((__flight_key, __flight_value));
            }
        };
        {
            let __flight_key = crate::host_value::<f64>("host.index");
            let __flight_value = (prev_buttons).clone();
            if let Some((_, value)) = prev
                .buttons
                .iter_mut()
                .find(|(key, _)| key == &__flight_key)
            {
                *value = __flight_value;
            } else {
                prev.buttons.push((__flight_key, __flight_value));
            }
        };
    }
}

// Source: upstream/packages/input/src/inputManager.ts:681 (sha256:d9865171e25134e43ba58ab9893d56d2c53f6885931d530ac6760a1fc39d60c9)
pub fn release_input_pointer_capture(element: crate::OpaqueHostValue, pointer_id: f64) -> () {
    match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        crate::host_value::<()>("host.releasePointerCapture");
    })) {
        Ok(_) => {}
        Err(_) => {}
    }
}

// Source: upstream/packages/input/src/inputManager.ts:694 (sha256:fddeaa7d2196a8dfbfc055b2cf368b9f22f58ae07443530998152414ca132438)
pub fn request_input_pointer_lock(element: crate::OpaqueHostValue) -> crate::FlightTask<bool> {
    let __flight_try_return: Option<crate::FlightTask<bool>> = match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| -> Option<crate::FlightTask<bool>> {
            {
                let result = crate::host_value::<()>("host.requestPointerLock");
                if false {
                    return Some(crate::host_task::<bool>("host.then"));
                }
                return Some(crate::FlightTask::ready(
                    true,
                    crate::FlightTaskOrigin {
                        package: "@flighthq/input",
                        source: "upstream/packages/input/src/inputManager.ts",
                        line: 703_u32,
                        column: 12_u32,
                        lexical_path: "requestInputPointerLock.ready:703:12:1a7aba0165f0",
                        fingerprint: "sha256:1a7aba0165f00509e11ac8dc73642aece499bb5941df4401f21b933076ae3787",
                    },
                ));
            }
            None
        }),
    ) {
        Ok(value) => value,
        Err(_) => (|| -> Option<crate::FlightTask<bool>> {
            {
                return Some(crate::FlightTask::ready(
                    false,
                    crate::FlightTaskOrigin {
                        package: "@flighthq/input",
                        source: "upstream/packages/input/src/inputManager.ts",
                        line: 705_u32,
                        column: 12_u32,
                        lexical_path: "requestInputPointerLock.ready:705:12:a913f3cb1f97",
                        fingerprint: "sha256:a913f3cb1f9734d159588904231d21b38dd02a4b990de3f76aaea042e535d166",
                    },
                ));
            }
            None
        })(),
    };
    return __flight_try_return.expect("TypeScript try/catch completed without returning");
}

// Source: upstream/packages/input/src/inputManager.ts:714 (sha256:1dfebf0bf8c055f63dcbff30aa18a098b528db1080a250015c7f66e20c281b04)
pub fn set_input_pointer_capture(element: crate::OpaqueHostValue, pointer_id: f64) -> () {
    crate::host_value::<()>("host.setPointerCapture");
}

// Source: upstream/packages/input/src/inputManager.ts:723 (sha256:6b9b431bc0a5aa662e0c1eef8d9104f8c374133131aaf7d247b3438c9e973e8a)
pub fn was_input_gamepad_button_pressed(state: &InputState, gamepad: f64, button: f64) -> bool {
    return state
        .just_pressed_gamepad_buttons
        .iter()
        .any(|item| item == &((gamepad * MAX_GAMEPAD_BUTTONS) + button));
}

// Source: upstream/packages/input/src/inputManager.ts:732 (sha256:62eadb4fb5dd9d10cf361cae0525eb6af3c6595c94601c2b95a37229a1e93f80)
pub fn was_input_gamepad_button_released(state: &InputState, gamepad: f64, button: f64) -> bool {
    return state
        .just_released_gamepad_buttons
        .iter()
        .any(|item| item == &((gamepad * MAX_GAMEPAD_BUTTONS) + button));
}

// Source: upstream/packages/input/src/inputManager.ts:740 (sha256:68f1833b2a1a1963e7e50ecfa9a65a0d7c961e8a3fa44a3c43221fae3bfadfed)
pub fn was_input_key_pressed(state: &InputState, key_code: f64) -> bool {
    return state.just_pressed_keys.iter().any(|item| item == &key_code);
}

// Source: upstream/packages/input/src/inputManager.ts:748 (sha256:89eda631688c9a03f610cf3f9af447cb258dec426db2e6e2fac397449b6535c7)
pub fn was_input_key_released(state: &InputState, key_code: f64) -> bool {
    return state
        .just_released_keys
        .iter()
        .any(|item| item == &key_code);
}

// Source: upstream/packages/input/src/inputManager.ts:752 (sha256:cc9a775df4f4bbbd6043540597797cb0b55fc8f23d5a8ad5f14c6cef55e4b82e)
fn get_key_code_from_dom_keyboard_code(code: String, location: f64) -> f64 {
    if (location == crate::host_value::<f64>("host.DOM_KEY_LOCATION_NUMPAD"))
        && ({
            let __flight_key = (code).clone();
            NUMPAD_KEY_CODES_BY_CODE
                .iter()
                .any(|(key, _)| key == &__flight_key)
        })
    {
        return NUMPAD_KEY_CODES_BY_CODE
            .iter()
            .find(|(entry_key, _)| entry_key == &(code).clone())
            .map(|(_, value)| value.clone())
            .clone()
            .unwrap();
    }
    return (KEY_CODES_BY_CODE
        .iter()
        .find(|(entry_key, _)| entry_key == &(code).clone())
        .map(|(_, value)| value.clone())
        .clone())
    .clone()
    .unwrap_or(KeyCode::UNKNOWN);
}

// Source: upstream/packages/input/src/inputManager.ts:759 (sha256:40ca75bc1bd4bfd77ec9b3bbaf7470427dbec0de9c3cc45156b4da748285bbd8)
fn get_pointer_type_from_dom_pointer_event(event: crate::OpaqueHostValue) -> String {
    return if ((crate::host_value::<String>("host.pointerType") == "mouse")
        || (crate::host_value::<String>("host.pointerType") == "pen"))
        || (crate::host_value::<String>("host.pointerType") == "touch")
    {
        crate::host_value::<String>("host.pointerType")
    } else {
        "unknown".to_owned()
    };
}

// Source: upstream/packages/input/src/inputManager.ts:765 (sha256:05999ef21f0a13c109c2b1b7a63d75b7fa006d9ca93a3429730edeecb15eddee)
fn set_input_keyboard_data(out: &mut InputKeyboardData, event: crate::OpaqueHostValue) -> () {
    let modifier = get_key_modifier_from_dom_keyboard_event((event).clone());
    out.alt_key = crate::host_value::<bool>("host.altKey");
    out.caps_lock = ((__flight_js_to_i32(modifier) & __flight_js_to_i32(KeyModifier::CAPS_LOCK))
        as f64
        != 0.0_f64);
    out.code = crate::host_value::<String>("host.code");
    out.ctrl_key = crate::host_value::<bool>("host.ctrlKey");
    out.key = crate::host_value::<String>("host.key");
    out.key_code = get_key_code_from_dom_keyboard_event((event).clone());
    out.location = crate::host_value::<f64>("host.location");
    out.meta_key = crate::host_value::<bool>("host.metaKey");
    out.modifier = modifier;
    out.num_lock = ((__flight_js_to_i32(modifier) & __flight_js_to_i32(KeyModifier::NUM_LOCK))
        as f64
        != 0.0_f64);
    out.repeat = crate::host_value::<bool>("host.repeat");
    out.shift_key = crate::host_value::<bool>("host.shiftKey");
    out.time_stamp = crate::host_value::<f64>("host.timeStamp");
}

// Source: upstream/packages/input/src/inputManager.ts:782 (sha256:b71b84e0464aa0c5c1646f3383784c6cf567c5b1c1b98213d71d63fff42fc607)
fn set_input_pointer_data(
    out: &mut InputPointerData,
    event: crate::OpaqueHostValue,
    delta_x: f64,
    delta_y: f64,
) -> () {
    out.alt_key = crate::host_value::<bool>("host.altKey");
    out.button = crate::host_value::<f64>("host.button");
    out.buttons = crate::host_value::<f64>("host.buttons");
    out.ctrl_key = crate::host_value::<bool>("host.ctrlKey");
    out.delta_x = delta_x;
    out.delta_y = delta_y;
    out.height = if false {
        crate::host_value::<f64>("host.height")
    } else {
        1.0_f64
    };
    out.is_primary = if false {
        crate::host_value::<bool>("host.isPrimary")
    } else {
        true
    };
    out.meta_key = crate::host_value::<bool>("host.metaKey");
    out.pointer_id = if false {
        crate::host_value::<f64>("host.pointerId")
    } else {
        0.0_f64
    };
    out.pointer_type = if false {
        get_pointer_type_from_dom_pointer_event((event).clone())
    } else {
        "mouse".to_owned()
    };
    out.pressure = if false {
        crate::host_value::<f64>("host.pressure")
    } else {
        0.0_f64
    };
    out.shift_key = crate::host_value::<bool>("host.shiftKey");
    out.tilt_x = if false {
        crate::host_value::<f64>("host.tiltX")
    } else {
        0.0_f64
    };
    out.tilt_y = if false {
        crate::host_value::<f64>("host.tiltY")
    } else {
        0.0_f64
    };
    out.time_stamp = crate::host_value::<f64>("host.timeStamp");
    out.twist = if false {
        crate::host_value::<f64>("host.twist")
    } else {
        0.0_f64
    };
    out.wheel_mode = "unknown".to_owned();
    out.width = if false {
        crate::host_value::<f64>("host.width")
    } else {
        1.0_f64
    };
    out.x = crate::host_value::<f64>("host.clientX");
    out.y = crate::host_value::<f64>("host.clientY");
}

// Source: upstream/packages/input/src/inputManager.ts:812 (sha256:374bdd0870bcb6c604033c20be13c0460a5a0d89a0e4304ce55eaafddf580b2e)
static _STANDARD_BUTTON_NAMES: std::sync::LazyLock<Vec<Option<GamepadButtonKind>>> =
    std::sync::LazyLock::new(|| {
        vec![
            Some((gamepad_button_kind_values_constant.button_south).clone()),
            Some((gamepad_button_kind_values_constant.button_east).clone()),
            Some((gamepad_button_kind_values_constant.button_west).clone()),
            Some((gamepad_button_kind_values_constant.button_north).clone()),
            Some((gamepad_button_kind_values_constant.shoulder_left).clone()),
            Some((gamepad_button_kind_values_constant.shoulder_right).clone()),
            Some((gamepad_button_kind_values_constant.trigger_left).clone()),
            Some((gamepad_button_kind_values_constant.trigger_right).clone()),
            Some((gamepad_button_kind_values_constant.select).clone()),
            Some((gamepad_button_kind_values_constant.start).clone()),
            Some((gamepad_button_kind_values_constant.stick_left).clone()),
            Some((gamepad_button_kind_values_constant.stick_right).clone()),
            Some((gamepad_button_kind_values_constant.dpad_up).clone()),
            Some((gamepad_button_kind_values_constant.dpad_down).clone()),
            Some((gamepad_button_kind_values_constant.dpad_left).clone()),
            Some((gamepad_button_kind_values_constant.dpad_right).clone()),
            Some((gamepad_button_kind_values_constant.home).clone()),
            Some((gamepad_button_kind_values_constant.touchpad).clone()),
        ]
    });

// Source: upstream/packages/input/src/inputManager.ts:834 (sha256:2cc9fdc0389d35de9c933a25ac0636c096c81c916622a99e8088aee49589332d)
static _STANDARD_AXIS_NAMES: std::sync::LazyLock<Vec<Option<GamepadAxisKind>>> =
    std::sync::LazyLock::new(|| {
        vec![
            Some((gamepad_axis_kind_values_constant.stick_left_x).clone()),
            Some((gamepad_axis_kind_values_constant.stick_left_y).clone()),
            Some((gamepad_axis_kind_values_constant.stick_right_x).clone()),
            Some((gamepad_axis_kind_values_constant.stick_right_y).clone()),
        ]
    });

// Source: upstream/packages/input/src/inputManager.ts:843 (sha256:0f4337acae92099459a68ae1f92e940ed8756e81d2a0fe32542bb626d6e8c910)
static KEY_CODES_BY_CODE: std::sync::LazyLock<Vec<(String, f64)>> =
    std::sync::LazyLock::new(|| {
        let mut __flight_record = Vec::new();
        __flight_record.push(("Again".to_owned(), KeyCode::AGAIN));
        __flight_record.push(("AltLeft".to_owned(), KeyCode::LEFT_ALT));
        __flight_record.push(("AltRight".to_owned(), KeyCode::RIGHT_ALT));
        __flight_record.push(("ArrowDown".to_owned(), KeyCode::DOWN));
        __flight_record.push(("ArrowLeft".to_owned(), KeyCode::LEFT));
        __flight_record.push(("ArrowRight".to_owned(), KeyCode::RIGHT));
        __flight_record.push(("ArrowUp".to_owned(), KeyCode::UP));
        __flight_record.push(("AudioVolumeDown".to_owned(), KeyCode::AUDIO_MUTE));
        __flight_record.push(("Backspace".to_owned(), KeyCode::BACKSPACE));
        __flight_record.push(("BrowserBack".to_owned(), KeyCode::APP_CONTROL_BACK));
        __flight_record.push((
            "BrowserBookmarks".to_owned(),
            KeyCode::APP_CONTROL_BOOKMARKS,
        ));
        __flight_record.push(("BrowserForward".to_owned(), KeyCode::APP_CONTROL_FORWARD));
        __flight_record.push(("BrowserHome".to_owned(), KeyCode::APP_CONTROL_HOME));
        __flight_record.push(("BrowserRefresh".to_owned(), KeyCode::APP_CONTROL_REFRESH));
        __flight_record.push(("BrowserSearch".to_owned(), KeyCode::APP_CONTROL_SEARCH));
        __flight_record.push(("BrowserStop".to_owned(), KeyCode::APP_CONTROL_STOP));
        __flight_record.push(("CapsLock".to_owned(), KeyCode::CAPS_LOCK));
        __flight_record.push(("ContextMenu".to_owned(), KeyCode::APPLICATION));
        __flight_record.push(("ControlLeft".to_owned(), KeyCode::LEFT_CTRL));
        __flight_record.push(("ControlRight".to_owned(), KeyCode::RIGHT_CTRL));
        __flight_record.push(("Convert".to_owned(), KeyCode::UNKNOWN));
        __flight_record.push(("Copy".to_owned(), KeyCode::COPY));
        __flight_record.push(("Cut".to_owned(), KeyCode::CUT));
        __flight_record.push(("Delete".to_owned(), KeyCode::DELETE));
        __flight_record.push(("Eject".to_owned(), KeyCode::EJECT));
        __flight_record.push(("End".to_owned(), KeyCode::END));
        __flight_record.push(("Enter".to_owned(), KeyCode::RETURN));
        __flight_record.push(("Escape".to_owned(), KeyCode::ESCAPE));
        __flight_record.push(("F1".to_owned(), KeyCode::F1));
        __flight_record.push(("F2".to_owned(), KeyCode::F2));
        __flight_record.push(("F3".to_owned(), KeyCode::F3));
        __flight_record.push(("F4".to_owned(), KeyCode::F4));
        __flight_record.push(("F5".to_owned(), KeyCode::F5));
        __flight_record.push(("F6".to_owned(), KeyCode::F6));
        __flight_record.push(("F7".to_owned(), KeyCode::F7));
        __flight_record.push(("F8".to_owned(), KeyCode::F8));
        __flight_record.push(("F9".to_owned(), KeyCode::F9));
        __flight_record.push(("F10".to_owned(), KeyCode::F10));
        __flight_record.push(("F11".to_owned(), KeyCode::F11));
        __flight_record.push(("F12".to_owned(), KeyCode::F12));
        __flight_record.push(("F13".to_owned(), KeyCode::F13));
        __flight_record.push(("F14".to_owned(), KeyCode::F14));
        __flight_record.push(("F15".to_owned(), KeyCode::F15));
        __flight_record.push(("F16".to_owned(), KeyCode::F16));
        __flight_record.push(("F17".to_owned(), KeyCode::F17));
        __flight_record.push(("F18".to_owned(), KeyCode::F18));
        __flight_record.push(("F19".to_owned(), KeyCode::F19));
        __flight_record.push(("F20".to_owned(), KeyCode::F20));
        __flight_record.push(("F21".to_owned(), KeyCode::F21));
        __flight_record.push(("F22".to_owned(), KeyCode::F22));
        __flight_record.push(("F23".to_owned(), KeyCode::F23));
        __flight_record.push(("F24".to_owned(), KeyCode::F24));
        __flight_record.push(("Find".to_owned(), KeyCode::FIND));
        __flight_record.push(("Help".to_owned(), KeyCode::HELP));
        __flight_record.push(("Home".to_owned(), KeyCode::HOME));
        __flight_record.push(("Insert".to_owned(), KeyCode::INSERT));
        __flight_record.push(("IntlBackslash".to_owned(), KeyCode::BACKSLASH));
        __flight_record.push(("LaunchApp1".to_owned(), KeyCode::COMPUTER));
        __flight_record.push(("LaunchApp2".to_owned(), KeyCode::CALCULATOR));
        __flight_record.push(("LaunchMail".to_owned(), KeyCode::MAIL));
        __flight_record.push(("LaunchMediaPlayer".to_owned(), KeyCode::MEDIA_SELECT));
        __flight_record.push(("MediaPlayPause".to_owned(), KeyCode::AUDIO_PLAY));
        __flight_record.push(("MediaStop".to_owned(), KeyCode::AUDIO_STOP));
        __flight_record.push(("MediaTrackNext".to_owned(), KeyCode::AUDIO_NEXT));
        __flight_record.push(("MediaTrackPrevious".to_owned(), KeyCode::AUDIO_PREVIOUS));
        __flight_record.push(("MetaLeft".to_owned(), KeyCode::LEFT_META));
        __flight_record.push(("MetaRight".to_owned(), KeyCode::RIGHT_META));
        __flight_record.push(("NonConvert".to_owned(), KeyCode::UNKNOWN));
        __flight_record.push(("NumLock".to_owned(), KeyCode::NUM_LOCK));
        __flight_record.push(("PageDown".to_owned(), KeyCode::PAGE_DOWN));
        __flight_record.push(("PageUp".to_owned(), KeyCode::PAGE_UP));
        __flight_record.push(("Paste".to_owned(), KeyCode::PASTE));
        __flight_record.push(("Pause".to_owned(), KeyCode::PAUSE));
        __flight_record.push(("Power".to_owned(), KeyCode::POWER));
        __flight_record.push(("PrintScreen".to_owned(), KeyCode::PRINT_SCREEN));
        __flight_record.push(("ScrollLock".to_owned(), KeyCode::SCROLL_LOCK));
        __flight_record.push(("Select".to_owned(), KeyCode::SELECT));
        __flight_record.push(("ShiftLeft".to_owned(), KeyCode::LEFT_SHIFT));
        __flight_record.push(("ShiftRight".to_owned(), KeyCode::RIGHT_SHIFT));
        __flight_record.push(("Sleep".to_owned(), KeyCode::SLEEP));
        __flight_record.push(("Space".to_owned(), KeyCode::SPACE));
        __flight_record.push(("Tab".to_owned(), KeyCode::TAB));
        __flight_record.push(("Undo".to_owned(), KeyCode::UNDO));
        __flight_record.push(("VolumeDown".to_owned(), KeyCode::VOLUME_DOWN));
        __flight_record.push(("VolumeMute".to_owned(), KeyCode::AUDIO_MUTE));
        __flight_record.push(("VolumeUp".to_owned(), KeyCode::VOLUME_UP));
        __flight_record.push(("WakeUp".to_owned(), KeyCode::UNKNOWN));
        __flight_record.push(("WWW".to_owned(), KeyCode::WWW));
        __flight_record
    });

// Source: upstream/packages/input/src/inputManager.ts:935 (sha256:104c63197539a394c06522f31880f2f39c73eb2241a9a0ebd27f049f1c6cb401)
static KEY_CODES_BY_KEY: std::sync::LazyLock<Vec<(String, f64)>> = std::sync::LazyLock::new(|| {
    let mut __flight_record = Vec::new();
    __flight_record.push(("Alt".to_owned(), KeyCode::LEFT_ALT));
    __flight_record.push(("ArrowDown".to_owned(), KeyCode::DOWN));
    __flight_record.push(("ArrowLeft".to_owned(), KeyCode::LEFT));
    __flight_record.push(("ArrowRight".to_owned(), KeyCode::RIGHT));
    __flight_record.push(("ArrowUp".to_owned(), KeyCode::UP));
    __flight_record.push(("Backspace".to_owned(), KeyCode::BACKSPACE));
    __flight_record.push(("CapsLock".to_owned(), KeyCode::CAPS_LOCK));
    __flight_record.push(("Control".to_owned(), KeyCode::LEFT_CTRL));
    __flight_record.push(("Delete".to_owned(), KeyCode::DELETE));
    __flight_record.push(("End".to_owned(), KeyCode::END));
    __flight_record.push(("Enter".to_owned(), KeyCode::RETURN));
    __flight_record.push(("Escape".to_owned(), KeyCode::ESCAPE));
    __flight_record.push(("Home".to_owned(), KeyCode::HOME));
    __flight_record.push(("Insert".to_owned(), KeyCode::INSERT));
    __flight_record.push(("Meta".to_owned(), KeyCode::LEFT_META));
    __flight_record.push(("NumLock".to_owned(), KeyCode::NUM_LOCK));
    __flight_record.push(("PageDown".to_owned(), KeyCode::PAGE_DOWN));
    __flight_record.push(("PageUp".to_owned(), KeyCode::PAGE_UP));
    __flight_record.push(("Pause".to_owned(), KeyCode::PAUSE));
    __flight_record.push(("PrintScreen".to_owned(), KeyCode::PRINT_SCREEN));
    __flight_record.push(("ScrollLock".to_owned(), KeyCode::SCROLL_LOCK));
    __flight_record.push(("Shift".to_owned(), KeyCode::LEFT_SHIFT));
    __flight_record.push(("Tab".to_owned(), KeyCode::TAB));
    __flight_record.push(("F1".to_owned(), KeyCode::F1));
    __flight_record.push(("F2".to_owned(), KeyCode::F2));
    __flight_record.push(("F3".to_owned(), KeyCode::F3));
    __flight_record.push(("F4".to_owned(), KeyCode::F4));
    __flight_record.push(("F5".to_owned(), KeyCode::F5));
    __flight_record.push(("F6".to_owned(), KeyCode::F6));
    __flight_record.push(("F7".to_owned(), KeyCode::F7));
    __flight_record.push(("F8".to_owned(), KeyCode::F8));
    __flight_record.push(("F9".to_owned(), KeyCode::F9));
    __flight_record.push(("F10".to_owned(), KeyCode::F10));
    __flight_record.push(("F11".to_owned(), KeyCode::F11));
    __flight_record.push(("F12".to_owned(), KeyCode::F12));
    __flight_record.push(("F13".to_owned(), KeyCode::F13));
    __flight_record.push(("F14".to_owned(), KeyCode::F14));
    __flight_record.push(("F15".to_owned(), KeyCode::F15));
    __flight_record.push(("F16".to_owned(), KeyCode::F16));
    __flight_record.push(("F17".to_owned(), KeyCode::F17));
    __flight_record.push(("F18".to_owned(), KeyCode::F18));
    __flight_record.push(("F19".to_owned(), KeyCode::F19));
    __flight_record.push(("F20".to_owned(), KeyCode::F20));
    __flight_record.push(("F21".to_owned(), KeyCode::F21));
    __flight_record.push(("F22".to_owned(), KeyCode::F22));
    __flight_record.push(("F23".to_owned(), KeyCode::F23));
    __flight_record.push(("F24".to_owned(), KeyCode::F24));
    __flight_record.push(("AudioVolumeDown".to_owned(), KeyCode::VOLUME_DOWN));
    __flight_record.push(("AudioVolumeMute".to_owned(), KeyCode::AUDIO_MUTE));
    __flight_record.push(("AudioVolumeUp".to_owned(), KeyCode::VOLUME_UP));
    __flight_record.push(("MediaPlayPause".to_owned(), KeyCode::AUDIO_PLAY));
    __flight_record.push(("MediaStop".to_owned(), KeyCode::AUDIO_STOP));
    __flight_record.push(("MediaTrackNext".to_owned(), KeyCode::AUDIO_NEXT));
    __flight_record.push(("MediaTrackPrevious".to_owned(), KeyCode::AUDIO_PREVIOUS));
    __flight_record.push(("BrowserBack".to_owned(), KeyCode::APP_CONTROL_BACK));
    __flight_record.push((
        "BrowserBookmarks".to_owned(),
        KeyCode::APP_CONTROL_BOOKMARKS,
    ));
    __flight_record.push(("BrowserForward".to_owned(), KeyCode::APP_CONTROL_FORWARD));
    __flight_record.push(("BrowserHome".to_owned(), KeyCode::APP_CONTROL_HOME));
    __flight_record.push(("BrowserRefresh".to_owned(), KeyCode::APP_CONTROL_REFRESH));
    __flight_record.push(("BrowserSearch".to_owned(), KeyCode::APP_CONTROL_SEARCH));
    __flight_record.push(("BrowserStop".to_owned(), KeyCode::APP_CONTROL_STOP));
    __flight_record.push(("ContextMenu".to_owned(), KeyCode::APPLICATION));
    __flight_record.push(("Copy".to_owned(), KeyCode::COPY));
    __flight_record.push(("Cut".to_owned(), KeyCode::CUT));
    __flight_record.push(("Find".to_owned(), KeyCode::FIND));
    __flight_record.push(("Help".to_owned(), KeyCode::HELP));
    __flight_record.push(("Paste".to_owned(), KeyCode::PASTE));
    __flight_record.push(("Select".to_owned(), KeyCode::SELECT));
    __flight_record.push(("Undo".to_owned(), KeyCode::UNDO));
    __flight_record
});

// Source: upstream/packages/input/src/inputManager.ts:1012 (sha256:825f4855a53f591e09fdfd48ca553b39c25649c7c83f3786c147e51bc9403307)
static NUMPAD_KEY_CODES_BY_CODE: std::sync::LazyLock<Vec<(String, f64)>> =
    std::sync::LazyLock::new(|| {
        let mut __flight_record = Vec::new();
        __flight_record.push(("Enter".to_owned(), KeyCode::NUMPAD_ENTER));
        __flight_record.push(("Numpad0".to_owned(), KeyCode::NUMPAD_0));
        __flight_record.push(("Numpad1".to_owned(), KeyCode::NUMPAD_1));
        __flight_record.push(("Numpad2".to_owned(), KeyCode::NUMPAD_2));
        __flight_record.push(("Numpad3".to_owned(), KeyCode::NUMPAD_3));
        __flight_record.push(("Numpad4".to_owned(), KeyCode::NUMPAD_4));
        __flight_record.push(("Numpad5".to_owned(), KeyCode::NUMPAD_5));
        __flight_record.push(("Numpad6".to_owned(), KeyCode::NUMPAD_6));
        __flight_record.push(("Numpad7".to_owned(), KeyCode::NUMPAD_7));
        __flight_record.push(("Numpad8".to_owned(), KeyCode::NUMPAD_8));
        __flight_record.push(("Numpad9".to_owned(), KeyCode::NUMPAD_9));
        __flight_record.push(("NumpadAdd".to_owned(), KeyCode::NUMPAD_PLUS));
        __flight_record.push(("NumpadBackspace".to_owned(), KeyCode::NUMPAD_BACKSPACE));
        __flight_record.push(("NumpadClear".to_owned(), KeyCode::NUMPAD_CLEAR));
        __flight_record.push(("NumpadClearEntry".to_owned(), KeyCode::NUMPAD_CLEAR_ENTRY));
        __flight_record.push(("NumpadComma".to_owned(), KeyCode::NUMPAD_COMMA));
        __flight_record.push(("NumpadDecimal".to_owned(), KeyCode::NUMPAD_PERIOD));
        __flight_record.push(("NumpadDivide".to_owned(), KeyCode::NUMPAD_DIVIDE));
        __flight_record.push(("NumpadEqual".to_owned(), KeyCode::NUMPAD_EQUALS));
        __flight_record.push(("NumpadHash".to_owned(), KeyCode::NUMPAD_HASH));
        __flight_record.push(("NumpadMemoryAdd".to_owned(), KeyCode::NUMPAD_MEM_ADD));
        __flight_record.push(("NumpadMemoryClear".to_owned(), KeyCode::NUMPAD_MEM_CLEAR));
        __flight_record.push(("NumpadMemoryRecall".to_owned(), KeyCode::NUMPAD_MEM_RECALL));
        __flight_record.push(("NumpadMemoryStore".to_owned(), KeyCode::NUMPAD_MEM_STORE));
        __flight_record.push((
            "NumpadMemorySubtract".to_owned(),
            KeyCode::NUMPAD_MEM_SUBTRACT,
        ));
        __flight_record.push(("NumpadMultiply".to_owned(), KeyCode::NUMPAD_MULTIPLY));
        __flight_record.push((
            "NumpadParenLeft".to_owned(),
            KeyCode::NUMPAD_LEFT_PARENTHESIS,
        ));
        __flight_record.push((
            "NumpadParenRight".to_owned(),
            KeyCode::NUMPAD_RIGHT_PARENTHESIS,
        ));
        __flight_record.push(("NumpadSubtract".to_owned(), KeyCode::NUMPAD_MINUS));
        __flight_record
    });

// Source: upstream/packages/input/src/inputManager.ts:1044 (sha256:13031aa1c37080dea531aeab043951b436a9b88717d3cace3ed6a55f4b8ffdaf)
static _KEYBOARD_DATA: std::sync::LazyLock<std::sync::Mutex<InputKeyboardData>> =
    std::sync::LazyLock::new(|| {
        std::sync::Mutex::new(InputKeyboardData {
            __flight_identity: std::sync::Arc::new(()),
            alt_key: false,
            caps_lock: false,
            code: "".to_owned(),
            ctrl_key: false,
            key: "".to_owned(),
            key_code: 0.0_f64,
            location: 0.0_f64,
            meta_key: false,
            modifier: 0.0_f64,
            num_lock: false,
            repeat: false,
            shift_key: false,
            time_stamp: 0.0_f64,
        })
    });

// Source: upstream/packages/input/src/inputManager.ts:1060 (sha256:58d0831b34643f8d09eae0844356fb502da25699b0329e5d192815f8412d6e18)
static _POINTER_DATA: std::sync::LazyLock<std::sync::Mutex<InputPointerData>> =
    std::sync::LazyLock::new(|| {
        std::sync::Mutex::new(InputPointerData {
            __flight_identity: std::sync::Arc::new(()),
            alt_key: false,
            button: 0.0_f64,
            buttons: 0.0_f64,
            ctrl_key: false,
            delta_x: 0.0_f64,
            delta_y: 0.0_f64,
            height: 1.0_f64,
            is_primary: true,
            meta_key: false,
            pointer_id: 0.0_f64,
            pointer_type: "mouse".to_owned(),
            pressure: 0.0_f64,
            shift_key: false,
            tilt_x: 0.0_f64,
            tilt_y: 0.0_f64,
            time_stamp: 0.0_f64,
            twist: 0.0_f64,
            wheel_mode: "unknown".to_owned(),
            width: 1.0_f64,
            x: 0.0_f64,
            y: 0.0_f64,
        })
    });

// Source: upstream/packages/input/src/inputManager.ts:1084 (sha256:9f3dbd8b669fb6a7fd83cc1bf3c3749626b19ea3b5c946a2f056c87585c419bb)
static _TEXT_DATA: std::sync::LazyLock<std::sync::Mutex<InputTextData>> =
    std::sync::LazyLock::new(|| {
        std::sync::Mutex::new(InputTextData {
            __flight_identity: std::sync::Arc::new(()),
            is_composing: false,
            text: "".to_owned(),
        })
    });

// Source: upstream/packages/input/src/inputManager.ts:1089 (sha256:6e924387b115045da35088047ded4d9cf1c8158783d45c1589373965f017fda9)
#[derive(Clone, Default)]
struct GamepadPollState {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub axes: Vec<(f64, Vec<f64>)>,
    pub buttons: Vec<(f64, Vec<bool>)>,
}
impl PartialEq for GamepadPollState {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/input/src/inputManager.ts:1094 (sha256:3c2a3f3bf7710b9b6ff8da2602f2f75927af5b0a27e6eb89ea56f012003c18b0)
static _GAMEPAD_POLL_STATES: std::sync::LazyLock<
    std::sync::Mutex<Vec<(InputManager, GamepadPollState)>>,
> = std::sync::LazyLock::new(|| std::sync::Mutex::new(Vec::new()));

// Source: upstream/packages/input/src/inputManager.ts:1096 (sha256:febd3f8de482fabe4ce53f7b882b9e135ca44a9db5ff610eb19c02d6a609556b)
#[derive(Clone, Default)]
struct GetOrCreateGamepadPollStateRecord2 {
    __flight_identity: std::sync::Arc<()>,
    axes: Vec<(crate::OpaqueHostValue, crate::OpaqueHostValue)>,
    buttons: Vec<(crate::OpaqueHostValue, crate::OpaqueHostValue)>,
}
impl PartialEq for GetOrCreateGamepadPollStateRecord2 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

fn get_or_create_gamepad_poll_state(manager: &InputManager) -> GamepadPollState {
    let mut state = (*_GAMEPAD_POLL_STATES.lock().unwrap())
        .iter()
        .find(|(entry_key, _)| entry_key == &(*manager).clone())
        .map(|(_, value)| value.clone());
    if ((state).clone()).is_none() {
        state = Some(GamepadPollState {
            __flight_identity: std::sync::Arc::new(()),
            axes: Vec::new(),
            buttons: Vec::new(),
        });
        {
            let __flight_key = (*manager).clone();
            let __flight_value = ((state).clone()).clone().unwrap();
            if let Some((_, value)) = (*_GAMEPAD_POLL_STATES.lock().unwrap())
                .iter_mut()
                .find(|(key, _)| key == &__flight_key)
            {
                *value = __flight_value;
            } else {
                (*_GAMEPAD_POLL_STATES.lock().unwrap()).push((__flight_key, __flight_value));
            }
        };
    }
    return ((state).clone().unwrap()).clone();
}

// Source: upstream/packages/input/src/inputManager.ts:1105 (sha256:2096e547be6d6693db0e48fc33c645052551a8e19119772ea738eebbe1d97276)
static _AXIS_DATA: std::sync::LazyLock<std::sync::Mutex<InputGamepadAxisData>> =
    std::sync::LazyLock::new(|| {
        std::sync::Mutex::new(InputGamepadAxisData {
            __flight_identity: std::sync::Arc::new(()),
            axis: 0.0_f64,
            gamepad: 0.0_f64,
            time_stamp: 0.0_f64,
            value: 0.0_f64,
        })
    });

// Source: upstream/packages/input/src/inputManager.ts:1106 (sha256:34b1a8814af92987ef63aae37ce961448fb61cb032977801c17b3ec6eca13b14)
static _BUTTON_DATA: std::sync::LazyLock<std::sync::Mutex<InputGamepadButtonData>> =
    std::sync::LazyLock::new(|| {
        std::sync::Mutex::new(InputGamepadButtonData {
            __flight_identity: std::sync::Arc::new(()),
            button: 0.0_f64,
            gamepad: 0.0_f64,
            time_stamp: 0.0_f64,
            value: 0.0_f64,
        })
    });

// Source: upstream/packages/input/src/inputManager.ts:1107 (sha256:497c555f5e599225a283f9f664c574557ada2adf00b20b9c3fe0c8ef1fff503b)
static _CONNECT_DATA: std::sync::LazyLock<std::sync::Mutex<InputGamepadConnectData>> =
    std::sync::LazyLock::new(|| {
        std::sync::Mutex::new(InputGamepadConnectData {
            __flight_identity: std::sync::Arc::new(()),
            gamepad: 0.0_f64,
            id: "".to_owned(),
            mapping: "".to_owned(),
        })
    });

// Source: upstream/packages/input/src/inputManager.ts:1113 (sha256:7c30850d86a43ccb069340985677781b99391ec53f07516fe7c884bc2663b163)
static K_GAMEPAD_INPUT: std::sync::LazyLock<crate::FlightSymbol> =
    std::sync::LazyLock::new(|| crate::FlightSymbol::new());

// Source: upstream/packages/input/src/inputManager.ts:1114 (sha256:7d114a1bd548815b37ecd6604f60dafdb0a05f26813a3369dcb36465ed6fdbeb)
static K_KEYBOARD_INPUT: std::sync::LazyLock<crate::FlightSymbol> =
    std::sync::LazyLock::new(|| crate::FlightSymbol::new());

// Source: upstream/packages/input/src/inputManager.ts:1115 (sha256:5cda7bc62922d557dcd8693203743d9e1b7c691d195d6e8db83ae375fb369139)
static K_POINTER_INPUT: std::sync::LazyLock<crate::FlightSymbol> =
    std::sync::LazyLock::new(|| crate::FlightSymbol::new());

// Source: upstream/packages/input/src/inputManager.ts:1116 (sha256:355a9d473d96204a6ffdf0ffec52bb4587067adfb4ac1f9937524c286677491c)
static K_RELATIVE_POINTER_INPUT: std::sync::LazyLock<crate::FlightSymbol> =
    std::sync::LazyLock::new(|| crate::FlightSymbol::new());

// Source: upstream/packages/input/src/inputManager.ts:1117 (sha256:1496d5e97f320acfbdf96b43c41e3daa10452eedc009a4e495fd72c4284a290c)
static K_TEXT_INPUT: std::sync::LazyLock<crate::FlightSymbol> =
    std::sync::LazyLock::new(|| crate::FlightSymbol::new());

// Source: upstream/packages/input/src/inputManager.ts:1118 (sha256:1c5869e4a8d26c235be779b1b25cb32761cf04b8c2b00098461f8ce6a4ba6c17)
static K_WHEEL_INPUT: std::sync::LazyLock<crate::FlightSymbol> =
    std::sync::LazyLock::new(|| crate::FlightSymbol::new());

// Source: upstream/packages/input/src/inputManager.ts:1120 (sha256:b95ac9198a94aaf9009a445f55266d2d1d2a3e481e14df95fa224e68b00100cf)
static _INPUT_BINDINGS: std::sync::LazyLock<
    std::sync::Mutex<
        Vec<(
            InputManager,
            Vec<(
                crate::OpaqueHostValue,
                Vec<(
                    crate::FlightSymbol,
                    std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>,
                )>,
            )>,
        )>,
    >,
> = std::sync::LazyLock::new(|| std::sync::Mutex::new(Vec::new()));

// Source: upstream/packages/input/src/inputManager.ts:1122 (sha256:9ed6326a6b2dcdb45fbcc4bf39edcc94adeeb70a345aa99b49cdd4e14bcdc56a)
fn clear_input_binding(
    manager: &InputManager,
    target: crate::OpaqueHostValue,
    kind: crate::FlightSymbol,
) -> () {
    let mut by_kind = (*_INPUT_BINDINGS.lock().unwrap())
        .iter()
        .find(|(entry_key, _)| entry_key == &(*manager).clone())
        .map(|(_, value)| value.clone())
        .as_ref()
        .and_then(|entries| {
            entries
                .iter()
                .find(|(entry_key, _)| entry_key == &(target).clone())
                .map(|(_, value)| value.clone())
        });
    let cleanup = by_kind.as_ref().and_then(|entries| {
        entries
            .iter()
            .find(|(entry_key, _)| entry_key == &kind)
            .map(|(_, value)| value.clone())
    });
    if (cleanup).is_none() {
        return;
    }
    {
        let __flight_callback = (cleanup.as_ref().unwrap()).clone();
        let __flight_result = __flight_callback.lock().unwrap()();
        __flight_result
    };
    {
        let __flight_key = kind;
        if let Some(__flight_index) = by_kind
            .as_mut()
            .unwrap()
            .iter()
            .position(|(key, _)| key == &__flight_key)
        {
            by_kind.as_mut().unwrap().remove(__flight_index);
            true
        } else {
            false
        }
    };
}

// Source: upstream/packages/input/src/inputManager.ts:1130 (sha256:6a125f6438126aa6ba940b682f59d207a2ddf46a3da012b1555440d23e42507f)
fn set_input_binding(
    manager: &InputManager,
    target: crate::OpaqueHostValue,
    kind: crate::FlightSymbol,
    cleanup: std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>,
) -> () {
    let mut by_target = (*_INPUT_BINDINGS.lock().unwrap())
        .iter()
        .find(|(entry_key, _)| entry_key == &(*manager).clone())
        .map(|(_, value)| value.clone());
    if ((by_target).clone()).is_none() {
        by_target = Some(Vec::new());
        {
            let __flight_key = (*manager).clone();
            let __flight_value = ((by_target).clone()).clone().unwrap();
            if let Some((_, value)) = (*_INPUT_BINDINGS.lock().unwrap())
                .iter_mut()
                .find(|(key, _)| key == &__flight_key)
            {
                *value = __flight_value;
            } else {
                (*_INPUT_BINDINGS.lock().unwrap()).push((__flight_key, __flight_value));
            }
        };
    }
    let mut by_kind = by_target
        .as_ref()
        .unwrap()
        .iter()
        .find(|(entry_key, _)| entry_key == &(target).clone())
        .map(|(_, value)| value.clone());
    if ((by_kind).clone()).is_none() {
        by_kind = Some(Vec::new());
        {
            let __flight_key = (target).clone();
            let __flight_value = ((by_kind).clone()).clone().unwrap();
            if let Some((_, value)) = by_target
                .as_mut()
                .unwrap()
                .iter_mut()
                .find(|(key, _)| key == &__flight_key)
            {
                *value = __flight_value;
            } else {
                by_target
                    .as_mut()
                    .unwrap()
                    .push((__flight_key, __flight_value));
            }
        };
    }
    {
        let __flight_callback = by_kind
            .as_ref()
            .unwrap()
            .iter()
            .find(|(entry_key, _)| entry_key == &kind)
            .map(|(_, value)| value.clone());
        __flight_callback
            .as_ref()
            .map(|callback| callback.lock().unwrap()())
    };
    {
        let __flight_key = kind;
        let __flight_value = (cleanup).clone();
        if let Some((_, value)) = by_kind
            .as_mut()
            .unwrap()
            .iter_mut()
            .find(|(key, _)| key == &__flight_key)
        {
            *value = __flight_value;
        } else {
            by_kind
                .as_mut()
                .unwrap()
                .push((__flight_key, __flight_value));
        }
    };
}
