// @generated from upstream/packages/debug/src/debug.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_log::{
    add_log_sink, clear_log_channel_levels, create_console_log_sink, get_log_level,
    remove_log_sink, set_log_channel_level, set_log_level,
};
use flighthq_render::{enable_color_adjustment_guards, enable_render_registry_guards};
use flighthq_types::{
    DebugOptions, DebugSubsystemHooks, DebugSubsystemName, LogLevel, LogSink, RenderState,
};

// Source: upstream/packages/debug/src/debug.ts:24 (sha256:db7c1086c01a5a0e0ff577301df07c656f8e538a6150f8e50b19d3127f90cd23)
pub fn disable_debug() -> () {
    if (!_ENABLED.load(std::sync::atomic::Ordering::Relaxed)) {
        return;
    }
    for hooks in (_ENABLED_SUBSYSTEMS.lock().unwrap()).iter().cloned() {
        {
            let __flight_callback = (hooks.disable_guards).clone();
            __flight_callback
                .as_ref()
                .map(|callback| callback.lock().unwrap()())
        };
    }
    _ENABLED_SUBSYSTEMS.lock().unwrap().clear();
    _remove_debug_sink();
    _restore_debug_levels();
    _ENABLED.store(false, std::sync::atomic::Ordering::Relaxed);
}

// Source: upstream/packages/debug/src/debug.ts:40 (sha256:397fe717277e7fb721cb4084d879a25046317cc91bf3c87c0682b0eeed22b965)
#[derive(Clone, Default)]
struct EnableDebugRecord9 {
    __flight_identity: std::sync::Arc<()>,
}
impl PartialEq for EnableDebugRecord9 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn enable_debug(options: Option<DebugOptions>) -> () {
    let options = options.unwrap_or(DebugOptions {
        __flight_identity: std::sync::Arc::new(()),
        subsystems: None,
        level: None,
        channels: None,
        sink: None,
    });
    if _ENABLED.load(std::sync::atomic::Ordering::Relaxed) {
        return;
    }
    let level = (options.level).unwrap_or(LogLevel::Debug);
    let subsystems = _resolve_debug_subsystems(&(options.subsystems));
    let channels = _collect_debug_channels(&subsystems, &(options.channels));
    (*_SAVED_GLOBAL_LEVEL.lock().unwrap()) = get_log_level();
    _apply_debug_levels(level, &channels);
    _install_debug_sink(((options.sink).clone()).unwrap_or(create_console_log_sink(None)));
    for hooks in (subsystems).iter().cloned() {
        {
            let __flight_callback = (hooks.enable_guards).clone();
            __flight_callback
                .as_ref()
                .map(|callback| callback.lock().unwrap()())
        };
        _ENABLED_SUBSYSTEMS
            .lock()
            .unwrap()
            .push(((hooks).clone()).clone());
    }
    _ENABLED.store(true, std::sync::atomic::Ordering::Relaxed);
}

// Source: upstream/packages/debug/src/debug.ts:56 (sha256:570508dbab5412bbf9f2898134fb8fd84d5c48f904c5d7de281b47eb7c3059c6)
pub fn enable_flight_diagnostics(state: &RenderState) -> () {
    enable_debug(None);
    enable_color_adjustment_guards(state);
    enable_render_registry_guards((state).clone());
}

// Source: upstream/packages/debug/src/debug.ts:63 (sha256:713183a8744dba3aa296c5c64a257d953e21ebfb93b9d8c5c1bde4baba1e9abb)
pub fn is_debug_enabled() -> bool {
    return _ENABLED.load(std::sync::atomic::Ordering::Relaxed);
}

// Source: upstream/packages/debug/src/debug.ts:72 (sha256:2af5c01acd2a4e220b3cc75d0ca125c005960aed9914a7746beff9473fc0d891)
pub fn register_debug_subsystem(name: DebugSubsystemName, hooks: &DebugSubsystemHooks) -> () {
    {
        let __flight_key = name;
        let __flight_value = (*hooks).clone();
        if let Some((_, value)) = (*_SUBSYSTEMS.lock().unwrap())
            .iter_mut()
            .find(|(key, _)| key == &__flight_key)
        {
            *value = __flight_value;
        } else {
            (*_SUBSYSTEMS.lock().unwrap()).push((__flight_key, __flight_value));
        }
    };
}

// Source: upstream/packages/debug/src/debug.ts:78 (sha256:01e44f7d2a82508d767c4c73efc36cc1697c702ae04ed80938473708bc94c38e)
pub fn unregister_debug_subsystem(name: DebugSubsystemName) -> bool {
    return {
        let __flight_key = name;
        if let Some(__flight_index) = (*_SUBSYSTEMS.lock().unwrap())
            .iter()
            .position(|(key, _)| key == &__flight_key)
        {
            (*_SUBSYSTEMS.lock().unwrap()).remove(__flight_index);
            true
        } else {
            false
        }
    };
}

// Source: upstream/packages/debug/src/debug.ts:82 (sha256:b446dff17a95f51c267401193476bb9eeab8df5a64c5e8384c43a9cb95072b78)
static _SUBSYSTEMS: std::sync::LazyLock<std::sync::Mutex<Vec<(String, DebugSubsystemHooks)>>> =
    std::sync::LazyLock::new(|| std::sync::Mutex::new(Vec::new()));

