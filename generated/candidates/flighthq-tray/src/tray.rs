// @generated from upstream/packages/tray/src/tray.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::{
    MenuItemTemplate, RectangleLike, TrayBackend, TrayBalloonOptions, TrayCapabilities,
    TrayEventData, TrayIcon, TrayIconOptions, Vector2Like,
};

// Source: upstream/packages/tray/src/tray.ts:16 (sha256:0ba4762c6910ff95bc416a52e1200ab0eeeddb7d94f09e2ce4b6511242310e58)
static WEB_CAPABILITIES: std::sync::LazyLock<TrayCapabilities> =
    std::sync::LazyLock::new(|| TrayCapabilities {
        __flight_identity: std::sync::Arc::new(()),
        balloon: false,
        bounds: false,
        click_events: false,
        drop_files: false,
        pressed_icon: false,
        title: false,
    });

// Source: upstream/packages/tray/src/tray.ts:27 (sha256:0c453f00fb1c689366535d89c6a0cb7e6d0e0e003f565a599117809f9c66395a)
#[derive(Clone, Default)]
struct CreateTrayIconRecord1 {
    __flight_identity: std::sync::Arc<()>,
}
impl PartialEq for CreateTrayIconRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn create_tray_icon(options: Option<TrayIconOptions>) -> Option<TrayIcon> {
    let id = {
        let __flight_callback = (get_tray_backend().create).clone();
        let __flight_result =
            __flight_callback.lock().unwrap()((options).clone().unwrap_or(TrayIconOptions {
                __flight_identity: std::sync::Arc::new(()),
                icon: None,
                icon_template: None,
                title: None,
                tooltip: None,
            }));
        __flight_result
    };
    return if (id < 0.0_f64) {
        None
    } else {
        Some(TrayIcon {
            __flight_identity: std::sync::Arc::new(()),
            id: id,
        })
    };
}

