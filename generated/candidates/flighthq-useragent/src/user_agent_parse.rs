// @generated from upstream/packages/useragent/src/userAgentParse.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::{
    DEVICE_FORM_FACTOR_CAR as device_form_factor_car_constant,
    DEVICE_FORM_FACTOR_DESKTOP as device_form_factor_desktop_constant,
    DEVICE_FORM_FACTOR_PHONE as device_form_factor_phone_constant,
    DEVICE_FORM_FACTOR_TABLET as device_form_factor_tablet_constant,
    DEVICE_FORM_FACTOR_TV as device_form_factor_tv_constant,
    DEVICE_FORM_FACTOR_UNKNOWN as device_form_factor_unknown_constant,
    DEVICE_FORM_FACTOR_WATCH as device_form_factor_watch_constant, DeviceFormFactor,
};

// Source: upstream/packages/useragent/src/userAgentParse.ts:22 (sha256:e6d7c8ee8d6242f09621e9fa6f31be404e05731280e2093aec27b41474b895a1)
pub fn parse_user_agent_form_factor(ua: String, max_touch_points: f64) -> DeviceFormFactor {
    if (regex::RegexBuilder::new("android auto|car browser|automotive")
        .case_insensitive(true)
        .multi_line(false)
        .dot_matches_new_line(false)
        .build()
        .expect("upstream TypeScript regular expression must be valid Rust regex syntax"))
    .is_match(&(ua))
    {
        return (device_form_factor_car_constant).to_owned();
    }
    if (regex::RegexBuilder::new(
        "smart[-_]?tv|smarttv|googletv|appletv|hbbtv|netcast|webos.*tv|tizen.*tv|tv safari",
    )
    .case_insensitive(true)
    .multi_line(false)
    .dot_matches_new_line(false)
    .build()
    .expect("upstream TypeScript regular expression must be valid Rust regex syntax"))
    .is_match(&(ua))
    {
        return (device_form_factor_tv_constant).to_owned();
    }
    if (regex::RegexBuilder::new("watch\\s*os|watch[_ ]?kit|wearable")
        .case_insensitive(true)
        .multi_line(false)
        .dot_matches_new_line(false)
        .build()
        .expect("upstream TypeScript regular expression must be valid Rust regex syntax"))
    .is_match(&(ua))
    {
        return (device_form_factor_watch_constant).to_owned();
    }
    if (regex::RegexBuilder::new("ipad")
        .case_insensitive(true)
        .multi_line(false)
        .dot_matches_new_line(false)
        .build()
        .expect("upstream TypeScript regular expression must be valid Rust regex syntax"))
    .is_match(&(ua))
    {
        return (device_form_factor_tablet_constant).to_owned();
    }
    if ((regex::RegexBuilder::new("android")
        .case_insensitive(true)
        .multi_line(false)
        .dot_matches_new_line(false)
        .build()
        .expect("upstream TypeScript regular expression must be valid Rust regex syntax"))
    .is_match(&(ua)))
        && (!(regex::RegexBuilder::new("mobile")
            .case_insensitive(true)
            .multi_line(false)
            .dot_matches_new_line(false)
            .build()
            .expect("upstream TypeScript regular expression must be valid Rust regex syntax"))
        .is_match(&(ua)))
    {
        return (device_form_factor_tablet_constant).to_owned();
    }
    if (regex::RegexBuilder::new("tablet\\s*pc|silk|kindle fire")
        .case_insensitive(true)
        .multi_line(false)
        .dot_matches_new_line(false)
        .build()
        .expect("upstream TypeScript regular expression must be valid Rust regex syntax"))
    .is_match(&(ua))
    {
        return (device_form_factor_tablet_constant).to_owned();
    }
    if (regex::RegexBuilder::new(
        "iphone|ipod|android.*mobile|windows phone|blackberry|bb\\d+|mobile safari",
    )
    .case_insensitive(true)
    .multi_line(false)
    .dot_matches_new_line(false)
    .build()
    .expect("upstream TypeScript regular expression must be valid Rust regex syntax"))
    .is_match(&(ua))
    {
        return (device_form_factor_phone_constant).to_owned();
    }
    if (regex::RegexBuilder::new("win(?:dows)?nt|macintosh|mac os x|linux(?!.*android)|x11")
        .case_insensitive(true)
        .multi_line(false)
        .dot_matches_new_line(false)
        .build()
        .expect("upstream TypeScript regular expression must be valid Rust regex syntax"))
    .is_match(&(ua))
    {
        return (device_form_factor_desktop_constant).to_owned();
    }
    if (max_touch_points == 0.0_f64) {
        return (device_form_factor_desktop_constant).to_owned();
    }
    return (device_form_factor_unknown_constant).to_owned();
}

