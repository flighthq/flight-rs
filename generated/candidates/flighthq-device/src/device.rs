// @generated from upstream/packages/device/src/device.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::{
    DEVICE_FORM_FACTOR_UNKNOWN as device_form_factor_unknown_constant, DeviceBackend,
    DeviceCapabilities, DeviceDisplayMetrics, DeviceInfo, SafeAreaInsets,
};
use flighthq_useragent::{
    parse_user_agent_arch, parse_user_agent_form_factor, parse_user_agent_os_name,
    parse_user_agent_os_version,
};

#[derive(Clone, Default)]
pub struct SharedStructuralRecord1 {
    pub __flight_identity: std::sync::Arc<()>,
    pub vendor: String,
    pub renderer: String,
}
impl PartialEq for SharedStructuralRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/device/src/device.ts:18 (sha256:f60608508463454406d94785e828a2e60c0af02b3708ebc17ef02b171ea4c86b)
#[derive(Clone, Default)]
struct CreateDeviceCapabilitiesRecord2 {
    __flight_identity: std::sync::Arc<()>,
    has_keyboard: bool,
    has_mouse: bool,
    has_stylus: bool,
}
impl PartialEq for CreateDeviceCapabilitiesRecord2 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn create_device_capabilities() -> DeviceCapabilities {
    return DeviceCapabilities {
        __flight_identity: std::sync::Arc::new(()),
        has_keyboard: false,
        has_mouse: false,
        has_stylus: false,
    };
}

// Source: upstream/packages/device/src/device.ts:28 (sha256:f64ef1e53152639f53a1ace22e5c0dec82dbe0b60e6b0ae105dd32649f397d2d)
#[derive(Clone, Default)]
struct CreateDeviceDisplayMetricsRecord2 {
    __flight_identity: std::sync::Arc<()>,
    color_depth: f64,
    density_dpi: f64,
    logical_height: f64,
    logical_width: f64,
    physical_height: f64,
    physical_width: f64,
    pixel_ratio: f64,
}
impl PartialEq for CreateDeviceDisplayMetricsRecord2 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn create_device_display_metrics() -> DeviceDisplayMetrics {
    return DeviceDisplayMetrics {
        __flight_identity: std::sync::Arc::new(()),
        color_depth: (-1.0_f64),
        density_dpi: (-1.0_f64),
        logical_height: (-1.0_f64),
        logical_width: (-1.0_f64),
        physical_height: (-1.0_f64),
        physical_width: (-1.0_f64),
        pixel_ratio: (-1.0_f64),
    };
}

// Source: upstream/packages/device/src/device.ts:42 (sha256:9b5cc2e27c29089c7b312f6580ff871764fe04d7992eda319f26498e4b5fb857)
pub fn create_device_info() -> DeviceInfo {
    return DeviceInfo {
        __flight_identity: std::sync::Arc::new(()),
        arch: "".to_owned(),
        available_memory: (-1.0_f64),
        board_name: "".to_owned(),
        color_gamut: "".to_owned(),
        cpu_cores: (-1.0_f64),
        font_scale: (-1.0_f64),
        form_factor: (device_form_factor_unknown_constant).to_owned(),
        gpu_renderer: "".to_owned(),
        gpu_vendor: "".to_owned(),
        is_hdr: false,
        is_jailbroken: false,
        is_low_end_device: false,
        is_rooted: false,
        is_virtual: false,
        manufacturer: "".to_owned(),
        marketing_name: "".to_owned(),
        model: "".to_owned(),
        os_build: "".to_owned(),
        os_name: "".to_owned(),
        os_version: "".to_owned(),
        platform_string: "".to_owned(),
        product_name: "".to_owned(),
        supported_abis: vec![],
        total_memory: (-1.0_f64),
        web_view_version: "".to_owned(),
    };
}

// Source: upstream/packages/device/src/device.ts:73 (sha256:541814e647a87911c1423d461f625f17b3c54e025750ea647528e28a4423d57c)
#[derive(Clone, Default)]
struct CreateSafeAreaInsetsRecord2 {
    __flight_identity: std::sync::Arc<()>,
    bottom: f64,
    left: f64,
    right: f64,
    top: f64,
}
impl PartialEq for CreateSafeAreaInsetsRecord2 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn create_safe_area_insets() -> SafeAreaInsets {
    return SafeAreaInsets {
        __flight_identity: std::sync::Arc::new(()),
        bottom: 0.0_f64,
        left: 0.0_f64,
        right: 0.0_f64,
        top: 0.0_f64,
    };
}

