// @generated from upstream/packages/screen/src/screen.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_entity::{allocate_entity, finish_entity};
use flighthq_signals::{clear_signal, create_signal, emit_signal};
use flighthq_types::{
    EntityConstruction, HostScreenChangeCapability, HostScreenDetailsCapability,
    HostScreenPermissionChangeCapability, HostScreenQueryCapability, RectangleLike,
    ScreenChangeEvent, ScreenInfo, ScreenMode, ScreenPermissionChange, ScreenPermissionState,
    ScreenSignals, Vector2, Vector2Like,
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

// Source: upstream/packages/screen/src/screen.ts:18 (sha256:2dc286ed3399e405bdf9164ed239fcac53ac58e6013532db8f00570bb334b2e1)
pub fn attach_screen_permission_change(
    host_screen_permission_change: &HostScreenPermissionChangeCapability,
    permission_change: ScreenPermissionChange,
) -> () {
    detach_screen_permission_change(&permission_change);
    let unsubscribe = {
        let __flight_callback = (host_screen_permission_change.subscribe).clone();
        let __flight_result = __flight_callback.lock().unwrap()(std::sync::Arc::new(
            std::sync::Mutex::new(Box::new({
                let permission_change = permission_change.clone();
                move |state: ScreenPermissionState| -> () {
                    emit_signal((permission_change.on_change).clone(), ((state).clone(),))
                }
            })
                as Box<dyn FnMut(ScreenPermissionState) -> () + Send + 'static>),
        ));
        __flight_result
    };
    {
        let __flight_key = (permission_change).clone();
        let __flight_value = (unsubscribe).clone();
        if let Some((_, value)) = (*_PERMISSION_SUBSCRIPTIONS.lock().unwrap())
            .iter_mut()
            .find(|(key, _)| key == &__flight_key)
        {
            *value = __flight_value;
        } else {
            (*_PERMISSION_SUBSCRIPTIONS.lock().unwrap()).push((__flight_key, __flight_value));
        }
    };
}

// Source: upstream/packages/screen/src/screen.ts:27 (sha256:59ad764d2084f2ad2cf4ee9de81b0a7cfb587bd1d2431d9e0eeb52e05e5ee86d)
pub fn attach_screen_signals(
    host_screen_change: &HostScreenChangeCapability,
    signals: ScreenSignals,
) -> () {
    detach_screen_signals(&signals);
    let unsubscribe = {
        let __flight_callback = (host_screen_change.subscribe).clone();
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

// Source: upstream/packages/screen/src/screen.ts:40 (sha256:94cc5f4be1381e3cf4768f08222bcdb55e22f3f9ced244545544e09f97a3fcd0)
pub fn create_screen_info() -> ScreenInfo {
    let mut out = allocate_entity();
    initialize_screen_info((out).clone());
    return finish_entity((out).clone());
}

// Source: upstream/packages/screen/src/screen.ts:46 (sha256:1a6f92a6597f0fe7cbf59afb7f14b4edec5f1d2a44ec46df8e00ec39f1fbf767)
pub fn create_screen_mode() -> ScreenMode {
    let mut out = allocate_entity();
    initialize_screen_mode((out).clone());
    return finish_entity((out).clone());
}

// Source: upstream/packages/screen/src/screen.ts:52 (sha256:46853e6c946fc83943046670397387314df049d6dc6e91c28e4e0705b492601a)
pub fn create_screen_permission_change() -> ScreenPermissionChange {
    let mut out = allocate_entity();
    initialize_screen_permission_change((out).clone());
    return finish_entity((out).clone());
}

// Source: upstream/packages/screen/src/screen.ts:58 (sha256:6ab620e7e70e67bf2ddaeb2c55573db42660a0d6312e31f285a4d9bcdf56f415)
pub fn create_screen_signals() -> ScreenSignals {
    let mut out = allocate_entity();
    initialize_screen_signals((out).clone());
    return finish_entity((out).clone());
}

// Source: upstream/packages/screen/src/screen.ts:64 (sha256:9be6efb1cfea0a1ee455f1262a2e56dacc00b4cc1aa978b70aa4057fa1674c23)
pub fn detach_screen_permission_change(permission_change: &ScreenPermissionChange) -> () {
    {
        let __flight_callback = (*_PERMISSION_SUBSCRIPTIONS.lock().unwrap())
            .iter()
            .find(|(entry_key, _)| entry_key == &(*permission_change).clone())
            .map(|(_, value)| value.clone());
        __flight_callback
            .as_ref()
            .map(|callback| callback.lock().unwrap()())
    };
    {
        let __flight_key = (*permission_change).clone();
        if let Some(__flight_index) = (*_PERMISSION_SUBSCRIPTIONS.lock().unwrap())
            .iter()
            .position(|(key, _)| key == &__flight_key)
        {
            (*_PERMISSION_SUBSCRIPTIONS.lock().unwrap()).remove(__flight_index);
            true
        } else {
            false
        }
    };
}

// Source: upstream/packages/screen/src/screen.ts:69 (sha256:27eb2dba2551e33bdfff4e7148f78ff69ad92b81b2c4070aafee410daa53c6cc)
pub fn detach_screen_signals(signals: &ScreenSignals) -> () {
    {
        let __flight_callback = (*_SIGNAL_SUBSCRIPTIONS.lock().unwrap())
            .iter()
            .find(|(entry_key, _)| entry_key == &(*signals).clone())
            .map(|(_, value)| value.clone());
        __flight_callback
            .as_ref()
            .map(|callback| callback.lock().unwrap()())
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

// Source: upstream/packages/screen/src/screen.ts:74 (sha256:f11e4869bdb0ab7cfbe75c95557cddc0d19e5777be82f117d7bf0a1c22e4e5fb)
pub fn dip_to_screen_point(
    screen: &ScreenInfo,
    point: &Vector2Like,
    out: &mut SharedStructuralRecord1,
) -> SharedStructuralRecord1 {
    out.x = ((point.x - screen.x) * screen.scale_factor);
    out.y = ((point.y - screen.y) * screen.scale_factor);
    return out.clone();
}

// Source: upstream/packages/screen/src/screen.ts:84 (sha256:4f3ada497d423fde83dc71caa95d61eeae32327d2e9e97d2d63316397038c2bf)
pub fn dip_to_screen_rect(
    screen: &ScreenInfo,
    rect: &RectangleLike,
    out: &mut SharedStructuralRecord2,
) -> SharedStructuralRecord2 {
    out.x = ((rect.x - screen.x) * screen.scale_factor);
    out.y = ((rect.y - screen.y) * screen.scale_factor);
    out.width = (rect.width * screen.scale_factor);
    out.height = (rect.height * screen.scale_factor);
    return out.clone();
}

// Source: upstream/packages/screen/src/screen.ts:96 (sha256:2e760200a233aef7688c5638b04b045acc94cd5045f29aa3fd96abecd92daa18)
pub fn dispose_screen_permission_change(permission_change: &mut ScreenPermissionChange) -> () {
    detach_screen_permission_change(permission_change);
    clear_signal(&mut permission_change.on_change);
}

// Source: upstream/packages/screen/src/screen.ts:101 (sha256:885f941243e7cd1b540c133cd38f7b4dc04b395356d2d847e175cbf775c5d029)
pub fn dispose_screen_signals(signals: &mut ScreenSignals) -> () {
    detach_screen_signals(signals);
    clear_signal(&mut signals.on_screen_added);
    clear_signal(&mut signals.on_screen_metrics_changed);
    clear_signal(&mut signals.on_screen_removed);
}

// Source: upstream/packages/screen/src/screen.ts:108 (sha256:c292b23bff162f6bd35575546625c37935fa69518974916c2eed2f7f97cd8d28)
pub fn get_primary_screen(
    host_screen_query: &HostScreenQueryCapability,
    out: &ScreenInfo,
) -> ScreenInfo {
    return {
        let __flight_callback = (host_screen_query.get_primary_screen).clone();
        let __flight_result = __flight_callback.lock().unwrap()((*out).clone());
        __flight_result
    };
}

// Source: upstream/packages/screen/src/screen.ts:112 (sha256:1699596e9d4cc803c6bcc1993c582b54d78fbacfa4fcf43f4abd96be80a026c3)
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

// Source: upstream/packages/screen/src/screen.ts:123 (sha256:e4843dc5ff8044847b405ea3833db756544dc329c3f9e1cfb4503e78d02031e3)
pub fn get_screen_by_id(
    host_screen_query: &HostScreenQueryCapability,
    id: f64,
    out: &ScreenInfo,
) -> Option<ScreenInfo> {
    (*_SCRATCH_SCREENS.lock().unwrap()).clear();
    get_screens(host_screen_query, &(*_SCRATCH_SCREENS.lock().unwrap()));
    let found = (*_SCRATCH_SCREENS.lock().unwrap())
        .iter()
        .find(|value| (|screen: ScreenInfo| -> bool { (screen.id == id) })((*value).clone()))
        .cloned();
    if (found).is_none() {
        return None;
    }
    copy_screen_info(&found.as_ref().unwrap(), out);
    return Some((*out).clone());
}

// Source: upstream/packages/screen/src/screen.ts:137 (sha256:4bd6905bb7ef8833b90520946b0972621a5f3520441afa8ab461ce7afe6a85f3)
pub fn get_screen_containing_rect(
    host_screen_query: &HostScreenQueryCapability,
    rect: &RectangleLike,
    out: &ScreenInfo,
) -> ScreenInfo {
    (*_SCRATCH_SCREENS.lock().unwrap()).clear();
    get_screens(host_screen_query, &(*_SCRATCH_SCREENS.lock().unwrap()));
    if (((*_SCRATCH_SCREENS.lock().unwrap()).len() as f64) == 0.0_f64) {
        return fill_default_screen_info(out);
    }
    let mut best = (*_SCRATCH_SCREENS.lock().unwrap())[0.0_f64 as usize].clone();
    let mut best_overlap = (-1.0_f64);
    for screen in (*_SCRATCH_SCREENS.lock().unwrap()).iter().cloned() {
        let overlap_x = (0.0_f64)
            .max(((rect.x + rect.width).min((screen.x + screen.width)) - (rect.x).max(screen.x)));
        let overlap_y = (0.0_f64)
            .max(((rect.y + rect.height).min((screen.y + screen.height)) - (rect.y).max(screen.y)));
        let overlap = (overlap_x * overlap_y);
        if (overlap > best_overlap) {
            best = (screen).clone();
            best_overlap = overlap;
        }
    }
    if (best_overlap <= 0.0_f64) {
        return get_screen_nearest_point(host_screen_query, &rect_center(rect), out);
    }
    copy_screen_info(&best, out);
    return out.clone();
}

// Source: upstream/packages/screen/src/screen.ts:162 (sha256:8501d304e7b24801c7947fa2955455db69aa7cb62dd8bfe2f5357c677b09ff9e)
pub fn get_screen_current_mode(screen: &ScreenInfo, out: &mut ScreenMode) -> ScreenMode {
    out.width = screen.width;
    out.height = screen.height;
    out.refresh_rate = screen.refresh_rate;
    out.color_depth = screen.color_depth;
    out.pixel_format = "".to_owned();
    return out.clone();
}

// Source: upstream/packages/screen/src/screen.ts:171 (sha256:2fb2342f3f38ab969e0702cfd790ed2193703b6d23e6177d573705242121fe64)
pub fn get_screen_cursor_position(
    host_screen_query: &HostScreenQueryCapability,
    out: &SharedStructuralRecord1,
) -> SharedStructuralRecord1 {
    return {
        let __flight_callback = (host_screen_query.get_cursor_position).clone();
        let __flight_result = __flight_callback.lock().unwrap()((*out).clone());
        __flight_result
    };
}

// Source: upstream/packages/screen/src/screen.ts:178 (sha256:877336c3ca30ff8d4fc64de135af2715b20844ef2e5a3bf651bb4bf621b93818)
pub fn get_screen_cursor_screen(
    host_screen_query: &HostScreenQueryCapability,
    out: &ScreenInfo,
) -> ScreenInfo {
    get_screen_cursor_position(host_screen_query, &{
        let __flight_source = &(_SCRATCH_POINT);
        SharedStructuralRecord1 {
            __flight_identity: std::sync::Arc::clone(&__flight_source.__flight_identity),
            x: __flight_source.x,
            y: __flight_source.y,
        }
    });
    return get_screen_nearest_point(
        host_screen_query,
        &{
            let __flight_source = &(_SCRATCH_POINT);
            Vector2Like {
                __flight_identity: std::sync::Arc::clone(&__flight_source.__flight_identity),
                __flight_entity_runtime: std::sync::Arc::clone(
                    &__flight_source.__flight_entity_runtime,
                ),
                __flight_entity_snapshot: __flight_source.__flight_entity_snapshot.clone(),
                x: __flight_source.x,
                y: __flight_source.y,
            }
        },
        out,
    );
}

// Source: upstream/packages/screen/src/screen.ts:186 (sha256:8d4109ff950baa99b277c6fbecdbe74c120bdacec3740eda7034389d2548cff9)
pub fn get_screen_detail_permission(
    host_screen_details: &HostScreenDetailsCapability,
) -> crate::FlightTask<ScreenPermissionState> {
    return {
        let __flight_callback = (host_screen_details.query_permission).clone();
        let __flight_result = __flight_callback.lock().unwrap()();
        __flight_result
    };
}

// Source: upstream/packages/screen/src/screen.ts:192 (sha256:4d0f8ded624c503854b0ae44f3fbce34260f2bcf0453bed51d672075cd2d7966)
pub fn get_screen_nearest_point(
    host_screen_query: &HostScreenQueryCapability,
    point: &Vector2Like,
    out: &ScreenInfo,
) -> ScreenInfo {
    (*_SCRATCH_SCREENS.lock().unwrap()).clear();
    get_screens(host_screen_query, &(*_SCRATCH_SCREENS.lock().unwrap()));
    if (((*_SCRATCH_SCREENS.lock().unwrap()).len() as f64) == 0.0_f64) {
        return fill_default_screen_info(out);
    }
    for screen in (*_SCRATCH_SCREENS.lock().unwrap()).iter().cloned() {
        if (((point.x >= screen.x) && (point.x < (screen.x + screen.width)))
            && (point.y >= screen.y))
            && (point.y < (screen.y + screen.height))
        {
            copy_screen_info(&screen, out);
            return out.clone();
        }
    }
    let mut best = (*_SCRATCH_SCREENS.lock().unwrap())[0.0_f64 as usize].clone();
    let mut best_distance = f64::INFINITY;
    for screen in (*_SCRATCH_SCREENS.lock().unwrap()).iter().cloned() {
        let dx = (point.x - (screen.x + (screen.width / 2.0_f64)));
        let dy = (point.y - (screen.y + (screen.height / 2.0_f64)));
        let distance = ((dx * dx) + (dy * dy));
        if (distance < best_distance) {
            best = (screen).clone();
            best_distance = distance;
        }
    }
    copy_screen_info(&best, out);
    return out.clone();
}

// Source: upstream/packages/screen/src/screen.ts:227 (sha256:f14ec0d29979181fc53515a04663c28bddf535edfa15386fbe1981d871f4bc68)
pub fn get_screen_nearest_rect(
    host_screen_query: &HostScreenQueryCapability,
    rect: RectangleLike,
    out: &ScreenInfo,
) -> ScreenInfo {
    (*_SCRATCH_SCREENS.lock().unwrap()).clear();
    get_screens(host_screen_query, &(*_SCRATCH_SCREENS.lock().unwrap()));
    let containing = (*_SCRATCH_SCREENS.lock().unwrap())
        .iter()
        .find(|value| {
            (|screen: ScreenInfo| -> bool {
                (((rect.x >= screen.x) && (rect.y >= screen.y))
                    && ((rect.x + rect.width) <= (screen.x + screen.width)))
                    && ((rect.y + rect.height) <= (screen.y + screen.height))
            })((*value).clone())
        })
        .cloned();
    if (containing).is_some() {
        copy_screen_info(&containing.as_ref().unwrap(), out);
        return out.clone();
    }
    return get_screen_nearest_point(host_screen_query, &rect_center(&rect), out);
}

// Source: upstream/packages/screen/src/screen.ts:249 (sha256:1dd22dff3b5902ea6b238a81e339af079e083aff6fcff1cfd8b6c3507fadae0d)
pub fn get_screens(
    host_screen_query: &HostScreenQueryCapability,
    out: &Vec<ScreenInfo>,
) -> Vec<ScreenInfo> {
    return {
        let __flight_callback = (host_screen_query.get_screens).clone();
        let __flight_result = __flight_callback.lock().unwrap()((*out).clone());
        __flight_result
    };
}

// Source: upstream/packages/screen/src/screen.ts:253 (sha256:9bd513b6668c54ae3b05c49908e593b469de7c93e68441b54a0ed9f55a376771)
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

// Source: upstream/packages/screen/src/screen.ts:264 (sha256:8cad08e6d91f54b2056eaf1fb0c65042341a10a6162d0a9a768725dfe2590d6b)
pub fn initialize_screen_info(out: EntityConstruction<ScreenInfo>) -> () {
    crate::host_set("host.id", 0.0_f64);
    crate::host_set("host.x", 0.0_f64);
    crate::host_set("host.y", 0.0_f64);
    crate::host_set("host.width", 0.0_f64);
    crate::host_set("host.height", 0.0_f64);
    crate::host_set("host.workWidth", 0.0_f64);
    crate::host_set("host.workHeight", 0.0_f64);
    crate::host_set("host.scaleFactor", 1.0_f64);
    crate::host_set("host.isPrimary", false);
    crate::host_set("host.rotation", (-1.0_f64));
    crate::host_set("host.orientation", "Landscape");
    crate::host_set("host.refreshRate", (-1.0_f64));
    crate::host_set("host.colorDepth", (-1.0_f64));
    crate::host_set("host.pixelDepth", (-1.0_f64));
    crate::host_set("host.physicalWidth", (-1.0_f64));
    crate::host_set("host.physicalHeight", (-1.0_f64));
    crate::host_set("host.isHdr", false);
    crate::host_set("host.colorSpace", "srgb");
    crate::host_set("host.maxLuminance", (-1.0_f64));
    crate::host_set("host.depthPerComponent", (-1.0_f64));
    crate::host_set("host.dpi", (-1.0_f64));
    crate::host_set("host.label", "");
    crate::host_set("host.internal", false);
    crate::host_set("host.touchSupport", "unknown");
    crate::host_set("host.monochrome", false);
}

// Source: upstream/packages/screen/src/screen.ts:292 (sha256:cfdebecfb9ccf82e3ac4c451450903f8880120cc042ac8b111a455eb6679bcbb)
pub fn initialize_screen_mode(out: EntityConstruction<ScreenMode>) -> () {
    crate::host_set("host.width", 0.0_f64);
    crate::host_set("host.height", 0.0_f64);
    crate::host_set("host.refreshRate", (-1.0_f64));
    crate::host_set("host.colorDepth", (-1.0_f64));
    crate::host_set("host.pixelFormat", "");
}

// Source: upstream/packages/screen/src/screen.ts:300 (sha256:9cab12e16c61208c5152c2d2593816b8d52c2b49b26df82acbc64cc26d803bf9)
pub fn initialize_screen_permission_change(out: EntityConstruction<ScreenPermissionChange>) -> () {
    crate::host_set("host.onChange", create_signal());
}

// Source: upstream/packages/screen/src/screen.ts:304 (sha256:ac27f56043097dddf70cfa93915d35ca83622333ff6b7b3bf64be77dbbcb9ef6)
pub fn initialize_screen_signals(out: EntityConstruction<ScreenSignals>) -> () {
    crate::host_set("host.onScreenAdded", create_signal());
    crate::host_set("host.onScreenMetricsChanged", create_signal());
    crate::host_set("host.onScreenRemoved", create_signal());
}

// Source: upstream/packages/screen/src/screen.ts:310 (sha256:854495f39790ae80ec485b82f92c2aa6b7a754eb0d46acbbc06a11f6bfa17e9a)
pub fn request_screen_details(
    host_screen_details: &HostScreenDetailsCapability,
) -> crate::FlightTask<bool> {
    return {
        let __flight_callback = (host_screen_details.request).clone();
        let __flight_result = __flight_callback.lock().unwrap()();
        __flight_result
    };
}

// Source: upstream/packages/screen/src/screen.ts:314 (sha256:87624c3008b8cd4045f2ac86a04def1aeadfc1ad407895c42cfed5f85fcff9bd)
pub fn screen_to_dip_point(
    screen: &ScreenInfo,
    point: &Vector2Like,
    out: &mut SharedStructuralRecord1,
) -> SharedStructuralRecord1 {
    out.x = ((point.x / screen.scale_factor) + screen.x);
    out.y = ((point.y / screen.scale_factor) + screen.y);
    return out.clone();
}

// Source: upstream/packages/screen/src/screen.ts:324 (sha256:3fc5d45e18a0a4dbacada37c9bf77fc98305bbfc9b5b923e3b0a5f6e81f6d0e3)
pub fn screen_to_dip_rect(
    screen: &ScreenInfo,
    rect: &RectangleLike,
    out: &mut SharedStructuralRecord2,
) -> SharedStructuralRecord2 {
    out.x = ((rect.x / screen.scale_factor) + screen.x);
    out.y = ((rect.y / screen.scale_factor) + screen.y);
    out.width = (rect.width / screen.scale_factor);
    out.height = (rect.height / screen.scale_factor);
    return out.clone();
}

// Source: upstream/packages/screen/src/screen.ts:336 (sha256:d0a5afa3d3552e135010d22af87e638250ebcda71bd1c42a8eab6c98346611be)
static _PERMISSION_SUBSCRIPTIONS: std::sync::LazyLock<
    std::sync::Mutex<
        Vec<(
            ScreenPermissionChange,
            std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>,
        )>,
    >,
> = std::sync::LazyLock::new(|| std::sync::Mutex::new(Vec::new()));

// Source: upstream/packages/screen/src/screen.ts:337 (sha256:6959a9457f8b332653a8f42fc2fff5e348cc9da6152f8664507ad27c7a56a0e6)
static _SIGNAL_SUBSCRIPTIONS: std::sync::LazyLock<
    std::sync::Mutex<
        Vec<(
            ScreenSignals,
            std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>,
        )>,
    >,
> = std::sync::LazyLock::new(|| std::sync::Mutex::new(Vec::new()));

// Source: upstream/packages/screen/src/screen.ts:338 (sha256:73604c011ae140a8578ea3a4bc8d91483155cadc9e970e79ffa1bbc94d2e2164)
static _SCRATCH_POINT: std::sync::LazyLock<Vector2> = std::sync::LazyLock::new(|| Vector2 {
    __flight_identity: std::sync::Arc::new(()),
    __flight_entity_snapshot: Default::default(),
    __flight_entity_runtime: Default::default(),
    x: 0.0_f64,
    y: 0.0_f64,
});

// Source: upstream/packages/screen/src/screen.ts:339 (sha256:550a831b1fefb1d5bba13fef537600ad02a2e2b617962fd13fb91cacce885f97)
static _SCRATCH_SCREENS: std::sync::LazyLock<std::sync::Mutex<Vec<ScreenInfo>>> =
    std::sync::LazyLock::new(|| std::sync::Mutex::new(vec![]));

// Source: upstream/packages/screen/src/screen.ts:341 (sha256:1e4bb4752aefadf7035633bd986a1689d2cbcb55ec81787fe75f022262260af3)
fn copy_screen_info(src: &ScreenInfo, dst: &ScreenInfo) -> () {
    crate::host_value::<()>("host.assign");
}

// Source: upstream/packages/screen/src/screen.ts:345 (sha256:6d699c5521395435c72a3211457f3b6da43a5d89413196b137966c67a8197377)
fn fill_default_screen_info(out: &ScreenInfo) -> ScreenInfo {
    copy_screen_info(&create_screen_info(), out);
    return out.clone();
}

// Source: upstream/packages/screen/src/screen.ts:350 (sha256:db7bda286fe6a94752404a65ce2219ca01df480c4a0c941a42ca8fa131cfd633)
fn rect_center(rect: &RectangleLike) -> Vector2Like {
    return Vector2Like {
        __flight_identity: std::sync::Arc::new(()),
        __flight_entity_snapshot: Default::default(),
        __flight_entity_runtime: Default::default(),
        x: (rect.x + (rect.width / 2.0_f64)),
        y: (rect.y + (rect.height / 2.0_f64)),
    };
}
