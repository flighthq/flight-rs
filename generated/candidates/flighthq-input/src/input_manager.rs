// @generated from upstream/packages/input/src/inputManager.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_entity::{allocate_entity, finish_entity};
use flighthq_signals::{connect_signal, create_signal, disconnect_signal, emit_signal};
use flighthq_types::{
    AttachInputOptions, EntityConstruction, GAMEPAD_AXIS_KIND as gamepad_axis_kind_values_constant,
    GAMEPAD_BUTTON_KIND as gamepad_button_kind_values_constant, GamepadAxisKind, GamepadButtonKind,
    GamepadMappingKind, HostInputIngressCapability, InputGamepadAxisData, InputGamepadButtonData,
    InputGamepadConnectData, InputIngressSink, InputIngressSource, InputKeyRepeatOptions,
    InputKeyRepeatTimer, InputKeyboardData, InputManager, InputPointerData, InputSignals,
    InputState, InputTextData,
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

// Source: upstream/packages/input/src/inputManager.ts:31 (sha256:4a7b3c7dce7eb29f8dee9e960ad0cbb80e6f554168eee1663e2da52aa3ba5dfd)
const MAX_GAMEPAD_AXES: f64 = 32.0_f64;

// Source: upstream/packages/input/src/inputManager.ts:32 (sha256:5dfdf4fefadc07a13c6091c66b2ff842f0a7d4b1ad77e192bc2773cd39cf1982)
const MAX_GAMEPAD_BUTTONS: f64 = 64.0_f64;

// Source: upstream/packages/input/src/inputManager.ts:40 (sha256:51129b696f0f63d20d02e53c97e4259d9b397cab47fb34b81ccea602322a60e3)
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

// Source: upstream/packages/input/src/inputManager.ts:59 (sha256:cc1bb9388bcde5ec54cce54c4c88a99913560d9ca5ceeb0c36088a56a0e051c7)
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

// Source: upstream/packages/input/src/inputManager.ts:76 (sha256:93bb2a4e920cc68ae2bfef9cec208c4073c27308b2e3ad08dffe1ca4f6cb0905)
pub fn attach_gamepad_input(
    host_input_ingress: &HostInputIngressCapability,
    manager: &InputManager,
    source: InputIngressSource,
    options: Option<AttachInputOptions>,
) -> () {
    let release = {
        let __flight_callback = (host_input_ingress.attach_gamepad).clone();
        let __flight_result = __flight_callback.lock().unwrap()(
            (source).clone(),
            get_input_ingress_sink((manager).clone()),
            (options).clone(),
        );
        __flight_result
    };
    set_input_binding(
        manager,
        (source).clone(),
        *K_GAMEPAD_INPUT,
        (release).clone(),
    );
}

// Source: upstream/packages/input/src/inputManager.ts:86 (sha256:11e1e6c2b97202a7c25f09e83eb6d9869cdeeb05e3c0498651b834eb962b49fb)
pub fn attach_keyboard_input(
    host_input_ingress: &HostInputIngressCapability,
    manager: &InputManager,
    source: InputIngressSource,
    options: Option<AttachInputOptions>,
) -> () {
    let release = {
        let __flight_callback = (host_input_ingress.attach_keyboard).clone();
        let __flight_result = __flight_callback.lock().unwrap()(
            (source).clone(),
            get_input_ingress_sink((manager).clone()),
            (options).clone(),
        );
        __flight_result
    };
    set_input_binding(
        manager,
        (source).clone(),
        *K_KEYBOARD_INPUT,
        (release).clone(),
    );
}

// Source: upstream/packages/input/src/inputManager.ts:96 (sha256:7cf7aa35efda0219a527b7d567cbd9bfb984c08fd0b7d752417b254382786d47)
pub fn attach_pointer_input(
    host_input_ingress: &HostInputIngressCapability,
    manager: &InputManager,
    source: InputIngressSource,
    options: Option<AttachInputOptions>,
) -> () {
    let release = {
        let __flight_callback = (host_input_ingress.attach_pointer).clone();
        let __flight_result = __flight_callback.lock().unwrap()(
            (source).clone(),
            get_input_ingress_sink((manager).clone()),
            (options).clone(),
        );
        __flight_result
    };
    set_input_binding(
        manager,
        (source).clone(),
        *K_POINTER_INPUT,
        (release).clone(),
    );
}

// Source: upstream/packages/input/src/inputManager.ts:106 (sha256:a957005576b647b6fac298506783989c5b1bf0cb2d3bcf4d68439384d22a721a)
pub fn attach_relative_pointer_input(
    host_input_ingress: &HostInputIngressCapability,
    manager: &InputManager,
    source: InputIngressSource,
    options: Option<AttachInputOptions>,
) -> () {
    let release = {
        let __flight_callback = (host_input_ingress.attach_relative_pointer).clone();
        let __flight_result = __flight_callback.lock().unwrap()(
            (source).clone(),
            get_input_ingress_sink((manager).clone()),
            (options).clone(),
        );
        __flight_result
    };
    set_input_binding(
        manager,
        (source).clone(),
        *K_RELATIVE_POINTER_INPUT,
        (release).clone(),
    );
}

// Source: upstream/packages/input/src/inputManager.ts:116 (sha256:569672c1f4f6b10553053c80b9029bec3a0aefd8ed2d3c6aa161afc3fd2277d4)
pub fn attach_text_input(
    host_input_ingress: &HostInputIngressCapability,
    manager: &InputManager,
    source: InputIngressSource,
    options: Option<AttachInputOptions>,
) -> () {
    let release = {
        let __flight_callback = (host_input_ingress.attach_text).clone();
        let __flight_result = __flight_callback.lock().unwrap()(
            (source).clone(),
            get_input_ingress_sink((manager).clone()),
            (options).clone(),
        );
        __flight_result
    };
    set_input_binding(manager, (source).clone(), *K_TEXT_INPUT, (release).clone());
}

// Source: upstream/packages/input/src/inputManager.ts:126 (sha256:cae32a21b471fbbfb4cab4a2b0c7e57dc9257a65c03944cf3fa1ef9a7b3a4ccc)
pub fn attach_wheel_input(
    host_input_ingress: &HostInputIngressCapability,
    manager: &InputManager,
    source: InputIngressSource,
    options: Option<AttachInputOptions>,
) -> () {
    let release = {
        let __flight_callback = (host_input_ingress.attach_wheel).clone();
        let __flight_result = __flight_callback.lock().unwrap()(
            (source).clone(),
            get_input_ingress_sink((manager).clone()),
            (options).clone(),
        );
        __flight_result
    };
    set_input_binding(manager, (source).clone(), *K_WHEEL_INPUT, (release).clone());
}

// Source: upstream/packages/input/src/inputManager.ts:143 (sha256:ada32cffd3993edc7dec0adecd235e8ca8b614fb9721c12a6d28fbc7482563d3)
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

// Source: upstream/packages/input/src/inputManager.ts:243 (sha256:13e18dd65ad6d7903a6631c9025d9b2263a82c530de372c3e792a631180e2540)
pub fn create_input_key_repeat_timer(options: &InputKeyRepeatOptions) -> InputKeyRepeatTimer {
    let mut out = allocate_entity();
    initialize_input_key_repeat_timer((out).clone(), (options).clone());
    return finish_entity((out).clone());
}

// Source: upstream/packages/input/src/inputManager.ts:249 (sha256:e9e3c2ef2dc2a0db5c673e08733b17f07461c1fe2ed675ff90c54a569417fd5e)
pub fn create_input_manager() -> InputManager {
    let mut out = allocate_entity();
    initialize_input_manager((out).clone());
    return finish_entity((out).clone());
}

// Source: upstream/packages/input/src/inputManager.ts:255 (sha256:8f69d0370e699f97bfd78198c61dbba72bd57aa81e938f737649e22abd969bf7)
pub fn create_input_signals() -> InputSignals {
    let mut out = allocate_entity();
    initialize_input_signals((out).clone());
    return finish_entity((out).clone());
}

// Source: upstream/packages/input/src/inputManager.ts:261 (sha256:a77904a2ac1685d864d744c9dc1c48c7e0421f43fe5aa9c2c0f8408f3287164c)
pub fn create_input_state() -> InputState {
    let mut out = allocate_entity();
    initialize_input_state((out).clone());
    return finish_entity((out).clone());
}

// Source: upstream/packages/input/src/inputManager.ts:267 (sha256:77ec4e3d2811f684b50bda53dd7f167eaae588d9d98c5711da25a6a35a2fcd9f)
pub fn detach_gamepad_input(manager: &InputManager, source: InputIngressSource) -> () {
    clear_input_binding(manager, (source).clone(), *K_GAMEPAD_INPUT);
}

// Source: upstream/packages/input/src/inputManager.ts:271 (sha256:182e965ad6bd410198b13cdedd38d7189e86a8b7e85e36e1657f5d7108c92f38)
pub fn detach_keyboard_input(manager: &InputManager, source: InputIngressSource) -> () {
    clear_input_binding(manager, (source).clone(), *K_KEYBOARD_INPUT);
}

// Source: upstream/packages/input/src/inputManager.ts:275 (sha256:13d7eaefaf597c7da99a735ef5c0fa835cebb1fbefce230eae77cf55a10e1b5b)
pub fn detach_pointer_input(manager: &InputManager, source: InputIngressSource) -> () {
    clear_input_binding(manager, (source).clone(), *K_POINTER_INPUT);
}

// Source: upstream/packages/input/src/inputManager.ts:279 (sha256:a23242eb1823e46791dd2c39901631551b112353d1223311d9840437e25b69ef)
pub fn detach_relative_pointer_input(manager: &InputManager, source: InputIngressSource) -> () {
    clear_input_binding(manager, (source).clone(), *K_RELATIVE_POINTER_INPUT);
}

// Source: upstream/packages/input/src/inputManager.ts:283 (sha256:987bb6c61ebb22ef837a552f00445167a76b0335065cfcd4519142ee8aa0e14d)
pub fn detach_text_input(manager: &InputManager, source: InputIngressSource) -> () {
    clear_input_binding(manager, (source).clone(), *K_TEXT_INPUT);
}

// Source: upstream/packages/input/src/inputManager.ts:287 (sha256:d5dbee270c7885358c3197d0a359b4c69288753324a39670aa33b5e63be841d6)
pub fn detach_wheel_input(manager: &InputManager, source: InputIngressSource) -> () {
    clear_input_binding(manager, (source).clone(), *K_WHEEL_INPUT);
}

// Source: upstream/packages/input/src/inputManager.ts:297 (sha256:62c3cee4a9a441a44a6e05892e37eb56d9f281561d6940b39564e6ea9ac768f3)
pub fn end_input_state_frame(state: &mut InputState) -> () {
    state.just_pressed_keys.clear();
    state.just_released_keys.clear();
    state.just_pressed_gamepad_buttons.clear();
    state.just_released_gamepad_buttons.clear();
}

// Source: upstream/packages/input/src/inputManager.ts:309 (sha256:e8d36ceb5c77b295dc1490db8b25584a70321ab0b26e492acfc90a29740c3c51)
pub fn get_gamepad_axis_name(mapping: GamepadMappingKind, index: f64) -> Option<GamepadAxisKind> {
    if (mapping != "standard") {
        return None;
    }
    return _STANDARD_AXIS_NAMES[index as usize].clone();
}

// Source: upstream/packages/input/src/inputManager.ts:319 (sha256:12c52e4b463da3aa621c12dc6bb73400de93c1fe3bc9857c7474cb1becf4aa3b)
pub fn get_gamepad_button_name(
    mapping: GamepadMappingKind,
    index: f64,
) -> Option<GamepadButtonKind> {
    if (mapping != "standard") {
        return None;
    }
    return _STANDARD_BUTTON_NAMES[index as usize].clone();
}

// Source: upstream/packages/input/src/inputManager.ts:328 (sha256:3238dbf90b348d67d363c9e668848ead648deab6796beafa80490a64cf88efad)
pub fn get_input_gamepad_axis(state: &InputState, gamepad: f64, axis: f64) -> f64 {
    return (state
        .axis_values
        .iter()
        .find(|(entry_key, _)| entry_key == &((gamepad * MAX_GAMEPAD_AXES) + axis))
        .map(|(_, value)| value.clone()))
    .unwrap_or(0.0_f64);
}

// Source: upstream/packages/input/src/inputManager.ts:352 (sha256:fb7b18efb9d63aa7fbb24c14422a904695c8e544cfbe62dfb85e5257662acb11)
pub fn initialize_input_key_repeat_timer(
    out: EntityConstruction<InputKeyRepeatTimer>,
    options: InputKeyRepeatOptions,
) -> () {
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
    crate::host_set("host.start", (start).clone());
    crate::host_set("host.stop", (stop).clone());
}

// Source: upstream/packages/input/src/inputManager.ts:376 (sha256:644e6db5750c59ca40811820ac71a6925eaa203eb0b9c83f22116a54b7bba247)
pub fn initialize_input_manager(out: EntityConstruction<InputManager>) -> () {
    let signals = create_input_signals();
    crate::host_set("host.enabled", true);
    crate::host_set(
        "host.onGamepadAxisMove",
        (signals.on_gamepad_axis_move).clone(),
    );
    crate::host_set(
        "host.onGamepadButtonDown",
        (signals.on_gamepad_button_down).clone(),
    );
    crate::host_set(
        "host.onGamepadButtonUp",
        (signals.on_gamepad_button_up).clone(),
    );
    crate::host_set(
        "host.onGamepadConnect",
        (signals.on_gamepad_connect).clone(),
    );
    crate::host_set(
        "host.onGamepadDisconnect",
        (signals.on_gamepad_disconnect).clone(),
    );
    crate::host_set("host.onKeyDown", (signals.on_key_down).clone());
    crate::host_set("host.onKeyUp", (signals.on_key_up).clone());
    crate::host_set("host.onPointerCancel", (signals.on_pointer_cancel).clone());
    crate::host_set("host.onPointerDown", (signals.on_pointer_down).clone());
    crate::host_set("host.onPointerMove", (signals.on_pointer_move).clone());
    crate::host_set(
        "host.onPointerMoveRelative",
        (signals.on_pointer_move_relative).clone(),
    );
    crate::host_set("host.onPointerUp", (signals.on_pointer_up).clone());
    crate::host_set("host.onTextEdit", (signals.on_text_edit).clone());
    crate::host_set("host.onTextInput", (signals.on_text_input).clone());
    crate::host_set("host.onWheel", (signals.on_wheel).clone());
}

// Source: upstream/packages/input/src/inputManager.ts:396 (sha256:f01de2dcd887d04fac3320e23de0962ae97ab4fe1f5a06507f6b5a79c2851a8c)
pub fn initialize_input_signals(out: EntityConstruction<InputSignals>) -> () {
    crate::host_set("host.onGamepadAxisMove", create_signal());
    crate::host_set("host.onGamepadButtonDown", create_signal());
    crate::host_set("host.onGamepadButtonUp", create_signal());
    crate::host_set("host.onGamepadConnect", create_signal());
    crate::host_set("host.onGamepadDisconnect", create_signal());
    crate::host_set("host.onKeyDown", create_signal());
    crate::host_set("host.onKeyUp", create_signal());
    crate::host_set("host.onPointerCancel", create_signal());
    crate::host_set("host.onPointerDown", create_signal());
    crate::host_set("host.onPointerMove", create_signal());
    crate::host_set("host.onPointerMoveRelative", create_signal());
    crate::host_set("host.onPointerUp", create_signal());
    crate::host_set("host.onTextEdit", create_signal());
    crate::host_set("host.onTextInput", create_signal());
    crate::host_set("host.onWheel", create_signal());
}

// Source: upstream/packages/input/src/inputManager.ts:420 (sha256:04b20c424028dd937dd0c2193238107af79f884e66387ea63ff18506607522dd)
pub fn initialize_input_state(out: EntityConstruction<InputState>) -> () {
    crate::host_set("host.axisValues", Vec::new());
    crate::host_set("host.gamepadButtonsDown", Vec::new());
    crate::host_set("host.justPressedGamepadButtons", Vec::new());
    crate::host_set("host.justPressedKeys", Vec::new());
    crate::host_set("host.justReleasedGamepadButtons", Vec::new());
    crate::host_set("host.justReleasedKeys", Vec::new());
    crate::host_set("host.keysDown", Vec::new());
    crate::host_set("host.pointerButtonsDown", Vec::new());
}

// Source: upstream/packages/input/src/inputManager.ts:435 (sha256:d927dcd51a3a70e13037227050648a1592dcf23ea1964f3017b0c965da8a6cae)
pub fn is_input_gamepad_button_down(state: &InputState, gamepad: f64, button: f64) -> bool {
    return state
        .gamepad_buttons_down
        .iter()
        .any(|item| item == &((gamepad * MAX_GAMEPAD_BUTTONS) + button));
}

// Source: upstream/packages/input/src/inputManager.ts:442 (sha256:6d7f88fbc380d7172c07cb7191aaf013b0052c41e1211f27ce9055ad2ce38f7c)
pub fn is_input_key_down(state: &InputState, key_code: f64) -> bool {
    return state.keys_down.iter().any(|item| item == &key_code);
}

// Source: upstream/packages/input/src/inputManager.ts:450 (sha256:7185bc6004109ad1c07574ab6d5f53e475c20a75ef4e6524cad9e345265891bf)
pub fn is_input_pointer_button_down(state: &InputState, pointer_id: f64, button: f64) -> bool {
    return ((__flight_js_to_i32(
        (state
            .pointer_buttons_down
            .iter()
            .find(|(entry_key, _)| entry_key == &pointer_id)
            .map(|(_, value)| value.clone()))
        .unwrap_or(0.0_f64),
    ) & __flight_js_to_i32(
        __flight_js_to_i32(1.0_f64).wrapping_shl((__flight_js_to_u32(button) & 31)) as f64,
    )) as f64
        != 0.0_f64);
}

// Source: upstream/packages/input/src/inputManager.ts:459 (sha256:6b9b431bc0a5aa662e0c1eef8d9104f8c374133131aaf7d247b3438c9e973e8a)
pub fn was_input_gamepad_button_pressed(state: &InputState, gamepad: f64, button: f64) -> bool {
    return state
        .just_pressed_gamepad_buttons
        .iter()
        .any(|item| item == &((gamepad * MAX_GAMEPAD_BUTTONS) + button));
}

// Source: upstream/packages/input/src/inputManager.ts:468 (sha256:62eadb4fb5dd9d10cf361cae0525eb6af3c6595c94601c2b95a37229a1e93f80)
pub fn was_input_gamepad_button_released(state: &InputState, gamepad: f64, button: f64) -> bool {
    return state
        .just_released_gamepad_buttons
        .iter()
        .any(|item| item == &((gamepad * MAX_GAMEPAD_BUTTONS) + button));
}

// Source: upstream/packages/input/src/inputManager.ts:476 (sha256:68f1833b2a1a1963e7e50ecfa9a65a0d7c961e8a3fa44a3c43221fae3bfadfed)
pub fn was_input_key_pressed(state: &InputState, key_code: f64) -> bool {
    return state.just_pressed_keys.iter().any(|item| item == &key_code);
}

// Source: upstream/packages/input/src/inputManager.ts:484 (sha256:89eda631688c9a03f610cf3f9af447cb258dec426db2e6e2fac397449b6535c7)
pub fn was_input_key_released(state: &InputState, key_code: f64) -> bool {
    return state
        .just_released_keys
        .iter()
        .any(|item| item == &key_code);
}

// Source: upstream/packages/input/src/inputManager.ts:489 (sha256:374bdd0870bcb6c604033c20be13c0460a5a0d89a0e4304ce55eaafddf580b2e)
static _STANDARD_BUTTON_NAMES: std::sync::LazyLock<Vec<Option<GamepadButtonKind>>> =
    std::sync::LazyLock::new(|| {
        vec![
            (gamepad_button_kind_values_constant.button_south).clone(),
            (gamepad_button_kind_values_constant.button_east).clone(),
            (gamepad_button_kind_values_constant.button_west).clone(),
            (gamepad_button_kind_values_constant.button_north).clone(),
            (gamepad_button_kind_values_constant.shoulder_left).clone(),
            (gamepad_button_kind_values_constant.shoulder_right).clone(),
            (gamepad_button_kind_values_constant.trigger_left).clone(),
            (gamepad_button_kind_values_constant.trigger_right).clone(),
            (gamepad_button_kind_values_constant.select).clone(),
            (gamepad_button_kind_values_constant.start).clone(),
            (gamepad_button_kind_values_constant.stick_left).clone(),
            (gamepad_button_kind_values_constant.stick_right).clone(),
            (gamepad_button_kind_values_constant.dpad_up).clone(),
            (gamepad_button_kind_values_constant.dpad_down).clone(),
            (gamepad_button_kind_values_constant.dpad_left).clone(),
            (gamepad_button_kind_values_constant.dpad_right).clone(),
            (gamepad_button_kind_values_constant.home).clone(),
            (gamepad_button_kind_values_constant.touchpad).clone(),
        ]
    });

// Source: upstream/packages/input/src/inputManager.ts:511 (sha256:2cc9fdc0389d35de9c933a25ac0636c096c81c916622a99e8088aee49589332d)
static _STANDARD_AXIS_NAMES: std::sync::LazyLock<Vec<Option<GamepadAxisKind>>> =
    std::sync::LazyLock::new(|| {
        vec![
            (gamepad_axis_kind_values_constant.stick_left_x).clone(),
            (gamepad_axis_kind_values_constant.stick_left_y).clone(),
            (gamepad_axis_kind_values_constant.stick_right_x).clone(),
            (gamepad_axis_kind_values_constant.stick_right_y).clone(),
        ]
    });

// Source: upstream/packages/input/src/inputManager.ts:518 (sha256:09f188ac4baf496006e84f17b6ed23db382365782bbe55f51269f6c0fe3a3e2b)
static _INPUT_INGRESS_SINKS: std::sync::LazyLock<
    std::sync::Mutex<Vec<(InputManager, InputIngressSink)>>,
> = std::sync::LazyLock::new(|| std::sync::Mutex::new(Vec::new()));

// Source: upstream/packages/input/src/inputManager.ts:520 (sha256:e8e00db22b7dde1d41fcdc8ba13db741806f7806c53cd6ca999a88b555a62ccf)
fn get_input_ingress_sink(manager: InputManager) -> InputIngressSink {
    let mut sink = (*_INPUT_INGRESS_SINKS.lock().unwrap())
        .iter()
        .find(|(entry_key, _)| entry_key == &(manager).clone())
        .map(|(_, value)| value.clone());
    if ((sink).clone()).is_some() {
        return ((sink.as_mut().unwrap()).clone()).clone();
    }
    sink = Some(InputIngressSink {
        __flight_identity: std::sync::Arc::new(()),
        gamepad_axis_move: std::sync::Arc::new(std::sync::Mutex::new(Box::new({
            let manager = manager.clone();
            move |data: InputGamepadAxisData| -> () {
                if manager.enabled {
                    emit_signal((manager.on_gamepad_axis_move).clone(), ((data).clone(),));
                }
            }
        })
            as Box<dyn FnMut(InputGamepadAxisData) -> () + Send + 'static>)),
        gamepad_button_down: std::sync::Arc::new(std::sync::Mutex::new(Box::new({
            let manager = manager.clone();
            move |data: InputGamepadButtonData| -> () {
                if manager.enabled {
                    emit_signal((manager.on_gamepad_button_down).clone(), ((data).clone(),));
                }
            }
        })
            as Box<dyn FnMut(InputGamepadButtonData) -> () + Send + 'static>)),
        gamepad_button_up: std::sync::Arc::new(std::sync::Mutex::new(Box::new({
            let manager = manager.clone();
            move |data: InputGamepadButtonData| -> () {
                if manager.enabled {
                    emit_signal((manager.on_gamepad_button_up).clone(), ((data).clone(),));
                }
            }
        })
            as Box<dyn FnMut(InputGamepadButtonData) -> () + Send + 'static>)),
        gamepad_connect: std::sync::Arc::new(std::sync::Mutex::new(Box::new({
            let manager = manager.clone();
            move |data: InputGamepadConnectData| -> () {
                if manager.enabled {
                    emit_signal((manager.on_gamepad_connect).clone(), ((data).clone(),));
                }
            }
        })
            as Box<dyn FnMut(InputGamepadConnectData) -> () + Send + 'static>)),
        gamepad_disconnect: std::sync::Arc::new(std::sync::Mutex::new(Box::new({
            let manager = manager.clone();
            move |data: InputGamepadConnectData| -> () {
                if manager.enabled {
                    emit_signal((manager.on_gamepad_disconnect).clone(), ((data).clone(),));
                }
            }
        })
            as Box<dyn FnMut(InputGamepadConnectData) -> () + Send + 'static>)),
        is_enabled: std::sync::Arc::new(std::sync::Mutex::new(Box::new({
            let manager = manager.clone();
            move || -> bool {
                return manager.enabled;
            }
        })
            as Box<dyn FnMut() -> bool + Send + 'static>)),
        key_down: std::sync::Arc::new(std::sync::Mutex::new(Box::new({
            let manager = manager.clone();
            move |data: InputKeyboardData| -> () {
                if manager.enabled {
                    emit_signal((manager.on_key_down).clone(), ((data).clone(),));
                }
            }
        })
            as Box<dyn FnMut(InputKeyboardData) -> () + Send + 'static>)),
        key_up: std::sync::Arc::new(std::sync::Mutex::new(Box::new({
            let manager = manager.clone();
            move |data: InputKeyboardData| -> () {
                if manager.enabled {
                    emit_signal((manager.on_key_up).clone(), ((data).clone(),));
                }
            }
        })
            as Box<dyn FnMut(InputKeyboardData) -> () + Send + 'static>)),
        pointer_cancel: std::sync::Arc::new(std::sync::Mutex::new(Box::new({
            let manager = manager.clone();
            move |data: InputPointerData| -> () {
                if manager.enabled {
                    emit_signal((manager.on_pointer_cancel).clone(), ((data).clone(),));
                }
            }
        })
            as Box<dyn FnMut(InputPointerData) -> () + Send + 'static>)),
        pointer_down: std::sync::Arc::new(std::sync::Mutex::new(Box::new({
            let manager = manager.clone();
            move |data: InputPointerData| -> () {
                if manager.enabled {
                    emit_signal((manager.on_pointer_down).clone(), ((data).clone(),));
                }
            }
        })
            as Box<dyn FnMut(InputPointerData) -> () + Send + 'static>)),
        pointer_move: std::sync::Arc::new(std::sync::Mutex::new(Box::new({
            let manager = manager.clone();
            move |data: InputPointerData| -> () {
                if manager.enabled {
                    emit_signal((manager.on_pointer_move).clone(), ((data).clone(),));
                }
            }
        })
            as Box<dyn FnMut(InputPointerData) -> () + Send + 'static>)),
        pointer_move_relative: std::sync::Arc::new(std::sync::Mutex::new(Box::new({
            let manager = manager.clone();
            move |data: InputPointerData| -> () {
                if manager.enabled {
                    emit_signal(
                        (manager.on_pointer_move_relative).clone(),
                        ((data).clone(),),
                    );
                }
            }
        })
            as Box<dyn FnMut(InputPointerData) -> () + Send + 'static>)),
        pointer_up: std::sync::Arc::new(std::sync::Mutex::new(Box::new({
            let manager = manager.clone();
            move |data: InputPointerData| -> () {
                if manager.enabled {
                    emit_signal((manager.on_pointer_up).clone(), ((data).clone(),));
                }
            }
        })
            as Box<dyn FnMut(InputPointerData) -> () + Send + 'static>)),
        text_edit: std::sync::Arc::new(std::sync::Mutex::new(Box::new({
            let manager = manager.clone();
            move |data: InputTextData| -> () {
                if manager.enabled {
                    emit_signal((manager.on_text_edit).clone(), ((data).clone(),));
                }
            }
        })
            as Box<dyn FnMut(InputTextData) -> () + Send + 'static>)),
        text_input: std::sync::Arc::new(std::sync::Mutex::new(Box::new({
            let manager = manager.clone();
            move |data: InputTextData| -> () {
                if manager.enabled {
                    emit_signal((manager.on_text_input).clone(), ((data).clone(),));
                }
            }
        })
            as Box<dyn FnMut(InputTextData) -> () + Send + 'static>)),
        wheel: std::sync::Arc::new(std::sync::Mutex::new(Box::new({
            let manager = manager.clone();
            move |data: InputPointerData| -> () {
                if manager.enabled {
                    emit_signal((manager.on_wheel).clone(), ((data).clone(),));
                }
            }
        })
            as Box<dyn FnMut(InputPointerData) -> () + Send + 'static>)),
    });
    {
        let __flight_key = (manager).clone();
        let __flight_value = ((sink).clone()).clone().unwrap();
        if let Some((_, value)) = (*_INPUT_INGRESS_SINKS.lock().unwrap())
            .iter_mut()
            .find(|(key, _)| key == &__flight_key)
        {
            *value = __flight_value;
        } else {
            (*_INPUT_INGRESS_SINKS.lock().unwrap()).push((__flight_key, __flight_value));
        }
    };
    return ((sink).clone().unwrap()).clone();
}

// Source: upstream/packages/input/src/inputManager.ts:581 (sha256:7c30850d86a43ccb069340985677781b99391ec53f07516fe7c884bc2663b163)
static K_GAMEPAD_INPUT: std::sync::LazyLock<crate::FlightSymbol> =
    std::sync::LazyLock::new(|| crate::FlightSymbol::new());

// Source: upstream/packages/input/src/inputManager.ts:582 (sha256:7d114a1bd548815b37ecd6604f60dafdb0a05f26813a3369dcb36465ed6fdbeb)
static K_KEYBOARD_INPUT: std::sync::LazyLock<crate::FlightSymbol> =
    std::sync::LazyLock::new(|| crate::FlightSymbol::new());

// Source: upstream/packages/input/src/inputManager.ts:583 (sha256:5cda7bc62922d557dcd8693203743d9e1b7c691d195d6e8db83ae375fb369139)
static K_POINTER_INPUT: std::sync::LazyLock<crate::FlightSymbol> =
    std::sync::LazyLock::new(|| crate::FlightSymbol::new());

// Source: upstream/packages/input/src/inputManager.ts:584 (sha256:355a9d473d96204a6ffdf0ffec52bb4587067adfb4ac1f9937524c286677491c)
static K_RELATIVE_POINTER_INPUT: std::sync::LazyLock<crate::FlightSymbol> =
    std::sync::LazyLock::new(|| crate::FlightSymbol::new());

// Source: upstream/packages/input/src/inputManager.ts:585 (sha256:1496d5e97f320acfbdf96b43c41e3daa10452eedc009a4e495fd72c4284a290c)
static K_TEXT_INPUT: std::sync::LazyLock<crate::FlightSymbol> =
    std::sync::LazyLock::new(|| crate::FlightSymbol::new());

// Source: upstream/packages/input/src/inputManager.ts:586 (sha256:1c5869e4a8d26c235be779b1b25cb32761cf04b8c2b00098461f8ce6a4ba6c17)
static K_WHEEL_INPUT: std::sync::LazyLock<crate::FlightSymbol> =
    std::sync::LazyLock::new(|| crate::FlightSymbol::new());

// Source: upstream/packages/input/src/inputManager.ts:588 (sha256:c646cd20f706ba8fccb3dc440a7b2cea3ea0dbc62b561c1d52bc2cbc9d533617)
static _INPUT_BINDINGS: std::sync::LazyLock<
    std::sync::Mutex<
        Vec<(
            InputManager,
            Vec<(
                InputIngressSource,
                Vec<(
                    crate::FlightSymbol,
                    std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>,
                )>,
            )>,
        )>,
    >,
> = std::sync::LazyLock::new(|| std::sync::Mutex::new(Vec::new()));

// Source: upstream/packages/input/src/inputManager.ts:590 (sha256:54362b2b5e14ff24a6c2b34beaff4580eaa582c7adc7a927f3391a31e5e506a0)
fn clear_input_binding(
    manager: &InputManager,
    source: InputIngressSource,
    kind: crate::FlightSymbol,
) -> () {
    let mut by_source = (*_INPUT_BINDINGS.lock().unwrap())
        .iter()
        .find(|(entry_key, _)| entry_key == &(*manager).clone())
        .map(|(_, value)| value.clone());
    let mut by_kind = by_source.as_ref().and_then(|entries| {
        entries
            .iter()
            .find(|(entry_key, _)| entry_key == &(source).clone())
            .map(|(_, value)| value.clone())
    });
    let release = by_kind.as_ref().and_then(|entries| {
        entries
            .iter()
            .find(|(entry_key, _)| entry_key == &kind)
            .map(|(_, value)| value.clone())
    });
    if (release).is_none() {
        return;
    }
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
    if ((by_kind.as_ref().unwrap().len() as f64) == 0.0_f64) {
        {
            let __flight_key = (source).clone();
            if let Some(__flight_index) = by_source
                .as_mut()
                .unwrap()
                .iter()
                .position(|(key, _)| key == &__flight_key)
            {
                by_source.as_mut().unwrap().remove(__flight_index);
                true
            } else {
                false
            }
        };
    }
    {
        let __flight_callback = (release.as_ref().unwrap()).clone();
        let __flight_result = __flight_callback.lock().unwrap()();
        __flight_result
    };
}

// Source: upstream/packages/input/src/inputManager.ts:600 (sha256:5f6e470d2f525a24e8b17d5af28b0a92b289a122ed9b0e0ae564f822c33a7f56)
fn set_input_binding(
    manager: &InputManager,
    source: InputIngressSource,
    kind: crate::FlightSymbol,
    release: std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>,
) -> () {
    let mut by_source = (*_INPUT_BINDINGS.lock().unwrap())
        .iter()
        .find(|(entry_key, _)| entry_key == &(*manager).clone())
        .map(|(_, value)| value.clone());
    if ((by_source).clone()).is_none() {
        by_source = Some(Vec::new());
        {
            let __flight_key = (*manager).clone();
            let __flight_value = ((by_source).clone()).clone().unwrap();
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
    let mut by_kind = by_source
        .as_ref()
        .unwrap()
        .iter()
        .find(|(entry_key, _)| entry_key == &(source).clone())
        .map(|(_, value)| value.clone());
    if ((by_kind).clone()).is_none() {
        by_kind = Some(Vec::new());
        {
            let __flight_key = (source).clone();
            let __flight_value = ((by_kind).clone()).clone().unwrap();
            if let Some((_, value)) = by_source
                .as_mut()
                .unwrap()
                .iter_mut()
                .find(|(key, _)| key == &__flight_key)
            {
                *value = __flight_value;
            } else {
                by_source
                    .as_mut()
                    .unwrap()
                    .push((__flight_key, __flight_value));
            }
        };
    }
    let previous = by_kind
        .as_ref()
        .unwrap()
        .iter()
        .find(|(entry_key, _)| entry_key == &kind)
        .map(|(_, value)| value.clone());
    if (previous).is_some() {
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
        {
            let __flight_callback = (previous.as_ref().unwrap()).clone();
            let __flight_result = __flight_callback.lock().unwrap()();
            __flight_result
        };
    }
    {
        let __flight_key = kind;
        let __flight_value = (release).clone();
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
