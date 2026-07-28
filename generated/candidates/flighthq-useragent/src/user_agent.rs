// @generated from upstream/packages/useragent/src/userAgent.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::{
    PlatformEndianness, PlatformEngine, PlatformKind, PlatformName, PlatformRuntime,
};

// Source: upstream/packages/useragent/src/userAgent.ts:6 (sha256:d5baef41853fbef770a4cc3efb2ecbe65ae1a7706cca4d80e2e1412ce0d1aa94)
pub fn detect_endianness() -> PlatformEndianness {
    let __flight_try_return: Option<PlatformEndianness> = match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| -> Option<PlatformEndianness> {
            {
                let buf = vec![0_u8; (2.0_f64) as usize];
                vec![0_u16; (buf) as usize][0.0_f64 as usize] = (258.0_f64) as u16;
                let bytes = vec![0_u8; (buf) as usize];
                if ((bytes[0.0_f64 as usize] as f64) == 1.0_f64) {
                    return Some("big".to_owned());
                }
                if ((bytes[0.0_f64 as usize] as f64) == 2.0_f64) {
                    return Some("little".to_owned());
                }
            }
            None
        }),
    ) {
        Ok(value) => value,
        Err(_) => (|| -> Option<PlatformEndianness> {
            {}
            None
        })(),
    };
    if let Some(__flight_return) = __flight_try_return {
        return __flight_return;
    }
    return "unknown".to_owned();
}

// Source: upstream/packages/useragent/src/userAgent.ts:28 (sha256:789f99df0bdee6a8cae820ac8e725eb72740523b08d122ab545007c54c27c25b)
pub fn parse_user_agent_arch(ua: String, uad_platform: Option<String>) -> String {
    if (uad_platform).is_some() {
        let p = (uad_platform.as_ref().unwrap()).to_lowercase();
        if (p).contains("arm") {
            return "arm64".to_owned();
        }
        if (((((p).contains("x86")) || ((p).contains("windows"))) || ((p).contains("linux")))
            || ((p).contains("mac")))
            || ((p).contains("chrome"))
        {
            return "x64".to_owned();
        }
    }
    if (regex::RegexBuilder::new("arm64|aarch64")
        .case_insensitive(true)
        .multi_line(false)
        .dot_matches_new_line(false)
        .build()
        .expect("upstream TypeScript regular expression must be valid Rust regex syntax"))
    .is_match(&(ua))
    {
        return "arm64".to_owned();
    }
    if (regex::RegexBuilder::new("arm")
        .case_insensitive(true)
        .multi_line(false)
        .dot_matches_new_line(false)
        .build()
        .expect("upstream TypeScript regular expression must be valid Rust regex syntax"))
    .is_match(&(ua))
    {
        return "arm".to_owned();
    }
    if (regex::RegexBuilder::new("x86_64|win64|wow64|x64")
        .case_insensitive(true)
        .multi_line(false)
        .dot_matches_new_line(false)
        .build()
        .expect("upstream TypeScript regular expression must be valid Rust regex syntax"))
    .is_match(&(ua))
    {
        return "x64".to_owned();
    }
    if (regex::RegexBuilder::new("i[3-6]86|x86")
        .case_insensitive(true)
        .multi_line(false)
        .dot_matches_new_line(false)
        .build()
        .expect("upstream TypeScript regular expression must be valid Rust regex syntax"))
    .is_match(&(ua))
    {
        return "x86".to_owned();
    }
    if (regex::RegexBuilder::new("riscv64")
        .case_insensitive(true)
        .multi_line(false)
        .dot_matches_new_line(false)
        .build()
        .expect("upstream TypeScript regular expression must be valid Rust regex syntax"))
    .is_match(&(ua))
    {
        return "riscv64".to_owned();
    }
    if (regex::RegexBuilder::new("mips64")
        .case_insensitive(true)
        .multi_line(false)
        .dot_matches_new_line(false)
        .build()
        .expect("upstream TypeScript regular expression must be valid Rust regex syntax"))
    .is_match(&(ua))
    {
        return "mips64".to_owned();
    }
    if (regex::RegexBuilder::new("mips")
        .case_insensitive(true)
        .multi_line(false)
        .dot_matches_new_line(false)
        .build()
        .expect("upstream TypeScript regular expression must be valid Rust regex syntax"))
    .is_match(&(ua))
    {
        return "mips".to_owned();
    }
    return "".to_owned();
}