// Source: upstream/packages/useragent/src/userAgentParse.ts:50 (sha256:30282dc1ccb3eb64badca066b2e824450bc3626941e673c624c86ec34113007e)
pub fn parse_user_agent_os_name(ua: String) -> String {
    if (regex::RegexBuilder::new("android")
        .case_insensitive(true)
        .multi_line(false)
        .dot_matches_new_line(false)
        .build()
        .expect("upstream TypeScript regular expression must be valid Rust regex syntax"))
    .is_match(&(ua))
    {
        return "Android".to_owned();
    }
    if (regex::RegexBuilder::new("ipad")
        .case_insensitive(true)
        .multi_line(false)
        .dot_matches_new_line(false)
        .build()
        .expect("upstream TypeScript regular expression must be valid Rust regex syntax"))
    .is_match(&(ua))
    {
        return "iPadOS".to_owned();
    }
    if (regex::RegexBuilder::new("iphone|ipod")
        .case_insensitive(true)
        .multi_line(false)
        .dot_matches_new_line(false)
        .build()
        .expect("upstream TypeScript regular expression must be valid Rust regex syntax"))
    .is_match(&(ua))
    {
        return "iOS".to_owned();
    }
    if (regex::RegexBuilder::new("cros")
        .case_insensitive(true)
        .multi_line(false)
        .dot_matches_new_line(false)
        .build()
        .expect("upstream TypeScript regular expression must be valid Rust regex syntax"))
    .is_match(&(ua))
    {
        return "ChromeOS".to_owned();
    }
    if (regex::RegexBuilder::new("windows nt|windows phone")
        .case_insensitive(true)
        .multi_line(false)
        .dot_matches_new_line(false)
        .build()
        .expect("upstream TypeScript regular expression must be valid Rust regex syntax"))
    .is_match(&(ua))
    {
        return "Windows".to_owned();
    }
    if (regex::RegexBuilder::new("macintosh|mac os x")
        .case_insensitive(true)
        .multi_line(false)
        .dot_matches_new_line(false)
        .build()
        .expect("upstream TypeScript regular expression must be valid Rust regex syntax"))
    .is_match(&(ua))
    {
        return "macOS".to_owned();
    }
    if (regex::RegexBuilder::new("freebsd")
        .case_insensitive(true)
        .multi_line(false)
        .dot_matches_new_line(false)
        .build()
        .expect("upstream TypeScript regular expression must be valid Rust regex syntax"))
    .is_match(&(ua))
    {
        return "FreeBSD".to_owned();
    }
    if (regex::RegexBuilder::new("openbsd")
        .case_insensitive(true)
        .multi_line(false)
        .dot_matches_new_line(false)
        .build()
        .expect("upstream TypeScript regular expression must be valid Rust regex syntax"))
    .is_match(&(ua))
    {
        return "OpenBSD".to_owned();
    }
    if (regex::RegexBuilder::new("netbsd")
        .case_insensitive(true)
        .multi_line(false)
        .dot_matches_new_line(false)
        .build()
        .expect("upstream TypeScript regular expression must be valid Rust regex syntax"))
    .is_match(&(ua))
    {
        return "NetBSD".to_owned();
    }
    if (regex::RegexBuilder::new("linux")
        .case_insensitive(true)
        .multi_line(false)
        .dot_matches_new_line(false)
        .build()
        .expect("upstream TypeScript regular expression must be valid Rust regex syntax"))
    .is_match(&(ua))
    {
        return "Linux".to_owned();
    }
    return "".to_owned();
}

