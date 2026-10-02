// @generated from upstream/packages/clipboard/src/clipboard.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_entity::{allocate_entity, finish_entity};
use flighthq_signals::{create_signal, emit_signal};
use flighthq_types::{
    CLIPBOARD_FORMAT_HTML as clipboard_format_html_constant,
    CLIPBOARD_FORMAT_RTF as clipboard_format_rtf_constant, ClipboardBookmark, ClipboardWatch,
    ClipboardWriteItem, EntityConstruction, HostClipboardBookmarkCapability,
    HostClipboardChangeCapability, HostClipboardFormatsCapability, HostClipboardImageCapability,
    HostClipboardTextCapability,
};

// Source: upstream/packages/clipboard/src/clipboard.ts:18 (sha256:cd1d492a676ddaa0241a02b7e71a5a01450236f3de36f3b9299f8c8187a7a418)
pub fn attach_clipboard_watch(
    host_clipboard_change: &HostClipboardChangeCapability,
    watch: ClipboardWatch,
) -> () {
    detach_clipboard_watch(&watch);
    {
        let __flight_key = (watch).clone();
        let __flight_value = {
            let __flight_callback = (host_clipboard_change.subscribe)
                .clone()
                .as_ref()
                .unwrap()
                .clone();
            let __flight_result = __flight_callback.lock().unwrap()(std::sync::Arc::new(
                std::sync::Mutex::new(Box::new({
                    let watch = watch.clone();
                    move || -> () { emit_signal((watch.on_change).clone(), ()) }
                })
                    as Box<dyn FnMut() -> () + Send + 'static>),
            ));
            __flight_result
        };
        if let Some((_, value)) = (*_WATCH_SUBSCRIPTIONS.lock().unwrap())
            .iter_mut()
            .find(|(key, _)| key == &__flight_key)
        {
            *value = __flight_value;
        } else {
            (*_WATCH_SUBSCRIPTIONS.lock().unwrap()).push((__flight_key, __flight_value));
        }
    };
}

// Source: upstream/packages/clipboard/src/clipboard.ts:30 (sha256:4cb15852d35e096b1137200cb711d4c7016a7a2a60970881e3cd17bc4072bf9e)
pub fn clear_clipboard(
    host_clipboard_text: &mut HostClipboardTextCapability,
) -> crate::FlightTask<bool> {
    return {
        let __flight_callback = (host_clipboard_text.clear).clone();
        let __flight_result = __flight_callback.lock().unwrap()();
        __flight_result
    };
}

// Source: upstream/packages/clipboard/src/clipboard.ts:36 (sha256:ea8b84e28ef40342d16cc18b00a8163fb9f05ab226f3d8b186186baa2b6d47d4)
pub fn create_clipboard_watch() -> ClipboardWatch {
    let mut out = allocate_entity();
    initialize_clipboard_watch((out).clone());
    return finish_entity((out).clone());
}

// Source: upstream/packages/clipboard/src/clipboard.ts:43 (sha256:ac847363940c8b4014a93a8ee4ae5ce938433a7a5039b54b59a8102d22092e61)
pub fn detach_clipboard_watch(watch: &ClipboardWatch) -> () {
    let unsubscribe = (*_WATCH_SUBSCRIPTIONS.lock().unwrap())
        .iter()
        .find(|(entry_key, _)| entry_key == &(*watch).clone())
        .map(|(_, value)| value.clone());
    if (unsubscribe).is_none() {
        return;
    }
    {
        let __flight_key = (*watch).clone();
        if let Some(__flight_index) = (*_WATCH_SUBSCRIPTIONS.lock().unwrap())
            .iter()
            .position(|(key, _)| key == &__flight_key)
        {
            (*_WATCH_SUBSCRIPTIONS.lock().unwrap()).remove(__flight_index);
            true
        } else {
            false
        }
    };
    {
        let __flight_callback = (unsubscribe.as_ref().unwrap()).clone();
        let __flight_result = __flight_callback.lock().unwrap()();
        __flight_result
    };
}

// Source: upstream/packages/clipboard/src/clipboard.ts:52 (sha256:daadc96d3083ee9961046e5efb99cf2b0d67f5ec7b0345d07f82421a28ac04f5)
pub fn dispose_clipboard_watch(watch: &ClipboardWatch) -> () {
    detach_clipboard_watch(watch);
}

// Source: upstream/packages/clipboard/src/clipboard.ts:57 (sha256:88ca7d20ecf433dfb7307b1add778feea1e49358de8a6edacdaaca85e94a7762)
pub fn get_clipboard_formats(
    host_clipboard_formats: &HostClipboardFormatsCapability,
) -> crate::FlightTask<Vec<String>> {
    return {
        let __flight_callback = (host_clipboard_formats.get_formats).clone();
        let __flight_result = __flight_callback.lock().unwrap()();
        __flight_result
    };
}

// Source: upstream/packages/clipboard/src/clipboard.ts:64 (sha256:b700e3630e71c316d0d99fd97149608f7f88489f5280124822cf845640a4c3c5)
pub fn has_clipboard_bookmark(
    host_clipboard_bookmark: HostClipboardBookmarkCapability,
) -> crate::FlightTask<bool> {
    crate::FlightTask::start(
        async move {
            return Ok((({
                let __flight_callback = (host_clipboard_bookmark.read_bookmark).clone();
                let __flight_result = __flight_callback.lock().unwrap()();
                __flight_result
            })
            .await?)
                .is_some());
        },
        crate::FlightTaskOrigin {
            package: "@flighthq/clipboard",
            source: "upstream/packages/clipboard/src/clipboard.ts",
            line: 64_u32,
            column: 1_u32,
            lexical_path: "hasClipboardBookmark",
            fingerprint: "sha256:b700e3630e71c316d0d99fd97149608f7f88489f5280124822cf845640a4c3c5",
        },
    )
}

// Source: upstream/packages/clipboard/src/clipboard.ts:71 (sha256:530973a16ef010bc76fcec59b96ac5a65a760513f19a0cd260b571a33beb7be8)
pub fn has_clipboard_format(
    host_clipboard_formats: &HostClipboardFormatsCapability,
    format: String,
) -> crate::FlightTask<bool> {
    return {
        let __flight_callback = (host_clipboard_formats.has_format).clone();
        let __flight_result = __flight_callback.lock().unwrap()((format).clone());
        __flight_result
    };
}

// Source: upstream/packages/clipboard/src/clipboard.ts:79 (sha256:cc68585417f5409d6de40f8f6633e316a336dfc398f96a1aff935cae8ca4f2bc)
pub fn has_clipboard_html(
    host_clipboard_formats: &HostClipboardFormatsCapability,
) -> crate::FlightTask<bool> {
    return {
        let __flight_callback = (host_clipboard_formats.has_format).clone();
        let __flight_result =
            __flight_callback.lock().unwrap()((clipboard_format_html_constant).to_owned());
        __flight_result
    };
}

// Source: upstream/packages/clipboard/src/clipboard.ts:84 (sha256:bc2f52d1cd01a3c40e29c1ae770d419c08d807ee6d649c768e1e40dbc108ef70)
pub fn has_clipboard_image(
    host_clipboard_image: &HostClipboardImageCapability,
) -> crate::FlightTask<bool> {
    return {
        let __flight_callback = (host_clipboard_image.has_image).clone();
        let __flight_result = __flight_callback.lock().unwrap()();
        __flight_result
    };
}

// Source: upstream/packages/clipboard/src/clipboard.ts:89 (sha256:3dac5d3a16095a186e350426872c4d539bad113fd8d4ea25c6263ff7a4fddabb)
pub fn has_clipboard_rtf(
    host_clipboard_formats: &HostClipboardFormatsCapability,
) -> crate::FlightTask<bool> {
    return {
        let __flight_callback = (host_clipboard_formats.has_format).clone();
        let __flight_result =
            __flight_callback.lock().unwrap()((clipboard_format_rtf_constant).to_owned());
        __flight_result
    };
}

// Source: upstream/packages/clipboard/src/clipboard.ts:94 (sha256:b51ae0d701fed05aad8cdd1a018fd5eb230d1d92644242ef42de9e9cab51157d)
pub fn has_clipboard_text(
    host_clipboard_text: &HostClipboardTextCapability,
) -> crate::FlightTask<bool> {
    return {
        let __flight_callback = (host_clipboard_text.has_text).clone();
        let __flight_result = __flight_callback.lock().unwrap()();
        __flight_result
    };
}

// Source: upstream/packages/clipboard/src/clipboard.ts:98 (sha256:41d74ed466ae0cf2eab709679ebd926beba93813f8590ea63878cb73eefe9fea)
pub fn initialize_clipboard_watch(out: EntityConstruction<ClipboardWatch>) -> () {
    crate::host_set("host.onChange", create_signal());
}

// Source: upstream/packages/clipboard/src/clipboard.ts:103 (sha256:e5ce9bf3ccf19eca3ce3c28c01da99ac986fc87bf4b2da74d9e92e60bdd00002)
pub fn read_clipboard(
    host_clipboard_formats: &HostClipboardFormatsCapability,
    formats: &Vec<String>,
) -> crate::FlightTask<Vec<(String, String)>> {
    return {
        let __flight_callback = (host_clipboard_formats.read_items).clone();
        let __flight_result = __flight_callback.lock().unwrap()((*formats).clone());
        __flight_result
    };
}

// Source: upstream/packages/clipboard/src/clipboard.ts:111 (sha256:98bdff27625ba8cf8200e556ada679c429b1b486391ee4762d6235af5aa49691)
pub fn read_clipboard_bookmark(
    host_clipboard_bookmark: &HostClipboardBookmarkCapability,
) -> crate::FlightTask<Option<ClipboardBookmark>> {
    return {
        let __flight_callback = (host_clipboard_bookmark.read_bookmark).clone();
        let __flight_result = __flight_callback.lock().unwrap()();
        __flight_result
    };
}

// Source: upstream/packages/clipboard/src/clipboard.ts:118 (sha256:bf81ff71f89d29197deb51d13f25670f3557822e5b0264fc22d962914a3bc991)
pub fn read_clipboard_format(
    host_clipboard_formats: &HostClipboardFormatsCapability,
    format: String,
) -> crate::FlightTask<String> {
    return {
        let __flight_callback = (host_clipboard_formats.read_format).clone();
        let __flight_result = __flight_callback.lock().unwrap()((format).clone());
        __flight_result
    };
}

// Source: upstream/packages/clipboard/src/clipboard.ts:126 (sha256:8a02b208af850f8782b4eefba8e66277c6e595f00df0426f3e2eb25e641ebf68)
pub fn read_clipboard_html(
    host_clipboard_formats: &HostClipboardFormatsCapability,
) -> crate::FlightTask<String> {
    return {
        let __flight_callback = (host_clipboard_formats.read_html).clone();
        let __flight_result = __flight_callback.lock().unwrap()();
        __flight_result
    };
}

// Source: upstream/packages/clipboard/src/clipboard.ts:131 (sha256:5ef9031b7212c78c44331e3c7d3e3ba3b6695af4d83ddfeb6118384b056cdb43)
pub fn read_clipboard_image(
    host_clipboard_image: &HostClipboardImageCapability,
) -> crate::FlightTask<String> {
    return {
        let __flight_callback = (host_clipboard_image.read_image).clone();
        let __flight_result = __flight_callback.lock().unwrap()();
        __flight_result
    };
}

// Source: upstream/packages/clipboard/src/clipboard.ts:136 (sha256:67a070eed53fc806279e76688811abe63fc2f53cf729d0504a7d381336d9dca1)
pub fn read_clipboard_rtf(
    host_clipboard_formats: &HostClipboardFormatsCapability,
) -> crate::FlightTask<String> {
    return {
        let __flight_callback = (host_clipboard_formats.read_rtf).clone();
        let __flight_result = __flight_callback.lock().unwrap()();
        __flight_result
    };
}

// Source: upstream/packages/clipboard/src/clipboard.ts:141 (sha256:76d951432e9ac772dbfdaad70709cde873db0866528be5aa72695a4a19e846c0)
pub fn read_clipboard_text(
    host_clipboard_text: &HostClipboardTextCapability,
) -> crate::FlightTask<String> {
    return {
        let __flight_callback = (host_clipboard_text.read_text).clone();
        let __flight_result = __flight_callback.lock().unwrap()();
        __flight_result
    };
}

// Source: upstream/packages/clipboard/src/clipboard.ts:146 (sha256:32844108c0ddfe5b16a26d78e8e285e2cd56b5ba4e3acf666c6d548e08c144c0)
pub fn write_clipboard(
    host_clipboard_formats: &HostClipboardFormatsCapability,
    items: &Vec<ClipboardWriteItem>,
) -> crate::FlightTask<bool> {
    return {
        let __flight_callback = (host_clipboard_formats.write_items).clone();
        let __flight_result = __flight_callback.lock().unwrap()((*items).clone());
        __flight_result
    };
}

// Source: upstream/packages/clipboard/src/clipboard.ts:154 (sha256:b1d8baab691e7360fa25dbbbd90d68aa2e9a037ed1a049f951578bd378bcabb6)
pub fn write_clipboard_bookmark(
    host_clipboard_bookmark: &HostClipboardBookmarkCapability,
    title: String,
    url: String,
) -> crate::FlightTask<bool> {
    return {
        let __flight_callback = (host_clipboard_bookmark.write_bookmark).clone();
        let __flight_result = __flight_callback.lock().unwrap()((title).clone(), (url).clone());
        __flight_result
    };
}

// Source: upstream/packages/clipboard/src/clipboard.ts:163 (sha256:41fa3cb4e9c4e29b6bb7624a3fee7f4f63fe0020291ff50be50af360fdbf9489)
pub fn write_clipboard_format(
    host_clipboard_formats: &HostClipboardFormatsCapability,
    format: String,
    data: String,
) -> crate::FlightTask<bool> {
    return {
        let __flight_callback = (host_clipboard_formats.write_format).clone();
        let __flight_result = __flight_callback.lock().unwrap()((format).clone(), (data).clone());
        __flight_result
    };
}

// Source: upstream/packages/clipboard/src/clipboard.ts:172 (sha256:e6a445669e47197bb55fb8ab3f127d6b13e019bf8c7d0ae5491bc3b90c7e9261)
pub fn write_clipboard_html(
    host_clipboard_formats: &HostClipboardFormatsCapability,
    html: String,
) -> crate::FlightTask<bool> {
    return {
        let __flight_callback = (host_clipboard_formats.write_html).clone();
        let __flight_result = __flight_callback.lock().unwrap()((html).clone());
        __flight_result
    };
}

// Source: upstream/packages/clipboard/src/clipboard.ts:180 (sha256:e916d22e1083fb0dab3141559ffe373c0bde71492803529ebe9e7ca4fba206e3)
pub fn write_clipboard_image(
    host_clipboard_image: &HostClipboardImageCapability,
    data_url: String,
) -> crate::FlightTask<bool> {
    return {
        let __flight_callback = (host_clipboard_image.write_image).clone();
        let __flight_result = __flight_callback.lock().unwrap()((data_url).clone());
        __flight_result
    };
}

// Source: upstream/packages/clipboard/src/clipboard.ts:188 (sha256:96755741ce06ad88f4f28046eca6c04e719e5e3788dd4036228f1d3cf2612d6b)
pub fn write_clipboard_rtf(
    host_clipboard_formats: &HostClipboardFormatsCapability,
    rtf: String,
) -> crate::FlightTask<bool> {
    return {
        let __flight_callback = (host_clipboard_formats.write_rtf).clone();
        let __flight_result = __flight_callback.lock().unwrap()((rtf).clone());
        __flight_result
    };
}

// Source: upstream/packages/clipboard/src/clipboard.ts:196 (sha256:e94fe9342e9a7b1b78bf141bc21e844514deb82a93a19dd33fcf93e6ff11257b)
pub fn write_clipboard_text(
    host_clipboard_text: &HostClipboardTextCapability,
    text: String,
) -> crate::FlightTask<bool> {
    return {
        let __flight_callback = (host_clipboard_text.write_text).clone();
        let __flight_result = __flight_callback.lock().unwrap()((text).clone());
        __flight_result
    };
}

// Source: upstream/packages/clipboard/src/clipboard.ts:205 (sha256:25bbebc3fd8ca429f2e8ea5bcc4533ed77fc37b83f9845691077bb21c63e2a50)
static _WATCH_SUBSCRIPTIONS: std::sync::LazyLock<
    std::sync::Mutex<
        Vec<(
            ClipboardWatch,
            std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>,
        )>,
    >,
> = std::sync::LazyLock::new(|| std::sync::Mutex::new(Vec::new()));
