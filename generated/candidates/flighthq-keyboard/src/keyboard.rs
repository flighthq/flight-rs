// @generated from upstream/packages/keyboard/src/keyboard.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_signals::{create_signal, emit_signal};
use flighthq_types::{
    SOFT_KEYBOARD_RESIZE_NONE_KIND as soft_keyboard_resize_none_kind_constant, SoftKeyboard,
    SoftKeyboardBackend, SoftKeyboardInfo, SoftKeyboardPhase, SoftKeyboardResizeMode,
    SoftKeyboardStyleKind, SoftKeyboardTransition,
};

// Source: upstream/packages/keyboard/src/keyboard.ts:18 (sha256:60a46339c430aff3ebe656222950308306c964d49d77f3067f674aad6bb61315)
pub fn attach_soft_keyboard(keyboard: SoftKeyboard) -> () {
    detach_soft_keyboard(&keyboard);
    let backend: std::sync::Arc<std::sync::Mutex<SoftKeyboardBackend>> =
        std::sync::Arc::new(std::sync::Mutex::new(get_soft_keyboard_backend()));
    let was_visible: std::sync::Arc<std::sync::Mutex<bool>> =
        std::sync::Arc::new(std::sync::Mutex::new(
            {
                let __flight_callback = ((*backend.lock().unwrap()).get_info).clone();
                let __flight_result =
                    __flight_callback.lock().unwrap()(((*_SCRATCH).clone()).clone());
                __flight_result
            }
            .visible,
        ));
    let unsubscribe = {
        let __flight_callback = ((*backend.lock().unwrap()).subscribe).clone();
        let __flight_result = __flight_callback.lock().unwrap()(std::sync::Arc::new(
            std::sync::Mutex::new(Box::new({
                let mut backend = backend.clone();
                let keyboard = keyboard.clone();
                let mut was_visible = was_visible.clone();
                move |phase: SoftKeyboardPhase, transition: SoftKeyboardTransition| -> () {
                    let prev_visible = (*was_visible.lock().unwrap()).clone();
                    let info = {
                        let __flight_callback = ((*backend.lock().unwrap()).get_info).clone();
                        let __flight_result =
                            __flight_callback.lock().unwrap()(((*_SCRATCH).clone()).clone());
                        __flight_result
                    };
                    let now_visible = info.visible;
                    if (phase == "will") {
                        if (now_visible) && (!prev_visible) {
                            emit_signal((keyboard.on_will_show).clone(), ((transition).clone(),));
                        } else {
                            if (!now_visible) && (prev_visible) {
                                emit_signal(
                                    (keyboard.on_will_hide).clone(),
                                    ((transition).clone(),),
                                );
                            } else {
                                emit_signal(
                                    (keyboard.on_will_resize).clone(),
                                    ((transition).clone(),),
                                );
                            }
                        }
                    } else {
                        emit_signal((keyboard.on_did_resize).clone(), (info.height,));
                        emit_signal((keyboard.on_resize).clone(), (info.height,));
                        if (now_visible != prev_visible) {
                            (*was_visible.lock().unwrap()) = now_visible;
                            if now_visible {
                                emit_signal((keyboard.on_did_show).clone(), (info.height,));
                                emit_signal((keyboard.on_show).clone(), (info.height,));
                            } else {
                                emit_signal((keyboard.on_did_hide).clone(), ());
                                emit_signal((keyboard.on_hide).clone(), ());
                            }
                        }
                    }
                }
            })
                as Box<
                    dyn FnMut(SoftKeyboardPhase, SoftKeyboardTransition) -> () + Send + 'static,
                >),
        ));
        __flight_result
    };
    {
        let __flight_key = (keyboard).clone();
        let __flight_value = (unsubscribe).clone();
        if let Some((_, value)) = (*_SUBSCRIPTIONS.lock().unwrap())
            .iter_mut()
            .find(|(key, _)| key == &__flight_key)
        {
            *value = __flight_value;
        } else {
            (*_SUBSCRIPTIONS.lock().unwrap()).push((__flight_key, __flight_value));
        }
    };
}