// Source: upstream/packages/debug/src/debug.ts:83 (sha256:143147ca42bc9a380d2bf56afb975db570d964828bae059a36dd4a2c83e75170)
static _ENABLED_SUBSYSTEMS: std::sync::LazyLock<std::sync::Mutex<Vec<DebugSubsystemHooks>>> =
    std::sync::LazyLock::new(|| std::sync::Mutex::new(vec![]));

// Source: upstream/packages/debug/src/debug.ts:85 (sha256:7229172ec48331971d8459b2c68e79d664373be87a769eb22f5997a746fc4f85)
static _ENABLED: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

// Source: upstream/packages/debug/src/debug.ts:86 (sha256:0314a5472218cc63a9e6c620c4f9fa1523454facfda37ad4828bbeb256cf2ea2)
static _INSTALLED_SINK: std::sync::LazyLock<std::sync::Mutex<Option<LogSink>>> =
    std::sync::LazyLock::new(|| std::sync::Mutex::new(None));

// Source: upstream/packages/debug/src/debug.ts:87 (sha256:a6b2878d479cb809f07923a4889a4fd7676257671f73fb1543691af57f9730c2)
static _SAVED_GLOBAL_LEVEL: std::sync::LazyLock<std::sync::Mutex<LogLevel>> =
    std::sync::LazyLock::new(|| std::sync::Mutex::new(LogLevel::Verbose));

// Source: upstream/packages/debug/src/debug.ts:90 (sha256:224426a6057cd2d451a818ec407bd76d5c12ac3f955be77270752996087c47fe)
fn _apply_debug_levels(level: LogLevel, channels: &Vec<String>) -> () {
    set_log_level(level);
    for channel in (channels).iter().cloned() {
        set_log_channel_level((channel).clone(), level);
    }
}

// Source: upstream/packages/debug/src/debug.ts:96 (sha256:743e28d1ab40a93d5a10647e50b50c3d60608d1f427f4de036a7e3bdbed9aec4)
fn _collect_debug_channels(
    subsystems: &Vec<DebugSubsystemHooks>,
    extra: &Option<Vec<String>>,
) -> Vec<String> {
    let mut channels: Vec<String> = vec![];
    for hooks in (subsystems).iter().cloned() {
        if ((hooks.channels).clone()).is_some() {
            {
                channels.extend(((hooks.channels).clone()).iter().cloned());
                channels.len() as f64
            };
        }
    }
    if (extra).is_some() {
        {
            channels.extend((extra.as_ref().unwrap()).iter().cloned());
            channels.len() as f64
        };
    }
    return channels;
}

// Source: upstream/packages/debug/src/debug.ts:109 (sha256:fe488eaa18eae0bb8fc093e7fab7a012fdf155f0de853ceb2480cc6769126be7)
fn _install_debug_sink(sink: LogSink) -> () {
    (*_INSTALLED_SINK.lock().unwrap()) = Some((sink).clone());
    add_log_sink((sink).clone());
}

// Source: upstream/packages/debug/src/debug.ts:115 (sha256:12765af0ae8bab60c885ab5edaa4668748aa1ca8382ae5455f84d5ed0a339668)
fn _remove_debug_sink() -> () {
    if ((*_INSTALLED_SINK.lock().unwrap()).clone()).is_none() {
        return;
    }
    remove_log_sink(((*_INSTALLED_SINK.lock().unwrap()).as_mut().unwrap()).clone());
    (*_INSTALLED_SINK.lock().unwrap()) = None;
}

// Source: upstream/packages/debug/src/debug.ts:123 (sha256:8f048e0281e53fbbea1c86cd627ffdf8c75f7351ceaf8ee25d781b64e555a65e)
fn _resolve_debug_subsystems(names: &Option<Vec<DebugSubsystemName>>) -> Vec<DebugSubsystemHooks> {
    if (names).is_none() {
        return {
            let mut __flight_array = Vec::new();
            __flight_array.extend(
                ((*_SUBSYSTEMS.lock().unwrap())
                    .iter()
                    .map(|(_, value)| value.clone())
                    .collect::<Vec<_>>())
                .iter()
                .cloned(),
            );
            __flight_array
        };
    }
    let mut resolved: Vec<DebugSubsystemHooks> = vec![];
    for name in (names.as_ref().unwrap()).iter().cloned() {
        let hooks = (*_SUBSYSTEMS.lock().unwrap())
            .iter()
            .find(|(entry_key, _)| entry_key == &(name).clone())
            .map(|(_, value)| value.clone());
        if ((hooks).clone()).is_some() {
            resolved.push(((hooks.as_ref().unwrap()).clone()).clone());
        }
    }
    return resolved;
}

// Source: upstream/packages/debug/src/debug.ts:136 (sha256:4f1d34bf8ef921904c56b309fc405dd874017df2c56f61ad4da7e32c7bcac860)
fn _restore_debug_levels() -> () {
    set_log_level((*_SAVED_GLOBAL_LEVEL.lock().unwrap()).clone());
    clear_log_channel_levels();
}