// Source: upstream/packages/useragent/src/userAgent.ts:55 (sha256:821d16b3d73466c711d659aeaad0d9fd219f7ec5aef7c09f6bc5f84da645537b)
pub fn parse_user_agent_engine(ua: String) -> PlatformEngine {
    if (regex::RegexBuilder::new("firefox")
        .case_insensitive(true)
        .multi_line(false)
        .dot_matches_new_line(false)
        .build()
        .expect("upstream TypeScript regular expression must be valid Rust regex syntax"))
    .is_match(&(ua))
    {
        return "gecko".to_owned();
    }
    if (regex::RegexBuilder::new("chrome|chromium|edg|opr|samsung")
        .case_insensitive(true)
        .multi_line(false)
        .dot_matches_new_line(false)
        .build()
        .expect("upstream TypeScript regular expression must be valid Rust regex syntax"))
    .is_match(&(ua))
    {
        return "blink".to_owned();
    }
    if (regex::RegexBuilder::new("safari|webkit")
        .case_insensitive(true)
        .multi_line(false)
        .dot_matches_new_line(false)
        .build()
        .expect("upstream TypeScript regular expression must be valid Rust regex syntax"))
    .is_match(&(ua))
    {
        return "webkit".to_owned();
    }
    return "unknown".to_owned();
}

// Source: upstream/packages/useragent/src/userAgent.ts:66 (sha256:8b33a022b46a616d9059379eb57eed50b22b2d13330f2f97a52dea850ee8a135)
pub fn parse_user_agent_engine_version(ua: String, engine: PlatformEngine) -> String {
    {
        let __switch_value = engine;
        let __flight_case = if __switch_value == "gecko" {
            0_usize
        } else if __switch_value == "blink" {
            1_usize
        } else if __switch_value == "webkit" {
            2_usize
        } else {
            3_usize
        };
        '__flight_switch: {
            if __flight_case <= 0_usize {
                {
                    let m = (regex::RegexBuilder::new("firefox\\/([\\d.]+)").case_insensitive(true).multi_line(false).dot_matches_new_line(false).build().expect("upstream TypeScript regular expression must be valid Rust regex syntax").exec)(ua);
                    return if m {
                        m[1.0_f64 as usize].clone()
                    } else {
                        "".to_owned()
                    };
                }
            }
            if __flight_case <= 1_usize {
                {
                    let edg = (regex::RegexBuilder::new("edg\\/([\\d.]+)").case_insensitive(true).multi_line(false).dot_matches_new_line(false).build().expect("upstream TypeScript regular expression must be valid Rust regex syntax").exec)(ua);
                    if edg {
                        return edg[1.0_f64 as usize].clone();
                    }
                    let opr = (regex::RegexBuilder::new("opr\\/([\\d.]+)").case_insensitive(true).multi_line(false).dot_matches_new_line(false).build().expect("upstream TypeScript regular expression must be valid Rust regex syntax").exec)(ua);
                    if opr {
                        return opr[1.0_f64 as usize].clone();
                    }
                    let chrome = (regex::RegexBuilder::new("chrome\\/([\\d.]+)").case_insensitive(true).multi_line(false).dot_matches_new_line(false).build().expect("upstream TypeScript regular expression must be valid Rust regex syntax").exec)(ua);
                    return if chrome {
                        chrome[1.0_f64 as usize].clone()
                    } else {
                        "".to_owned()
                    };
                }
            }
            if __flight_case <= 2_usize {
                {
                    let ver = (regex::RegexBuilder::new("version\\/([\\d.]+)").case_insensitive(true).multi_line(false).dot_matches_new_line(false).build().expect("upstream TypeScript regular expression must be valid Rust regex syntax").exec)(ua);
                    if ver {
                        return ver[1.0_f64 as usize].clone();
                    }
                    let wk = (regex::RegexBuilder::new("applewebkit\\/([\\d.]+)").case_insensitive(true).multi_line(false).dot_matches_new_line(false).build().expect("upstream TypeScript regular expression must be valid Rust regex syntax").exec)(ua);
                    return if wk {
                        wk[1.0_f64 as usize].clone()
                    } else {
                        "".to_owned()
                    };
                }
            }
            if __flight_case <= 3_usize {
                return "".to_owned();
            }
        }
    }
}