// Source: upstream/packages/keyboard/src/keyboard.ts:55 (sha256:641c21f16ec25a767ef4b6260fdc6d5d47cad03f1fd3d4b5ffda890962909af9)
pub fn create_soft_keyboard() -> SoftKeyboard {
    return SoftKeyboard {
        __flight_identity: std::sync::Arc::new(()),
        on_show: create_signal(),
        on_hide: create_signal(),
        on_resize: create_signal(),
        on_will_show: create_signal(),
        on_will_hide: create_signal(),
        on_will_resize: create_signal(),
        on_did_show: create_signal(),
        on_did_hide: create_signal(),
        on_did_resize: create_signal(),
    };
}

// Source: upstream/packages/keyboard/src/keyboard.ts:70 (sha256:5435cdf55efda8197a7892a9456e0ec242d961f535e5341d3ddb0d17e437fe50)
#[derive(Clone, Default)]
struct CreateSoftKeyboardInfoRecord1 {
    __flight_identity: std::sync::Arc<()>,
    visible: bool,
    height: f64,
    x: f64,
    y: f64,
    width: f64,
}
impl PartialEq for CreateSoftKeyboardInfoRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn create_soft_keyboard_info() -> SoftKeyboardInfo {
    return SoftKeyboardInfo {
        __flight_identity: std::sync::Arc::new(()),
        visible: false,
        height: 0.0_f64,
        x: 0.0_f64,
        y: 0.0_f64,
        width: 0.0_f64,
    };
}

// Source: upstream/packages/keyboard/src/keyboard.ts:75 (sha256:d624e47c835466e138f17a7580c26b825a973c719c19721dd145c578233033ee)
#[derive(Clone, Default)]
struct CreateSoftKeyboardTransitionRecord1 {
    __flight_identity: std::sync::Arc<()>,
    duration_seconds: f64,
    height: f64,
}
impl PartialEq for CreateSoftKeyboardTransitionRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn create_soft_keyboard_transition() -> SoftKeyboardTransition {
    return SoftKeyboardTransition {
        __flight_identity: std::sync::Arc::new(()),
        duration_seconds: 0.0_f64,
        height: 0.0_f64,
    };
}

