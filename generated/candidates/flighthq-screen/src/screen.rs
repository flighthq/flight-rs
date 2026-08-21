// @generated from upstream/packages/screen/src/screen.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_signals::{create_signal, emit_signal};
use flighthq_types::{
    RectangleLike, ScreenBackend, ScreenChangeEvent, ScreenChangedMetrics, ScreenColorSpace,
    ScreenInfo, ScreenMode, ScreenOrientation, ScreenSignals, Vector2Like,
};

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

#[derive(Clone, Default)]
pub struct SharedStructuralRecord2 {
    pub __flight_identity: std::sync::Arc<()>,
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}
impl PartialEq for SharedStructuralRecord2 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/screen/src/screen.ts:18 (sha256:0bf7d851a1f9084064b501e6bbe4ddafb22f4fa14d980ad49aa442f539cbfd71)
pub fn attach_screen_signals(signals: ScreenSignals) -> () {
    detach_screen_signals(&signals);
    let unsubscribe = {
        let __flight_callback = (get_screen_backend().subscribe).clone();
        let __flight_result = __flight_callback.lock().unwrap()(std::sync::Arc::new(
            std::sync::Mutex::new(Box::new({
                let signals = signals.clone();
                move |event: ScreenChangeEvent| -> () {
                    if ((event.kind).clone() == "ScreenAdded") {
                        emit_signal((signals.on_screen_added).clone(), ((event.screen).clone(),));
                    } else {
                        if ((event.kind).clone() == "ScreenRemoved") {
                            emit_signal(
                                (signals.on_screen_removed).clone(),
                                ((event.screen).clone(),),
                            );
                        } else {
                            emit_signal(
                                (signals.on_screen_metrics_changed).clone(),
                                ((event).clone(),),
                            );
                        }
                    }
                }
            })
                as Box<dyn FnMut(ScreenChangeEvent) -> () + Send + 'static>),
        ));
        __flight_result
    };
    {
        let __flight_key = (signals).clone();
        let __flight_value = (unsubscribe).clone();
        if let Some((_, value)) = (*_SIGNAL_SUBSCRIPTIONS.lock().unwrap())
            .iter_mut()
            .find(|(key, _)| key == &__flight_key)
        {
            *value = __flight_value;
        } else {
            (*_SIGNAL_SUBSCRIPTIONS.lock().unwrap()).push((__flight_key, __flight_value));
        }
    };
}

// Source: upstream/packages/screen/src/screen.ts:34 (sha256:3245c9cd28c972bfdccb1e53247b4e70f21d69e62ea2247d9c7ebcb1c3389776)
#[derive(Clone, Default)]
struct CreateScreenInfoRecord3 {
    __flight_identity: std::sync::Arc<()>,
    id: f64,
    x: f64,
    y: f64,
    width: f64,
    height: f64,
    work_width: f64,
    work_height: f64,
    scale_factor: f64,
    is_primary: bool,
    rotation: f64,
    orientation: String,
    refresh_rate: f64,
    color_depth: f64,
    pixel_depth: f64,
    physical_width: f64,
    physical_height: f64,
    is_hdr: bool,
    color_space: String,
    max_luminance: f64,
    depth_per_component: f64,
    dpi: f64,
    label: String,
    internal: bool,
    touch_support: String,
    monochrome: bool,
}
impl PartialEq for CreateScreenInfoRecord3 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn create_screen_info() -> ScreenInfo {
    return ScreenInfo {
        __flight_identity: std::sync::Arc::new(()),
        id: 0.0_f64,
        x: 0.0_f64,
        y: 0.0_f64,
        width: 0.0_f64,
        height: 0.0_f64,
        work_width: 0.0_f64,
        work_height: 0.0_f64,
        scale_factor: 1.0_f64,
        is_primary: false,
        rotation: (-1.0_f64),
        orientation: "Landscape".to_owned(),
        refresh_rate: (-1.0_f64),
        color_depth: (-1.0_f64),
        pixel_depth: (-1.0_f64),
        physical_width: (-1.0_f64),
        physical_height: (-1.0_f64),
        is_hdr: false,
        color_space: "srgb".to_owned(),
        max_luminance: (-1.0_f64),
        depth_per_component: (-1.0_f64),
        dpi: (-1.0_f64),
        label: "".to_owned(),
        internal: false,
        touch_support: "unknown".to_owned(),
        monochrome: false,
    };
}

// Source: upstream/packages/screen/src/screen.ts:65 (sha256:2f53e195e8eeb299f3d2c020fd52f17423e785b033aea91d314fd9fcdc44e5eb)
#[derive(Clone, Default)]
struct CreateScreenModeRecord3 {
    __flight_identity: std::sync::Arc<()>,
    width: f64,
    height: f64,
    refresh_rate: f64,
    color_depth: f64,
    pixel_format: String,
}
impl PartialEq for CreateScreenModeRecord3 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn create_screen_mode() -> ScreenMode {
    return ScreenMode {
        __flight_identity: std::sync::Arc::new(()),
        width: 0.0_f64,
        height: 0.0_f64,
        refresh_rate: (-1.0_f64),
        color_depth: (-1.0_f64),
        pixel_format: "".to_owned(),
    };
}

// Source: upstream/packages/screen/src/screen.ts:76 (sha256:788cc83e9b1a826dbe5199fb1483764718a74db2c38a1a8c7c5d367a7eab57d1)
pub fn create_screen_signals() -> ScreenSignals {
    return ScreenSignals {
        __flight_identity: std::sync::Arc::new(()),
        on_screen_added: create_signal(),
        on_screen_metrics_changed: create_signal(),
        on_screen_removed: create_signal(),
    };
}

// Source: upstream/packages/screen/src/screen.ts:90 (sha256:e60a29415022aae75d8c4a8c8bcab6086095712fe2e745602bceb7e5e56f89e6)
#[derive(Clone, Default)]
pub(crate) struct ScreenDetailed {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub avail_left: f64,
    pub avail_top: f64,
    pub avail_width: f64,
    pub avail_height: f64,
    pub color_depth: f64,
    pub device_pixel_ratio: f64,
    pub height: f64,
    pub is_extended: Option<bool>,
    pub is_internal: Option<bool>,
    pub is_primary: Option<bool>,
    pub label: String,
    pub left: f64,
    pub pixel_depth: f64,
    pub refresh_rate: Option<f64>,
    pub top: f64,
    pub width: f64,
}
impl PartialEq for ScreenDetailed {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/screen/src/screen.ts:109 (sha256:1c972c21516f572a21aa598490cf68c4bea1a17935a1c6f4bc1b5e57d90774f3)
#[derive(Clone)]
pub(crate) struct ScreenDetails {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub current_screen: ScreenDetailed,
    pub screens: Vec<ScreenDetailed>,
    pub add_event_listener: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(
                        String,
                        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>,
                    ) -> ()
                    + Send
                    + 'static,
            >,
        >,
    >,
    pub remove_event_listener: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(
                        String,
                        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>,
                    ) -> ()
                    + Send
                    + 'static,
            >,
        >,
    >,
}
impl PartialEq for ScreenDetails {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/screen/src/screen.ts:120 (sha256:b12e330d2d697aef338fe81af6b883712ab54906ef44fb2b70581ccdcb767281)
#[derive(Clone)]
struct CreateWebScreenBackendRecord3 {
    __flight_identity: std::sync::Arc<()>,
    get_screens: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut(Vec<ScreenInfo>) -> Vec<ScreenInfo> + Send + 'static>>,
    >,
    get_primary_screen:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(ScreenInfo) -> ScreenInfo + Send + 'static>>>,
    subscribe: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(
                        std::sync::Arc<
                            std::sync::Mutex<
                                Box<dyn FnMut(ScreenChangeEvent) -> () + Send + 'static>,
                            >,
                        >,
                    ) -> std::sync::Arc<
                        std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>,
                    > + Send
                    + 'static,
            >,
        >,
    >,
    get_cursor_position: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(
                        flighthq_types::ScreenBackendRecord1,
                    ) -> flighthq_types::ScreenBackendRecord1
                    + Send
                    + 'static,
            >,
        >,
    >,
    get_modes: Option<
        std::sync::Arc<
            std::sync::Mutex<
                Box<dyn FnMut(ScreenInfo, Vec<ScreenMode>) -> Vec<ScreenMode> + Send + 'static>,
            >,
        >,
    >,
    _upgrade:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(ScreenDetails) -> () + Send + 'static>>>,
}
impl PartialEq for CreateWebScreenBackendRecord3 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn create_web_screen_backend() -> ScreenBackend {
    let _cursor_x: std::sync::Arc<std::sync::Mutex<f64>> =
        std::sync::Arc::new(std::sync::Mutex::new(0.0_f64));
    let _cursor_y: std::sync::Arc<std::sync::Mutex<f64>> =
        std::sync::Arc::new(std::sync::Mutex::new(0.0_f64));
    let _cursor_tracking: std::sync::Arc<std::sync::Mutex<bool>> =
        std::sync::Arc::new(std::sync::Mutex::new(false));
    let _cached_screens: std::sync::Arc<std::sync::Mutex<Option<Vec<ScreenInfo>>>> =
        std::sync::Arc::new(std::sync::Mutex::new(None));
    let _screen_details: std::sync::Arc<std::sync::Mutex<Option<ScreenDetails>>> =
        std::sync::Arc::new(std::sync::Mutex::new(None));
    let mut ensure_cursor_tracking: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>,
    > = std::sync::Arc::new(std::sync::Mutex::new(Box::new({
        let mut _cursor_tracking = _cursor_tracking.clone();
        let mut _cursor_x = _cursor_x.clone();
        let mut _cursor_y = _cursor_y.clone();
        move || -> () {
            return;
        }
    })
        as Box<dyn FnMut() -> () + Send + 'static>));
    let mut upgrade_to_screen_details: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut(ScreenDetails) -> () + Send + 'static>>,
    > = std::sync::Arc::new(std::sync::Mutex::new(Box::new({
        let mut _cached_screens = _cached_screens.clone();
        let mut _screen_details = _screen_details.clone();
        move |details: ScreenDetails| -> () {
            (*_screen_details.lock().unwrap()) = Some((details).clone());
            (*_cached_screens.lock().unwrap()) = None;
        }
    })
        as Box<dyn FnMut(ScreenDetails) -> () + Send + 'static>));
    let mut build_screen_info_from_detailed: std::sync::Arc<
        std::sync::Mutex<
            Box<dyn FnMut(ScreenDetailed, f64, f64, ScreenInfo) -> () + Send + 'static>,
        >,
    > = std::sync::Arc::new(std::sync::Mutex::new(Box::new(
        move |sd: ScreenDetailed, index: f64, primary_index: f64, mut out: ScreenInfo| -> () {
            out.id = index;
            out.x = sd.left;
            out.y = sd.top;
            out.width = sd.width;
            out.height = sd.height;
            out.work_width = sd.avail_width;
            out.work_height = sd.avail_height;
            out.scale_factor = sd.device_pixel_ratio;
            out.is_primary =
                (index == primary_index) || ((sd.is_primary).clone().unwrap_or((index == 0.0_f64)));
            out.rotation = get_web_rotation();
            out.orientation = get_web_orientation();
            out.refresh_rate = if (((sd.refresh_rate).as_ref().map_or("undefined", |_| "number"))
                .to_owned()
                == "number")
                && ((sd.refresh_rate)
                    .as_ref()
                    .is_some_and(|value| *value > 0.0_f64))
            {
                (sd.refresh_rate).unwrap()
            } else {
                (-1.0_f64)
            };
            out.color_depth = sd.color_depth;
            out.pixel_depth = sd.pixel_depth;
            out.physical_width = (out.width * out.scale_factor).round();
            out.physical_height = (out.height * out.scale_factor).round();
            out.is_hdr = get_web_is_hdr();
            out.color_space = get_web_color_space();
            out.max_luminance = (-1.0_f64);
            out.depth_per_component = (-1.0_f64);
            out.dpi = (-1.0_f64);
            out.label = (sd.label).clone();
            out.internal = (sd.is_internal).clone().unwrap_or(false);
            out.touch_support = "unknown".to_owned();
            out.monochrome = false;
        },
    )
        as Box<dyn FnMut(ScreenDetailed, f64, f64, ScreenInfo) -> () + Send + 'static>));
    let mut build_current_screen_info: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut(ScreenInfo) -> () + Send + 'static>>,
    > = std::sync::Arc::new(std::sync::Mutex::new(Box::new({
        let mut _screen_details = _screen_details.clone();
        let build_screen_info_from_detailed = build_screen_info_from_detailed.clone();
        move |mut out: ScreenInfo| -> () {
            let __flight_forward_s: std::sync::Arc<
                std::sync::Mutex<Option<crate::OpaqueHostValue>>,
            > = std::sync::Arc::new(std::sync::Mutex::new(None));
            {
                fill_default_screen_info(&mut out);
                return;
            }
        }
    })
        as Box<dyn FnMut(ScreenInfo) -> () + Send + 'static>));
    let backend: CreateWebScreenBackendRecord3 = CreateWebScreenBackendRecord3 {
        __flight_identity: std::sync::Arc::new(()),
        _upgrade: (upgrade_to_screen_details).clone(),
        get_screens: std::sync::Arc::new(std::sync::Mutex::new(Box::new({
            let mut _cached_screens = _cached_screens.clone();
            let mut _screen_details = _screen_details.clone();
            let build_current_screen_info = build_current_screen_info.clone();
            let build_screen_info_from_detailed = build_screen_info_from_detailed.clone();
            move |mut out: Vec<ScreenInfo>| -> Vec<ScreenInfo> {
                {
                    out.clear();
                    return out;
                }
            }
        })
            as Box<dyn FnMut(Vec<ScreenInfo>) -> Vec<ScreenInfo> + Send + 'static>)),
        get_primary_screen: std::sync::Arc::new(std::sync::Mutex::new(Box::new({
            let mut _screen_details = _screen_details.clone();
            let build_current_screen_info = build_current_screen_info.clone();
            let build_screen_info_from_detailed = build_screen_info_from_detailed.clone();
            move |mut out: ScreenInfo| -> ScreenInfo {
                {
                    fill_default_screen_info(&mut out);
                    return out;
                }
            }
        })
            as Box<dyn FnMut(ScreenInfo) -> ScreenInfo + Send + 'static>)),
        subscribe: std::sync::Arc::new(std::sync::Mutex::new(Box::new({
            let mut _cached_screens = _cached_screens.clone();
            let mut _screen_details = _screen_details.clone();
            let build_current_screen_info = build_current_screen_info.clone();
            let build_screen_info_from_detailed = build_screen_info_from_detailed.clone();
            move |listener: std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(ScreenChangeEvent) -> () + Send + 'static>>>| -> std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>> {
      return std::sync::Arc::new(std::sync::Mutex::new(Box::new(move || -> () {

      }) as Box<dyn FnMut() -> () + Send + 'static>));
    }
        })
            as Box<
                dyn FnMut(
                        std::sync::Arc<
                            std::sync::Mutex<
                                Box<dyn FnMut(ScreenChangeEvent) -> () + Send + 'static>,
                            >,
                        >,
                    ) -> std::sync::Arc<
                        std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>,
                    > + Send
                    + 'static,
            >)),
        get_cursor_position: std::sync::Arc::new(std::sync::Mutex::new(Box::new({
            let mut _cursor_x = _cursor_x.clone();
            let mut _cursor_y = _cursor_y.clone();
            let ensure_cursor_tracking = ensure_cursor_tracking.clone();
            move |mut out: flighthq_types::ScreenBackendRecord1| -> flighthq_types::ScreenBackendRecord1 {
      { let __flight_callback = (ensure_cursor_tracking).clone(); let __flight_result = __flight_callback.lock().unwrap()(); __flight_result };
      out.x = ((*_cursor_x.lock().unwrap())).clone();
      out.y = ((*_cursor_y.lock().unwrap())).clone();
      return out;
    }
        })
            as Box<
                dyn FnMut(
                        flighthq_types::ScreenBackendRecord1,
                    ) -> flighthq_types::ScreenBackendRecord1
                    + Send
                    + 'static,
            >)),
        get_modes: Some(std::sync::Arc::new(std::sync::Mutex::new(Box::new(
            move |screen: ScreenInfo, mut out: Vec<ScreenMode>| -> Vec<ScreenMode> {
                out.truncate((1.0_f64) as usize);
                if out.get((0.0_f64) as usize).is_none() {
                    {
                        let __flight_index = (0.0_f64) as usize;
                        let __flight_value = create_screen_mode();
                        if __flight_index == out.len() {
                            out.push(__flight_value);
                        } else {
                            out[__flight_index] = __flight_value;
                        }
                    };
                }
                out[0.0_f64 as usize].width = screen.width;
                out[0.0_f64 as usize].height = screen.height;
                out[0.0_f64 as usize].refresh_rate = screen.refresh_rate;
                out[0.0_f64 as usize].color_depth = screen.color_depth;
                out[0.0_f64 as usize].pixel_format = "".to_owned();
                return out;
            },
        )
            as Box<
                dyn FnMut(ScreenInfo, Vec<ScreenMode>) -> Vec<ScreenMode> + Send + 'static,
            >))),
    };
    return {
        let __flight_source = &(backend);
        ScreenBackend {
            __flight_identity: std::sync::Arc::clone(&__flight_source.__flight_identity),
            get_screens: (__flight_source.get_screens).clone(),
            get_primary_screen: (__flight_source.get_primary_screen).clone(),
            subscribe: (__flight_source.subscribe).clone(),
            get_cursor_position: (__flight_source.get_cursor_position).clone(),
            get_modes: (__flight_source.get_modes).clone(),
        }
    };
}

// Source: upstream/packages/screen/src/screen.ts:365 (sha256:173b6a22f2a1bb39de4996e96b4e86f93bc48f2a475bd828beca8544867e222b)
pub fn detach_screen_signals(signals: &ScreenSignals) -> () {
    let unsubscribe = (*_SIGNAL_SUBSCRIPTIONS.lock().unwrap())
        .iter()
        .find(|(entry_key, _)| entry_key == &(*signals).clone())
        .map(|(_, value)| value.clone());
    if (unsubscribe).is_some() {
        {
            let __flight_callback = (unsubscribe.as_ref().unwrap()).clone();
            let __flight_result = __flight_callback.lock().unwrap()();
            __flight_result
        };
        {
            let __flight_key = (*signals).clone();
            if let Some(__flight_index) = (*_SIGNAL_SUBSCRIPTIONS.lock().unwrap())
                .iter()
                .position(|(key, _)| key == &__flight_key)
            {
                (*_SIGNAL_SUBSCRIPTIONS.lock().unwrap()).remove(__flight_index);
                true
            } else {
                false
            }
        };
    }
}

// Source: upstream/packages/screen/src/screen.ts:376 (sha256:b846264d9cd448d91ff804180d6efa756769962d770ed1f17845e7e720b7fbbb)
pub fn dip_to_screen_point(
    screen: &ScreenInfo,
    point: &Vector2Like,
    out: &mut SharedStructuralRecord1,
) -> SharedStructuralRecord1 {
    let px = point.x;
    let py = point.y;
    out.x = ((px - screen.x) * screen.scale_factor);
    out.y = ((py - screen.y) * screen.scale_factor);
    return out.clone();
}

// Source: upstream/packages/screen/src/screen.ts:390 (sha256:278a0b9d8d48626a11f8fbfbd7a5d54717c4c1bdcde4644d66c1a4a82d009fec)
pub fn dip_to_screen_rect(
    screen: &ScreenInfo,
    rect: &RectangleLike,
    out: &mut SharedStructuralRecord2,
) -> SharedStructuralRecord2 {
    let rx = rect.x;
    let ry = rect.y;
    let rw = rect.width;
    let rh = rect.height;
    let sf = screen.scale_factor;
    out.x = ((rx - screen.x) * sf);
    out.y = ((ry - screen.y) * sf);
    out.width = (rw * sf);
    out.height = (rh * sf);
    return out.clone();
}

// Source: upstream/packages/screen/src/screen.ts:409 (sha256:460d1de1e6249bba632cae61c7a959eb120eca9a55e2ef5b0d44b815b784c64d)
pub fn dispose_screen_signals(signals: &ScreenSignals) -> () {
    detach_screen_signals(signals);
}

// Source: upstream/packages/screen/src/screen.ts:415 (sha256:0a457b6569f799c8bfab6c1c1f93a3038cad745770a259eb3c775f72f8f4f560)
pub fn enable_screen_signals() -> ScreenSignals {
    return create_screen_signals();
}

// Source: upstream/packages/screen/src/screen.ts:421 (sha256:fed7cab052c25b38db7f90531fd4d666d27b2a3498dd2dc8dd5beec3ecf75b0a)
pub fn get_primary_screen(out: &ScreenInfo) -> ScreenInfo {
    return {
        let __flight_callback = (get_screen_backend().get_primary_screen).clone();
        let __flight_result = __flight_callback.lock().unwrap()((*out).clone());
        __flight_result
    };
}

// Source: upstream/packages/screen/src/screen.ts:426 (sha256:630ac4a06b2d318150fab4355fe4dbab2be09cc949263c8f3843f6c2efd892e4)
pub fn get_screen_backend() -> ScreenBackend {
    if ((*_BACKEND.lock().unwrap()).clone()).is_none() {
        (*_BACKEND.lock().unwrap()) = Some(create_web_screen_backend());
    }
    return (((*_BACKEND.lock().unwrap()).clone()).clone().unwrap()).clone();
}

// Source: upstream/packages/screen/src/screen.ts:432 (sha256:1699596e9d4cc803c6bcc1993c582b54d78fbacfa4fcf43f4abd96be80a026c3)
pub fn get_screen_bounds(
    screen: &ScreenInfo,
    out: &mut SharedStructuralRecord2,
) -> SharedStructuralRecord2 {
    out.x = screen.x;
    out.y = screen.y;
    out.width = screen.width;
    out.height = screen.height;
    return out.clone();
}

// Source: upstream/packages/screen/src/screen.ts:444 (sha256:fa1f16b612cc9d5f48c70aab7d8f9d832a076f208dd43eb17b8b0e1d4125a457)
pub fn get_screen_by_id(id: f64, out: &mut ScreenInfo) -> Option<ScreenInfo> {
    let screens: Vec<ScreenInfo> = vec![];
    get_screens(&screens);
    for screen in (screens).iter().cloned() {
        if (screen.id == id) {
            copy_screen_info(&screen, out);
            return Some((*out).clone());
        }
    }
    return None;
}

// Source: upstream/packages/screen/src/screen.ts:458 (sha256:ee07c4b7ab11e0190cf9a5271aab8d69c37da431893281854ff5135834d2e6ba)
pub fn get_screen_containing_rect(rect: &RectangleLike, out: &mut ScreenInfo) -> ScreenInfo {
    let mut screens: Vec<ScreenInfo> = vec![];
    get_screens(&screens);
    if ((screens.len() as f64) == 0.0_f64) {
        fill_default_screen_info(out);
        return out.clone();
    }
    let mut best_screen = screens[0.0_f64 as usize].clone();
    let mut best_overlap = (-1.0_f64);
    for screen in (screens).iter().cloned() {
        let ox = (0.0_f64)
            .max(((rect.x + rect.width).min((screen.x + screen.width)) - (rect.x).max(screen.x)));
        let oy = (0.0_f64)
            .max(((rect.y + rect.height).min((screen.y + screen.height)) - (rect.y).max(screen.y)));
        let overlap = (ox * oy);
        if (overlap > best_overlap) {
            best_overlap = overlap;
            best_screen = (screen).clone();
        }
    }
    if (best_overlap <= 0.0_f64) {
        let cx = (rect.x + (rect.width / 2.0_f64));
        let cy = (rect.y + (rect.height / 2.0_f64));
        let mut best_dist = f64::INFINITY;
        for screen in (screens).iter().cloned() {
            let scx = (screen.x + (screen.width / 2.0_f64));
            let scy = (screen.y + (screen.height / 2.0_f64));
            let dx = (cx - scx);
            let dy = (cy - scy);
            let dist = ((dx * dx) + (dy * dy));
            if (dist < best_dist) {
                best_dist = dist;
                best_screen = (screen).clone();
            }
        }
    }
    copy_screen_info(&best_screen, out);
    return out.clone();
}

// Source: upstream/packages/screen/src/screen.ts:503 (sha256:8501d304e7b24801c7947fa2955455db69aa7cb62dd8bfe2f5357c677b09ff9e)
pub fn get_screen_current_mode(screen: &ScreenInfo, out: &mut ScreenMode) -> ScreenMode {
    out.width = screen.width;
    out.height = screen.height;
    out.refresh_rate = screen.refresh_rate;
    out.color_depth = screen.color_depth;
    out.pixel_format = "".to_owned();
    return out.clone();
}

// Source: upstream/packages/screen/src/screen.ts:515 (sha256:0a12423f918349fa3ba5ef09732af940e5c3650342955d71b9a56f0e6ec0a03d)
pub fn get_screen_cursor_position(out: &SharedStructuralRecord1) -> SharedStructuralRecord1 {
    return {
        let __flight_source = &({
            let __flight_callback = (get_screen_backend().get_cursor_position).clone();
            let __flight_result = __flight_callback.lock().unwrap()({
                let __flight_source = &((*out).clone());
                flighthq_types::ScreenBackendRecord1 {
                    __flight_identity: std::sync::Arc::clone(&__flight_source.__flight_identity),
                    x: __flight_source.x,
                    y: __flight_source.y,
                }
            });
            __flight_result
        });
        SharedStructuralRecord1 {
            __flight_identity: std::sync::Arc::clone(&__flight_source.__flight_identity),
            x: __flight_source.x,
            y: __flight_source.y,
        }
    };
}

// Source: upstream/packages/screen/src/screen.ts:521 (sha256:be2a9b2f913f87a41259e57f4d2f61735e8043b828492838c5c3d5e0784cb8d2)
pub fn get_screen_cursor_screen(out: &mut ScreenInfo) -> ScreenInfo {
    get_screen_cursor_position(&{
        let __flight_source = &(_SCRATCH_POINT);
        SharedStructuralRecord1 {
            __flight_identity: std::sync::Arc::clone(&__flight_source.__flight_identity),
            x: __flight_source.x,
            y: __flight_source.y,
        }
    });
    return get_screen_nearest_point(
        &{
            let __flight_source = &(_SCRATCH_POINT);
            Vector2Like {
                __flight_identity: std::sync::Arc::clone(&__flight_source.__flight_identity),
                __flight_entity_runtime: Default::default(),
                x: __flight_source.x,
                y: __flight_source.y,
            }
        },
        out,
    );
}

// Source: upstream/packages/screen/src/screen.ts:530 (sha256:d8cf40fc979d5a3f700e818acbaf83c4a8782cf0e2f81dbb95cae4928a164ea2)
pub fn get_screen_detail_permission() -> crate::FlightTask<String> {
    crate::FlightTask::start(
        async move {
            return Ok("prompt".to_owned());
        },
        crate::FlightTaskOrigin {
            package: "@flighthq/screen",
            source: "upstream/packages/screen/src/screen.ts",
            line: 530_u32,
            column: 1_u32,
            lexical_path: "getScreenDetailPermission",
            fingerprint: "sha256:d8cf40fc979d5a3f700e818acbaf83c4a8782cf0e2f81dbb95cae4928a164ea2",
        },
    )
}

// Source: upstream/packages/screen/src/screen.ts:544 (sha256:c89dfe8423ca73c894b254646993a68a641b914d62311412b9d782e960be8089)
pub fn get_screen_modes(screen: &ScreenInfo, out: &mut Vec<ScreenMode>) -> Vec<ScreenMode> {
    let backend = get_screen_backend();
    if ((backend.get_modes).clone()).is_some() {
        return {
            let __flight_callback = (backend.get_modes).clone().as_ref().unwrap().clone();
            let __flight_result =
                __flight_callback.lock().unwrap()((*screen).clone(), (*out).clone());
            __flight_result
        };
    }
    out.truncate((1.0_f64) as usize);
    if out.get((0.0_f64) as usize).is_none() {
        {
            let __flight_index = (0.0_f64) as usize;
            let __flight_value = create_screen_mode();
            if __flight_index == out.len() {
                out.push(__flight_value);
            } else {
                out[__flight_index] = __flight_value;
            }
        };
    }
    get_screen_current_mode(screen, &mut out[0.0_f64 as usize]);
    return out.clone();
}

// Source: upstream/packages/screen/src/screen.ts:558 (sha256:745c3bbef566870b9118277bfaf3d9745be4e0f9d58c0ed846ce51a7d5ebf0ab)
pub fn get_screen_nearest_point(point: &Vector2Like, out: &mut ScreenInfo) -> ScreenInfo {
    let mut screens: Vec<ScreenInfo> = vec![];
    get_screens(&screens);
    if ((screens.len() as f64) == 0.0_f64) {
        fill_default_screen_info(out);
        return out.clone();
    }
    for screen in (screens).iter().cloned() {
        if (((point.x >= screen.x) && (point.x < (screen.x + screen.width)))
            && (point.y >= screen.y))
            && (point.y < (screen.y + screen.height))
        {
            copy_screen_info(&screen, out);
            return out.clone();
        }
    }
    let mut best_screen = screens[0.0_f64 as usize].clone();
    let mut best_dist = f64::INFINITY;
    for screen in (screens).iter().cloned() {
        let cx = (screen.x + (screen.width / 2.0_f64));
        let cy = (screen.y + (screen.height / 2.0_f64));
        let dx = (point.x - cx);
        let dy = (point.y - cy);
        let dist = ((dx * dx) + (dy * dy));
        if (dist < best_dist) {
            best_dist = dist;
            best_screen = (screen).clone();
        }
    }
    copy_screen_info(&best_screen, out);
    return out.clone();
}

// Source: upstream/packages/screen/src/screen.ts:604 (sha256:fe2cd8ac2d3d002d5c7f7c414302c150294365b1100e2147e49d9a118863b3da)
pub fn get_screen_nearest_rect(rect: &RectangleLike, out: &mut ScreenInfo) -> ScreenInfo {
    let mut screens: Vec<ScreenInfo> = vec![];
    get_screens(&screens);
    if ((screens.len() as f64) == 0.0_f64) {
        fill_default_screen_info(out);
        return out.clone();
    }
    for screen in (screens).iter().cloned() {
        if (((rect.x >= screen.x) && (rect.y >= screen.y))
            && ((rect.x + rect.width) <= (screen.x + screen.width)))
            && ((rect.y + rect.height) <= (screen.y + screen.height))
        {
            copy_screen_info(&screen, out);
            return out.clone();
        }
    }
    let cx = (rect.x + (rect.width / 2.0_f64));
    let cy = (rect.y + (rect.height / 2.0_f64));
    let mut best_screen = screens[0.0_f64 as usize].clone();
    let mut best_dist = f64::INFINITY;
    for screen in (screens).iter().cloned() {
        let scx = (screen.x + (screen.width / 2.0_f64));
        let scy = (screen.y + (screen.height / 2.0_f64));
        let dx = (cx - scx);
        let dy = (cy - scy);
        let dist = ((dx * dx) + (dy * dy));
        if (dist < best_dist) {
            best_dist = dist;
            best_screen = (screen).clone();
        }
    }
    copy_screen_info(&best_screen, out);
    return out.clone();
}

// Source: upstream/packages/screen/src/screen.ts:651 (sha256:57b2f23541faeac62df3c28435ae020a1f7831542a65a34492f6f18e699f7363)
pub fn get_screens(out: &Vec<ScreenInfo>) -> Vec<ScreenInfo> {
    return {
        let __flight_callback = (get_screen_backend().get_screens).clone();
        let __flight_result = __flight_callback.lock().unwrap()((*out).clone());
        __flight_result
    };
}

// Source: upstream/packages/screen/src/screen.ts:656 (sha256:9bd513b6668c54ae3b05c49908e593b469de7c93e68441b54a0ed9f55a376771)
pub fn get_screen_work_area(
    screen: &ScreenInfo,
    out: &mut SharedStructuralRecord2,
) -> SharedStructuralRecord2 {
    out.x = screen.x;
    out.y = screen.y;
    out.width = screen.work_width;
    out.height = screen.work_height;
    return out.clone();
}

// Source: upstream/packages/screen/src/screen.ts:670 (sha256:830f07d42d0c685540f1d735d4e46a47947049bfd0a56ea2feb29d13be9b84ea)
pub fn on_screen_change(
    listener: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut(ScreenChangeEvent) -> () + Send + 'static>>,
    >,
) -> std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>> {
    return {
        let __flight_callback = (get_screen_backend().subscribe).clone();
        let __flight_result = __flight_callback.lock().unwrap()((listener).clone());
        __flight_result
    };
}

// Source: upstream/packages/screen/src/screen.ts:679 (sha256:ed3ef35d9e5bc275e32db76390061efe0bf46d5522d2ec54cde48fc51d4bc6b8)
pub fn on_screen_detail_permission_change(
    listener: std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(String) -> () + Send + 'static>>>,
) -> std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>> {
    return std::sync::Arc::new(std::sync::Mutex::new(
        Box::new(move || -> () {}) as Box<dyn FnMut() -> () + Send + 'static>
    ));
}

// Source: upstream/packages/screen/src/screen.ts:703 (sha256:b22f7025653881bba5becb49d6754145df42e97147008daf415fb064207d394b)
pub fn refresh_screens() -> () {}

// Source: upstream/packages/screen/src/screen.ts:718 (sha256:b0810a06330b382391847f094e14db0df4a90f6bac18db9fac03a59be8a83ff4)
pub fn request_screen_details() -> crate::FlightTask<bool> {
    crate::FlightTask::start(
        async move {
            return Ok(false);
        },
        crate::FlightTaskOrigin {
            package: "@flighthq/screen",
            source: "upstream/packages/screen/src/screen.ts",
            line: 718_u32,
            column: 1_u32,
            lexical_path: "requestScreenDetails",
            fingerprint: "sha256:b0810a06330b382391847f094e14db0df4a90f6bac18db9fac03a59be8a83ff4",
        },
    )
}

// Source: upstream/packages/screen/src/screen.ts:736 (sha256:81d06e07b97fb5e0927de45fe4703065aaca7b145e872af5581a17e3d18707db)
pub fn screen_to_dip_point(
    screen: &ScreenInfo,
    point: &Vector2Like,
    out: &mut SharedStructuralRecord1,
) -> SharedStructuralRecord1 {
    let px = point.x;
    let py = point.y;
    out.x = ((px / screen.scale_factor) + screen.x);
    out.y = ((py / screen.scale_factor) + screen.y);
    return out.clone();
}

// Source: upstream/packages/screen/src/screen.ts:750 (sha256:50a0bbe056f573cc0b0e69c7c5c21336534ae1e53e8ca538ac4e36438555f9af)
pub fn screen_to_dip_rect(
    screen: &ScreenInfo,
    rect: &RectangleLike,
    out: &mut SharedStructuralRecord2,
) -> SharedStructuralRecord2 {
    let rx = rect.x;
    let ry = rect.y;
    let rw = rect.width;
    let rh = rect.height;
    let sf = screen.scale_factor;
    out.x = ((rx / sf) + screen.x);
    out.y = ((ry / sf) + screen.y);
    out.width = (rw / sf);
    out.height = (rh / sf);
    return out.clone();
}

// Source: upstream/packages/screen/src/screen.ts:768 (sha256:31514bc7574cf09f3e59a43ab28401928aabd6ba0c3fca9edd57972ed0e323b7)
pub fn set_screen_backend(backend: &Option<ScreenBackend>) -> () {
    (*_BACKEND.lock().unwrap()) = (*backend).clone();
}

// Source: upstream/packages/screen/src/screen.ts:772 (sha256:478b59e31f645b973aceecaab3659584e2eed603e31838e2afa9caf628c4690c)
static _BACKEND: std::sync::LazyLock<std::sync::Mutex<Option<ScreenBackend>>> =
    std::sync::LazyLock::new(|| std::sync::Mutex::new(None));

// Source: upstream/packages/screen/src/screen.ts:773 (sha256:6959a9457f8b332653a8f42fc2fff5e348cc9da6152f8664507ad27c7a56a0e6)
static _SIGNAL_SUBSCRIPTIONS: std::sync::LazyLock<
    std::sync::Mutex<
        Vec<(
            ScreenSignals,
            std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>,
        )>,
    >,
> = std::sync::LazyLock::new(|| std::sync::Mutex::new(Vec::new()));

// Source: upstream/packages/screen/src/screen.ts:774 (sha256:73604c011ae140a8578ea3a4bc8d91483155cadc9e970e79ffa1bbc94d2e2164)
#[derive(Clone, Default)]
pub(crate) struct ScratchPoint {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub x: f64,
    pub y: f64,
}
impl PartialEq for ScratchPoint {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

static _SCRATCH_POINT: std::sync::LazyLock<ScratchPoint> =
    std::sync::LazyLock::new(|| ScratchPoint {
        __flight_identity: std::sync::Arc::new(()),
        x: 0.0_f64,
        y: 0.0_f64,
    });

// Source: upstream/packages/screen/src/screen.ts:777 (sha256:5f65aea7b8ef8174e95bc26b2ec39e3a671f58113b9bb90dfae71afb12b9dd0c)
fn copy_screen_info(src: &ScreenInfo, dst: &mut ScreenInfo) -> () {
    dst.id = src.id;
    dst.x = src.x;
    dst.y = src.y;
    dst.width = src.width;
    dst.height = src.height;
    dst.work_width = src.work_width;
    dst.work_height = src.work_height;
    dst.scale_factor = src.scale_factor;
    dst.is_primary = src.is_primary;
    dst.rotation = src.rotation;
    dst.orientation = (src.orientation).clone();
    dst.refresh_rate = src.refresh_rate;
    dst.color_depth = src.color_depth;
    dst.pixel_depth = src.pixel_depth;
    dst.physical_width = src.physical_width;
    dst.physical_height = src.physical_height;
    dst.is_hdr = src.is_hdr;
    dst.color_space = (src.color_space).clone();
    dst.max_luminance = src.max_luminance;
    dst.depth_per_component = src.depth_per_component;
    dst.dpi = src.dpi;
    dst.label = (src.label).clone();
    dst.internal = src.internal;
    dst.touch_support = (src.touch_support).clone();
    dst.monochrome = src.monochrome;
}

// Source: upstream/packages/screen/src/screen.ts:806 (sha256:d83acc8b99da7b9ec44425756ac1b6fd5a730cdd91823c815f16c475c528498c)
fn diff_screen_info(prev: &ScreenInfo, curr: &ScreenInfo) -> Option<ScreenChangedMetrics> {
    let bounds_changed = (((prev.x != curr.x) || (prev.y != curr.y)) || (prev.width != curr.width))
        || (prev.height != curr.height);
    let work_area_changed =
        (prev.work_width != curr.work_width) || (prev.work_height != curr.work_height);
    let scale_changed = (prev.scale_factor != curr.scale_factor);
    let orientation_changed = (prev.rotation != curr.rotation)
        || ((prev.orientation).clone() != (curr.orientation).clone());
    if (((!bounds_changed) && (!work_area_changed)) && (!scale_changed)) && (!orientation_changed) {
        return None;
    }
    return Some(ScreenChangedMetrics {
        __flight_identity: std::sync::Arc::new(()),
        bounds: bounds_changed,
        work_area: work_area_changed,
        scale_factor: scale_changed,
        orientation: orientation_changed,
    });
}

// Source: upstream/packages/screen/src/screen.ts:822 (sha256:abcc45780cc371ea2d6a67e273180d29c799a76b4a8d6b61371e3b8576b8ffd0)
fn fill_default_screen_info(out: &mut ScreenInfo) -> () {
    out.id = 0.0_f64;
    out.x = 0.0_f64;
    out.y = 0.0_f64;
    out.width = 0.0_f64;
    out.height = 0.0_f64;
    out.work_width = 0.0_f64;
    out.work_height = 0.0_f64;
    out.scale_factor = 1.0_f64;
    out.is_primary = false;
    out.rotation = (-1.0_f64);
    out.orientation = "Landscape".to_owned();
    out.refresh_rate = (-1.0_f64);
    out.color_depth = (-1.0_f64);
    out.pixel_depth = (-1.0_f64);
    out.physical_width = (-1.0_f64);
    out.physical_height = (-1.0_f64);
    out.is_hdr = false;
    out.color_space = "srgb".to_owned();
    out.max_luminance = (-1.0_f64);
    out.depth_per_component = (-1.0_f64);
    out.dpi = (-1.0_f64);
    out.label = "".to_owned();
    out.internal = false;
    out.touch_support = "unknown".to_owned();
    out.monochrome = false;
}

// Source: upstream/packages/screen/src/screen.ts:850 (sha256:53fcc57f2524aab09a52e4b5c51cb6e54304aaed9704f40cd888a6df8e8e6058)
#[derive(Clone, Default)]
pub(crate) struct WebScreenOrientationObject {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub angle: Option<f64>,
    pub type_: Option<String>,
    pub add_event_listener: Option<
        std::sync::Arc<
            std::sync::Mutex<
                Box<
                    dyn FnMut(
                            String,
                            std::sync::Arc<
                                std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>,
                            >,
                        ) -> ()
                        + Send
                        + 'static,
                >,
            >,
        >,
    >,
    pub remove_event_listener: Option<
        std::sync::Arc<
            std::sync::Mutex<
                Box<
                    dyn FnMut(
                            String,
                            std::sync::Arc<
                                std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>,
                            >,
                        ) -> ()
                        + Send
                        + 'static,
                >,
            >,
        >,
    >,
}
impl PartialEq for WebScreenOrientationObject {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/screen/src/screen.ts:857 (sha256:a0b9f8c0703f5df20c6683988a6a5e56c4b6d5189d32f75849b32ff4cff82674)
fn get_web_color_space() -> ScreenColorSpace {
    return "srgb".to_owned();
}

// Source: upstream/packages/screen/src/screen.ts:864 (sha256:ea18cebb7895dfc2644e420056a9feced456e68c688ba0802dd98fc2c5da4e06)
fn get_web_is_hdr() -> bool {
    return false;
}

// Source: upstream/packages/screen/src/screen.ts:869 (sha256:4c2cb10dd0e30ef93aefbdb0a3379538b92ef52031ef2938498f6e6c9e80ba94)
fn get_web_orientation() -> ScreenOrientation {
    let obj = get_web_screen_orientation_object();
    let type_ = (obj.as_ref().and_then(|value| (value.type_).clone()))
        .clone()
        .unwrap_or("".to_owned());
    if (type_).starts_with(("portrait-primary".to_owned()).as_str()) {
        return "Portrait".to_owned();
    }
    if (type_).starts_with(("portrait-secondary".to_owned()).as_str()) {
        return "PortraitFlipped".to_owned();
    }
    if (type_).starts_with(("landscape-secondary".to_owned()).as_str()) {
        return "LandscapeFlipped".to_owned();
    }
    return "Landscape".to_owned();
}

// Source: upstream/packages/screen/src/screen.ts:878 (sha256:fee49e70662f987d91152dd172832ff4d1972b552eb0c0cf1ecfc2fbfc7de418)
fn get_web_rotation() -> f64 {
    let obj = get_web_screen_orientation_object();
    let angle = obj.as_ref().and_then(|value| value.angle);
    if (((angle).as_ref().map_or("undefined", |_| "number")).to_owned() == "number") {
        return (angle).clone().unwrap();
    }
    return (-1.0_f64);
}

// Source: upstream/packages/screen/src/screen.ts:885 (sha256:9d54c7c81e3a45b71de47bad0122c6c29b3d660eba5b9a639fafaa18d7b1c6ae)
fn get_web_screen_orientation_object() -> Option<WebScreenOrientationObject> {
    return None;
}