// Source: upstream/packages/useragent/src/userAgent.ts:97 (sha256:c92f549e1575efbe4e108a020ad87b125eb57262130bbab1e7e8620ba29d7195)
pub fn parse_user_agent_kind(name: PlatformName) -> PlatformKind {
    if (name == "ios") || (name == "android") {
        return "mobile".to_owned();
    }
    return "web".to_owned();
}

// Source: upstream/packages/useragent/src/userAgent.ts:104 (sha256:f1cc850d0dd4c9d09876a1673900e25c4c2210a6a0fc0c79a2099e3b55322682)
pub fn parse_user_agent_name(ua: String) -> PlatformName {
    if (regex::RegexBuilder::new("android")
        .case_insensitive(true)
        .multi_line(false)
        .dot_matches_new_line(false)
        .build()
        .expect("upstream TypeScript regular expression must be valid Rust regex syntax"))
    .is_match(&(ua))
    {
        return "android".to_owned();
    }
    if (regex::RegexBuilder::new("iphone|ipad|ipod")
        .case_insensitive(true)
        .multi_line(false)
        .dot_matches_new_line(false)
        .build()
        .expect("upstream TypeScript regular expression must be valid Rust regex syntax"))
    .is_match(&(ua))
    {
        return "ios".to_owned();
    }
    if (regex::RegexBuilder::new("win")
        .case_insensitive(true)
        .multi_line(false)
        .dot_matches_new_line(false)
        .build()
        .expect("upstream TypeScript regular expression must be valid Rust regex syntax"))
    .is_match(&(ua))
    {
        return "windows".to_owned();
    }
    if (regex::RegexBuilder::new("mac")
        .case_insensitive(true)
        .multi_line(false)
        .dot_matches_new_line(false)
        .build()
        .expect("upstream TypeScript regular expression must be valid Rust regex syntax"))
    .is_match(&(ua))
    {
        return "macos".to_owned();
    }
    if (regex::RegexBuilder::new("linux")
        .case_insensitive(true)
        .multi_line(false)
        .dot_matches_new_line(false)
        .build()
        .expect("upstream TypeScript regular expression must be valid Rust regex syntax"))
    .is_match(&(ua))
    {
        return "linux".to_owned();
    }
    return "web".to_owned();
}

// Source: upstream/packages/useragent/src/userAgent.ts:115 (sha256:3f48bb4e95e7fa77f2cfe72dc2a369c302c423f6192c84fbc04eb9a12a115909)
pub fn parse_user_agent_pointer_width(arch: String) -> f64 {
    if (arch == "x64") || (arch == "arm64") {
        return 64.0_f64;
    }
    if (arch == "x86") || (arch == "arm") {
        return 32.0_f64;
    }
    return (-1.0_f64);
}

