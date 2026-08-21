// @generated from upstream/packages/protocol/src/protocol.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_signals::{create_signal, emit_signal};
use flighthq_types::{ParsedProtocolUrl, ProtocolBackend, ProtocolHandler};

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
pub struct FlightPartialRecord1 {
    pub __flight_identity: std::sync::Arc<()>,
    pub scheme: Option<String>,
    pub host: Option<String>,
    pub path: Option<String>,
    pub query: Option<Vec<(String, String)>>,
}
impl PartialEq for FlightPartialRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/protocol/src/protocol.ts:9 (sha256:00af5c866b81c185220d029c7614ebc4e769a8032fdb5b6ba44dbb8b0538131e)
pub fn attach_protocol_handler(handler: ProtocolHandler) -> () {
    detach_protocol_handler(&handler);
    let backend = get_protocol_backend();
    let pending = {
        let __flight_callback = (backend.drain_pending_urls).clone();
        let __flight_result = __flight_callback.lock().unwrap()();
        __flight_result
    };
    for url in (pending).iter().cloned() {
        emit_signal((handler.on_open_url).clone(), ((url).clone(),));
    }
    let unsubscribe = {
        let __flight_callback = (backend.subscribe).clone();
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
    {
        let __flight_key = (handler).clone();
        let __flight_value = (unsubscribe).clone();
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

// Source: upstream/packages/protocol/src/protocol.ts:23 (sha256:e576424f2209c08fb6116c26a4c53797b4c5e23ff3570992d38c28879eb1bf13)
pub fn create_protocol_handler() -> ProtocolHandler {
    return ProtocolHandler {
        __flight_identity: std::sync::Arc::new(()),
        on_open_url: create_signal(),
    };
}

// Source: upstream/packages/protocol/src/protocol.ts:30 (sha256:26195253814f0fc0b4a2631049f6906ebedecb01aebec722307c617dd34640c9)
pub fn create_protocol_url(parts: &FlightPartialRecord1) -> String {
    let scheme = ((parts.scheme).clone())
        .clone()
        .unwrap_or("unknown".to_owned());
    let host = ((parts.host).clone()).clone().unwrap_or("".to_owned());
    let path = ((parts.path).clone()).clone().unwrap_or("".to_owned());
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
                let k = __parameter0.0.clone();
                return ((k.encode_utf16().count() as f64) > 0.0_f64);
            };
            (((query.as_ref().unwrap()).clone()).clone())
                .iter()
                .cloned()
                .filter(|value| __flight_filter(value.clone()))
                .collect::<Vec<_>>()
        };
        if ((entries.len() as f64) > 0.0_f64) {
            let qs = ((entries)
                .iter()
                .cloned()
                .map(|__parameter1: (String, String)| -> String {
                    let k = __parameter1.0.clone();
                    let v = __parameter1.1.clone();
                    return format!(
                        "{}={}",
                        __flight_encode_uri_component(&((k).clone())),
                        __flight_encode_uri_component(&((v).clone()))
                    );
                })
                .collect::<Vec<_>>())
            .iter()
            .map(|value| value.to_string())
            .collect::<Vec<_>>()
            .join(("&".to_owned()).as_str());
            url.push_str(&(format!("?{}", (qs).clone())));
        }
    }
    return url;
}