// Source: upstream/packages/device/src/device.ts:79 (sha256:0c79deaa5d9400647deba6a3ce29caa5a78517be1f817060d7ed5218566beb5f)
#[derive(Clone, Default)]
struct CreateWebDeviceBackendRecord2 {
    __flight_identity: std::sync::Arc<()>,
    user_agent_data: Option<CreateWebDeviceBackendRecord3>,
}
impl PartialEq for CreateWebDeviceBackendRecord2 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
struct CreateWebDeviceBackendRecord3 {
    __flight_identity: std::sync::Arc<()>,
    platform: Option<String>,
}
impl PartialEq for CreateWebDeviceBackendRecord3 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
struct CreateWebDeviceBackendRecord4 {
    __flight_identity: std::sync::Arc<()>,
    device_memory: Option<f64>,
}
impl PartialEq for CreateWebDeviceBackendRecord4 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn create_web_device_backend() -> DeviceBackend {
    return DeviceBackend {
        __flight_identity: std::sync::Arc::new(()),
        get_capabilities: std::sync::Arc::new(std::sync::Mutex::new(Box::new(
            move |mut out: DeviceCapabilities| -> DeviceCapabilities {
                let nav: Option<crate::OpaqueHostValue> = None;
                let max_touch = (-1.0_f64);
                out.has_mouse = (max_touch == 0.0_f64);
                let ua = (None::<String>).unwrap_or("".to_owned());
                out.has_keyboard = detect_desktop_ua((ua).clone());
                out.has_stylus = false;
                return out;
            },
        )
            as Box<dyn FnMut(DeviceCapabilities) -> DeviceCapabilities + Send + 'static>)),
        get_display_metrics: std::sync::Arc::new(std::sync::Mutex::new(Box::new(
            move |mut out: DeviceDisplayMetrics| -> DeviceDisplayMetrics {
                let win: Option<crate::OpaqueHostValue> = None;
                let scr: Option<crate::OpaqueHostValue> = None;
                out.color_depth = (-1.0_f64);
                out.density_dpi = (-1.0_f64);
                out.logical_height = (-1.0_f64);
                out.logical_width = (-1.0_f64);
                let pixel_ratio = (-1.0_f64);
                out.pixel_ratio = (pixel_ratio).clone();
                out.physical_width = (-1.0_f64);
                out.physical_height = (-1.0_f64);
                return out;
            },
        )
            as Box<dyn FnMut(DeviceDisplayMetrics) -> DeviceDisplayMetrics + Send + 'static>)),
        get_id: std::sync::Arc::new(std::sync::Mutex::new(Box::new(move || -> String {
            let __flight_try_return: Option<String> = match std::panic::catch_unwind(
                std::panic::AssertUnwindSafe(|| -> Option<String> {
                    {
                        let key = "__flighthq_device_id";
                        let existing: Option<crate::OpaqueHostValue> = None;
                        return Some("".to_owned());
                    }
                    None
                }),
            ) {
                Ok(value) => value,
                Err(_) => (|| -> Option<String> {
                    {
                        return Some("".to_owned());
                    }
                    None
                })(),
            };
            return __flight_try_return.expect("TypeScript try/catch completed without returning");
        })
            as Box<dyn FnMut() -> String + Send + 'static>)),
        get_info: std::sync::Arc::new(std::sync::Mutex::new(Box::new(
            move |mut out: DeviceInfo| -> DeviceInfo {
                let nav: Option<crate::OpaqueHostValue> = None;
                let ua = (None::<String>).unwrap_or("".to_owned());
                let uad_platform: Option<String> = None::<String>;
                out.arch = parse_user_agent_arch(
                    (ua).clone(),
                    Some(((uad_platform).clone().unwrap()).clone()),
                );
                out.available_memory = (-1.0_f64);
                out.board_name = "".to_owned();
                out.color_gamut = "".to_owned();
                let cores = (-1.0_f64);
                out.cpu_cores = (cores).clone();
                out.font_scale = (-1.0_f64);
                out.form_factor = parse_user_agent_form_factor((ua).clone(), (-1.0_f64));
                let gpu_info = read_web_gpu_info();
                out.gpu_renderer = (gpu_info.renderer).clone();
                out.gpu_vendor = (gpu_info.vendor).clone();
                out.is_hdr = false;
                out.is_jailbroken = false;
                let dev_mem = (-1.0_f64);
                out.is_low_end_device = detect_low_end_device(dev_mem, (cores).clone());
                out.is_rooted = false;
                out.is_virtual = false;
                out.manufacturer = "".to_owned();
                out.marketing_name = "".to_owned();
                out.model = "".to_owned();
                out.os_build = "".to_owned();
                out.os_name = parse_user_agent_os_name((ua).clone());
                out.os_version = parse_user_agent_os_version((ua).clone());
                out.platform_string = ua;
                out.product_name = "".to_owned();
                out.supported_abis = vec![];
                out.total_memory = if (dev_mem >= 0.0_f64) {
                    (((dev_mem * 1024.0_f64) * 1024.0_f64) * 1024.0_f64)
                } else {
                    (-1.0_f64)
                };
                out.web_view_version = "".to_owned();
                return out;
            },
        )
            as Box<dyn FnMut(DeviceInfo) -> DeviceInfo + Send + 'static>)),
        get_safe_area_insets: std::sync::Arc::new(std::sync::Mutex::new(Box::new(
            move |mut out: SafeAreaInsets| -> SafeAreaInsets {
                let insets = (*_SAFE_AREA_INSETS.lock().unwrap()).clone();
                if (insets).is_some() {
                    out.bottom = insets.as_ref().unwrap().bottom;
                    out.left = insets.as_ref().unwrap().left;
                    out.right = insets.as_ref().unwrap().right;
                    out.top = insets.as_ref().unwrap().top;
                } else {
                    out.bottom = 0.0_f64;
                    out.left = 0.0_f64;
                    out.right = 0.0_f64;
                    out.top = 0.0_f64;
                }
                return out;
            },
        )
            as Box<dyn FnMut(SafeAreaInsets) -> SafeAreaInsets + Send + 'static>)),
    };
}

// Source: upstream/packages/device/src/device.ts:194 (sha256:909a209c65755bf8409e8077ebd8edf662d2e365fa93a98762926d83823aa79c)
pub fn enable_web_safe_area_insets()
-> std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>> {
    return std::sync::Arc::new(std::sync::Mutex::new(
        Box::new(move || -> () {}) as Box<dyn FnMut() -> () + Send + 'static>
    ));
}

// Source: upstream/packages/device/src/device.ts:228 (sha256:9ae85737f910b941a66ccfce4a85eeb72c454a3d730a27586b1ff640c724198a)
pub fn get_device_backend() -> DeviceBackend {
    if ((*_BACKEND.lock().unwrap()).clone()).is_none() {
        (*_BACKEND.lock().unwrap()) = Some(create_web_device_backend());
    }
    return (((*_BACKEND.lock().unwrap()).clone()).clone().unwrap()).clone();
}

// Source: upstream/packages/device/src/device.ts:235 (sha256:3fd09980c2efbca37a5d19d41a6d3cbb2157dbf5af66c6e092ab955b3256d805)
pub fn get_device_capabilities(out: &DeviceCapabilities) -> DeviceCapabilities {
    return {
        let __flight_callback = (get_device_backend().get_capabilities).clone();
        let __flight_result = __flight_callback.lock().unwrap()((*out).clone());
        __flight_result
    };
}

// Source: upstream/packages/device/src/device.ts:241 (sha256:9a131fca30fa4ac0294cbf15fd239ba8978bf9ca27612e8e8dd3ac90922028ea)
pub fn get_device_display_metrics(out: &DeviceDisplayMetrics) -> DeviceDisplayMetrics {
    return {
        let __flight_callback = (get_device_backend().get_display_metrics).clone();
        let __flight_result = __flight_callback.lock().unwrap()((*out).clone());
        __flight_result
    };
}

// Source: upstream/packages/device/src/device.ts:249 (sha256:eefb40d4123b712488e2251166de2b6704948d203b3f1feaffbd40cb6e366fc3)
pub fn get_device_id() -> String {
    return {
        let __flight_callback = (get_device_backend().get_id).clone();
        let __flight_result = __flight_callback.lock().unwrap()();
        __flight_result
    };
}

// Source: upstream/packages/device/src/device.ts:254 (sha256:08e4d71e290d60ada19c9ca88bb70baabf3c0478f4a1d9b6af9d6278a8dabbdd)
pub fn get_device_info(out: &DeviceInfo) -> DeviceInfo {
    return {
        let __flight_callback = (get_device_backend().get_info).clone();
        let __flight_result = __flight_callback.lock().unwrap()((*out).clone());
        __flight_result
    };
}

// Source: upstream/packages/device/src/device.ts:260 (sha256:09a82151bf7086a0d36544cb36e483a4792020302df95eb2ee977dae540c5367)
pub fn get_safe_area_insets(out: &SafeAreaInsets) -> SafeAreaInsets {
    return {
        let __flight_callback = (get_device_backend().get_safe_area_insets).clone();
        let __flight_result = __flight_callback.lock().unwrap()((*out).clone());
        __flight_result
    };
}

// Source: upstream/packages/device/src/device.ts:269 (sha256:af90cea0fbbcf91290228096f7270eb7176e3c6fbce0301f8586114d0f4e3e97)
#[derive(Clone, Default)]
struct RefreshDeviceInfoRecord2 {
    __flight_identity: std::sync::Arc<()>,
    refresh: Option<std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>>,
}
impl PartialEq for RefreshDeviceInfoRecord2 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn refresh_device_info() -> () {
    let backend = get_device_backend();
    let maybe_refreshable = {
        let __flight_source = &((backend).clone());
        RefreshDeviceInfoRecord2 {
            __flight_identity: std::sync::Arc::clone(&__flight_source.__flight_identity),
            refresh: None,
        }
    };
    if (((maybe_refreshable.refresh).clone())
        .as_ref()
        .map_or("undefined", |_| "function")
        == "function")
    {
        {
            let __flight_callback = maybe_refreshable.refresh.as_ref().unwrap().clone();
            let __flight_result = __flight_callback.lock().unwrap()();
            __flight_result
        };
    }
}

// Source: upstream/packages/device/src/device.ts:278 (sha256:d69b965d0766fbeb7652e32917d2419ea7199bbad7a49a8fd9684f328ee862c5)
pub fn set_device_backend(backend: Option<DeviceBackend>) -> () {
    (*_BACKEND.lock().unwrap()) = (backend).clone();
}

// Source: upstream/packages/device/src/device.ts:282 (sha256:c960578c7b63a4b738c0a38a41cf0fcd3072a5b069f7277c9e9ef2dca2c2744a)
static _BACKEND: std::sync::LazyLock<std::sync::Mutex<Option<DeviceBackend>>> =
    std::sync::LazyLock::new(|| std::sync::Mutex::new(None));

// Source: upstream/packages/device/src/device.ts:283 (sha256:76ad4f74dd874cb05e004e07e4986898dc2136fcc87a1086de5a8c737e392c3d)
static _SAFE_AREA_INSETS: std::sync::LazyLock<std::sync::Mutex<Option<SafeAreaInsets>>> =
    std::sync::LazyLock::new(|| std::sync::Mutex::new(None));

// Source: upstream/packages/device/src/device.ts:289 (sha256:a6b63b09e0e7ee1e436aaff5f1d53d51a22541e86c8d3e02f265116df220063e)
fn detect_desktop_ua(ua: String) -> bool {
    return (regex::RegexBuilder::new(
        "win(?:dows)?nt|macintosh|mac os x|linux(?!.*android)|cros|x11",
    )
    .case_insensitive(true)
    .multi_line(false)
    .dot_matches_new_line(false)
    .build()
    .expect("upstream TypeScript regular expression must be valid Rust regex syntax"))
    .is_match(&(ua));
}

// Source: upstream/packages/device/src/device.ts:293 (sha256:048d9dd469962bed053b458b6924d34183c0e6efa706b9f3b1f2f524e1cdf175)
fn detect_low_end_device(device_memory_gib: f64, cores: f64) -> bool {
    if (device_memory_gib > 0.0_f64) && (device_memory_gib <= 1.0_f64) {
        return true;
    }
    if (cores > 0.0_f64) && (cores <= 2.0_f64) {
        return true;
    }
    return false;
}

// Source: upstream/packages/device/src/device.ts:300 (sha256:c27bed01e4ce73f12f917d8702cd6841a0f62a2737448446ed1a03f625ca525d)
#[derive(Clone, Default)]
struct ReadWebGpuInfoRecord2 {
    __flight_identity: std::sync::Arc<()>,
    renderer: String,
    vendor: String,
}
impl PartialEq for ReadWebGpuInfoRecord2 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

fn read_web_gpu_info() -> SharedStructuralRecord1 {
    let __flight_try_return: Option<SharedStructuralRecord1> = match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| -> Option<SharedStructuralRecord1> {
            {
                return Some(SharedStructuralRecord1 {
                    __flight_identity: std::sync::Arc::new(()),
                    renderer: "".to_owned(),
                    vendor: "".to_owned(),
                });
            }
            None
        }),
    ) {
        Ok(value) => value,
        Err(_) => (|| -> Option<SharedStructuralRecord1> {
            {
                return Some(SharedStructuralRecord1 {
                    __flight_identity: std::sync::Arc::new(()),
                    renderer: "".to_owned(),
                    vendor: "".to_owned(),
                });
            }
            None
        })(),
    };
    return __flight_try_return.expect("TypeScript try/catch completed without returning");
}