// Source: upstream/packages/useragent/src/userAgent.ts:124 (sha256:c23d0d9818a2b32349680c667b0203c07f336594eb9dbe37ce12b90774559b08)
pub fn parse_user_agent_runtime(
    win: Option<Vec<(String, crate::OpaqueHostValue)>>,
) -> PlatformRuntime {
    if (win).is_none() {
        return "unknown".to_owned();
    }
    let proc = win
        .as_ref()
        .unwrap()
        .iter()
        .find(|(key, _)| key == &"process".to_owned())
        .map(|(_, value)| value.clone())
        .expect("TypeScript Record key was absent");
    if (proc.as_ref().and_then(|entries| {
        entries
            .iter()
            .find(|(key, _)| key == &"versions".to_owned())
            .map(|(_, value)| value.clone())
    }) && proc
        .as_ref()
        .unwrap()
        .iter()
        .find(|(key, _)| key == &"versions".to_owned())
        .map(|(_, value)| value.clone())
        .expect("TypeScript Record key was absent")
        .iter()
        .find(|(key, _)| key == &"electron".to_owned())
        .map(|(_, value)| value.clone())
        .expect("TypeScript Record key was absent"))
    .is_some()
    {
        return "electron".to_owned();
    }
    if win
        .as_ref()
        .unwrap()
        .iter()
        .find(|(key, _)| key == &"__TAURI__".to_owned())
        .map(|(_, value)| value.clone())
        .expect("TypeScript Record key was absent")
    {
        return "tauri".to_owned();
    }
    if win
        .as_ref()
        .unwrap()
        .iter()
        .find(|(key, _)| key == &"Capacitor".to_owned())
        .map(|(_, value)| value.clone())
        .expect("TypeScript Record key was absent")
    {
        return "capacitor".to_owned();
    }
    return "web".to_owned();
}

// Source: upstream/packages/useragent/src/userAgent.ts:139 (sha256:37fd95e41b1bb77ead555c52f2ea886be66312e331d4c758d3af0e371b3627e5)
pub fn parse_user_agent_version(ua: String, name: PlatformName) -> String {
    {
        let __switch_value = name;
        let __flight_case = if __switch_value == "windows" {
            0_usize
        } else if __switch_value == "macos" {
            1_usize
        } else if __switch_value == "ios" {
            2_usize
        } else if __switch_value == "android" {
            3_usize
        } else if __switch_value == "linux" {
            4_usize
        } else {
            5_usize
        };
        '__flight_switch: {
            if __flight_case <= 0_usize {
                {
                    let m = (regex::RegexBuilder::new("windows nt ([\\d.]+)").case_insensitive(true).multi_line(false).dot_matches_new_line(false).build().expect("upstream TypeScript regular expression must be valid Rust regex syntax").exec)(ua);
                    return if m {
                        m[1.0_f64 as usize].clone()
                    } else {
                        "".to_owned()
                    };
                }
            }
            if __flight_case <= 1_usize {
                {
                    let m = (regex::RegexBuilder::new("mac os x ([\\d_.]+)").case_insensitive(true).multi_line(false).dot_matches_new_line(false).build().expect("upstream TypeScript regular expression must be valid Rust regex syntax").exec)(ua);
                    return if m {
                        (m[1.0_f64 as usize].replace)(regex::RegexBuilder::new("_").case_insensitive(false).multi_line(false).dot_matches_new_line(false).build().expect("upstream TypeScript regular expression must be valid Rust regex syntax"), ".")
                    } else {
                        "".to_owned()
                    };
                }
            }
            if __flight_case <= 2_usize {
                {
                    let m = (regex::RegexBuilder::new("cpu(?: iphone)? os ([\\d_]+)").case_insensitive(true).multi_line(false).dot_matches_new_line(false).build().expect("upstream TypeScript regular expression must be valid Rust regex syntax").exec)(ua);
                    return if m {
                        (m[1.0_f64 as usize].replace)(regex::RegexBuilder::new("_").case_insensitive(false).multi_line(false).dot_matches_new_line(false).build().expect("upstream TypeScript regular expression must be valid Rust regex syntax"), ".")
                    } else {
                        "".to_owned()
                    };
                }
            }
            if __flight_case <= 3_usize {
                {
                    let m = (regex::RegexBuilder::new("android ([\\d.]+)").case_insensitive(true).multi_line(false).dot_matches_new_line(false).build().expect("upstream TypeScript regular expression must be valid Rust regex syntax").exec)(ua);
                    return if m {
                        m[1.0_f64 as usize].clone()
                    } else {
                        "".to_owned()
                    };
                }
            }
            if __flight_case <= 4_usize {
                return "".to_owned();
            }
            if __flight_case <= 5_usize {
                return "".to_owned();
            }
        }
    }
}
