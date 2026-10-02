// @generated from upstream/packages/protocol/src/protocol.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_entity::{allocate_entity, finish_entity};
use flighthq_signals::{clear_signal, create_signal, emit_signal};
use flighthq_types::{
    EntityConstruction, HostProtocolDefaultCapability, HostProtocolLaunchCapability,
    HostProtocolOpenCapability, HostProtocolRegistrationCapability,
    HostProtocolRegistrationQueryCapability, HostProtocolUnregistrationCapability,
    ParsedProtocolUrl, ProtocolHandler,
};

#[inline]

fn __flight_string_index_of(value: &str, search: &str, position: f64) -> f64 {
    let value: Vec<u16> = value.encode_utf16().collect();
    let search: Vec<u16> = search.encode_utf16().collect();
    let start = if position.is_nan() || position <= 0.0_f64 {
        0_usize
    } else if position >= value.len() as f64 {
        value.len()
    } else {
        position.trunc() as usize
    };
    if search.is_empty() {
        return start as f64;
    }
    value[start..]
        .windows(search.len())
        .position(|window| window == search)
        .map_or(-1.0_f64, |index| (start + index) as f64)
}

#[inline]

fn __flight_string_slice(value: &str, start: f64, end: Option<f64>) -> String {
    let value: Vec<u16> = value.encode_utf16().collect();
    let length = value.len();
    let relative = |index: f64| -> usize {
        if index.is_nan() {
            0
        } else if index < 0.0_f64 {
            length.saturating_sub((-index.trunc()) as usize)
        } else {
            (index.trunc() as usize).min(length)
        }
    };
    let start = relative(start);
    let end = end.map_or(length, relative);
    String::from_utf16_lossy(&value[start..end.max(start)])
}

#[inline]

fn __flight_encode_uri_component(value: &str) -> String {
    const HEX: &[u8; 16] = b"0123456789ABCDEF";
    let mut encoded = String::with_capacity(value.len());
    for byte in value.bytes() {
        if byte.is_ascii_alphanumeric() || b"-_.!~*'()".contains(&byte) {
            encoded.push(char::from(byte));
        } else {
            encoded.push('%');
            encoded.push(char::from(HEX[(byte >> 4) as usize]));
            encoded.push(char::from(HEX[(byte & 0x0F) as usize]));
        }
    }
    encoded
}

#[inline]

fn __flight_decode_uri_component(value: &str) -> String {
    let bytes = value.as_bytes();
    let mut decoded = Vec::with_capacity(bytes.len());
    let mut index = 0_usize;
    while index < bytes.len() {
        if bytes[index] != b'%' {
            decoded.push(bytes[index]);
            index += 1;
            continue;
        }
        assert!(
            index + 2 < bytes.len(),
            "decodeURIComponent received an incomplete escape"
        );
        let digit = |byte: u8| -> Option<u8> {
            match byte {
                b'0'..=b'9' => Some(byte - b'0'),
                b'a'..=b'f' => Some(byte - b'a' + 10),
                b'A'..=b'F' => Some(byte - b'A' + 10),
                _ => None,
            }
        };
        let high = digit(bytes[index + 1]).expect("decodeURIComponent received a malformed escape");
        let low = digit(bytes[index + 2]).expect("decodeURIComponent received a malformed escape");
        decoded.push((high << 4) | low);
        index += 3;
    }
    String::from_utf8(decoded).expect("decodeURIComponent received invalid UTF-8")
}