// Source: upstream/packages/tray/src/tray.ts:35 (sha256:545869e6c5d74232d60374bc0d8ba1ee9a5a2f48e9e1ab49eddcbb26a43bd76b)
pub fn create_web_tray_backend() -> TrayBackend {
    return TrayBackend {
        __flight_identity: std::sync::Arc::new(()),
        create: std::sync::Arc::new(std::sync::Mutex::new(Box::new(
            move |__flight_unused_0: TrayIconOptions| -> f64 {
                return (-1.0_f64);
            },
        )
            as Box<dyn FnMut(TrayIconOptions) -> f64 + Send + 'static>)),
        destroy: std::sync::Arc::new(std::sync::Mutex::new(Box::new(
            move |__flight_unused_0: f64| -> () {},
        )
            as Box<dyn FnMut(f64) -> () + Send + 'static>)),
        display_balloon: std::sync::Arc::new(std::sync::Mutex::new(Box::new(
            move |__flight_unused_0: f64, __flight_unused_1: TrayBalloonOptions| -> () {},
        )
            as Box<dyn FnMut(f64, TrayBalloonOptions) -> () + Send + 'static>)),
        get_bounds: std::sync::Arc::new(std::sync::Mutex::new(Box::new(
            move |__flight_unused_0: f64| -> Option<RectangleLike> {
                return None;
            },
        )
            as Box<dyn FnMut(f64) -> Option<RectangleLike> + Send + 'static>)),
        get_capabilities: std::sync::Arc::new(std::sync::Mutex::new(Box::new(
            move || -> TrayCapabilities {
                return ((*WEB_CAPABILITIES).clone()).clone();
            },
        )
            as Box<dyn FnMut() -> TrayCapabilities + Send + 'static>)),
        get_title: std::sync::Arc::new(std::sync::Mutex::new(Box::new(
            move |__flight_unused_0: f64| -> String {
                return "".to_owned();
            },
        )
            as Box<dyn FnMut(f64) -> String + Send + 'static>)),
        get_tooltip: std::sync::Arc::new(std::sync::Mutex::new(Box::new(
            move |__flight_unused_0: f64| -> String {
                return "".to_owned();
            },
        )
            as Box<dyn FnMut(f64) -> String + Send + 'static>)),
        is_destroyed: std::sync::Arc::new(std::sync::Mutex::new(Box::new(
            move |__flight_unused_0: f64| -> bool {
                return true;
            },
        )
            as Box<dyn FnMut(f64) -> bool + Send + 'static>)),
        list_ids: std::sync::Arc::new(std::sync::Mutex::new(Box::new(move || -> Vec<f64> {
            return vec![];
        })
            as Box<dyn FnMut() -> Vec<f64> + Send + 'static>)),
        pop_up_context_menu: std::sync::Arc::new(std::sync::Mutex::new(Box::new(
            move |__flight_unused_0: f64, __flight_unused_1: Option<Vector2Like>| -> () {},
        )
            as Box<dyn FnMut(f64, Option<Vector2Like>) -> () + Send + 'static>)),
        remove_balloon: std::sync::Arc::new(std::sync::Mutex::new(Box::new(
            move |__flight_unused_0: f64| -> () {},
        )
            as Box<dyn FnMut(f64) -> () + Send + 'static>)),
        set_context_menu: std::sync::Arc::new(std::sync::Mutex::new(Box::new(
            move |__flight_unused_0: f64, __flight_unused_1: Vec<MenuItemTemplate>| -> () {},
        )
            as Box<dyn FnMut(f64, Vec<MenuItemTemplate>) -> () + Send + 'static>)),
        set_icon: std::sync::Arc::new(std::sync::Mutex::new(Box::new(
            move |__flight_unused_0: f64, __flight_unused_1: String| -> () {},
        )
            as Box<dyn FnMut(f64, String) -> () + Send + 'static>)),
        set_ignore_double_click_events: std::sync::Arc::new(std::sync::Mutex::new(Box::new(
            move |__flight_unused_0: f64, __flight_unused_1: bool| -> () {},
        )
            as Box<dyn FnMut(f64, bool) -> () + Send + 'static>)),
        set_pressed_icon: std::sync::Arc::new(std::sync::Mutex::new(Box::new(
            move |__flight_unused_0: f64, __flight_unused_1: String| -> () {},
        )
            as Box<dyn FnMut(f64, String) -> () + Send + 'static>)),
        set_template: std::sync::Arc::new(std::sync::Mutex::new(Box::new(
            move |__flight_unused_0: f64, __flight_unused_1: bool| -> () {},
        )
            as Box<dyn FnMut(f64, bool) -> () + Send + 'static>)),
        set_title: std::sync::Arc::new(std::sync::Mutex::new(Box::new(
            move |__flight_unused_0: f64, __flight_unused_1: String| -> () {},
        )
            as Box<dyn FnMut(f64, String) -> () + Send + 'static>)),
        set_tooltip: std::sync::Arc::new(std::sync::Mutex::new(Box::new(
            move |__flight_unused_0: f64, __flight_unused_1: String| -> () {},
        )
            as Box<dyn FnMut(f64, String) -> () + Send + 'static>)),
        subscribe: std::sync::Arc::new(std::sync::Mutex::new(
            Box::new(
                move |__flight_unused_0: std::sync::Arc<
                    std::sync::Mutex<Box<dyn FnMut(TrayEventData) -> () + Send + 'static>>,
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
                                    Box<dyn FnMut(TrayEventData) -> () + Send + 'static>,
                                >,
                            >,
                        ) -> std::sync::Arc<
                            std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>,
                        > + Send
                        + 'static,
                >,
        )),
    };
}

// Source: upstream/packages/tray/src/tray.ts:109 (sha256:3314445639d87ddca0e71b3882c6e31b1f60a6b92dd4b2f78a0769c795c78768)
pub fn destroy_tray_icon(tray: &TrayIcon) -> () {
    stop_tray_icon_animation(tray);
    {
        let __flight_callback = (get_tray_backend().destroy).clone();
        let __flight_result = __flight_callback.lock().unwrap()(tray.id);
        __flight_result
    };
}

// Source: upstream/packages/tray/src/tray.ts:116 (sha256:6cb2242ef39a04714138e3fa0da224e35d5ba8dfd94d6226596e42c55bee5397)
pub fn display_tray_balloon(tray: &TrayIcon, options: &TrayBalloonOptions) -> () {
    {
        let __flight_callback = (get_tray_backend().display_balloon).clone();
        let __flight_result = __flight_callback.lock().unwrap()(tray.id, (*options).clone());
        __flight_result
    };
}

// Source: upstream/packages/tray/src/tray.ts:121 (sha256:704fae703e30fb7809d4b127a6b32d71b2954177ea10e3401b388a41df1edb5f)
pub fn get_tray_backend() -> TrayBackend {
    if ((*_BACKEND.lock().unwrap()).clone()).is_none() {
        (*_BACKEND.lock().unwrap()) = Some(create_web_tray_backend());
    }
    return (((*_BACKEND.lock().unwrap()).clone()).clone().unwrap()).clone();
}

// Source: upstream/packages/tray/src/tray.ts:129 (sha256:baf99e6ac6e4f3b16d2dc789b0192f0c039bfb4e08ac650410cb58144a8e29fe)
pub fn get_tray_capabilities() -> TrayCapabilities {
    return {
        let __flight_callback = (get_tray_backend().get_capabilities).clone();
        let __flight_result = __flight_callback.lock().unwrap()();
        __flight_result
    };
}

// Source: upstream/packages/tray/src/tray.ts:135 (sha256:cf34f2a2cbfcecb14c0b251679ac5975455cadddb791cde176b6ff0b35cd1a19)
pub fn get_tray_icon_bounds(tray: &TrayIcon) -> Option<RectangleLike> {
    return {
        let __flight_callback = (get_tray_backend().get_bounds).clone();
        let __flight_result = __flight_callback.lock().unwrap()(tray.id);
        __flight_result
    };
}

// Source: upstream/packages/tray/src/tray.ts:140 (sha256:a1b9ca170ae37495e197a95b7a1886a843f091f8875f7e8eefccae2adaf1964f)
pub fn get_tray_icons() -> Vec<TrayIcon> {
    return ({
        let __flight_callback = (get_tray_backend().list_ids).clone();
        let __flight_result = __flight_callback.lock().unwrap()();
        __flight_result
    })
    .iter()
    .cloned()
    .map(|id: f64| -> TrayIcon {
        TrayIcon {
            __flight_identity: std::sync::Arc::new(()),
            id: id,
        }
    })
    .collect::<Vec<_>>();
}

// Source: upstream/packages/tray/src/tray.ts:147 (sha256:03bd55662087ae16a5af5582e5813b44775f862cd9d4b18dce0286456885b557)
pub fn get_tray_icon_title(tray: &TrayIcon) -> String {
    return {
        let __flight_callback = (get_tray_backend().get_title).clone();
        let __flight_result = __flight_callback.lock().unwrap()(tray.id);
        __flight_result
    };
}

// Source: upstream/packages/tray/src/tray.ts:152 (sha256:4934353b072c2de100c21fbd337e59225f2aa30eaa13839720de82418e70d694)
pub fn get_tray_icon_tooltip(tray: &TrayIcon) -> String {
    return {
        let __flight_callback = (get_tray_backend().get_tooltip).clone();
        let __flight_result = __flight_callback.lock().unwrap()(tray.id);
        __flight_result
    };
}

// Source: upstream/packages/tray/src/tray.ts:158 (sha256:785f9414289c4508c4578ed3e2854028f5cd46caa947fb0c19ff72634c5e1148)
pub fn is_tray_destroyed(tray: &TrayIcon) -> bool {
    return {
        let __flight_callback = (get_tray_backend().is_destroyed).clone();
        let __flight_result = __flight_callback.lock().unwrap()(tray.id);
        __flight_result
    };
}

// Source: upstream/packages/tray/src/tray.ts:164 (sha256:0c8f656c9e62d0088af5131bb5a183f03dd0f0b0ec41f4cc7cabd1aa2db84207)
pub fn is_tray_icon_animating(tray: &TrayIcon) -> bool {
    return (*_ANIMATIONS.lock().unwrap())
        .iter()
        .any(|(entry_key, _)| entry_key == &tray.id);
}

// Source: upstream/packages/tray/src/tray.ts:171 (sha256:5b304a5a7a0f435e1046e1363090d30000b42dd1b2a11106a8dbc4030573ff75)
pub fn on_tray_event(
    listener: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut(TrayEventData) -> () + Send + 'static>>,
    >,
) -> std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>> {
    return {
        let __flight_callback = (get_tray_backend().subscribe).clone();
        let __flight_result = __flight_callback.lock().unwrap()((listener).clone());
        __flight_result
    };
}

// Source: upstream/packages/tray/src/tray.ts:178 (sha256:e6676e7b534d90f76e44ab953e4d60eff2470359bd18e155a83cf77fb029548a)
pub fn popup_tray_context_menu(tray: &TrayIcon, position: Option<Vector2Like>) -> () {
    {
        let __flight_callback = (get_tray_backend().pop_up_context_menu).clone();
        let __flight_result = __flight_callback.lock().unwrap()(tray.id, (position).clone());
        __flight_result
    };
}

// Source: upstream/packages/tray/src/tray.ts:183 (sha256:0155faa7eac5bcdd1274ff5c6dd0e8e6888ddd7049c4bfaae7f3335da003b894)
pub fn remove_tray_balloon(tray: &TrayIcon) -> () {
    {
        let __flight_callback = (get_tray_backend().remove_balloon).clone();
        let __flight_result = __flight_callback.lock().unwrap()(tray.id);
        __flight_result
    };
}

// Source: upstream/packages/tray/src/tray.ts:190 (sha256:711a5d4a9073ce51a882881c46565a35f9620ff100f1f2f4600282d13e84df83)
pub fn set_tray_animation_guard(
    guard: &Option<
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(TrayIcon, f64, f64) -> () + Send + 'static>>>,
    >,
) -> () {
    (*_ANIMATION_GUARD.lock().unwrap()) = (*guard).clone();
}

// Source: upstream/packages/tray/src/tray.ts:197 (sha256:ed5db2690ca2145d420c2de53ae9a2138c09619f0466d87979d40647ddb7d811)
pub fn set_tray_backend(backend: &Option<TrayBackend>) -> () {
    (*_BACKEND.lock().unwrap()) = (*backend).clone();
}

// Source: upstream/packages/tray/src/tray.ts:203 (sha256:ddd4e62b08e87f64cdf3604baae3e924170e22526b2469bfa032376b203bfe65)
pub fn set_tray_icon(tray: &TrayIcon, icon: String) -> () {
    {
        let __flight_callback = (get_tray_backend().set_icon).clone();
        let __flight_result = __flight_callback.lock().unwrap()(tray.id, (icon).clone());
        __flight_result
    };
}

// Source: upstream/packages/tray/src/tray.ts:209 (sha256:06a332f7f53868c58a9225490853721d9a583d8393d5252112e3844308f81422)
pub fn set_tray_icon_context_menu(tray: &TrayIcon, items: &Vec<MenuItemTemplate>) -> () {
    {
        let __flight_callback = (get_tray_backend().set_context_menu).clone();
        let __flight_result = __flight_callback.lock().unwrap()(tray.id, (*items).clone());
        __flight_result
    };
}

// Source: upstream/packages/tray/src/tray.ts:216 (sha256:358302780e25c59d36380af77812f5f8700050ffacc746681e826ff423895b84)
pub fn set_tray_icon_template(tray: &TrayIcon, is_template: bool) -> () {
    {
        let __flight_callback = (get_tray_backend().set_template).clone();
        let __flight_result = __flight_callback.lock().unwrap()(tray.id, is_template);
        __flight_result
    };
}

// Source: upstream/packages/tray/src/tray.ts:221 (sha256:3a9478314cbee75acb43e1aad6518fcd28c29c6cc4b520c2b92049f1aae66309)
pub fn set_tray_icon_title(tray: &TrayIcon, title: String) -> () {
    {
        let __flight_callback = (get_tray_backend().set_title).clone();
        let __flight_result = __flight_callback.lock().unwrap()(tray.id, (title).clone());
        __flight_result
    };
}

// Source: upstream/packages/tray/src/tray.ts:226 (sha256:68cff5f38cb47b6c2fd48b9fec754dbdacdd607518cdb492da400942ff545358)
pub fn set_tray_icon_tooltip(tray: &TrayIcon, tooltip: String) -> () {
    {
        let __flight_callback = (get_tray_backend().set_tooltip).clone();
        let __flight_result = __flight_callback.lock().unwrap()(tray.id, (tooltip).clone());
        __flight_result
    };
}

// Source: upstream/packages/tray/src/tray.ts:232 (sha256:7e980eb6a465f3e37ca84a32a424d16574e0ad4e237806e145a2f0262b81eb28)
pub fn set_tray_ignore_double_click_events(tray: &TrayIcon, ignore: bool) -> () {
    {
        let __flight_callback = (get_tray_backend().set_ignore_double_click_events).clone();
        let __flight_result = __flight_callback.lock().unwrap()(tray.id, ignore);
        __flight_result
    };
}

// Source: upstream/packages/tray/src/tray.ts:238 (sha256:bf553831f7250d49b561c242ad9ce8e0f6798698aeba390c77f9ef94bf52023d)
pub fn set_tray_pressed_icon(tray: &TrayIcon, icon: String) -> () {
    {
        let __flight_callback = (get_tray_backend().set_pressed_icon).clone();
        let __flight_result = __flight_callback.lock().unwrap()(tray.id, (icon).clone());
        __flight_result
    };
}

// Source: upstream/packages/tray/src/tray.ts:254 (sha256:5fa5f330f06ac537fd93dcc2a0c92edb63ec79bdd7c4a11c5a5deaafaf93fb04)
pub fn start_tray_icon_animation(
    tray: TrayIcon,
    frames: Vec<String>,
    interval_ms: f64,
) -> std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>> {
    if ((frames.len() as f64) == 0.0_f64) {
        return std::sync::Arc::new(std::sync::Mutex::new(
            Box::new(move || -> () { _noop_stop() }) as Box<dyn FnMut() -> () + Send + 'static>,
        ));
    }
    {
        let __flight_callback = (*_ANIMATION_GUARD.lock().unwrap()).clone();
        __flight_callback.as_ref().map(|callback| {
            callback.lock().unwrap()((tray).clone(), (frames.len() as f64), interval_ms)
        })
    };
    stop_tray_icon_animation(&tray);
    let index: std::sync::Arc<std::sync::Mutex<f64>> =
        std::sync::Arc::new(std::sync::Mutex::new(0.0_f64));
    set_tray_icon(
        &tray,
        frames[(*index.lock().unwrap()).clone() as usize].clone(),
    );
    let handle: Option<crate::FlightTimeout> = crate::set_interval(
        {
            let frames = frames.clone();
            let mut index = index.clone();
            let tray = tray.clone();
            move || -> () {
                (*index.lock().unwrap()) =
                    (((*index.lock().unwrap()).clone() + 1.0_f64) % (frames.len() as f64));
                set_tray_icon(
                    &tray,
                    frames[(*index.lock().unwrap()).clone() as usize].clone(),
                );
            }
        },
        interval_ms,
    );
    {
        let __flight_key = tray.id;
        let __flight_value = ((handle).clone()).clone().unwrap();
        if let Some((_, value)) = (*_ANIMATIONS.lock().unwrap())
            .iter_mut()
            .find(|(key, _)| key == &__flight_key)
        {
            *value = __flight_value;
        } else {
            (*_ANIMATIONS.lock().unwrap()).push((__flight_key, __flight_value));
        }
    };
    return std::sync::Arc::new(std::sync::Mutex::new(Box::new({
        let tray = tray.clone();
        move || -> () {
            if ((*_ANIMATIONS.lock().unwrap())
                .iter()
                .find(|(entry_key, _)| entry_key == &tray.id)
                .map(|(_, value)| value.clone())
                != handle)
            {
                return;
            }
            if let Some(__flight_timer) = ((handle).clone()).clone() {
                crate::clear_interval(__flight_timer);
            };
            {
                let __flight_key = tray.id;
                if let Some(__flight_index) = (*_ANIMATIONS.lock().unwrap())
                    .iter()
                    .position(|(key, _)| key == &__flight_key)
                {
                    (*_ANIMATIONS.lock().unwrap()).remove(__flight_index);
                    true
                } else {
                    false
                }
            };
        }
    })
        as Box<dyn FnMut() -> () + Send + 'static>));
}

// Source: upstream/packages/tray/src/tray.ts:273 (sha256:b924015eb42cce05d846f6e70591ba457d85bd37a90a958452cf83d034a1e703)
static _BACKEND: std::sync::LazyLock<std::sync::Mutex<Option<TrayBackend>>> =
    std::sync::LazyLock::new(|| std::sync::Mutex::new(None));

// Source: upstream/packages/tray/src/tray.ts:277 (sha256:b9f7a2bb64367186635b5cc4af89cf380991925df8054a199976735e26dba284)
static _ANIMATIONS: std::sync::LazyLock<std::sync::Mutex<Vec<(f64, crate::FlightTimeout)>>> =
    std::sync::LazyLock::new(|| std::sync::Mutex::new(Vec::new()));

// Source: upstream/packages/tray/src/tray.ts:279 (sha256:7e36c0e4b954f7911fee03987b802ddf98003187a33d45ee1333896680f0e398)
fn _noop_stop() -> () {}

// Source: upstream/packages/tray/src/tray.ts:285 (sha256:78e486ac69324739689aa16a3b3da8774b79aa20e1318ede73b2a206112ce87d)
pub fn stop_tray_icon_animation(tray: &TrayIcon) -> () {
    let handle: Option<crate::FlightTimeout> = (*_ANIMATIONS.lock().unwrap())
        .iter()
        .find(|(entry_key, _)| entry_key == &tray.id)
        .map(|(_, value)| value.clone());
    if (handle).is_none() {
        return;
    }
    crate::clear_interval((handle.as_ref().unwrap()).clone());
    {
        let __flight_key = tray.id;
        if let Some(__flight_index) = (*_ANIMATIONS.lock().unwrap())
            .iter()
            .position(|(key, _)| key == &__flight_key)
        {
            (*_ANIMATIONS.lock().unwrap()).remove(__flight_index);
            true
        } else {
            false
        }
    };
}

// Source: upstream/packages/tray/src/tray.ts:292 (sha256:a7a7020c0cb5c62f55face48c4f747c8869304df19b4847633b5bafb1abfb438)
static _ANIMATION_GUARD: std::sync::LazyLock<
    std::sync::Mutex<
        Option<
            std::sync::Arc<
                std::sync::Mutex<Box<dyn FnMut(TrayIcon, f64, f64) -> () + Send + 'static>>,
            >,
        >,
    >,
> = std::sync::LazyLock::new(|| std::sync::Mutex::new(None));