// Source: upstream/packages/protocol/src/protocol.ts:50 (sha256:23a54378b0ad4aeec1e23686d4d8957a1537cb9077984fd0ded58015c5f30f0c)
#[derive(Clone, Default)]
struct CreateWebProtocolBackendRecord2 {
    __flight_identity: std::sync::Arc<()>,
    register_protocol_handler: Option<
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(String, String) -> () + Send + 'static>>>,
    >,
}
impl PartialEq for CreateWebProtocolBackendRecord2 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn create_web_protocol_backend() -> ProtocolBackend {
    let _registered_schemes: std::sync::Arc<std::sync::Mutex<Vec<String>>> =
        std::sync::Arc::new(std::sync::Mutex::new(vec![]));
    return ProtocolBackend {
        __flight_identity: std::sync::Arc::new(()),
        register: std::sync::Arc::new(std::sync::Mutex::new(Box::new({
            let mut _registered_schemes = _registered_schemes.clone();
            move |scheme: String| -> bool {
                return false;
            }
        })
            as Box<dyn FnMut(String) -> bool + Send + 'static>)),
        unregister: std::sync::Arc::new(std::sync::Mutex::new(Box::new({
            let mut _registered_schemes = _registered_schemes.clone();
            move |scheme: String| -> bool {
                let idx = {
                    let __flight_value = (scheme).clone();
                    ((*_registered_schemes.lock().unwrap()).clone())
                        .iter()
                        .position(|item| item == &__flight_value)
                        .map_or(-1.0_f64, |index| index as f64)
                };
                if (idx >= 0.0_f64) {
                    (*_registered_schemes.lock().unwrap())
                        .splice((idx) as usize..((idx) + (1.0_f64)) as usize, vec![]);
                }
                return false;
            }
        })
            as Box<dyn FnMut(String) -> bool + Send + 'static>)),
        is_registered: std::sync::Arc::new(std::sync::Mutex::new(Box::new(
            move |__flight_unused_0: String| -> bool {
                return false;
            },
        )
            as Box<dyn FnMut(String) -> bool + Send + 'static>)),
        get_registered_schemes: std::sync::Arc::new(std::sync::Mutex::new(Box::new({
            let mut _registered_schemes = _registered_schemes.clone();
            move || -> Vec<String> {
                return ((*_registered_schemes.lock().unwrap()).clone()).clone();
            }
        })
            as Box<dyn FnMut() -> Vec<String> + Send + 'static>)),
        set_as_default: std::sync::Arc::new(std::sync::Mutex::new(Box::new(
            move |__flight_unused_0: String| -> bool {
                return false;
            },
        )
            as Box<dyn FnMut(String) -> bool + Send + 'static>)),
        is_default: std::sync::Arc::new(std::sync::Mutex::new(Box::new(
            move |__flight_unused_0: String| -> bool {
                return false;
            },
        )
            as Box<dyn FnMut(String) -> bool + Send + 'static>)),
        remove_as_default: std::sync::Arc::new(std::sync::Mutex::new(Box::new(
            move |__flight_unused_0: String| -> bool {
                return false;
            },
        )
            as Box<dyn FnMut(String) -> bool + Send + 'static>)),
        get_launch_url: std::sync::Arc::new(std::sync::Mutex::new(Box::new(
            move || -> Option<String> {
                return None;
            },
        )
            as Box<dyn FnMut() -> Option<String> + Send + 'static>)),
        drain_pending_urls: std::sync::Arc::new(std::sync::Mutex::new(Box::new(
            move || -> Vec<String> {
                return vec![];
            },
        )
            as Box<dyn FnMut() -> Vec<String> + Send + 'static>)),
        subscribe: std::sync::Arc::new(std::sync::Mutex::new(
            Box::new(
                move |__flight_unused_0: std::sync::Arc<
                    std::sync::Mutex<Box<dyn FnMut(String) -> () + Send + 'static>>,
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
                                std::sync::Mutex<Box<dyn FnMut(String) -> () + Send + 'static>>,
                            >,
                        ) -> std::sync::Arc<
                            std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>,
                        > + Send
                        + 'static,
                >,
        )),
    };
}

// Source: upstream/packages/protocol/src/protocol.ts:118 (sha256:4f8413b9f63b4733b6aa719f7a29654c130cafaa46c68153b236a88c5ef73213)
pub fn detach_protocol_handler(handler: &ProtocolHandler) -> () {
    let unsubscribe = (*_SUBSCRIPTIONS.lock().unwrap())
        .iter()
        .find(|(entry_key, _)| entry_key == &(*handler).clone())
        .map(|(_, value)| value.clone());
    if (unsubscribe).is_some() {
        {
            let __flight_callback = (unsubscribe.as_ref().unwrap()).clone();
            let __flight_result = __flight_callback.lock().unwrap()();
            __flight_result
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
}

// Source: upstream/packages/protocol/src/protocol.ts:128 (sha256:8b3a79b2fb43d395e5e9483e2947ca946b5588bc42866c1037e134842a55e6d9)
pub fn dispose_protocol_handler(handler: &ProtocolHandler) -> () {
    detach_protocol_handler(handler);
}

// Source: upstream/packages/protocol/src/protocol.ts:133 (sha256:05e3172532be0192af3ce44982e0bc665306a2cb26b10718ab2544d8629658b5)
pub fn get_protocol_backend() -> ProtocolBackend {
    if ((*_BACKEND.lock().unwrap()).clone()).is_none() {
        (*_BACKEND.lock().unwrap()) = Some(create_web_protocol_backend());
    }
    return (((*_BACKEND.lock().unwrap()).clone()).clone().unwrap()).clone();
}

// Source: upstream/packages/protocol/src/protocol.ts:141 (sha256:0762b424a8b78b6e5cd68f89e4140b5e582878100a05f539bc7e755f8957df5f)
pub fn get_protocol_launch_url() -> Option<String> {
    return {
        let __flight_callback = (get_protocol_backend().get_launch_url).clone();
        let __flight_result = __flight_callback.lock().unwrap()();
        __flight_result
    };
}

// Source: upstream/packages/protocol/src/protocol.ts:147 (sha256:bf592a399a5a84a855f012a9e3ccc9c9c59cf4e4a8e473f2a5726975c98d47e9)
pub fn get_registered_protocol_schemes() -> Vec<String> {
    return {
        let __flight_callback = (get_protocol_backend().get_registered_schemes).clone();
        let __flight_result = __flight_callback.lock().unwrap()();
        __flight_result
    };
}

// Source: upstream/packages/protocol/src/protocol.ts:153 (sha256:0d0b40650511cd9d73a5ea7e079550c89c9b7c91a19de5092623aa920d9dd992)
pub fn is_protocol_scheme_default(scheme: String) -> bool {
    return {
        let __flight_callback = (get_protocol_backend().is_default).clone();
        let __flight_result = __flight_callback.lock().unwrap()((scheme).clone());
        __flight_result
    };
}

// Source: upstream/packages/protocol/src/protocol.ts:158 (sha256:22de5027235039e5ca9d5e26a0ddbe8b237e06138231d3385511d1a622a07f4d)
pub fn is_protocol_scheme_registered(scheme: String) -> bool {
    return {
        let __flight_callback = (get_protocol_backend().is_registered).clone();
        let __flight_result = __flight_callback.lock().unwrap()((scheme).clone());
        __flight_result
    };
}

// Source: upstream/packages/protocol/src/protocol.ts:166 (sha256:6ae6c76401417f2cfe6ec13b993ab35cbfaf6007e2a07e1fba3d42ee474cbdfd)
pub fn is_valid_protocol_scheme(scheme: String) -> bool {
    if ("string".to_owned() != "string") || ((scheme.encode_utf16().count() as f64) == 0.0_f64) {
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

// Source: upstream/packages/protocol/src/protocol.ts:177 (sha256:842271fdf1801daed7958666e5bdde5e70b53dae56d7e560c3bbdd31f571bc8c)
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
    if ("string".to_owned() != "string") || ((url.encode_utf16().count() as f64) == 0.0_f64) {
        return None;
    }
    let colon_idx = __flight_string_index_of(&(url), &(":".to_owned()), 0.0_f64);
    if (colon_idx <= 0.0_f64) {
        return None;
    }
    let scheme = (__flight_string_slice(&(url), 0.0_f64, Some(colon_idx))).to_lowercase();
    if (!((*_SCHEME_PATTERN).clone()).is_match(&((scheme).clone()))) {
        return None;
    }
    let mut rest = __flight_string_slice(&(url), (colon_idx + 1.0_f64), None);
    let mut host = "".to_owned();
    if (rest).starts_with(("//".to_owned()).as_str()) {
        rest = __flight_string_slice(&(rest), 2.0_f64, None);
        let slash_idx = __flight_string_index_of(&(rest), &("/".to_owned()), 0.0_f64);
        let q_idx = __flight_string_index_of(&(rest), &("?".to_owned()), 0.0_f64);
        let mut host_end: f64;
        if (slash_idx >= 0.0_f64) && ((q_idx < 0.0_f64) || (slash_idx < q_idx)) {
            host_end = slash_idx;
        } else {
            if (q_idx >= 0.0_f64) {
                host_end = q_idx;
            } else {
                host_end = (rest.encode_utf16().count() as f64);
            }
        }
        host = __flight_string_slice(&(rest), 0.0_f64, Some(host_end));
        rest = __flight_string_slice(&(rest), host_end, None);
    }
    let q_idx = __flight_string_index_of(&(rest), &("?".to_owned()), 0.0_f64);
    let mut path: String;
    let mut query_string: String;
    if (q_idx >= 0.0_f64) {
        path = __flight_string_slice(&(rest), 0.0_f64, Some(q_idx));
        query_string = __flight_string_slice(&(rest), (q_idx + 1.0_f64), None);
    } else {
        path = (rest).clone();
        query_string = "".to_owned();
    }
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
            let eq_idx = __flight_string_index_of(&((pair).clone()), &("=".to_owned()), 0.0_f64);
            if (eq_idx < 0.0_f64) {
                let k = _safe_decode((pair).clone());
                if ((k.encode_utf16().count() as f64) > 0.0_f64) {
                    {
                        let __flight_key = (k).clone();
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
                let k = _safe_decode(__flight_string_slice(
                    &((pair).clone()),
                    0.0_f64,
                    Some(eq_idx),
                ));
                if ((k.encode_utf16().count() as f64) > 0.0_f64) {
                    {
                        let __flight_key = (k).clone();
                        let __flight_value = _safe_decode(__flight_string_slice(
                            &((pair).clone()),
                            (eq_idx + 1.0_f64),
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
        scheme: (scheme).clone(),
        host: (host).clone(),
        path: (path).clone(),
        query: (query).clone(),
    });
}

// Source: upstream/packages/protocol/src/protocol.ts:237 (sha256:3dcca0999a40c4496c5c9439541593ccee6e40da9d953c102519d8012b9fb71f)
pub fn register_protocol_scheme(scheme: String) -> bool {
    if (!is_valid_protocol_scheme((scheme).clone())) {
        return false;
    }
    return {
        let __flight_callback = (get_protocol_backend().register).clone();
        let __flight_result = __flight_callback.lock().unwrap()((scheme).clone());
        __flight_result
    };
}

// Source: upstream/packages/protocol/src/protocol.ts:244 (sha256:c7f672589962c3ce702a40c7fd0be2c8421dfecece76332726c783a8a7b68096)
pub fn register_protocol_schemes(schemes: &Vec<String>) -> bool {
    let backend = get_protocol_backend();
    let mut all_ok = true;
    for scheme in (schemes).iter().cloned() {
        if (!is_valid_protocol_scheme((scheme).clone()))
            || (!{
                let __flight_callback = (backend.register).clone();
                let __flight_result = __flight_callback.lock().unwrap()((scheme).clone());
                __flight_result
            })
        {
            all_ok = false;
        }
    }
    return all_ok;
}

// Source: upstream/packages/protocol/src/protocol.ts:255 (sha256:92beb2ec31bde884f3c58c188e129e48f76ca9f9e052372fded52982679d2bfb)
pub fn remove_protocol_scheme_as_default(scheme: String) -> bool {
    return {
        let __flight_callback = (get_protocol_backend().remove_as_default).clone();
        let __flight_result = __flight_callback.lock().unwrap()((scheme).clone());
        __flight_result
    };
}

// Source: upstream/packages/protocol/src/protocol.ts:260 (sha256:18f3370317e900c9f55789f988059c5cb157aac2ab84cb754b19c6803890de8f)
pub fn set_protocol_backend(backend: &Option<ProtocolBackend>) -> () {
    (*_BACKEND.lock().unwrap()) = (*backend).clone();
}

// Source: upstream/packages/protocol/src/protocol.ts:266 (sha256:c2508dec8c18d102a4667af00aeef82119709bda578702d181d548a58f8bac6b)
pub fn set_protocol_scheme_as_default(scheme: String) -> bool {
    return {
        let __flight_callback = (get_protocol_backend().set_as_default).clone();
        let __flight_result = __flight_callback.lock().unwrap()((scheme).clone());
        __flight_result
    };
}

// Source: upstream/packages/protocol/src/protocol.ts:272 (sha256:e5d079839f7f59eeef27f75636f8896b1a9471bb141af5675d12f401da0271c8)
pub fn unregister_protocol_scheme(scheme: String) -> bool {
    return {
        let __flight_callback = (get_protocol_backend().unregister).clone();
        let __flight_result = __flight_callback.lock().unwrap()((scheme).clone());
        __flight_result
    };
}

// Source: upstream/packages/protocol/src/protocol.ts:277 (sha256:78c3a6443380f019b5332f223a41d35d275af899124f34c0631cf27894628bb0)
pub fn unregister_protocol_schemes(schemes: &Vec<String>) -> bool {
    let backend = get_protocol_backend();
    let mut all_ok = true;
    for scheme in (schemes).iter().cloned() {
        if (!{
            let __flight_callback = (backend.unregister).clone();
            let __flight_result = __flight_callback.lock().unwrap()((scheme).clone());
            __flight_result
        }) {
            all_ok = false;
        }
    }
    return all_ok;
}

// Source: upstream/packages/protocol/src/protocol.ts:287 (sha256:4b3a5a97814daa1d957dc05d78fa3b7007a00de8f5b8d5b8b254db4d09132c32)
static _SCHEME_PATTERN: std::sync::LazyLock<regex::Regex> = std::sync::LazyLock::new(|| {
    regex::RegexBuilder::new("^[a-z][a-z0-9+\\-.]*$")
        .case_insensitive(false)
        .multi_line(false)
        .dot_matches_new_line(false)
        .build()
        .expect("upstream TypeScript regular expression must be valid Rust regex syntax")
});

// Source: upstream/packages/protocol/src/protocol.ts:290 (sha256:f0179456dc2c7e6df3db60520cbbe54131d861d67bd5a5111d7f0dc937fcf82d)
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

// Source: upstream/packages/protocol/src/protocol.ts:292 (sha256:4e64113d99273aab4a45879563e102b62bdd03b5928a650c6d5e4e0d5e602b5b)
static _BACKEND: std::sync::LazyLock<std::sync::Mutex<Option<ProtocolBackend>>> =
    std::sync::LazyLock::new(|| std::sync::Mutex::new(None));

// Source: upstream/packages/protocol/src/protocol.ts:293 (sha256:f548928043116bccfcc87d35b6db0ba05b01ccea6779e0b9c2bd08df6bf61fd4)
static _SUBSCRIPTIONS: std::sync::LazyLock<
    std::sync::Mutex<
        Vec<(
            ProtocolHandler,
            std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>,
        )>,
    >,
> = std::sync::LazyLock::new(|| std::sync::Mutex::new(Vec::new()));

// Source: upstream/packages/protocol/src/protocol.ts:295 (sha256:1b4a595b0c64eed2d1e67c5bf52a674fc728e97e36d0e73d2b1dc74c58f18f18)
fn _safe_decode(s: String) -> String {
    let __flight_try_return: Option<String> = match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| -> Option<String> {
            {
                return Some(__flight_decode_uri_component(&((regex::RegexBuilder::new("\\+").case_insensitive(false).multi_line(false).dot_matches_new_line(false).build().expect("upstream TypeScript regular expression must be valid Rust regex syntax")).replace_all(&(s), " ".to_owned()).into_owned())));
            }
            None
        }),
    ) {
        Ok(value) => value,
        Err(_) => (|| -> Option<String> {
            {
                return Some(s);
            }
            None
        })(),
    };
    return __flight_try_return.expect("TypeScript try/catch completed without returning");
}
