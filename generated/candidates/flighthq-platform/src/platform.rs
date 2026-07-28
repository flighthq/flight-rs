// @generated from upstream/packages/platform/src/platform.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::{
    PlatformBackend, PlatformEngine, PlatformInfo, PlatformKind, PlatformName, PlatformRuntime,
};
use flighthq_useragent::{
    detect_endianness, parse_user_agent_arch, parse_user_agent_engine,
    parse_user_agent_engine_version, parse_user_agent_kind, parse_user_agent_name,
    parse_user_agent_pointer_width, parse_user_agent_runtime, parse_user_agent_version,
};

// Source: upstream/packages/platform/src/platform.ts:24 (sha256:0dddb614ea146f040bd05043a69f12181be46593801b37a11b28a8dc63b6c9d7)
pub fn compare_platform_versions(a: String, b: String) -> f64 {
    if (a == b) {
        return 0.0_f64;
    }
    let a_parts = if (a == "") {
        vec![]
    } else {
        (a).split(".".to_owned().as_str())
            .map(|part| part.to_owned())
            .collect::<Vec<_>>()
    };
    let b_parts = if (b == "") {
        vec![]
    } else {
        (b).split(".".to_owned().as_str())
            .map(|part| part.to_owned())
            .collect::<Vec<_>>()
    };
    let len = (a_parts.len() as f64).max((b_parts.len() as f64));
    {
        let mut i = 0.0_f64;
        while (i < len) {
            let a_num = if (i < (a_parts.len() as f64)) {
                {
                    let __flight_value = a_parts[i as usize].clone();
                    let __flight_radix = (10.0_f64) as u32;
                    i64::from_str_radix(__flight_value.trim(), __flight_radix)
                        .map_or(f64::NAN, |value| value as f64)
                }
            } else {
                0.0_f64
            };
            let b_num = if (i < (b_parts.len() as f64)) {
                {
                    let __flight_value = b_parts[i as usize].clone();
                    let __flight_radix = (10.0_f64) as u32;
                    i64::from_str_radix(__flight_value.trim(), __flight_radix)
                        .map_or(f64::NAN, |value| value as f64)
                }
            } else {
                0.0_f64
            };
            let a_n = if (a_num).is_nan() { 0.0_f64 } else { a_num };
            let b_n = if (b_num).is_nan() { 0.0_f64 } else { b_num };
            if (a_n < b_n) {
                return (-1.0_f64);
            }
            if (a_n > b_n) {
                return 1.0_f64;
            }
            {
                i += 1.0;
                i
            };
        }
    }
    return 0.0_f64;
}

// Source: upstream/packages/platform/src/platform.ts:41 (sha256:421be3157885a78a82f887e34e754ea0d04303d4ad8d9dc5e1747742d5a1be9c)
#[derive(Clone, Default)]
struct CreatePlatformInfoRecord1 {
    __flight_identity: std::sync::Arc<()>,
    arch: String,
    distro: String,
    distro_version: String,
    endianness: String,
    engine: String,
    engine_version: String,
    is_touch: bool,
    kind: String,
    locale: String,
    name: String,
    os_build: String,
    pointer_width: f64,
    runtime: String,
    version: String,
}
impl PartialEq for CreatePlatformInfoRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn create_platform_info() -> PlatformInfo {
    return PlatformInfo {
        __flight_identity: std::sync::Arc::new(()),
        arch: "".to_owned(),
        distro: "".to_owned(),
        distro_version: "".to_owned(),
        endianness: "unknown".to_owned(),
        engine: "unknown".to_owned(),
        engine_version: "".to_owned(),
        is_touch: false,
        kind: "unknown".to_owned(),
        locale: "".to_owned(),
        name: "unknown".to_owned(),
        os_build: "".to_owned(),
        pointer_width: (-1.0_f64),
        runtime: "unknown".to_owned(),
        version: "".to_owned(),
    };
}

