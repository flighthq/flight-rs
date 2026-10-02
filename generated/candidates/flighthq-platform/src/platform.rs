// @generated from upstream/packages/platform/src/platform.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_entity::{allocate_entity, finish_entity};
use flighthq_types::{
    EntityConstruction, HostPlatformCapability, PlatformEngine, PlatformInfo, PlatformKind,
    PlatformName, PlatformRuntime,
};

// Source: upstream/packages/platform/src/platform.ts:15 (sha256:0dddb614ea146f040bd05043a69f12181be46593801b37a11b28a8dc63b6c9d7)
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

// Source: upstream/packages/platform/src/platform.ts:31 (sha256:3725667475fd8e9626f31c98f95fa89843fef49edb6c0f7941065a60f7079c59)
pub fn create_platform_info() -> PlatformInfo {
    let mut out = allocate_entity();
    initialize_platform_info((out).clone());
    return finish_entity((out).clone());
}

// Source: upstream/packages/platform/src/platform.ts:39 (sha256:9f97a92ec9eb1876b85b9c4ed80480acdcf799afa36698726f433a188df87b24)
pub fn get_platform_engine(host_platform: &HostPlatformCapability) -> PlatformEngine {
    return (get_platform_info(host_platform, &_SCRATCH).engine).clone();
}

// Source: upstream/packages/platform/src/platform.ts:44 (sha256:a0ed85fac7b2a1b60718669d6eae15eb3e808b87f5adfae10f5f4dbe7161e98d)
pub fn get_platform_info(
    host_platform: &HostPlatformCapability,
    out: &PlatformInfo,
) -> PlatformInfo {
    return {
        let __flight_callback = (host_platform.get_info).clone();
        let __flight_result = __flight_callback.lock().unwrap()((*out).clone());
        __flight_result
    };
}

// Source: upstream/packages/platform/src/platform.ts:49 (sha256:028ac1280887bf25c71a1a490f1bd2b422604a9d244b0381c39e277352798ab3)
pub fn get_platform_kind(host_platform: &HostPlatformCapability) -> PlatformKind {
    return (get_platform_info(host_platform, &_SCRATCH).kind).clone();
}

// Source: upstream/packages/platform/src/platform.ts:54 (sha256:116278c3063ec9625f61173a686c6fbde9f98527b8e0fe8cc47a978a265f713a)
pub fn get_platform_name(host_platform: &HostPlatformCapability) -> PlatformName {
    return (get_platform_info(host_platform, &_SCRATCH).name).clone();
}

// Source: upstream/packages/platform/src/platform.ts:60 (sha256:536099fb9b8bfd25952c560edd230e6e9a5266864d33f72c0bf3939ff96284ea)
pub fn get_platform_runtime(host_platform: &HostPlatformCapability) -> PlatformRuntime {
    return (get_platform_info(host_platform, &_SCRATCH).runtime).clone();
}

// Source: upstream/packages/platform/src/platform.ts:65 (sha256:11bd354a85d63c4c6599ee583573a7a1d5ddcc8ba14ef7353e92b9bf3dd710e3)
pub fn initialize_platform_info(out: EntityConstruction<PlatformInfo>) -> () {
    crate::host_set("host.arch", "");
    crate::host_set("host.distro", "");
    crate::host_set("host.distroVersion", "");
    crate::host_set("host.endianness", "unknown");
    crate::host_set("host.engine", "unknown");
    crate::host_set("host.engineVersion", "");
    crate::host_set("host.isTouch", false);
    crate::host_set("host.kind", "unknown");
    crate::host_set("host.locale", "");
    crate::host_set("host.name", "unknown");
    crate::host_set("host.osBuild", "");
    crate::host_set("host.pointerWidth", (-1.0_f64));
    crate::host_set("host.runtime", "unknown");
    crate::host_set("host.version", "");
}

// Source: upstream/packages/platform/src/platform.ts:83 (sha256:b922f95a2d8977634e0add1858bd34c4f5ca69352e15404c32217fdc7e70a1ba)
pub fn is_platform_desktop(host_platform: &HostPlatformCapability) -> bool {
    return (get_platform_kind(host_platform) == "desktop");
}

// Source: upstream/packages/platform/src/platform.ts:88 (sha256:27814dfc70c43789020e1f78947efe4c6f95525cb12591b9dd596ee0117ef95c)
pub fn is_platform_mobile(host_platform: &HostPlatformCapability) -> bool {
    return (get_platform_kind(host_platform) == "mobile");
}

// Source: upstream/packages/platform/src/platform.ts:94 (sha256:5fdd18f8f2ac14373cbbafdd9618ff464f317c230d52d097e6a17ece8e852663)
pub fn is_platform_native(host_platform: &HostPlatformCapability) -> bool {
    let runtime = get_platform_runtime(host_platform);
    return (runtime != "web") && (runtime != "unknown");
}

// Source: upstream/packages/platform/src/platform.ts:100 (sha256:836c60ad7e4e43440eca04649263182a9c94329ae7e27474f9fbc0c0de5cd68b)
pub fn is_platform_touch(host_platform: &HostPlatformCapability) -> bool {
    return get_platform_info(host_platform, &_SCRATCH).is_touch;
}

// Source: upstream/packages/platform/src/platform.ts:107 (sha256:4fec380eca306f2230161b3930472713ef37eb5827f92c39a94ab7ffa2892a71)
pub fn is_platform_version_at_least(
    host_platform: &HostPlatformCapability,
    minimum: String,
) -> bool {
    let version = (get_platform_info(host_platform, &_SCRATCH).version).clone();
    if (version == "") {
        return false;
    }
    return (compare_platform_versions((version).clone(), (minimum).clone()) >= 0.0_f64);
}

// Source: upstream/packages/platform/src/platform.ts:114 (sha256:29eb62d4e1fe8c9003fa552729ee06f85a433aefdd63e075c817f7db6d536d00)
pub fn is_platform_web(host_platform: &HostPlatformCapability) -> bool {
    return (get_platform_kind(host_platform) == "web");
}

// Source: upstream/packages/platform/src/platform.ts:120 (sha256:f3714260795db64c73eea4d1ccaacb716390e45087f3c0e0cfb6a5749ef1a5dd)
static _SCRATCH: std::sync::LazyLock<PlatformInfo> =
    std::sync::LazyLock::new(|| create_platform_info());