// Source: upstream/packages/useragent/src/userAgentParse.ts:66 (sha256:0734439e3ba3affde4953ec382dcbabee352bc2500bbac86b93ca1a0436739b1)
pub fn parse_user_agent_os_version(ua: String) -> String {
    let android = {
        let __flight_regex = regex::RegexBuilder::new("android\\s+([\\d.]+)")
            .case_insensitive(true)
            .multi_line(false)
            .dot_matches_new_line(false)
            .build()
            .expect("upstream TypeScript regular expression must be valid Rust regex syntax");
        __flight_regex.captures(&(ua)).map(|captures| {
            (0..captures.len())
                .map(|index| {
                    captures
                        .get(index)
                        .map_or("", |matched| matched.as_str())
                        .to_owned()
                })
                .collect::<Vec<_>>()
        })
    };
    if (android).is_some() {
        return android.as_ref().unwrap()[1.0_f64 as usize].clone();
    }
    let ios = {
        let __flight_regex = regex::RegexBuilder::new("(?:iphone|ipad|ipod).*?os\\s+([\\d_]+)")
            .case_insensitive(true)
            .multi_line(false)
            .dot_matches_new_line(false)
            .build()
            .expect("upstream TypeScript regular expression must be valid Rust regex syntax");
        __flight_regex.captures(&(ua)).map(|captures| {
            (0..captures.len())
                .map(|index| {
                    captures
                        .get(index)
                        .map_or("", |matched| matched.as_str())
                        .to_owned()
                })
                .collect::<Vec<_>>()
        })
    };
    if (ios).is_some() {
        return (regex::RegexBuilder::new("_")
            .case_insensitive(false)
            .multi_line(false)
            .dot_matches_new_line(false)
            .build()
            .expect("upstream TypeScript regular expression must be valid Rust regex syntax"))
        .replace_all(&(ios.as_ref().unwrap()[1.0_f64 as usize].clone()), ".")
        .into_owned();
    }
    let win = {
        let __flight_regex = regex::RegexBuilder::new("windows nt\\s+([\\d.]+)")
            .case_insensitive(true)
            .multi_line(false)
            .dot_matches_new_line(false)
            .build()
            .expect("upstream TypeScript regular expression must be valid Rust regex syntax");
        __flight_regex.captures(&(ua)).map(|captures| {
            (0..captures.len())
                .map(|index| {
                    captures
                        .get(index)
                        .map_or("", |matched| matched.as_str())
                        .to_owned()
                })
                .collect::<Vec<_>>()
        })
    };
    if (win).is_some() {
        return win.as_ref().unwrap()[1.0_f64 as usize].clone();
    }
    let mac = {
        let __flight_regex = regex::RegexBuilder::new("mac os x\\s+([\\d_.]+)")
            .case_insensitive(true)
            .multi_line(false)
            .dot_matches_new_line(false)
            .build()
            .expect("upstream TypeScript regular expression must be valid Rust regex syntax");
        __flight_regex.captures(&(ua)).map(|captures| {
            (0..captures.len())
                .map(|index| {
                    captures
                        .get(index)
                        .map_or("", |matched| matched.as_str())
                        .to_owned()
                })
                .collect::<Vec<_>>()
        })
    };
    if (mac).is_some() {
        return (regex::RegexBuilder::new("_")
            .case_insensitive(false)
            .multi_line(false)
            .dot_matches_new_line(false)
            .build()
            .expect("upstream TypeScript regular expression must be valid Rust regex syntax"))
        .replace_all(&(mac.as_ref().unwrap()[1.0_f64 as usize].clone()), ".")
        .into_owned();
    }
    let cros = {
        let __flight_regex = regex::RegexBuilder::new("cros\\s+\\S+\\s+([\\d.]+)")
            .case_insensitive(true)
            .multi_line(false)
            .dot_matches_new_line(false)
            .build()
            .expect("upstream TypeScript regular expression must be valid Rust regex syntax");
        __flight_regex.captures(&(ua)).map(|captures| {
            (0..captures.len())
                .map(|index| {
                    captures
                        .get(index)
                        .map_or("", |matched| matched.as_str())
                        .to_owned()
                })
                .collect::<Vec<_>>()
        })
    };
    if (cros).is_some() {
        return cros.as_ref().unwrap()[1.0_f64 as usize].clone();
    }
    return "".to_owned();
}