// Source: upstream/packages/platform/src/platform.ts:62 (sha256:a14c252def9909e7552e944832f95292c431ed393f6540daabb316cdece51215)
pub fn create_web_platform_backend() -> PlatformBackend {
    return PlatformBackend {
        __flight_identity: std::sync::Arc::new(()),
        get_info: std::sync::Arc::new(std::sync::Mutex::new(Box::new(
            move |mut __flight_argument_0: PlatformInfo| -> PlatformInfo {
                get_web_platform_info(&mut __flight_argument_0)
            },
        )
            as Box<dyn FnMut(PlatformInfo) -> PlatformInfo + Send + 'static>)),
    };
}

// Source: upstream/packages/platform/src/platform.ts:68 (sha256:dc6201a2c26831e924e117cf19298ae584bc8e326cae8eb6b4eb4b6f4d4137b6)
pub fn get_platform_backend() -> PlatformBackend {
    if ((*_BACKEND.lock().unwrap()).clone()).is_none() {
        (*_BACKEND.lock().unwrap()) = Some(create_web_platform_backend());
    }
    return (((*_BACKEND.lock().unwrap()).clone()).clone().unwrap()).clone();
}

// Source: upstream/packages/platform/src/platform.ts:75 (sha256:8794dae5f515151c2b70b0984b07baec9a6ae6ff89231e5ffd41b1391f4c73b6)
pub fn get_platform_engine() -> PlatformEngine {
    return (get_platform_info(&_SCRATCH).engine).clone();
}

// Source: upstream/packages/platform/src/platform.ts:80 (sha256:5df92e2236bab9cbb8cc38c95ccfb0f671a43b664dbb2c6aa039805f4da158a0)
pub fn get_platform_info(out: &PlatformInfo) -> PlatformInfo {
    return {
        let __flight_callback = (get_platform_backend().get_info).clone();
        let __flight_result = __flight_callback.lock().unwrap()((*out).clone());
        __flight_result
    };
}

// Source: upstream/packages/platform/src/platform.ts:85 (sha256:d204d54270a9ddb274e34c239fb21d0b2a349ba970c52409ce7c040ca2c4ad4c)
pub fn get_platform_kind() -> PlatformKind {
    return (get_platform_info(&_SCRATCH).kind).clone();
}

// Source: upstream/packages/platform/src/platform.ts:90 (sha256:91786d10a34232a5feb6b24afe82751f4846756b5291a7217c89d572e338c457)
pub fn get_platform_name() -> PlatformName {
    return (get_platform_info(&_SCRATCH).name).clone();
}

// Source: upstream/packages/platform/src/platform.ts:96 (sha256:3c72cf2f89b957dec0a35c4c3bf728694eb9f1591c4b470c28dcd1d6fafd1046)
pub fn get_platform_runtime() -> PlatformRuntime {
    return (get_platform_info(&_SCRATCH).runtime).clone();
}

// Source: upstream/packages/platform/src/platform.ts:101 (sha256:7765c987c4154709eb9f7e335706f1516306d6d4e43bde10ed2696f8b7f2fcdb)
pub fn is_platform_desktop() -> bool {
    return (get_platform_kind() == "desktop");
}

// Source: upstream/packages/platform/src/platform.ts:106 (sha256:681dd799266844e7a716dc83caefd98c71c286938962a67ec246bb4d7a92b9fe)
pub fn is_platform_mobile() -> bool {
    return (get_platform_kind() == "mobile");
}

// Source: upstream/packages/platform/src/platform.ts:112 (sha256:5ff86c4ff8e07551c06c987e3bbf4480185ccf40cf036eb91f994c0d1c086941)
pub fn is_platform_native() -> bool {
    let runtime = get_platform_runtime();
    return (runtime != "web") && (runtime != "unknown");
}

// Source: upstream/packages/platform/src/platform.ts:118 (sha256:28a64f09d4f5e946b6c8c6ce468d58bb93ed8382ae6a32a4c774c6eb73d9dd5d)
pub fn is_platform_touch() -> bool {
    return get_platform_info(&_SCRATCH).is_touch;
}

// Source: upstream/packages/platform/src/platform.ts:125 (sha256:713bf3b2a3106972c39735890e684e98da00860f75dcd35637e7d7802bbdeb5a)
pub fn is_platform_version_at_least(minimum: String) -> bool {
    let version = (get_platform_info(&_SCRATCH).version).clone();
    if (version == "") {
        return false;
    }
    return (compare_platform_versions((version).clone(), (minimum).clone()) >= 0.0_f64);
}

// Source: upstream/packages/platform/src/platform.ts:132 (sha256:7460039359904e3025a0452b27d0db8becebcd35f60c6b048f109cfbfcc4cf8b)
pub fn is_platform_web() -> bool {
    return (get_platform_kind() == "web");
}

// Source: upstream/packages/platform/src/platform.ts:138 (sha256:ba3889e62e09f8b5a4186a7360dc4b179e1b66a315b53ffe7895d89bc04aea3b)
pub fn set_platform_backend(backend: Option<PlatformBackend>) -> () {
    (*_BACKEND.lock().unwrap()) = (backend).clone();
}

// Source: upstream/packages/platform/src/platform.ts:142 (sha256:0f0f7822ea6cb0c631bc817e0c215306a940ec400a10a15a67356fc70aa857eb)
static _BACKEND: std::sync::LazyLock<std::sync::Mutex<Option<PlatformBackend>>> =
    std::sync::LazyLock::new(|| std::sync::Mutex::new(None));

// Source: upstream/packages/platform/src/platform.ts:145 (sha256:f3714260795db64c73eea4d1ccaacb716390e45087f3c0e0cfb6a5749ef1a5dd)
static _SCRATCH: std::sync::LazyLock<PlatformInfo> =
    std::sync::LazyLock::new(|| create_platform_info());

// Source: upstream/packages/platform/src/platform.ts:147 (sha256:b1ba0076fe22d520a58f593a7a68b357f4dd667dfaf57a6a7aca76930301e048)
fn get_web_platform_info(out: &mut PlatformInfo) -> PlatformInfo {
    let nav: Option<crate::OpaqueHostValue> = None;
    let ua = (None::<String>).unwrap_or("".to_owned());
    out.name = parse_user_agent_name((ua).clone());
    out.kind = parse_user_agent_kind((out.name).clone());
    out.version = parse_user_agent_version((ua).clone(), (out.name).clone());
    out.arch = parse_user_agent_arch((ua).clone(), None);
    out.locale = (None::<String>).unwrap_or("".to_owned());
    out.is_touch = false;
    out.runtime = parse_user_agent_runtime((None).clone());
    out.engine = parse_user_agent_engine((ua).clone());
    out.engine_version = parse_user_agent_engine_version((ua).clone(), (out.engine).clone());
    out.endianness = detect_endianness();
    out.pointer_width = parse_user_agent_pointer_width((out.arch).clone());
    out.os_build = "".to_owned();
    out.distro = "".to_owned();
    out.distro_version = "".to_owned();
    return out.clone();
}