#[derive(Clone, Default)]
pub struct FlightPartialRecord458042621 {
    pub __flight_identity: std::sync::Arc<()>,
    pub scheme: Option<String>,
    pub host: Option<String>,
    pub path: Option<String>,
    pub query: Option<Vec<(String, String)>>,
}
impl PartialEq for FlightPartialRecord458042621 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/protocol/src/protocol.ts:15 (sha256:c975559a160f1dff8689bb4f2d2d3da4ec0afaf49fd788a9dbf74467d0323175)
pub fn attach_protocol_handler(
    host_protocol_open: &HostProtocolOpenCapability,
    handler: ProtocolHandler,
) -> () {
    detach_protocol_handler(&handler);
    {
        let __flight_key = (handler).clone();
        let __flight_value = {
            let __flight_callback = (host_protocol_open.subscribe).clone();
            let __flight_result = __flight_callback.lock().unwrap()(std::sync::Arc::new(
                std::sync::Mutex::new(Box::new({
                    let handler = handler.clone();
                    move |url: String| -> () {
                        emit_signal((handler.on_open_url).clone(), ((url).clone(),))
                    }
                })
                    as Box<dyn FnMut(String) -> () + Send + 'static>),
            ));
            __flight_result
        };
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

// Source: upstream/packages/protocol/src/protocol.ts:27 (sha256:71f61c16347c0ad47ba06459424d821f4e2fa68304f05900053221732ef704b3)
pub fn create_protocol_handler() -> ProtocolHandler {
    let mut out = allocate_entity();
    initialize_protocol_handler((out).clone());
    return finish_entity((out).clone());
}

// Source: upstream/packages/protocol/src/protocol.ts:33 (sha256:cf860ce2741fe40de7c2a6a4282b104b4fceb0be546db338fcabf08aa7fbb17a)
pub fn create_protocol_url(parts: &FlightPartialRecord458042621) -> String {
    let scheme = ((parts.scheme).clone()).unwrap_or("unknown".to_owned());
    let host = ((parts.host).clone()).unwrap_or("".to_owned());
    let path = ((parts.path).clone()).unwrap_or("".to_owned());
    let query = (parts.query).clone();
    let authority = if !(host).is_empty() {
        format!("//{}", (host).clone())
    } else {
        "".to_owned()
    };
    let normalized_path =
        if (!(path).is_empty()) && (!(path).starts_with(("/".to_owned()).as_str())) {
            format!("/{}", (path).clone())
        } else {
            (path).clone()
        };
    let mut url = format!(
        "{}:{}{}",
        (scheme).clone(),
        (authority).clone(),
        (normalized_path).clone()
    );
    if (query).is_some() {
        let entries = {
            let mut __flight_filter = |__parameter0: (String, String)| -> bool {
                let key = __parameter0.0.clone();
                return ((key.encode_utf16().count() as f64) > 0.0_f64);
            };
            (((query.as_ref().unwrap()).clone()).clone())
                .iter()
                .cloned()
                .filter(|value| __flight_filter(value.clone()))
                .collect::<Vec<_>>()
        };
        if ((entries.len() as f64) > 0.0_f64) {
            let query_string = ((entries)
                .iter()
                .cloned()
                .map(|__parameter1: (String, String)| -> String {
                    let key = __parameter1.0.clone();
                    let value = __parameter1.1.clone();
                    return format!(
                        "{}={}",
                        __flight_encode_uri_component(&((key).clone())),
                        __flight_encode_uri_component(&((value).clone()))
                    );
                })
                .collect::<Vec<_>>())
            .iter()
            .map(|value| value.to_string())
            .collect::<Vec<_>>()
            .join(("&".to_owned()).as_str());
            url.push_str(&(format!("?{}", (query_string).clone())));
        }
    }
    return url;
}

// Source: upstream/packages/protocol/src/protocol.ts:53 (sha256:e0602d6ffbda511697004ba0dff697f9637eb45ed4abcbdd7e9f9fdd9f5db365)
pub fn detach_protocol_handler(handler: &ProtocolHandler) -> () {
    {
        let __flight_callback = (*_SUBSCRIPTIONS.lock().unwrap())
            .iter()
            .find(|(entry_key, _)| entry_key == &(*handler).clone())
            .map(|(_, value)| value.clone());
        __flight_callback
            .as_ref()
            .map(|callback| callback.lock().unwrap()())
    };
    {
        let __flight_key = (*handler).clone();
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

// Source: upstream/packages/protocol/src/protocol.ts:58 (sha256:1c47b598ef46002f2ab3cb76d400be8bafd8bf708835ec6a3b1f52deab6efe5d)
pub fn dispose_protocol_handler(handler: &mut ProtocolHandler) -> () {
    detach_protocol_handler(handler);
    clear_signal(&mut handler.on_open_url);
}

// Source: upstream/packages/protocol/src/protocol.ts:63 (sha256:2dcc84e7fe2ca7ee8c8064f7c0dbaab86d7240a94ef8ce4b9ac2f32bc5514181)
pub fn get_protocol_launch_url(
    host_protocol_launch: &HostProtocolLaunchCapability,
) -> Option<String> {
    return {
        let __flight_callback = (host_protocol_launch.get_launch_url).clone();
        let __flight_result = __flight_callback.lock().unwrap()();
        __flight_result
    };
}

// Source: upstream/packages/protocol/src/protocol.ts:67 (sha256:87c96d4dc0aca06678939a9c3b5822aa9d70404e7df8dad0209d60d83870448e)
pub fn get_registered_protocol_schemes(
    host_protocol_registration: &HostProtocolRegistrationCapability,
) -> Vec<String> {
    return {
        let __flight_callback = (host_protocol_registration.get_registered_schemes).clone();
        let __flight_result = __flight_callback.lock().unwrap()();
        __flight_result
    };
}

// Source: upstream/packages/protocol/src/protocol.ts:73 (sha256:a7c91dc07879881166bed15b9414adb7543d8dd01b434625dfcf5f8220b58e7a)
pub fn initialize_protocol_handler(out: EntityConstruction<ProtocolHandler>) -> () {
    crate::host_set("host.onOpenUrl", create_signal());
}

// Source: upstream/packages/protocol/src/protocol.ts:77 (sha256:94849dc8959603bad561bd31160daee2a54d85353d9ff20586acac2fff37206d)
pub fn is_protocol_scheme_default(
    host_protocol_default: &HostProtocolDefaultCapability,
    scheme: String,
) -> bool {
    return (is_valid_protocol_scheme((scheme).clone()))
        && ({
            let __flight_callback = (host_protocol_default.is_default).clone();
            let __flight_result = __flight_callback.lock().unwrap()((scheme).clone());
            __flight_result
        });
}

// Source: upstream/packages/protocol/src/protocol.ts:84 (sha256:de958b298555c1ab63f48b98fe9fa01dd5a8ba2eed6ade5a84fff9a6f01f5925)
pub fn is_protocol_scheme_registered(
    host_protocol_registration_query: &HostProtocolRegistrationQueryCapability,
    scheme: String,
) -> bool {
    return (is_valid_protocol_scheme((scheme).clone()))
        && ({
            let __flight_callback = (host_protocol_registration_query.is_registered).clone();
            let __flight_result = __flight_callback.lock().unwrap()((scheme).clone());
            __flight_result
        });
}

// Source: upstream/packages/protocol/src/protocol.ts:91 (sha256:6ae6c76401417f2cfe6ec13b993ab35cbfaf6007e2a07e1fba3d42ee474cbdfd)
pub fn is_valid_protocol_scheme(scheme: String) -> bool {
    let __flight_utf16_scheme: std::sync::Arc<Vec<u16>> =
        std::sync::Arc::new(scheme.encode_utf16().collect());
    if ("string".to_owned() != "string") || ((__flight_utf16_scheme.len() as f64) == 0.0_f64) {
        return false;
    }
    let lower = (scheme).to_lowercase();
    if _RESERVED_SCHEMES
        .iter()
        .any(|item| item == &(lower).clone())
    {
        return false;
    }
    return ((*_SCHEME_PATTERN).clone()).is_match(&(lower));
}

// Source: upstream/packages/protocol/src/protocol.ts:98 (sha256:4ad66fee5e1488fe2606b57f37c24c5a9b339c733e8c531701d90953552bb59a)
#[derive(Clone, Default)]
struct ParseProtocolUrlRecord2 {
    __flight_identity: std::sync::Arc<()>,
}
impl PartialEq for ParseProtocolUrlRecord2 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn parse_protocol_url(url: String) -> Option<ParsedProtocolUrl> {
    let __flight_utf16_url: std::sync::Arc<Vec<u16>> =
        std::sync::Arc::new(url.encode_utf16().collect());
    if ("string".to_owned() != "string") || ((__flight_utf16_url.len() as f64) == 0.0_f64) {
        return None;
    }
    let colon_index = __flight_string_index_of(&(url), &(":".to_owned()), 0.0_f64);
    if (colon_index <= 0.0_f64) {
        return None;
    }
    let scheme = (__flight_string_slice(&(url), 0.0_f64, Some(colon_index))).to_lowercase();
    if (!((*_SCHEME_PATTERN).clone()).is_match(&((scheme).clone()))) {
        return None;
    }
    let mut rest = __flight_string_slice(&(url), (colon_index + 1.0_f64), None);
    let mut host = "".to_owned();
    if (rest).starts_with(("//".to_owned()).as_str()) {
        rest = __flight_string_slice(&(rest), 2.0_f64, None);
        let slash_index = __flight_string_index_of(&(rest), &("/".to_owned()), 0.0_f64);
        let query_index = __flight_string_index_of(&(rest), &("?".to_owned()), 0.0_f64);
        let host_end = if (slash_index >= 0.0_f64)
            && ((query_index < 0.0_f64) || (slash_index < query_index))
        {
            slash_index
        } else {
            if (query_index >= 0.0_f64) {
                query_index
            } else {
                (rest.encode_utf16().count() as f64)
            }
        };
        host = __flight_string_slice(&(rest), 0.0_f64, Some(host_end));
        rest = __flight_string_slice(&(rest), host_end, None);
    }
    let query_index = __flight_string_index_of(&(rest), &("?".to_owned()), 0.0_f64);
    let path = if (query_index >= 0.0_f64) {
        __flight_string_slice(&((rest).clone()), 0.0_f64, Some(query_index))
    } else {
        (rest).clone()
    };
    let query_string = if (query_index >= 0.0_f64) {
        __flight_string_slice(&(rest), (query_index + 1.0_f64), None)
    } else {
        "".to_owned()
    };
    let mut query: Vec<(String, String)> = {
        let mut __flight_record = Vec::new();
        __flight_record
    };
    if ((query_string.encode_utf16().count() as f64) > 0.0_f64) {
        for pair in ((query_string)
            .split("&".to_owned().as_str())
            .map(|part| part.to_owned())
            .collect::<Vec<_>>())
        .iter()
        .cloned()
        {
            let equals_index =
                __flight_string_index_of(&((pair).clone()), &("=".to_owned()), 0.0_f64);
            if (equals_index < 0.0_f64) {
                let key = safe_decode_protocol_component((pair).clone());
                if ((key.encode_utf16().count() as f64) > 0.0_f64) {
                    {
                        let __flight_key = (key).clone();
                        let __flight_value = "".to_owned();
                        if let Some((_, value)) =
                            query.iter_mut().find(|(key, _)| key == &__flight_key)
                        {
                            *value = __flight_value;
                        } else {
                            query.push((__flight_key, __flight_value));
                        }
                    };
                }
            } else {
                let key = safe_decode_protocol_component(__flight_string_slice(
                    &((pair).clone()),
                    0.0_f64,
                    Some(equals_index),
                ));
                if ((key.encode_utf16().count() as f64) > 0.0_f64) {
                    {
                        let __flight_key = (key).clone();
                        let __flight_value = safe_decode_protocol_component(__flight_string_slice(
                            &((pair).clone()),
                            (equals_index + 1.0_f64),
                            None,
                        ));
                        if let Some((_, value)) =
                            query.iter_mut().find(|(key, _)| key == &__flight_key)
                        {
                            *value = __flight_value;
                        } else {
                            query.push((__flight_key, __flight_value));
                        }
                    };
                }
            }
        }
    }
    return Some(ParsedProtocolUrl {
        __flight_identity: std::sync::Arc::new(()),
        host: (host).clone(),
        path: (path).clone(),
        query: (query).clone(),
        scheme: (scheme).clone(),
    });
}

// Source: upstream/packages/protocol/src/protocol.ts:140 (sha256:d066982d1ffd9967d0258b9bb39fc40054ee278da42b254f2fae7305b49b1df2)
pub fn register_protocol_scheme(
    host_protocol_registration: &HostProtocolRegistrationCapability,
    scheme: String,
) -> bool {
    return (is_valid_protocol_scheme((scheme).clone()))
        && ({
            let __flight_callback = (host_protocol_registration.register).clone();
            let __flight_result = __flight_callback.lock().unwrap()((scheme).clone());
            __flight_result
        });
}

// Source: upstream/packages/protocol/src/protocol.ts:147 (sha256:a2745abd3c74ab520f6ba822b29a40865a3a6dc970b1c971be3874d8fef0f602)
pub fn register_protocol_schemes(
    host_protocol_registration: &HostProtocolRegistrationCapability,
    schemes: &Vec<String>,
) -> bool {
    if (!(schemes)
        .iter()
        .cloned()
        .all(|__flight_item| is_valid_protocol_scheme((__flight_item).clone())))
    {
        return false;
    }
    let mut all_succeeded = true;
    for scheme in (schemes).iter().cloned() {
        if (!{
            let __flight_callback = (host_protocol_registration.register).clone();
            let __flight_result = __flight_callback.lock().unwrap()((scheme).clone());
            __flight_result
        }) {
            all_succeeded = false;
        }
    }
    return all_succeeded;
}

// Source: upstream/packages/protocol/src/protocol.ts:160 (sha256:76fc263f6bfe04c48e00e2933f4cc2dbcf2fafd1b47d62214b1412ba3252c1f2)
pub fn remove_protocol_scheme_as_default(
    host_protocol_default: &HostProtocolDefaultCapability,
    scheme: String,
) -> bool {
    return (is_valid_protocol_scheme((scheme).clone()))
        && ({
            let __flight_callback = (host_protocol_default.remove_as_default).clone();
            let __flight_result = __flight_callback.lock().unwrap()((scheme).clone());
            __flight_result
        });
}

// Source: upstream/packages/protocol/src/protocol.ts:167 (sha256:d2b93f16642152f22f2c0a93f77f7c66ac83aa5f044a7743f3348d3344271ca2)
pub fn set_protocol_scheme_as_default(
    host_protocol_default: &HostProtocolDefaultCapability,
    scheme: String,
) -> bool {
    return (is_valid_protocol_scheme((scheme).clone()))
        && ({
            let __flight_callback = (host_protocol_default.set_as_default).clone();
            let __flight_result = __flight_callback.lock().unwrap()((scheme).clone());
            __flight_result
        });
}

// Source: upstream/packages/protocol/src/protocol.ts:174 (sha256:97d64871ff87c19799ce9e0aa6728d6f222bd0601f5d9320400b95def50e7a78)
pub fn unregister_protocol_scheme(
    host_protocol_unregistration: &HostProtocolUnregistrationCapability,
    scheme: String,
) -> bool {
    return (is_valid_protocol_scheme((scheme).clone()))
        && ({
            let __flight_callback = (host_protocol_unregistration.unregister).clone();
            let __flight_result = __flight_callback.lock().unwrap()((scheme).clone());
            __flight_result
        });
}

// Source: upstream/packages/protocol/src/protocol.ts:181 (sha256:e83393c68662a277ecc6c26fce0d7bb9b7ca76973c9fd57c83a15b2bb7ed78ab)
pub fn unregister_protocol_schemes(
    host_protocol_unregistration: &HostProtocolUnregistrationCapability,
    schemes: &Vec<String>,
) -> bool {
    if (!(schemes)
        .iter()
        .cloned()
        .all(|__flight_item| is_valid_protocol_scheme((__flight_item).clone())))
    {
        return false;
    }
    let mut all_succeeded = true;
    for scheme in (schemes).iter().cloned() {
        if (!{
            let __flight_callback = (host_protocol_unregistration.unregister).clone();
            let __flight_result = __flight_callback.lock().unwrap()((scheme).clone());
            __flight_result
        }) {
            all_succeeded = false;
        }
    }
    return all_succeeded;
}

// Source: upstream/packages/protocol/src/protocol.ts:194 (sha256:4b3a5a97814daa1d957dc05d78fa3b7007a00de8f5b8d5b8b254db4d09132c32)
static _SCHEME_PATTERN: std::sync::LazyLock<regex::Regex> = std::sync::LazyLock::new(|| {
    regex::RegexBuilder::new("^[a-z][a-z0-9+\\-.]*$")
        .case_insensitive(false)
        .multi_line(false)
        .dot_matches_new_line(false)
        .build()
        .expect("upstream TypeScript regular expression must be valid Rust regex syntax")
});

// Source: upstream/packages/protocol/src/protocol.ts:195 (sha256:f0179456dc2c7e6df3db60520cbbe54131d861d67bd5a5111d7f0dc937fcf82d)
static _RESERVED_SCHEMES: std::sync::LazyLock<Vec<String>> = std::sync::LazyLock::new(|| {
    let mut __flight_set = Vec::new();
    for __flight_value in vec![
        "file".to_owned(),
        "ftp".to_owned(),
        "ftps".to_owned(),
        "http".to_owned(),
        "https".to_owned(),
        "mailto".to_owned(),
    ] {
        if !__flight_set.contains(&__flight_value) {
            __flight_set.push(__flight_value);
        }
    }
    __flight_set
});

// Source: upstream/packages/protocol/src/protocol.ts:196 (sha256:f548928043116bccfcc87d35b6db0ba05b01ccea6779e0b9c2bd08df6bf61fd4)
static _SUBSCRIPTIONS: std::sync::LazyLock<
    std::sync::Mutex<
        Vec<(
            ProtocolHandler,
            std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>,
        )>,
    >,
> = std::sync::LazyLock::new(|| std::sync::Mutex::new(Vec::new()));

// Source: upstream/packages/protocol/src/protocol.ts:198 (sha256:a60865d1bbd29dde5f30114ec2bb053f1cad403c16aac4ca5824f60a8ca61a8f)
fn safe_decode_protocol_component(value: String) -> String {
    let __flight_try_return: Option<String> = match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| -> Option<String> {
            {
                return Some(__flight_decode_uri_component(&((regex::RegexBuilder::new("\\+").case_insensitive(false).multi_line(false).dot_matches_new_line(false).build().expect("upstream TypeScript regular expression must be valid Rust regex syntax")).replace_all(&(value), " ".to_owned()).into_owned())));
            }
            None
        }),
    ) {
        Ok(value) => value,
        Err(_) => (|| -> Option<String> {
            {
                return Some(value);
            }
            None
        })(),
    };
    return __flight_try_return.expect("TypeScript try/catch completed without returning");
}