// Source: upstream/packages/keyboard/src/keyboard.ts:85 (sha256:0a917a6c0eb1eb7ce65d4fd005fba2a912fa04cf4ef475061320f64aa6349804)
#[derive(Clone, Default)]
struct CreateWebSoftKeyboardBackendRecord1 {
    __flight_identity: std::sync::Arc<()>,
    duration_seconds: f64,
    height: f64,
}
impl PartialEq for CreateWebSoftKeyboardBackendRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn create_web_soft_keyboard_backend() -> SoftKeyboardBackend {
    return SoftKeyboardBackend {
        __flight_identity: std::sync::Arc::new(()),
        get_info: std::sync::Arc::new(std::sync::Mutex::new(Box::new(
            move |mut out: SoftKeyboardInfo| -> SoftKeyboardInfo {
                let geo = get_web_keyboard_geometry();
                out.height = geo.height;
                out.visible = (geo.height > 0.0_f64);
                out.x = geo.x;
                out.y = geo.y;
                out.width = geo.width;
                return out;
            },
        )
            as Box<dyn FnMut(SoftKeyboardInfo) -> SoftKeyboardInfo + Send + 'static>)),
        subscribe: std::sync::Arc::new(std::sync::Mutex::new(
            Box::new(
                move |listener: std::sync::Arc<
                    std::sync::Mutex<
                        Box<
                            dyn FnMut(SoftKeyboardPhase, SoftKeyboardTransition) -> ()
                                + Send
                                + 'static,
                        >,
                    >,
                >|
                      -> std::sync::Arc<
                    std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>,
                > {
                    return std::sync::Arc::new(std::sync::Mutex::new(
                        Box::new(move || -> () {}) as Box<dyn FnMut() -> () + Send + 'static>
                    ));
                },
            )
                as Box<
                    dyn FnMut(
                            std::sync::Arc<
                                std::sync::Mutex<
                                    Box<
                                        dyn FnMut(SoftKeyboardPhase, SoftKeyboardTransition) -> ()
                                            + Send
                                            + 'static,
                                    >,
                                >,
                            >,
                        ) -> std::sync::Arc<
                            std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>,
                        > + Send
                        + 'static,
                >,
        )),
        show: std::sync::Arc::new(std::sync::Mutex::new(Box::new(move || -> () {
            let vk = get_virtual_keyboard();
            if (vk).is_some() {
                {
                    let __flight_callback = (vk.as_ref().unwrap().show).clone();
                    let __flight_result = __flight_callback.lock().unwrap()();
                    __flight_result
                };
            }
        })
            as Box<dyn FnMut() -> () + Send + 'static>)),
        hide: std::sync::Arc::new(std::sync::Mutex::new(Box::new(move || -> () {
            let vk = get_virtual_keyboard();
            if (vk).is_some() {
                {
                    let __flight_callback = (vk.as_ref().unwrap().hide).clone();
                    let __flight_result = __flight_callback.lock().unwrap()();
                    __flight_result
                };
            }
        })
            as Box<dyn FnMut() -> () + Send + 'static>)),
        get_resize_mode: None,
        set_resize_mode: None,
        get_accessory_bar_visible: None,
        set_accessory_bar_visible: None,
        get_scroll_assist_enabled: None,
        set_scroll_assist_enabled: None,
        set_style: None,
    };
}

// Source: upstream/packages/keyboard/src/keyboard.ts:138 (sha256:a64b0ebcb6d13a7bd12ac1785299e755db8ad0bf9160b5c0b02ca4b70b9825be)
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

// Source: upstream/packages/keyboard/src/keyboard.ts:148 (sha256:a2c3a4a86f6f4b90ed9bf30e21e46cce502c4cc31d69ae255dbfce5839f496db)
pub fn dispose_soft_keyboard(keyboard: &SoftKeyboard) -> () {
    detach_soft_keyboard(keyboard);
}

// Source: upstream/packages/keyboard/src/keyboard.ts:153 (sha256:865cf74a7b8870165bc81e42e635c5c90e525844f43d1e04265a226183071c63)
pub fn get_soft_keyboard_backend() -> SoftKeyboardBackend {
    if ((*_BACKEND.lock().unwrap()).clone()).is_none() {
        (*_BACKEND.lock().unwrap()) = Some(create_web_soft_keyboard_backend());
    }
    return (((*_BACKEND.lock().unwrap()).clone()).clone().unwrap()).clone();
}

// Source: upstream/packages/keyboard/src/keyboard.ts:159 (sha256:f1a1656832cbf57286e7298285927d7987837003f8ecb14a60fac217222f6f87)
pub fn get_soft_keyboard_height() -> f64 {
    return {
        let __flight_callback = (get_soft_keyboard_backend().get_info).clone();
        let __flight_result = __flight_callback.lock().unwrap()(((*_SCRATCH).clone()).clone());
        __flight_result
    }
    .height;
}

// Source: upstream/packages/keyboard/src/keyboard.ts:164 (sha256:8c08d15eaec239e80dc296922470cdb4a8794314bb0322565b575856bc946316)
pub fn get_soft_keyboard_info(out: &SoftKeyboardInfo) -> SoftKeyboardInfo {
    return {
        let __flight_callback = (get_soft_keyboard_backend().get_info).clone();
        let __flight_result = __flight_callback.lock().unwrap()((*out).clone());
        __flight_result
    };
}

// Source: upstream/packages/keyboard/src/keyboard.ts:170 (sha256:aee49082e07962c5ef24cf4454430c1d2dd466e61a0146a666420d549c2b40dd)
pub fn get_soft_keyboard_resize_mode() -> SoftKeyboardResizeMode {
    let backend = get_soft_keyboard_backend();
    return ({
        let __flight_callback = (backend.get_resize_mode).clone();
        __flight_callback
            .as_ref()
            .map(|callback| callback.lock().unwrap()())
    })
    .clone()
    .unwrap_or((soft_keyboard_resize_none_kind_constant).to_owned());
}

// Source: upstream/packages/keyboard/src/keyboard.ts:177 (sha256:64e095f2b1fb6799ef1045d084f3e318fe3219344f5bb0ffa6b17f460d53f234)
pub fn hide_soft_keyboard() -> () {
    {
        let __flight_callback = (get_soft_keyboard_backend().hide).clone();
        let __flight_result = __flight_callback.lock().unwrap()();
        __flight_result
    };
}

// Source: upstream/packages/keyboard/src/keyboard.ts:183 (sha256:913e54393814936b1a379662956df7786518c435b2bdb8eb5d569a8e7716a5e3)
pub fn is_soft_keyboard_accessory_bar_visible() -> bool {
    return ({
        let __flight_callback = (get_soft_keyboard_backend().get_accessory_bar_visible).clone();
        __flight_callback
            .as_ref()
            .map(|callback| callback.lock().unwrap()())
    })
    .clone()
    .unwrap_or(false);
}

// Source: upstream/packages/keyboard/src/keyboard.ts:188 (sha256:30d3d951150a8b97370cd879e48d0267b2283813e616e9811b0bd30fcf8ef914)
pub fn is_soft_keyboard_scroll_assist_enabled() -> bool {
    return ({
        let __flight_callback = (get_soft_keyboard_backend().get_scroll_assist_enabled).clone();
        __flight_callback
            .as_ref()
            .map(|callback| callback.lock().unwrap()())
    })
    .clone()
    .unwrap_or(false);
}

// Source: upstream/packages/keyboard/src/keyboard.ts:194 (sha256:8f334ce2c0b7129d455d66bf50c95078808fe8dd0f7be0cd8bf4ce0a05cb4dcf)
pub fn set_soft_keyboard_accessory_bar_visible(visible: bool) -> () {
    {
        let __flight_callback = (get_soft_keyboard_backend().set_accessory_bar_visible).clone();
        __flight_callback
            .as_ref()
            .map(|callback| callback.lock().unwrap()(visible))
    };
}

// Source: upstream/packages/keyboard/src/keyboard.ts:199 (sha256:4e256bc0e7d467fe02b2b1d97098ce8e0aa18acdd785ee3eccc6e7df1e41d4d5)
pub fn set_soft_keyboard_backend(backend: &Option<SoftKeyboardBackend>) -> () {
    (*_BACKEND.lock().unwrap()) = (*backend).clone();
}

// Source: upstream/packages/keyboard/src/keyboard.ts:205 (sha256:80467d4f41e840e50c535c493c1609a659972976429038427faabfd871e284ca)
pub fn set_soft_keyboard_resize_mode(mode: SoftKeyboardResizeMode) -> () {
    {
        let __flight_callback = (get_soft_keyboard_backend().set_resize_mode).clone();
        __flight_callback
            .as_ref()
            .map(|callback| callback.lock().unwrap()((mode).clone()))
    };
}

// Source: upstream/packages/keyboard/src/keyboard.ts:211 (sha256:b3449b37089ba053ff997ae11ba75a6e9dbe1990f840fad5750cfbcb37dd10dc)
pub fn set_soft_keyboard_scroll_assist_enabled(enabled: bool) -> () {
    {
        let __flight_callback = (get_soft_keyboard_backend().set_scroll_assist_enabled).clone();
        __flight_callback
            .as_ref()
            .map(|callback| callback.lock().unwrap()(enabled))
    };
}

// Source: upstream/packages/keyboard/src/keyboard.ts:217 (sha256:2fed296fea1aead7567e3fa9120838d301f30746cd08bb21e2ae808941c32b90)
pub fn set_soft_keyboard_style(style: SoftKeyboardStyleKind) -> () {
    {
        let __flight_callback = (get_soft_keyboard_backend().set_style).clone();
        __flight_callback
            .as_ref()
            .map(|callback| callback.lock().unwrap()((style).clone()))
    };
}

// Source: upstream/packages/keyboard/src/keyboard.ts:223 (sha256:9e4b07099556feaed93d37523056ea043e7cb870cb9714871048a1a64ce46b06)
pub fn show_soft_keyboard() -> () {
    {
        let __flight_callback = (get_soft_keyboard_backend().show).clone();
        let __flight_result = __flight_callback.lock().unwrap()();
        __flight_result
    };
}

// Source: upstream/packages/keyboard/src/keyboard.ts:227 (sha256:09753dcee3eeb8c287eef9972b9c4e8963b68fd28c816e1ad86d573dae7a7488)
static _BACKEND: std::sync::LazyLock<std::sync::Mutex<Option<SoftKeyboardBackend>>> =
    std::sync::LazyLock::new(|| std::sync::Mutex::new(None));

// Source: upstream/packages/keyboard/src/keyboard.ts:228 (sha256:a3ca0082a9deb7ab73c0f38b14a36c4cc328fcdc1c97193377dd7dcb23a27816)
static _SCRATCH: std::sync::LazyLock<SoftKeyboardInfo> =
    std::sync::LazyLock::new(|| create_soft_keyboard_info());

// Source: upstream/packages/keyboard/src/keyboard.ts:229 (sha256:6b335bd5a7a32d0ee477947e847f50fa75fc126cd453b3cc0654603778a8beb8)
static _SUBSCRIPTIONS: std::sync::LazyLock<
    std::sync::Mutex<
        Vec<(
            SoftKeyboard,
            std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>,
        )>,
    >,
> = std::sync::LazyLock::new(|| std::sync::Mutex::new(Vec::new()));

// Source: upstream/packages/keyboard/src/keyboard.ts:233 (sha256:d6daf7877b2414bc1e63170e00bb97a5bdac8df5fda4e8dffe13e5662763cebf)
#[derive(Clone)]
struct VirtualKeyboard {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub bounding_rect: crate::OpaqueHostValue,
    pub overlays_content: bool,
    pub show: std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>,
    pub hide: std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>,
}
impl PartialEq for VirtualKeyboard {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/keyboard/src/keyboard.ts:240 (sha256:2ff1a381f178211ebdd7c09ad164f231a69f2d1089ed4679c5404f045294562c)
fn get_virtual_keyboard() -> Option<VirtualKeyboard> {
    return None;
}

// Source: upstream/packages/keyboard/src/keyboard.ts:246 (sha256:e7187dd15738a3ec74992e267cb3d5a884aa121faa5649de8cfe7529d2ce8038)
#[derive(Clone, Default)]
struct WebKeyboardGeometry {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub height: f64,
    pub width: f64,
    pub x: f64,
    pub y: f64,
}
impl PartialEq for WebKeyboardGeometry {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/keyboard/src/keyboard.ts:253 (sha256:53b48ac5bcef0e1bcd24f61fc8ce5e0b31b78ea2c6c49dc3a736f2a6b964ac5c)
#[derive(Clone, Default)]
struct GetWebKeyboardGeometryRecord1 {
    __flight_identity: std::sync::Arc<()>,
    height: f64,
    width: f64,
    x: f64,
    y: f64,
}
impl PartialEq for GetWebKeyboardGeometryRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

fn get_web_keyboard_geometry() -> WebKeyboardGeometry {
    return WebKeyboardGeometry {
        __flight_identity: std::sync::Arc::new(()),
        height: 0.0_f64,
        width: 0.0_f64,
        x: 0.0_f64,
        y: 0.0_f64,
    };
}
