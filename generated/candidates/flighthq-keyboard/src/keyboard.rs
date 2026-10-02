// @generated from upstream/packages/keyboard/src/keyboard.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_entity::{allocate_entity, finish_entity};
use flighthq_signals::{create_signal, emit_signal};
use flighthq_types::{
    EntityConstruction, HostSoftKeyboardAccessoryBarCapability, HostSoftKeyboardChangeCapability,
    HostSoftKeyboardInfoCapability, HostSoftKeyboardResizeModeWriteCapability,
    HostSoftKeyboardScrollAssistCapability, HostSoftKeyboardStyleCapability,
    HostSoftKeyboardVisibilityCapability, Signal, SoftKeyboard, SoftKeyboardAttachResult,
    SoftKeyboardInfo, SoftKeyboardResizeMode, SoftKeyboardSetterResult, SoftKeyboardStyleKind,
    SoftKeyboardVisibilityResult,
};

#[derive(Clone)]
pub struct SharedStructuralRecord1 {
    pub __flight_identity: std::sync::Arc<()>,
    pub on_show:
        Signal<std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(f64) -> () + Send + 'static>>>>,
    pub on_hide: Signal<std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>>,
    pub on_resize:
        Signal<std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(f64) -> () + Send + 'static>>>>,
}
impl PartialEq for SharedStructuralRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/keyboard/src/keyboard.ts:22 (sha256:bc0ca3767a5d25e20bd986a8c951a11a2db3c50203734943a5850308f96b73b5)
pub fn attach_soft_keyboard(
    host_soft_keyboard_change: HostSoftKeyboardChangeCapability,
    host_soft_keyboard_info: HostSoftKeyboardInfoCapability,
    keyboard: SoftKeyboard,
) -> crate::FlightTask<SoftKeyboardAttachResult> {
    crate::FlightTask::start(
        async move {
            detach_soft_keyboard(&keyboard);
            let prev_height: std::sync::Arc<std::sync::Mutex<f64>> =
                std::sync::Arc::new(std::sync::Mutex::new(
                    {
                        let __flight_callback = (host_soft_keyboard_info.get_info).clone();
                        let __flight_result =
                            __flight_callback.lock().unwrap()(((*_SCRATCH).clone()).clone());
                        __flight_result
                    }
                    .height,
                ));
            let subscription = ({
                let __flight_callback = (host_soft_keyboard_change.subscribe).clone();
                let __flight_result = __flight_callback.lock().unwrap()(std::sync::Arc::new(
                    std::sync::Mutex::new(Box::new({
                        let info = info.clone();
                        let keyboard = keyboard.clone();
                        let mut prev_height = prev_height.clone();
                        move || -> () {
                            let now_info = {
                                let __flight_callback = (info.get_info).clone();
                                let __flight_result = __flight_callback.lock().unwrap()(
                                    ((*_SCRATCH).clone()).clone(),
                                );
                                __flight_result
                            };
                            let now_height = now_info.height;
                            let was_visible = ((*prev_height.lock().unwrap()).clone() > 0.0_f64);
                            let now_visible = (now_height > 0.0_f64);
                            if (now_visible) && (!was_visible) {
                                (*prev_height.lock().unwrap()) = now_height;
                                emit_signal((keyboard.on_show).clone(), (now_height,));
                            } else {
                                if (!now_visible) && (was_visible) {
                                    (*prev_height.lock().unwrap()) = 0.0_f64;
                                    emit_signal((keyboard.on_hide).clone(), ());
                                } else {
                                    if (now_visible)
                                        && (now_height != (*prev_height.lock().unwrap()).clone())
                                    {
                                        (*prev_height.lock().unwrap()) = now_height;
                                        emit_signal((keyboard.on_resize).clone(), (now_height,));
                                    }
                                }
                            }
                        }
                    })
                        as Box<dyn FnMut() -> () + Send + 'static>),
                ));
                __flight_result
            })
            .await?;
            if ((subscription.result).clone() != "ok") {
                return Ok((subscription.result).clone());
            }
            {
                let __flight_key = (keyboard).clone();
                let __flight_value = ((subscription.unsubscribe).clone()).unwrap();
                if let Some((_, value)) = (*_SUBSCRIPTIONS.lock().unwrap())
                    .iter_mut()
                    .find(|(key, _)| key == &__flight_key)
                {
                    *value = __flight_value;
                } else {
                    (*_SUBSCRIPTIONS.lock().unwrap()).push((__flight_key, __flight_value));
                }
            };
            return Ok("ok".to_owned());
        },
        crate::FlightTaskOrigin {
            package: "@flighthq/keyboard",
            source: "upstream/packages/keyboard/src/keyboard.ts",
            line: 22_u32,
            column: 1_u32,
            lexical_path: "attachSoftKeyboard",
            fingerprint: "sha256:bc0ca3767a5d25e20bd986a8c951a11a2db3c50203734943a5850308f96b73b5",
        },
    )
}

// Source: upstream/packages/keyboard/src/keyboard.ts:52 (sha256:1bece756ef57a9b53dd6c70e874e4ccfcbeb1991f710f2f25b812ae2c33ff389)
pub fn create_soft_keyboard() -> SharedStructuralRecord1 {
    let mut out = allocate_entity();
    initialize_soft_keyboard((out).clone());
    return finish_entity((out).clone());
}

// Source: upstream/packages/keyboard/src/keyboard.ts:58 (sha256:a64b0ebcb6d13a7bd12ac1785299e755db8ad0bf9160b5c0b02ca4b70b9825be)
pub fn detach_soft_keyboard(keyboard: &SoftKeyboard) -> () {
    let unsubscribe = (*_SUBSCRIPTIONS.lock().unwrap())
        .iter()
        .find(|(entry_key, _)| entry_key == &(*keyboard).clone())
        .map(|(_, value)| value.clone());
    if (unsubscribe).is_some() {
        {
            let __flight_callback = (unsubscribe.as_ref().unwrap()).clone();
            let __flight_result = __flight_callback.lock().unwrap()();
            __flight_result
        };
        {
            let __flight_key = (*keyboard).clone();
            if let Some(__flight_index) = (*_SUBSCRIPTIONS.lock().unwrap())
                .iter()
                .position(|(key, _)| key == &__flight_key)
            {
                (*_SUBSCRIPTIONS.lock().unwrap()).remove(__flight_index);
                true
            } else {
                false
            }
        };
    }
}

// Source: upstream/packages/keyboard/src/keyboard.ts:66 (sha256:a2c3a4a86f6f4b90ed9bf30e21e46cce502c4cc31d69ae255dbfce5839f496db)
pub fn dispose_soft_keyboard(keyboard: &SoftKeyboard) -> () {
    detach_soft_keyboard(keyboard);
}

// Source: upstream/packages/keyboard/src/keyboard.ts:70 (sha256:b401123d14f6140d78f3a824770490308f79c4e52d17feebe0d447f7a4df1f4a)
pub fn get_soft_keyboard_height(host_soft_keyboard_info: &HostSoftKeyboardInfoCapability) -> f64 {
    return {
        let __flight_callback = (host_soft_keyboard_info.get_info).clone();
        let __flight_result = __flight_callback.lock().unwrap()(((*_SCRATCH).clone()).clone());
        __flight_result
    }
    .height;
}

// Source: upstream/packages/keyboard/src/keyboard.ts:74 (sha256:113df42e2b20a20ee830e5b03f2c46a62c1aa4a2b03f077a617be7e32380ba62)
pub fn get_soft_keyboard_info(
    host_soft_keyboard_info: &HostSoftKeyboardInfoCapability,
    out: &SoftKeyboardInfo,
) -> SoftKeyboardInfo {
    return {
        let __flight_callback = (host_soft_keyboard_info.get_info).clone();
        let __flight_result = __flight_callback.lock().unwrap()((*out).clone());
        __flight_result
    };
}

// Source: upstream/packages/keyboard/src/keyboard.ts:81 (sha256:37513d855176abf5e8f3c72ac7998383c9af4d516a34034e4c13d6270ed25abc)
pub fn hide_soft_keyboard(
    host_soft_keyboard_visibility: &HostSoftKeyboardVisibilityCapability,
) -> crate::FlightTask<SoftKeyboardVisibilityResult> {
    return {
        let __flight_callback = (host_soft_keyboard_visibility.hide).clone();
        let __flight_result = __flight_callback.lock().unwrap()();
        __flight_result
    };
}

// Source: upstream/packages/keyboard/src/keyboard.ts:87 (sha256:e5fc10e0d2db703a6a84fa00a6a5dc04c1d1cd859f4282be2b96596f97f9e4f5)
pub fn initialize_soft_keyboard(out: EntityConstruction<SharedStructuralRecord1>) -> () {
    crate::host_set("host.onHide", create_signal());
    crate::host_set("host.onResize", create_signal());
    crate::host_set("host.onShow", create_signal());
}

// Source: upstream/packages/keyboard/src/keyboard.ts:93 (sha256:ef0cdbc3fa0ce6f2bd779632144e06144b34da834bb533238fc324a48a19624c)
pub fn is_soft_keyboard_visible(host_soft_keyboard_info: &HostSoftKeyboardInfoCapability) -> bool {
    return {
        let __flight_callback = (host_soft_keyboard_info.get_info).clone();
        let __flight_result = __flight_callback.lock().unwrap()(((*_SCRATCH).clone()).clone());
        __flight_result
    }
    .visible;
}

// Source: upstream/packages/keyboard/src/keyboard.ts:97 (sha256:614e541e2245c8b7ae3b2f6b4f61be0612a4ef53c51c330f356f7633007ab81f)
pub fn set_soft_keyboard_accessory_bar_visible(
    host_soft_keyboard_accessory_bar: &HostSoftKeyboardAccessoryBarCapability,
    visible: bool,
) -> crate::FlightTask<SoftKeyboardSetterResult> {
    return {
        let __flight_callback =
            (host_soft_keyboard_accessory_bar.set_accessory_bar_visible).clone();
        let __flight_result = __flight_callback.lock().unwrap()(visible);
        __flight_result
    };
}

// Source: upstream/packages/keyboard/src/keyboard.ts:104 (sha256:8ca118ccb08a5dfc2161685f3e2b4f897b7c21ada3a07d483e8d9f0eaf2e4d25)
pub fn set_soft_keyboard_resize_mode(
    host_soft_keyboard_resize_mode_write: &HostSoftKeyboardResizeModeWriteCapability,
    mode: SoftKeyboardResizeMode,
) -> crate::FlightTask<SoftKeyboardSetterResult> {
    return {
        let __flight_callback = (host_soft_keyboard_resize_mode_write.set_resize_mode).clone();
        let __flight_result = __flight_callback.lock().unwrap()((mode).clone());
        __flight_result
    };
}

// Source: upstream/packages/keyboard/src/keyboard.ts:111 (sha256:555bc4f1f99b007b54f1b0164dabf14876b9a42cb8ce66ecb37867226ce4808b)
pub fn set_soft_keyboard_scroll_assist_enabled(
    host_soft_keyboard_scroll_assist: &HostSoftKeyboardScrollAssistCapability,
    enabled: bool,
) -> crate::FlightTask<SoftKeyboardSetterResult> {
    return {
        let __flight_callback =
            (host_soft_keyboard_scroll_assist.set_scroll_assist_enabled).clone();
        let __flight_result = __flight_callback.lock().unwrap()(enabled);
        __flight_result
    };
}

// Source: upstream/packages/keyboard/src/keyboard.ts:118 (sha256:aedbe4855eda4a3c43feb1cce43dee7bdbacaae338f1916b648288a94582c2c4)
pub fn set_soft_keyboard_style(
    host_soft_keyboard_style: &HostSoftKeyboardStyleCapability,
    style: SoftKeyboardStyleKind,
) -> crate::FlightTask<SoftKeyboardSetterResult> {
    return {
        let __flight_callback = (host_soft_keyboard_style.set_style).clone();
        let __flight_result = __flight_callback.lock().unwrap()((style).clone());
        __flight_result
    };
}

// Source: upstream/packages/keyboard/src/keyboard.ts:125 (sha256:164a9246200c9b075f870fc1f5c17016f4a7decb7f4311d36527ea17229ad549)
pub fn show_soft_keyboard(
    host_soft_keyboard_visibility: &HostSoftKeyboardVisibilityCapability,
) -> crate::FlightTask<SoftKeyboardVisibilityResult> {
    return {
        let __flight_callback = (host_soft_keyboard_visibility.show).clone();
        let __flight_result = __flight_callback.lock().unwrap()();
        __flight_result
    };
}

// Source: upstream/packages/keyboard/src/keyboard.ts:131 (sha256:cac54c0a9b96473c0adfbb1bd563438a94416ae508541c42543ac3da4860a63e)
static _SCRATCH: std::sync::LazyLock<SoftKeyboardInfo> =
    std::sync::LazyLock::new(|| SoftKeyboardInfo {
        __flight_identity: std::sync::Arc::new(()),
        visible: false,
        height: 0.0_f64,
        x: 0.0_f64,
        y: 0.0_f64,
        width: 0.0_f64,
    });

// Source: upstream/packages/keyboard/src/keyboard.ts:132 (sha256:6b335bd5a7a32d0ee477947e847f50fa75fc126cd453b3cc0654603778a8beb8)
static _SUBSCRIPTIONS: std::sync::LazyLock<
    std::sync::Mutex<
        Vec<(
            SoftKeyboard,
            std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>,
        )>,
    >,
> = std::sync::LazyLock::new(|| std::sync::Mutex::new(Vec::new()));
