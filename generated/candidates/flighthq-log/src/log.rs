// @generated from upstream/packages/log/src/log.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_signals::{create_signal, emit_signal};
use flighthq_types::{
    BufferedLogSink, FileLogSink, LogContext, LogData, LogDataProvider, LogEntry, LogFormatter,
    LogLevel, LogSignals, LogSink, LogSpan, LogTimer, LogTransportBackend, MemoryLogSink,
    RateLimitedLogSink,
};

#[inline]

fn __flight_number_to_fixed(value: f64, digits: f64) -> String {
    assert!(
        digits.is_finite() && digits.fract() == 0.0_f64 && (0.0_f64..=100.0_f64).contains(&digits),
        "Number.toFixed digits must be between 0 and 100"
    );
    if value.is_nan() {
        return "NaN".to_owned();
    }
    if value == f64::INFINITY {
        return "Infinity".to_owned();
    }
    if value == f64::NEG_INFINITY {
        return "-Infinity".to_owned();
    }
    let value = if value == 0.0_f64 { 0.0_f64 } else { value };
    format!("{:.*}", digits as usize, value)
}

#[inline]

fn __flight_string_repeat(value: &str, count: f64) -> String {
    assert!(
        count.is_finite() && count >= 0.0_f64,
        "String.repeat count must be finite and non-negative"
    );
    value.repeat(count.trunc() as usize)
}

#[derive(Clone, Default)]
pub struct SharedStructuralRecord1 {
    pub __flight_identity: std::sync::Arc<()>,
    pub size: Option<f64>,
    pub interval_ms: Option<f64>,
}
impl PartialEq for SharedStructuralRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct SharedStructuralRecord2 {
    pub __flight_identity: std::sync::Arc<()>,
    pub formatter: Option<LogFormatter>,
}
impl PartialEq for SharedStructuralRecord2 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct SharedStructuralRecord3 {
    pub __flight_identity: std::sync::Arc<()>,
    pub per_channel: Option<bool>,
    pub max_per_interval: f64,
    pub interval_ms: f64,
}
impl PartialEq for SharedStructuralRecord3 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct SharedStructuralRecord4 {
    pub __flight_identity: std::sync::Arc<()>,
    pub indent_groups: Option<bool>,
    pub level_prefix: Option<bool>,
    pub timestamp: Option<bool>,
}
impl PartialEq for SharedStructuralRecord4 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/log/src/log.ts:34 (sha256:34450be1d7c3fb6b2ac86b159ae896ee3115d2c99debe6a8e22e68795d65933a)
pub fn add_log_sink(sink: LogSink) -> () {
    if {
        let __flight_value = (sink).clone();
        (_SINKS.lock().unwrap())
            .iter()
            .any(|item| std::sync::Arc::ptr_eq(item, &__flight_value))
    } {
        return;
    }
    _SINKS.lock().unwrap().push(((sink).clone()).clone());
}

// Source: upstream/packages/log/src/log.ts:42 (sha256:f3e0592dd7ef366c458cba8c8d1327288f2616be8d82712e38fc913b012da634)
pub fn begin_log_group(label: String, channel: Option<String>) -> () {
    {
        (*_GROUP_DEPTH.lock().unwrap()) += 1.0;
        (*_GROUP_DEPTH.lock().unwrap())
    };
    if (!_passes_level_gate(LogLevel::Debug, ((channel).clone()).clone())) {
        return;
    }
    _emit_to_sinks(&LogEntry {
        __flight_identity: std::sync::Arc::new(()),
        level: LogLevel::Debug,
        channel: (channel).clone(),
        data: flighthq_types::LogData::B({
            let mut __flight_record = Vec::new();
            __flight_record.push(("msg".to_owned(), {
                let __flight_portable_source = (label).clone();
                crate::FlightValue::String((&__flight_portable_source).clone())
            }));
            __flight_record.push(("group".to_owned(), {
                let __flight_portable_source = "begin".to_owned();
                crate::FlightValue::String((&__flight_portable_source).clone())
            }));
            __flight_record.push(("depth".to_owned(), {
                let __flight_portable_source = (*_GROUP_DEPTH.lock().unwrap()).clone();
                crate::FlightValue::Number(*(&__flight_portable_source) as f64)
            }));
            __flight_record
        }),
    });
}

// Source: upstream/packages/log/src/log.ts:49 (sha256:bbab9f6f05c874d2ed77ce5eaaf2e802f76299f5a3540ec85b02ee3df3638973)
pub fn clear_log_channel_levels() -> () {
    (*_CHANNEL_LEVELS.lock().unwrap()).clear();
}

// Source: upstream/packages/log/src/log.ts:55 (sha256:675a4640247241705bfcaba7f8a9b0f6f978ed4ff2d2aebcd1c38042571877f5)
pub fn clear_log_groups() -> () {
    (*_GROUP_DEPTH.lock().unwrap()) = 0.0_f64;
}

// Source: upstream/packages/log/src/log.ts:61 (sha256:416743460c28682c91c7f97721c7bb000a4e3fe7f8f0271e176fe6e7121a3152)
pub fn clear_log_once_keys() -> () {
    (*_ONCE_KEYS.lock().unwrap()).clear();
}

// Source: upstream/packages/log/src/log.ts:66 (sha256:91951b5221d256f87b4d5f03330767a57a7e5a99110b3549cfa2aa23360416b7)
pub fn clear_log_redaction_paths() -> () {
    _REDACTION_PATHS.lock().unwrap().clear();
}

// Source: upstream/packages/log/src/log.ts:71 (sha256:c1e9b460b86435c0be9b8e07faea99cf7718d1b531ea3fed81582dcd1501b307)
pub fn clear_log_serializers() -> () {
    (*_SERIALIZERS.lock().unwrap()).clear();
}

// Source: upstream/packages/log/src/log.ts:76 (sha256:bac4c1c076df20e95fa1d0c40383238f711cbacc98116ffcd78d8af4214f3346)
pub fn clear_log_sinks() -> () {
    _SINKS.lock().unwrap().clear();
}

// Source: upstream/packages/log/src/log.ts:81 (sha256:89cf20d90874d9d0d92424c2f62fd7a6897c4feeb8042ae491c1bc74b919e5ca)
pub fn clear_memory_log_sink(handle: &MemoryLogSink) -> () {
    let mut state = (*_MEMORY_SINK_STATES.lock().unwrap())
        .iter()
        .find(|(entry_key, _)| entry_key == &(*handle).clone())
        .map(|(_, value)| value.clone());
    if (state).is_none() {
        return;
    }
    state.as_mut().unwrap().buf.clear();
    state.as_mut().unwrap().head = 0.0_f64;
}

// Source: upstream/packages/log/src/log.ts:92 (sha256:5770641e75d944ce595a14f878d4c8ca86e3c23c02e3dfb54882e6da0c9f9656)
#[derive(Clone, Default)]
struct CreateBufferedLogSinkRecord5 {
    __flight_identity: std::sync::Arc<()>,
}
impl PartialEq for CreateBufferedLogSinkRecord5 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn create_buffered_log_sink(
    target: LogSink,
    options: Option<SharedStructuralRecord1>,
) -> BufferedLogSink {
    let __flight_forward_handle: std::sync::Arc<std::sync::Mutex<Option<BufferedLogSink>>> =
        std::sync::Arc::new(std::sync::Mutex::new(None));
    let options = options.unwrap_or(SharedStructuralRecord1 {
        __flight_identity: std::sync::Arc::new(()),
        size: None,
        interval_ms: None,
    });
    let size = (options.size).clone().unwrap_or(100.0_f64);
    let interval_ms = (options.interval_ms).clone().unwrap_or(1000.0_f64);
    let mut flush: std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>> =
        std::sync::Arc::new(std::sync::Mutex::new(Box::new({
            let __flight_forward_handle = __flight_forward_handle.clone();
            let target = target.clone();
            move || -> () {
                let mut state = (*_BUFFERED_SINK_STATES.lock().unwrap())
                    .iter()
                    .find(|(entry_key, _)| {
                        entry_key
                            == &(__flight_forward_handle
                                .lock()
                                .unwrap()
                                .as_ref()
                                .unwrap()
                                .clone())
                            .clone()
                    })
                    .map(|(_, value)| value.clone());
                if ((state.as_mut().unwrap().buf.len() as f64) == 0.0_f64) {
                    return;
                }
                let batch = {
                    let __flight_start = (0.0_f64);
                    let __flight_count =
                        ((state.as_mut().unwrap().buf.len() as f64) - __flight_start);
                    state
                        .as_mut()
                        .unwrap()
                        .buf
                        .splice(
                            (__flight_start) as usize..(__flight_start + __flight_count) as usize,
                            vec![],
                        )
                        .collect::<Vec<_>>()
                };
                for entry in (batch).iter().cloned() {
                    {
                        let __flight_callback = (target).clone();
                        let __flight_result = __flight_callback.lock().unwrap()(entry);
                        __flight_result
                    };
                }
            }
        })
            as Box<dyn FnMut() -> () + Send + 'static>));
    let mut sink: LogSink = std::sync::Arc::new(std::sync::Mutex::new(Box::new({
        let __flight_forward_handle = __flight_forward_handle.clone();
        let flush = flush.clone();
        move |entry: LogEntry| -> () {
            let mut state = (*_BUFFERED_SINK_STATES.lock().unwrap())
                .iter()
                .find(|(entry_key, _)| {
                    entry_key
                        == &(__flight_forward_handle
                            .lock()
                            .unwrap()
                            .as_ref()
                            .unwrap()
                            .clone())
                        .clone()
                })
                .map(|(_, value)| value.clone());
            state.as_mut().unwrap().buf.push(LogEntry {
                __flight_identity: std::sync::Arc::new(()),
                level: entry.level,
                channel: (entry.channel).clone(),
                data: (entry.data).clone(),
            });
            if ((state.as_mut().unwrap().buf.len() as f64) >= size) {
                {
                    let __flight_callback = (flush).clone();
                    let __flight_result = __flight_callback.lock().unwrap()();
                    __flight_result
                };
            }
        }
    })
        as Box<dyn FnMut(LogEntry) -> () + Send + 'static>));
    let handle: BufferedLogSink = BufferedLogSink {
        __flight_identity: std::sync::Arc::new(()),
        sink: (sink).clone(),
    };
    *__flight_forward_handle.lock().unwrap() = Some(handle.clone());
    let mut timer: Option<crate::FlightTimeout> = None;
    {
        let __flight_key = (__flight_forward_handle
            .lock()
            .unwrap()
            .as_ref()
            .unwrap()
            .clone())
        .clone();
        let __flight_value = BufferedLogSinkState {
            __flight_identity: std::sync::Arc::new(()),
            buf: vec![],
            timer: (timer).clone(),
            flush: (flush).clone(),
        };
        if let Some((_, value)) = (*_BUFFERED_SINK_STATES.lock().unwrap())
            .iter_mut()
            .find(|(key, _)| key == &__flight_key)
        {
            *value = __flight_value;
        } else {
            (*_BUFFERED_SINK_STATES.lock().unwrap()).push((__flight_key, __flight_value));
        }
    };
    return __flight_forward_handle
        .lock()
        .unwrap()
        .as_ref()
        .unwrap()
        .clone();
}

// Source: upstream/packages/log/src/log.ts:123 (sha256:a84b6cbb9aa2f7ebc2c6d0477d4cfd9d03069f96d80c24b4f3d09470fd8760d1)
pub fn create_child_log_context(
    parent: &LogContext,
    fields: &Vec<(String, crate::FlightValue)>,
    channel: Option<String>,
) -> LogContext {
    let merged = {
        let mut __flight_record = Vec::new();
        let __flight_spread_0 = (parent.fields).clone();
        for (__flight_key, __flight_value) in __flight_spread_0.iter().cloned() {
            if let Some((_, __flight_existing)) = __flight_record
                .iter_mut()
                .find(|(key, _)| key == &__flight_key)
            {
                *__flight_existing = __flight_value;
            } else {
                __flight_record.push((__flight_key, __flight_value));
            }
        }
        let __flight_spread_1 = (*fields).clone();
        for (__flight_key, __flight_value) in __flight_spread_1.iter().cloned() {
            if let Some((_, __flight_existing)) = __flight_record
                .iter_mut()
                .find(|(key, _)| key == &__flight_key)
            {
                *__flight_existing = __flight_value;
            } else {
                __flight_record.push((__flight_key, __flight_value));
            }
        }
        __flight_record
    };
    return LogContext {
        __flight_identity: std::sync::Arc::new(()),
        channel: if (channel).is_some() {
            Some((channel.as_ref().unwrap()).clone())
        } else {
            (parent.channel).clone()
        },
        fields: (merged).clone(),
    };
}

// Source: upstream/packages/log/src/log.ts:137 (sha256:ad8e586670674d592a53d775334c032dee2b80a23e4ec1b1ef85728098dbf7df)
#[derive(Clone, Default)]
struct CreateConsoleCaptureSinkRecord5 {
    __flight_identity: std::sync::Arc<()>,
}
impl PartialEq for CreateConsoleCaptureSinkRecord5 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn create_console_capture_sink(options: Option<SharedStructuralRecord2>) -> LogSink {
    let options = options.unwrap_or(SharedStructuralRecord2 {
        __flight_identity: std::sync::Arc::new(()),
        formatter: None,
    });
    let envelope_formatter = ((options.formatter).clone())
        .clone()
        .unwrap_or(std::sync::Arc::new(std::sync::Mutex::new(Box::new({
            let _default_json_formatter = _default_json_formatter.clone();
            move |__flight_argument_0: LogEntry| -> String {
                _default_json_formatter(&__flight_argument_0)
            }
        })
            as Box<dyn FnMut(LogEntry) -> String + Send + 'static>)));
    return std::sync::Arc::new(std::sync::Mutex::new(Box::new({
        let envelope_formatter = envelope_formatter.clone();
        move |entry: LogEntry| -> () {
            _write_console_capture_entry(&entry, (envelope_formatter).clone())
        }
    })
        as Box<dyn FnMut(LogEntry) -> () + Send + 'static>));
}

// Source: upstream/packages/log/src/log.ts:142 (sha256:b820bbc75b2aaa44b4898dde07435c1f910e8e9d00a74fed37189798614958e7)
#[derive(Clone, Default)]
struct CreateConsoleLogSinkRecord5 {
    __flight_identity: std::sync::Arc<()>,
    level_prefix: bool,
}
impl PartialEq for CreateConsoleLogSinkRecord5 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
struct CreateConsoleLogSinkRecord6 {
    __flight_identity: std::sync::Arc<()>,
}
impl PartialEq for CreateConsoleLogSinkRecord6 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn create_console_log_sink(options: Option<SharedStructuralRecord2>) -> LogSink {
    let options = options.unwrap_or(SharedStructuralRecord2 {
        __flight_identity: std::sync::Arc::new(()),
        formatter: None,
    });
    let formatter = ((options.formatter).clone())
        .clone()
        .unwrap_or(create_text_log_formatter(Some(SharedStructuralRecord4 {
            __flight_identity: std::sync::Arc::new(()),
            level_prefix: Some(true),
            indent_groups: None,
            timestamp: None,
        })));
    return std::sync::Arc::new(std::sync::Mutex::new(Box::new({
        let formatter = formatter.clone();
        move |entry: LogEntry| -> () { _write_console_log_entry(&entry, (formatter).clone()) }
    })
        as Box<dyn FnMut(LogEntry) -> () + Send + 'static>));
}

// Source: upstream/packages/log/src/log.ts:148 (sha256:84b35500f0fb53ca77b8bd75f8e244a11e75367b05399e9b3675c92e2e49212d)
pub fn create_fanout_log_sink(sinks: Vec<LogSink>) -> LogSink {
    let list = (sinks).clone();
    return std::sync::Arc::new(std::sync::Mutex::new(Box::new({
        let list = list.clone();
        move |entry: LogEntry| -> () {
            for s in (list).iter().cloned() {
                {
                    let __flight_callback = (s).clone();
                    let __flight_result = __flight_callback.lock().unwrap()((entry).clone());
                    __flight_result
                };
            }
        }
    })
        as Box<dyn FnMut(LogEntry) -> () + Send + 'static>));
}

// Source: upstream/packages/log/src/log.ts:159 (sha256:6663cd844459d879c8c658224c2d277383ec4ff298e901ac99fc703fd7968121)
#[derive(Clone, Default)]
struct CreateFileLogSinkRecord5 {
    __flight_identity: std::sync::Arc<()>,
}
impl PartialEq for CreateFileLogSinkRecord5 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn create_file_log_sink(options: Option<SharedStructuralRecord2>) -> FileLogSink {
    let options = options.unwrap_or(SharedStructuralRecord2 {
        __flight_identity: std::sync::Arc::new(()),
        formatter: None,
    });
    let formatter = ((options.formatter).clone())
        .clone()
        .unwrap_or(create_json_log_formatter());
    let mut sink: LogSink = std::sync::Arc::new(std::sync::Mutex::new(Box::new({
        let formatter = formatter.clone();
        move |entry: LogEntry| -> () {
            let backend = (*_TRANSPORT_BACKEND.lock().unwrap()).clone();
            if (backend).is_none() {
                return;
            }
            {
                let __flight_callback = (backend.as_ref().unwrap().write).clone();
                let __flight_result = __flight_callback.lock().unwrap()(format!(
                    "{}{}",
                    {
                        let __flight_callback = (formatter).clone();
                        let __flight_result = __flight_callback.lock().unwrap()((entry).clone());
                        __flight_result
                    },
                    "\n"
                ));
                __flight_result
            };
        }
    })
        as Box<dyn FnMut(LogEntry) -> () + Send + 'static>));
    let handle: FileLogSink = FileLogSink {
        __flight_identity: std::sync::Arc::new(()),
        sink: (sink).clone(),
    };
    return handle;
}

// Source: upstream/packages/log/src/log.ts:172 (sha256:d954767afa16f54af08878d86c98bc8d114e38de21bffeba5ddd627a44c44d0f)
pub fn create_filter_log_sink(
    target: LogSink,
    predicate: std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(LogEntry) -> bool + Send + 'static>>>,
) -> LogSink {
    return std::sync::Arc::new(std::sync::Mutex::new(Box::new({
        let predicate = predicate.clone();
        let target = target.clone();
        move |entry: LogEntry| -> () {
            if {
                let __flight_callback = (predicate).clone();
                let __flight_result = __flight_callback.lock().unwrap()((entry).clone());
                __flight_result
            } {
                {
                    let __flight_callback = (target).clone();
                    let __flight_result = __flight_callback.lock().unwrap()((entry).clone());
                    __flight_result
                };
            }
        }
    })
        as Box<dyn FnMut(LogEntry) -> () + Send + 'static>));
}

// Source: upstream/packages/log/src/log.ts:181 (sha256:82e0dab668f2be06cc078bd5c3e0372d3e673f240706254a4a2171c8022d9b49)
pub fn create_json_log_formatter() -> LogFormatter {
    return std::sync::Arc::new(std::sync::Mutex::new(
        Box::new(move |entry: LogEntry| -> String {
            let level = entry.level;
            let channel = (entry.channel).clone();
            let serialized = _apply_serializers(&if ((match &(entry.data) {
                crate::FlightUnion2::A(_) => "string",
                crate::FlightUnion2::B(value) => "object",
            })
            .to_owned()
                == "string")
            {
                {
                    let mut __flight_record = Vec::new();
                    __flight_record.push(("msg".to_owned(), {
                        let __flight_portable_source = match (entry.data).clone() {
                            LogData::A(value) => value,
                            LogData::B(_) => panic!("TypeScript union narrowing failed"),
                        };
                        crate::FlightValue::String((&__flight_portable_source).clone())
                    }));
                    __flight_record
                }
            } else {
                match (entry.data).clone() {
                    LogData::A(_) => panic!("TypeScript union narrowing failed"),
                    LogData::B(value) => value,
                }
            });
            let redacted = if ((_REDACTION_PATHS.lock().unwrap().len() as f64) > 0.0_f64) {
                _apply_redaction(&serialized)
            } else {
                (serialized).clone()
            };
            return crate::flight_json_stringify(
                &(crate::FlightValue::Record({
                    let mut __flight_record = Vec::new();
                    let __flight_key_0 = "__flight".to_owned();
                    let __flight_value_0 = {
                        let __flight_portable_source = true;
                        crate::FlightValue::Bool(*(&__flight_portable_source))
                    };
                    if let Some((_, __flight_existing)) = __flight_record
                        .iter_mut()
                        .find(|(existing, _)| existing == &__flight_key_0)
                    {
                        *__flight_existing = __flight_value_0;
                    } else {
                        __flight_record.push((__flight_key_0, __flight_value_0));
                    }
                    let __flight_key_1 = "t".to_owned();
                    let __flight_value_1 = {
                        let __flight_portable_source = _timestamp();
                        crate::FlightValue::Number(*(&__flight_portable_source) as f64)
                    };
                    if let Some((_, __flight_existing)) = __flight_record
                        .iter_mut()
                        .find(|(existing, _)| existing == &__flight_key_1)
                    {
                        *__flight_existing = __flight_value_1;
                    } else {
                        __flight_record.push((__flight_key_1, __flight_value_1));
                    }
                    let __flight_key_2 = "level".to_owned();
                    let __flight_value_2 = {
                        let __flight_portable_source = _LEVEL_NAMES
                            .iter()
                            .find(|(entry_key, _)| entry_key == &level)
                            .map(|(_, value)| value.clone())
                            .clone();
                        match (&__flight_portable_source).as_ref() {
                            Some(value) => crate::FlightValue::String((value).clone()),
                            None => crate::FlightValue::Null,
                        }
                    };
                    if let Some((_, __flight_existing)) = __flight_record
                        .iter_mut()
                        .find(|(existing, _)| existing == &__flight_key_2)
                    {
                        *__flight_existing = __flight_value_2;
                    } else {
                        __flight_record.push((__flight_key_2, __flight_value_2));
                    }
                    let __flight_key_3 = "channel".to_owned();
                    let __flight_value_3 = {
                        let __flight_portable_source = (channel).clone();
                        match (&__flight_portable_source).as_ref() {
                            Some(value) => crate::FlightValue::String((value).clone()),
                            None => crate::FlightValue::Null,
                        }
                    };
                    if let Some((_, __flight_existing)) = __flight_record
                        .iter_mut()
                        .find(|(existing, _)| existing == &__flight_key_3)
                    {
                        *__flight_existing = __flight_value_3;
                    } else {
                        __flight_record.push((__flight_key_3, __flight_value_3));
                    }
                    let __flight_key_4 = "data".to_owned();
                    let __flight_value_4 = {
                        let __flight_portable_source = (redacted).clone();
                        crate::FlightValue::Record(
                            (&__flight_portable_source)
                                .iter()
                                .map(|(key, value)| (key.clone(), (value).clone()))
                                .collect(),
                        )
                    };
                    if let Some((_, __flight_existing)) = __flight_record
                        .iter_mut()
                        .find(|(existing, _)| existing == &__flight_key_4)
                    {
                        *__flight_existing = __flight_value_4;
                    } else {
                        __flight_record.push((__flight_key_4, __flight_value_4));
                    }
                    __flight_record
                })),
            )
            .expect("JSON.stringify encountered an opaque host object")
            .expect("JSON.stringify returned undefined where Rust requires String");
        }) as Box<dyn FnMut(LogEntry) -> String + Send + 'static>,
    ));
}

// Source: upstream/packages/log/src/log.ts:198 (sha256:f5cd1259db90774dda9574357354a3e352d31e294532ae60de8784784a03657b)
#[derive(Clone, Default)]
struct CreateLogContextRecord5 {
    __flight_identity: std::sync::Arc<()>,
}
impl PartialEq for CreateLogContextRecord5 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn create_log_context(
    channel: Option<String>,
    fields: Option<Vec<(String, crate::FlightValue)>>,
) -> LogContext {
    let fields = fields.unwrap_or({
        let mut __flight_record = Vec::new();
        __flight_record
    });
    return LogContext {
        __flight_identity: std::sync::Arc::new(()),
        channel: (channel).clone(),
        fields: (fields).clone(),
    };
}

// Source: upstream/packages/log/src/log.ts:207 (sha256:525de5789c62ff6c626952366477b6c2c37eacf5462dc3446eebd996ba7c25d8)
#[derive(Clone, Default)]
struct CreateLogSpanRecord5 {
    __flight_identity: std::sync::Arc<()>,
}
impl PartialEq for CreateLogSpanRecord5 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn create_log_span(
    name: String,
    fields: Option<Vec<(String, crate::FlightValue)>>,
    channel: Option<String>,
) -> LogSpan {
    let fields = fields.unwrap_or({
        let mut __flight_record = Vec::new();
        __flight_record
    });
    return LogSpan {
        __flight_identity: std::sync::Arc::new(()),
        name: (name).clone(),
        fields: (fields).clone(),
        channel: (channel).clone(),
    };
}

// Source: upstream/packages/log/src/log.ts:217 (sha256:440044197dc724b1e134b8f72cf61020759e535b79bbd52828aeb71f95a06707)
pub fn create_memory_log_sink(capacity: f64) -> MemoryLogSink {
    let state: std::sync::Arc<std::sync::Mutex<MemoryLogSinkState>> =
        std::sync::Arc::new(std::sync::Mutex::new(MemoryLogSinkState {
            __flight_identity: std::sync::Arc::new(()),
            buf: vec![],
            head: 0.0_f64,
        }));
    let mut sink: LogSink = std::sync::Arc::new(std::sync::Mutex::new(Box::new({
        let mut state = state.clone();
        move |entry: LogEntry| -> () {
            let stored: LogEntry = LogEntry {
                __flight_identity: std::sync::Arc::new(()),
                level: entry.level,
                channel: (entry.channel).clone(),
                data: (entry.data).clone(),
            };
            if (((*state.lock().unwrap()).buf.len() as f64) < capacity) {
                (*state.lock().unwrap())
                    .buf
                    .push(((stored).clone()).clone());
            } else {
                {
                    let __flight_index = ((*state.lock().unwrap()).head) as usize;
                    let __flight_value = (stored).clone();
                    if __flight_index == (*state.lock().unwrap()).buf.len() {
                        (*state.lock().unwrap()).buf.push(__flight_value);
                    } else {
                        (*state.lock().unwrap()).buf[__flight_index] = __flight_value;
                    }
                };
                (*state.lock().unwrap()).head =
                    (((*state.lock().unwrap()).head + 1.0_f64) % capacity);
            }
        }
    })
        as Box<dyn FnMut(LogEntry) -> () + Send + 'static>));
    let handle: MemoryLogSink = MemoryLogSink {
        __flight_identity: std::sync::Arc::new(()),
        sink: (sink).clone(),
    };
    {
        let __flight_key = (handle).clone();
        let __flight_value = (*state.lock().unwrap()).clone();
        if let Some((_, value)) = (*_MEMORY_SINK_STATES.lock().unwrap())
            .iter_mut()
            .find(|(key, _)| key == &__flight_key)
        {
            *value = __flight_value;
        } else {
            (*_MEMORY_SINK_STATES.lock().unwrap()).push((__flight_key, __flight_value));
        }
    };
    return handle;
}

// Source: upstream/packages/log/src/log.ts:235 (sha256:8c8881378bfba8a4555e122bf6bc7b58f73676f09a729adab0622483f6176177)
pub fn create_rate_limited_log_sink(
    target: LogSink,
    options: &SharedStructuralRecord3,
) -> RateLimitedLogSink {
    let per_channel = (options.per_channel).clone().unwrap_or(false);
    let max_per_interval = options.max_per_interval;
    let interval_ms = options.interval_ms;
    let counts: std::sync::Arc<std::sync::Mutex<Vec<(Option<String>, f64)>>> =
        std::sync::Arc::new(std::sync::Mutex::new(Vec::new()));
    let window_start: std::sync::Arc<std::sync::Mutex<f64>> =
        std::sync::Arc::new(std::sync::Mutex::new(_timestamp()));
    let mut sink: LogSink = std::sync::Arc::new(std::sync::Mutex::new(Box::new({
        let mut counts = counts.clone();
        let target = target.clone();
        let mut window_start = window_start.clone();
        move |entry: LogEntry| -> () {
            let now = _timestamp();
            if ((now - (*window_start.lock().unwrap()).clone()) >= interval_ms) {
                (*counts.lock().unwrap()).clear();
                (*window_start.lock().unwrap()) = now;
            }
            let key = if per_channel {
                (entry.channel).clone()
            } else {
                None
            };
            let current = ((*counts.lock().unwrap())
                .iter()
                .find(|(entry_key, _)| entry_key == &(key).clone())
                .map(|(_, value)| value.clone()))
            .clone()
            .unwrap_or(0.0_f64);
            if (current >= max_per_interval) {
                return;
            }
            {
                let __flight_key = (key).clone();
                let __flight_value = (current + 1.0_f64);
                if let Some((_, value)) = (*counts.lock().unwrap())
                    .iter_mut()
                    .find(|(key, _)| key == &__flight_key)
                {
                    *value = __flight_value;
                } else {
                    (*counts.lock().unwrap()).push((__flight_key, __flight_value));
                }
            };
            {
                let __flight_callback = (target).clone();
                let __flight_result = __flight_callback.lock().unwrap()((entry).clone());
                __flight_result
            };
        }
    })
        as Box<dyn FnMut(LogEntry) -> () + Send + 'static>));
    return RateLimitedLogSink {
        __flight_identity: std::sync::Arc::new(()),
        sink: (sink).clone(),
    };
}

// Source: upstream/packages/log/src/log.ts:259 (sha256:53f5be6d14d8e73ff5bd5d50da1e07c3a171ba24f5280a0a234591369ce68419)
pub fn create_sampled_log_sink(target: LogSink, rate: f64) -> LogSink {
    if (rate <= 1.0_f64) {
        return target;
    }
    let counter: std::sync::Arc<std::sync::Mutex<f64>> =
        std::sync::Arc::new(std::sync::Mutex::new(0.0_f64));
    return std::sync::Arc::new(std::sync::Mutex::new(Box::new({
        let mut counter = counter.clone();
        let target = target.clone();
        move |entry: LogEntry| -> () {
            (*counter.lock().unwrap()) = (((*counter.lock().unwrap()).clone() + 1.0_f64) % rate);
            if ((*counter.lock().unwrap()).clone() == 0.0_f64) {
                {
                    let __flight_callback = (target).clone();
                    let __flight_result = __flight_callback.lock().unwrap()((entry).clone());
                    __flight_result
                };
            }
        }
    })
        as Box<dyn FnMut(LogEntry) -> () + Send + 'static>));
}

// Source: upstream/packages/log/src/log.ts:270 (sha256:7703c0e1be271f3ef06a8e0e688760b30f41f62b875ed4b32d2e2ff9d120f808)
#[derive(Clone, Default)]
struct CreateTextLogFormatterRecord5 {
    __flight_identity: std::sync::Arc<()>,
}
impl PartialEq for CreateTextLogFormatterRecord5 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn create_text_log_formatter(options: Option<SharedStructuralRecord4>) -> LogFormatter {
    let options = options.unwrap_or(SharedStructuralRecord4 {
        __flight_identity: std::sync::Arc::new(()),
        indent_groups: None,
        level_prefix: None,
        timestamp: None,
    });
    return std::sync::Arc::new(std::sync::Mutex::new(Box::new({
        let options = options.clone();
        move |entry: LogEntry| -> String {
            let level = entry.level;
            let channel = (entry.channel).clone();
            let mut parts: Vec<String> = vec![];
            if (options.timestamp).unwrap_or(false) {
                parts.push(format!(
                    "t={}",
                    __flight_number_to_fixed(_timestamp(), 2.0_f64)
                ));
            }
            if (options.level_prefix).unwrap_or(false) {
                parts.push(
                    (_LEVEL_NAMES
                        .iter()
                        .find(|(entry_key, _)| entry_key == &level)
                        .map(|(_, value)| value.clone())
                        .clone())
                    .clone()
                    .unwrap_or("unknown".to_owned()),
                );
            }
            parts.push(if (channel).is_some() {
                format!("[{}]", (channel.as_ref().unwrap()).clone())
            } else {
                "[flight]".to_owned()
            });
            if ((options.indent_groups).unwrap_or(false))
                && ((*_GROUP_DEPTH.lock().unwrap()).clone() > 0.0_f64)
            {
                parts.push(__flight_string_repeat(
                    &("  "),
                    (*_GROUP_DEPTH.lock().unwrap()).clone(),
                ));
            }
            if ((match &(entry.data) {
                crate::FlightUnion2::A(_) => "string",
                crate::FlightUnion2::B(value) => "object",
            })
            .to_owned()
                == "string")
            {
                parts.push(
                    (match (entry.data).clone() {
                        LogData::A(value) => value,
                        LogData::B(_) => panic!("TypeScript union narrowing failed"),
                    })
                    .clone(),
                );
            } else {
                parts.push(
                    crate::flight_json_stringify(
                        &({
                            let __flight_portable_source = match (entry.data).clone() {
                                LogData::A(_) => panic!("TypeScript union narrowing failed"),
                                LogData::B(value) => value,
                            };
                            crate::FlightValue::Record(
                                (&__flight_portable_source)
                                    .iter()
                                    .map(|(key, value)| (key.clone(), (value).clone()))
                                    .collect(),
                            )
                        }),
                    )
                    .expect("JSON.stringify encountered an opaque host object")
                    .expect("JSON.stringify returned undefined where Rust requires String"),
                );
            }
            return (parts)
                .iter()
                .map(|value| value.to_string())
                .collect::<Vec<_>>()
                .join((" ".to_owned()).as_str());
        }
    })
        as Box<dyn FnMut(LogEntry) -> String + Send + 'static>));
}

// Source: upstream/packages/log/src/log.ts:292 (sha256:4f48bed54c81a6b3c0d3148adce50599cd38e153499063332f86a0b8312741c1)
pub fn create_web_log_transport_backend() -> LogTransportBackend {
    return LogTransportBackend {
        __flight_identity: std::sync::Arc::new(()),
        write: std::sync::Arc::new(std::sync::Mutex::new(
            Box::new(move |_line: String| -> () {})
                as Box<dyn FnMut(String) -> () + Send + 'static>,
        )),
        flush: None,
        dispose: None,
    };
}

// Source: upstream/packages/log/src/log.ts:304 (sha256:f64a380b1123ffcc0c5053f093fad7a58382db3dc50a3c18f0629fcc3bb18be4)
pub fn dispose_file_log_sink(_handle: &FileLogSink) -> () {
    let backend = (*_TRANSPORT_BACKEND.lock().unwrap()).clone();
    if (backend).is_none() {
        return;
    }
    if ((backend.as_ref().unwrap().flush).clone()).is_some() {
        {
            let __flight_callback = (backend.as_ref().unwrap().flush)
                .clone()
                .as_ref()
                .unwrap()
                .clone();
            let __flight_result = __flight_callback.lock().unwrap()();
            __flight_result
        };
    }
    if ((backend.as_ref().unwrap().dispose).clone()).is_some() {
        {
            let __flight_callback = (backend.as_ref().unwrap().dispose)
                .clone()
                .as_ref()
                .unwrap()
                .clone();
            let __flight_result = __flight_callback.lock().unwrap()();
            __flight_result
        };
    }
}

// Source: upstream/packages/log/src/log.ts:314 (sha256:ef92cffec055b3fa03880b3c30e311cb8f1cace523a3436039987e907b0cf105)
pub fn dispose_log_sink(handle: &BufferedLogSink) -> () {
    let mut state = (*_BUFFERED_SINK_STATES.lock().unwrap())
        .iter()
        .find(|(entry_key, _)| entry_key == &(*handle).clone())
        .map(|(_, value)| value.clone());
    if (state).is_none() {
        return;
    }
    if ((state.as_mut().unwrap().timer).clone()).is_some() {
        if let Some(__flight_timer) = ((state.as_mut().unwrap().timer).clone()).clone() {
            crate::clear_interval(__flight_timer);
        };
    }
    {
        let __flight_callback = (state.as_mut().unwrap().flush).clone();
        let __flight_result = __flight_callback.lock().unwrap()();
        __flight_result
    };
    state.as_mut().unwrap().timer = None;
}

// Source: upstream/packages/log/src/log.ts:325 (sha256:9d86e7f43964874d66aed4565b3b61360b7fe5559436e4e495c4721309cd70b9)
pub fn enable_log_signals() -> LogSignals {
    if ((*_LOG_SIGNALS.lock().unwrap()).clone()).is_some() {
        return (((*_LOG_SIGNALS.lock().unwrap()).as_mut().unwrap()).clone()).clone();
    }
    (*_LOG_SIGNALS.lock().unwrap()) = Some(LogSignals {
        __flight_identity: std::sync::Arc::new(()),
        on_log_entry: create_signal(),
        on_log_error: create_signal(),
    });
    return (((*_LOG_SIGNALS.lock().unwrap()).clone()).clone().unwrap()).clone();
}

// Source: upstream/packages/log/src/log.ts:336 (sha256:d273c7109458b21dc7e26d14c0a3274b16749282373308695d6ed40151e5ef12)
pub fn end_log_group(channel: Option<String>) -> () {
    if ((*_GROUP_DEPTH.lock().unwrap()).clone() <= 0.0_f64) {
        return;
    }
    {
        (*_GROUP_DEPTH.lock().unwrap()) -= 1.0;
        (*_GROUP_DEPTH.lock().unwrap())
    };
    if (!_passes_level_gate(LogLevel::Debug, ((channel).clone()).clone())) {
        return;
    }
    _emit_to_sinks(&LogEntry {
        __flight_identity: std::sync::Arc::new(()),
        level: LogLevel::Debug,
        channel: (channel).clone(),
        data: flighthq_types::LogData::B({
            let mut __flight_record = Vec::new();
            __flight_record.push(("group".to_owned(), {
                let __flight_portable_source = "end".to_owned();
                crate::FlightValue::String((&__flight_portable_source).clone())
            }));
            __flight_record.push(("depth".to_owned(), {
                let __flight_portable_source = ((*_GROUP_DEPTH.lock().unwrap()).clone() + 1.0_f64);
                crate::FlightValue::Number(*(&__flight_portable_source) as f64)
            }));
            __flight_record
        }),
    });
}

// Source: upstream/packages/log/src/log.ts:345 (sha256:c96c86f9427e61d81a2ffb3ec9dcf35cfb1c255bcfeeb08f363dae583e85b647)
pub fn end_log_timer(timer: &LogTimer) -> f64 {
    let elapsed = (_timestamp() - timer.started_at);
    log_debug(
        &(crate::FlightUnion2::<LogData, LogDataProvider>::A(crate::FlightUnion2::<
            String,
            Vec<(String, crate::FlightValue)>,
        >::B({
            let mut __flight_record = Vec::new();
            __flight_record.push(("label".to_owned(), {
                let __flight_portable_source = (timer.label).clone();
                crate::FlightValue::String((&__flight_portable_source).clone())
            }));
            __flight_record.push(("elapsedMs".to_owned(), {
                let __flight_portable_source = elapsed;
                crate::FlightValue::Number(*(&__flight_portable_source) as f64)
            }));
            __flight_record
        }))),
        ((timer.channel).clone()).clone(),
    );
    return elapsed;
}

// Source: upstream/packages/log/src/log.ts:354 (sha256:a979ff11edae00f2dc2cdf1436e2774bb70330480ef0c56b08a7f9766f7a3bec)
pub fn enter_log_span(span: &LogSpan) -> () {
    _SPAN_STACK.lock().unwrap().push(((*span).clone()).clone());
}

// Source: upstream/packages/log/src/log.ts:361 (sha256:e817ecd27778e010d38bb648dfa04441158fdc5feafd6d99898741f6643f4090)
pub fn exit_log_span(span: &LogSpan) -> () {
    let idx = {
        let __flight_value = (*span).clone();
        (_SPAN_STACK.lock().unwrap())
            .iter()
            .position(|item| item == &__flight_value)
            .map_or(-1.0_f64, |index| index as f64)
    };
    if (idx >= 0.0_f64) {
        {
            let __flight_start = (idx);
            let __flight_count = (1.0_f64);
            _SPAN_STACK
                .lock()
                .unwrap()
                .splice(
                    (__flight_start) as usize..(__flight_start + __flight_count) as usize,
                    vec![],
                )
                .collect::<Vec<_>>()
        };
    }
}

// Source: upstream/packages/log/src/log.ts:367 (sha256:0b35f60b6ef747a1891a851476b331c469f4e1fcb00fce4298c7446f0fd34603)
pub fn flush_log_sink(handle: &BufferedLogSink) -> () {
    let state = (*_BUFFERED_SINK_STATES.lock().unwrap())
        .iter()
        .find(|(entry_key, _)| entry_key == &(*handle).clone())
        .map(|(_, value)| value.clone());
    if (state).is_some() {
        {
            let __flight_callback = (state.as_ref().unwrap().flush).clone();
            let __flight_result = __flight_callback.lock().unwrap()();
            __flight_result
        };
    }
}

// Source: upstream/packages/log/src/log.ts:374 (sha256:8495159d8aff70a3fb12821f87aea83e277bf015dac71850625c7a46559f8d07)
pub fn get_log_channel_level(channel: String) -> Option<LogLevel> {
    return (*_CHANNEL_LEVELS.lock().unwrap())
        .iter()
        .find(|(entry_key, _)| entry_key == &(channel).clone())
        .map(|(_, value)| value.clone());
}

// Source: upstream/packages/log/src/log.ts:380 (sha256:592a47d515f41f8214af2e36844d892fcc3f15bec0576ad558bb727033341e73)
pub fn get_log_console_level() -> LogLevel {
    return (*_CONSOLE_LEVEL.lock().unwrap()).clone();
}

// Source: upstream/packages/log/src/log.ts:386 (sha256:7e1d6de6f2e20e06f1a1047f35c6e027da5b92f17345d810141ff38c588048e3)
pub fn get_log_level() -> LogLevel {
    return (*_LEVEL.lock().unwrap()).clone();
}

// Source: upstream/packages/log/src/log.ts:391 (sha256:bed3bd73dc2081d09c6d8b9f61977e35b17a8778409bc627995c0d1f72b84a58)
pub fn get_log_level_name(level: LogLevel) -> String {
    return (_LEVEL_NAMES
        .iter()
        .find(|(entry_key, _)| entry_key == &level)
        .map(|(_, value)| value.clone())
        .clone())
    .clone()
    .unwrap_or("unknown".to_owned());
}

// Source: upstream/packages/log/src/log.ts:396 (sha256:2966a53ba71e6e4b24da3e0fe6e4b77d6872a7dca6fe47e958c664b5618d56e5)
pub fn get_log_transport_backend() -> Option<LogTransportBackend> {
    return (*_TRANSPORT_BACKEND.lock().unwrap()).clone();
}

// Source: upstream/packages/log/src/log.ts:401 (sha256:36cad86a2d11562eee6a370eb738df9ea02f045fece8c3092068fa8b1991ced7)
pub fn get_memory_log_sink_entries(handle: &MemoryLogSink) -> Vec<LogEntry> {
    let state = (*_MEMORY_SINK_STATES.lock().unwrap())
        .iter()
        .find(|(entry_key, _)| entry_key == &(*handle).clone())
        .map(|(_, value)| value.clone());
    if (state).is_none() {
        return vec![];
    }
    let __destructure3 = state;
    let head = __destructure3.as_ref().unwrap().head;
    if (head == 0.0_f64) {
        return (__destructure3.as_ref().unwrap().buf).clone();
    }
    return {
        let mut __flight_array = Vec::new();
        __flight_array.extend(
            ((__destructure3.as_ref().unwrap().buf)
                [(head) as usize..((__destructure3.as_ref().unwrap().buf).len() as f64) as usize]
                .to_vec())
            .iter()
            .cloned(),
        );
        __flight_array.extend(
            ((__destructure3.as_ref().unwrap().buf)[(0.0_f64) as usize..(head) as usize].to_vec())
                .iter()
                .cloned(),
        );
        __flight_array
    };
}

// Source: upstream/packages/log/src/log.ts:416 (sha256:8fd1fa56b915047b9faee5e8d8c0a7daf708f3b2c928d39ff9e582e062584e49)
pub fn log(
    level: LogLevel,
    data: &crate::FlightUnion2<LogData, LogDataProvider>,
    channel: Option<String>,
) -> () {
    if (!_passes_level_gate(level, ((channel).clone()).clone())) {
        return;
    }
    let resolved: LogData = if ((match &(data) {
        crate::FlightUnion2::A(_) => "object",
        crate::FlightUnion2::B(value) => "function",
    })
    .to_owned()
        == "function")
    {
        {
            let __flight_callback = match (*data).clone() {
                crate::FlightUnion2::A(_) => panic!("TypeScript union narrowing failed"),
                crate::FlightUnion2::B(value) => value,
            };
            let __flight_result = __flight_callback.lock().unwrap()();
            __flight_result
        }
    } else {
        match (*data).clone() {
            crate::FlightUnion2::A(value) => value,
            crate::FlightUnion2::B(_) => panic!("TypeScript union narrowing failed"),
        }
    };
    let entry: LogEntry = LogEntry {
        __flight_identity: std::sync::Arc::new(()),
        level: level,
        channel: (channel).clone(),
        data: _merge_span_fields(&resolved, ((channel).clone()).clone()),
    };
    _emit_to_sinks(&entry);
}

// Source: upstream/packages/log/src/log.ts:425 (sha256:32d3fac652432dfe87a1a8ff3b407f405432fb59c4c2b5e72986eb44b3a1eb82)
pub fn log_assert(
    condition: bool,
    data: &crate::FlightUnion2<LogData, LogDataProvider>,
    channel: Option<String>,
) -> () {
    if condition {
        return;
    }
    if (!_passes_level_gate(LogLevel::Error, ((channel).clone()).clone())) {
        return;
    }
    let resolved: LogData = if ((match &(data) {
        crate::FlightUnion2::A(_) => "object",
        crate::FlightUnion2::B(value) => "function",
    })
    .to_owned()
        == "function")
    {
        {
            let __flight_callback = match (*data).clone() {
                crate::FlightUnion2::A(_) => panic!("TypeScript union narrowing failed"),
                crate::FlightUnion2::B(value) => value,
            };
            let __flight_result = __flight_callback.lock().unwrap()();
            __flight_result
        }
    } else {
        match (*data).clone() {
            crate::FlightUnion2::A(value) => value,
            crate::FlightUnion2::B(_) => panic!("TypeScript union narrowing failed"),
        }
    };
    _emit_to_sinks(&LogEntry {
        __flight_identity: std::sync::Arc::new(()),
        level: LogLevel::Error,
        channel: (channel).clone(),
        data: _merge_span_fields(&resolved, ((channel).clone()).clone()),
    });
}

// Source: upstream/packages/log/src/log.ts:433 (sha256:e1be73cc9d17c07aad2acedbecaf0e46a2d6c1b64cabc4ec932de6072f20259f)
pub fn log_debug(
    data: &crate::FlightUnion2<LogData, LogDataProvider>,
    channel: Option<String>,
) -> () {
    if (!_passes_level_gate(LogLevel::Debug, ((channel).clone()).clone())) {
        return;
    }
    let resolved: LogData = if ((match &(data) {
        crate::FlightUnion2::A(_) => "object",
        crate::FlightUnion2::B(value) => "function",
    })
    .to_owned()
        == "function")
    {
        {
            let __flight_callback = match (*data).clone() {
                crate::FlightUnion2::A(_) => panic!("TypeScript union narrowing failed"),
                crate::FlightUnion2::B(value) => value,
            };
            let __flight_result = __flight_callback.lock().unwrap()();
            __flight_result
        }
    } else {
        match (*data).clone() {
            crate::FlightUnion2::A(value) => value,
            crate::FlightUnion2::B(_) => panic!("TypeScript union narrowing failed"),
        }
    };
    _emit_to_sinks(&LogEntry {
        __flight_identity: std::sync::Arc::new(()),
        level: LogLevel::Debug,
        channel: (channel).clone(),
        data: _merge_span_fields(&resolved, ((channel).clone()).clone()),
    });
}

// Source: upstream/packages/log/src/log.ts:439 (sha256:eb096702c3277a5e3a0bbef8254e1f39be9877a5c62b95a06ffc0e36ae9e8921)
pub fn log_debug_with(
    context: &LogContext,
    data: &crate::FlightUnion2<LogData, LogDataProvider>,
) -> () {
    let channel = (context.channel).clone();
    if (!_passes_level_gate(LogLevel::Debug, ((channel).clone()).clone())) {
        return;
    }
    let resolved: LogData = if ((match &(data) {
        crate::FlightUnion2::A(_) => "object",
        crate::FlightUnion2::B(value) => "function",
    })
    .to_owned()
        == "function")
    {
        {
            let __flight_callback = match (*data).clone() {
                crate::FlightUnion2::A(_) => panic!("TypeScript union narrowing failed"),
                crate::FlightUnion2::B(value) => value,
            };
            let __flight_result = __flight_callback.lock().unwrap()();
            __flight_result
        }
    } else {
        match (*data).clone() {
            crate::FlightUnion2::A(value) => value,
            crate::FlightUnion2::B(_) => panic!("TypeScript union narrowing failed"),
        }
    };
    _emit_to_sinks(&LogEntry {
        __flight_identity: std::sync::Arc::new(()),
        level: LogLevel::Debug,
        channel: (channel).clone(),
        data: _merge_context_fields(
            context,
            &_merge_span_fields(&resolved, ((channel).clone()).clone()),
        ),
    });
}

// Source: upstream/packages/log/src/log.ts:450 (sha256:dd391ef28f69eebe58d8b413282399ca7b325d04cd3ae6eff3b519079d875a50)
pub fn log_error(
    data: &crate::FlightUnion2<LogData, LogDataProvider>,
    channel: Option<String>,
) -> () {
    if (!_passes_level_gate(LogLevel::Error, ((channel).clone()).clone())) {
        return;
    }
    let resolved: LogData = if ((match &(data) {
        crate::FlightUnion2::A(_) => "object",
        crate::FlightUnion2::B(value) => "function",
    })
    .to_owned()
        == "function")
    {
        {
            let __flight_callback = match (*data).clone() {
                crate::FlightUnion2::A(_) => panic!("TypeScript union narrowing failed"),
                crate::FlightUnion2::B(value) => value,
            };
            let __flight_result = __flight_callback.lock().unwrap()();
            __flight_result
        }
    } else {
        match (*data).clone() {
            crate::FlightUnion2::A(value) => value,
            crate::FlightUnion2::B(_) => panic!("TypeScript union narrowing failed"),
        }
    };
    _emit_to_sinks(&LogEntry {
        __flight_identity: std::sync::Arc::new(()),
        level: LogLevel::Error,
        channel: (channel).clone(),
        data: _merge_span_fields(&resolved, ((channel).clone()).clone()),
    });
}

// Source: upstream/packages/log/src/log.ts:456 (sha256:81340501ac779bf99d6df3590de337531b1bdc2b4f8d48c2e91fbf411af0e2f4)
pub fn log_error_with(
    context: &LogContext,
    data: &crate::FlightUnion2<LogData, LogDataProvider>,
) -> () {
    let channel = (context.channel).clone();
    if (!_passes_level_gate(LogLevel::Error, ((channel).clone()).clone())) {
        return;
    }
    let resolved: LogData = if ((match &(data) {
        crate::FlightUnion2::A(_) => "object",
        crate::FlightUnion2::B(value) => "function",
    })
    .to_owned()
        == "function")
    {
        {
            let __flight_callback = match (*data).clone() {
                crate::FlightUnion2::A(_) => panic!("TypeScript union narrowing failed"),
                crate::FlightUnion2::B(value) => value,
            };
            let __flight_result = __flight_callback.lock().unwrap()();
            __flight_result
        }
    } else {
        match (*data).clone() {
            crate::FlightUnion2::A(value) => value,
            crate::FlightUnion2::B(_) => panic!("TypeScript union narrowing failed"),
        }
    };
    _emit_to_sinks(&LogEntry {
        __flight_identity: std::sync::Arc::new(()),
        level: LogLevel::Error,
        channel: (channel).clone(),
        data: _merge_context_fields(
            context,
            &_merge_span_fields(&resolved, ((channel).clone()).clone()),
        ),
    });
}

// Source: upstream/packages/log/src/log.ts:467 (sha256:c78fb840d5c7456c8a6e530b1a5e4631e9ce9da8dc5ef19f95c367f078cdef41)
pub fn log_info(
    data: &crate::FlightUnion2<LogData, LogDataProvider>,
    channel: Option<String>,
) -> () {
    if (!_passes_level_gate(LogLevel::Info, ((channel).clone()).clone())) {
        return;
    }
    let resolved: LogData = if ((match &(data) {
        crate::FlightUnion2::A(_) => "object",
        crate::FlightUnion2::B(value) => "function",
    })
    .to_owned()
        == "function")
    {
        {
            let __flight_callback = match (*data).clone() {
                crate::FlightUnion2::A(_) => panic!("TypeScript union narrowing failed"),
                crate::FlightUnion2::B(value) => value,
            };
            let __flight_result = __flight_callback.lock().unwrap()();
            __flight_result
        }
    } else {
        match (*data).clone() {
            crate::FlightUnion2::A(value) => value,
            crate::FlightUnion2::B(_) => panic!("TypeScript union narrowing failed"),
        }
    };
    _emit_to_sinks(&LogEntry {
        __flight_identity: std::sync::Arc::new(()),
        level: LogLevel::Info,
        channel: (channel).clone(),
        data: _merge_span_fields(&resolved, ((channel).clone()).clone()),
    });
}

// Source: upstream/packages/log/src/log.ts:473 (sha256:4133590045e222428f2bcfd816eaf14e294c55e6241b7646037335a44b6f5a09)
pub fn log_info_with(
    context: &LogContext,
    data: &crate::FlightUnion2<LogData, LogDataProvider>,
) -> () {
    let channel = (context.channel).clone();
    if (!_passes_level_gate(LogLevel::Info, ((channel).clone()).clone())) {
        return;
    }
    let resolved: LogData = if ((match &(data) {
        crate::FlightUnion2::A(_) => "object",
        crate::FlightUnion2::B(value) => "function",
    })
    .to_owned()
        == "function")
    {
        {
            let __flight_callback = match (*data).clone() {
                crate::FlightUnion2::A(_) => panic!("TypeScript union narrowing failed"),
                crate::FlightUnion2::B(value) => value,
            };
            let __flight_result = __flight_callback.lock().unwrap()();
            __flight_result
        }
    } else {
        match (*data).clone() {
            crate::FlightUnion2::A(value) => value,
            crate::FlightUnion2::B(_) => panic!("TypeScript union narrowing failed"),
        }
    };
    _emit_to_sinks(&LogEntry {
        __flight_identity: std::sync::Arc::new(()),
        level: LogLevel::Info,
        channel: (channel).clone(),
        data: _merge_context_fields(
            context,
            &_merge_span_fields(&resolved, ((channel).clone()).clone()),
        ),
    });
}

// Source: upstream/packages/log/src/log.ts:486 (sha256:43dda93135fb86ab9ba5e7998118766cf5b11c2a44751e6625c6b1086bb90e4b)
pub fn log_once(
    key: String,
    level: LogLevel,
    data: &crate::FlightUnion2<LogData, LogDataProvider>,
    channel: Option<String>,
) -> bool {
    if (*_ONCE_KEYS.lock().unwrap())
        .iter()
        .any(|item| item == &(key).clone())
    {
        return false;
    }
    {
        let __flight_value = (key).clone();
        if !(*_ONCE_KEYS.lock().unwrap()).contains(&__flight_value) {
            (*_ONCE_KEYS.lock().unwrap()).push(__flight_value);
        }
    };
    log(level, &((*data).clone()), ((channel).clone()).clone());
    return true;
}

// Source: upstream/packages/log/src/log.ts:498 (sha256:a5bdc1fe27ffdd72d4be7adde388007779b726d2bd79806e2c3c004ea7522137)
pub fn log_verbose(
    data: &crate::FlightUnion2<LogData, LogDataProvider>,
    channel: Option<String>,
) -> () {
    if (!_passes_level_gate(LogLevel::Verbose, ((channel).clone()).clone())) {
        return;
    }
    let resolved: LogData = if ((match &(data) {
        crate::FlightUnion2::A(_) => "object",
        crate::FlightUnion2::B(value) => "function",
    })
    .to_owned()
        == "function")
    {
        {
            let __flight_callback = match (*data).clone() {
                crate::FlightUnion2::A(_) => panic!("TypeScript union narrowing failed"),
                crate::FlightUnion2::B(value) => value,
            };
            let __flight_result = __flight_callback.lock().unwrap()();
            __flight_result
        }
    } else {
        match (*data).clone() {
            crate::FlightUnion2::A(value) => value,
            crate::FlightUnion2::B(_) => panic!("TypeScript union narrowing failed"),
        }
    };
    _emit_to_sinks(&LogEntry {
        __flight_identity: std::sync::Arc::new(()),
        level: LogLevel::Verbose,
        channel: (channel).clone(),
        data: _merge_span_fields(&resolved, ((channel).clone()).clone()),
    });
}

// Source: upstream/packages/log/src/log.ts:504 (sha256:ef760ce1376a133f6b3a5af8bac40df85095100199b229bf00d205c500f575bf)
pub fn log_verbose_with(
    context: &LogContext,
    data: &crate::FlightUnion2<LogData, LogDataProvider>,
) -> () {
    let channel = (context.channel).clone();
    if (!_passes_level_gate(LogLevel::Verbose, ((channel).clone()).clone())) {
        return;
    }
    let resolved: LogData = if ((match &(data) {
        crate::FlightUnion2::A(_) => "object",
        crate::FlightUnion2::B(value) => "function",
    })
    .to_owned()
        == "function")
    {
        {
            let __flight_callback = match (*data).clone() {
                crate::FlightUnion2::A(_) => panic!("TypeScript union narrowing failed"),
                crate::FlightUnion2::B(value) => value,
            };
            let __flight_result = __flight_callback.lock().unwrap()();
            __flight_result
        }
    } else {
        match (*data).clone() {
            crate::FlightUnion2::A(value) => value,
            crate::FlightUnion2::B(_) => panic!("TypeScript union narrowing failed"),
        }
    };
    _emit_to_sinks(&LogEntry {
        __flight_identity: std::sync::Arc::new(()),
        level: LogLevel::Verbose,
        channel: (channel).clone(),
        data: _merge_context_fields(
            context,
            &_merge_span_fields(&resolved, ((channel).clone()).clone()),
        ),
    });
}

// Source: upstream/packages/log/src/log.ts:515 (sha256:f388295752499ff678248accf26680e20be2cac4b0a03280aa6bc860b4ef35ee)
pub fn log_warn(
    data: &crate::FlightUnion2<LogData, LogDataProvider>,
    channel: Option<String>,
) -> () {
    if (!_passes_level_gate(LogLevel::Warn, ((channel).clone()).clone())) {
        return;
    }
    let resolved: LogData = if ((match &(data) {
        crate::FlightUnion2::A(_) => "object",
        crate::FlightUnion2::B(value) => "function",
    })
    .to_owned()
        == "function")
    {
        {
            let __flight_callback = match (*data).clone() {
                crate::FlightUnion2::A(_) => panic!("TypeScript union narrowing failed"),
                crate::FlightUnion2::B(value) => value,
            };
            let __flight_result = __flight_callback.lock().unwrap()();
            __flight_result
        }
    } else {
        match (*data).clone() {
            crate::FlightUnion2::A(value) => value,
            crate::FlightUnion2::B(_) => panic!("TypeScript union narrowing failed"),
        }
    };
    _emit_to_sinks(&LogEntry {
        __flight_identity: std::sync::Arc::new(()),
        level: LogLevel::Warn,
        channel: (channel).clone(),
        data: _merge_span_fields(&resolved, ((channel).clone()).clone()),
    });
}

// Source: upstream/packages/log/src/log.ts:521 (sha256:864c88ae290eaac07c21a16c4f8813efd12a848d4e42af1e2990b2c2cae100ce)
pub fn log_warn_with(
    context: &LogContext,
    data: &crate::FlightUnion2<LogData, LogDataProvider>,
) -> () {
    let channel = (context.channel).clone();
    if (!_passes_level_gate(LogLevel::Warn, ((channel).clone()).clone())) {
        return;
    }
    let resolved: LogData = if ((match &(data) {
        crate::FlightUnion2::A(_) => "object",
        crate::FlightUnion2::B(value) => "function",
    })
    .to_owned()
        == "function")
    {
        {
            let __flight_callback = match (*data).clone() {
                crate::FlightUnion2::A(_) => panic!("TypeScript union narrowing failed"),
                crate::FlightUnion2::B(value) => value,
            };
            let __flight_result = __flight_callback.lock().unwrap()();
            __flight_result
        }
    } else {
        match (*data).clone() {
            crate::FlightUnion2::A(value) => value,
            crate::FlightUnion2::B(_) => panic!("TypeScript union narrowing failed"),
        }
    };
    _emit_to_sinks(&LogEntry {
        __flight_identity: std::sync::Arc::new(()),
        level: LogLevel::Warn,
        channel: (channel).clone(),
        data: _merge_context_fields(
            context,
            &_merge_span_fields(&resolved, ((channel).clone()).clone()),
        ),
    });
}

// Source: upstream/packages/log/src/log.ts:534 (sha256:7047b19b6c12858b8166aa2f0720162db6f6115c6b7b2ca94300d8309f4b4d87)
pub fn log_with(
    context: &LogContext,
    level: LogLevel,
    data: &crate::FlightUnion2<LogData, LogDataProvider>,
) -> () {
    let channel = (context.channel).clone();
    if (!_passes_level_gate(level, ((channel).clone()).clone())) {
        return;
    }
    let resolved: LogData = if ((match &(data) {
        crate::FlightUnion2::A(_) => "object",
        crate::FlightUnion2::B(value) => "function",
    })
    .to_owned()
        == "function")
    {
        {
            let __flight_callback = match (*data).clone() {
                crate::FlightUnion2::A(_) => panic!("TypeScript union narrowing failed"),
                crate::FlightUnion2::B(value) => value,
            };
            let __flight_result = __flight_callback.lock().unwrap()();
            __flight_result
        }
    } else {
        match (*data).clone() {
            crate::FlightUnion2::A(value) => value,
            crate::FlightUnion2::B(_) => panic!("TypeScript union narrowing failed"),
        }
    };
    _emit_to_sinks(&LogEntry {
        __flight_identity: std::sync::Arc::new(()),
        level: level,
        channel: (channel).clone(),
        data: _merge_context_fields(
            context,
            &_merge_span_fields(&resolved, ((channel).clone()).clone()),
        ),
    });
}

// Source: upstream/packages/log/src/log.ts:543 (sha256:6642774e2696facd569ee4544b79b8c31e214d49bb0e60bb0f761d324b5401e0)
pub fn parse_log_level(name: String) -> Option<LogLevel> {
    return _LEVEL_BY_NAME
        .iter()
        .find(|(entry_key, _)| entry_key == &(name).to_lowercase())
        .map(|(_, value)| value.clone());
}

// Source: upstream/packages/log/src/log.ts:551 (sha256:c31bc58e7172611c1cf33cb82e6378a1c06526f4d3cf998430e771e0c3dd8818)
pub fn register_log_serializer(
    kind: String,
    fn_: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(crate::FlightValue) -> Vec<(String, crate::FlightValue)> + Send + 'static,
            >,
        >,
    >,
) -> () {
    {
        let __flight_key = (kind).clone();
        let __flight_value = (fn_).clone();
        if let Some((_, value)) = (*_SERIALIZERS.lock().unwrap())
            .iter_mut()
            .find(|(key, _)| key == &__flight_key)
        {
            *value = __flight_value;
        } else {
            (*_SERIALIZERS.lock().unwrap()).push((__flight_key, __flight_value));
        }
    };
}

// Source: upstream/packages/log/src/log.ts:557 (sha256:420a9eb436739a816784eac52e7c6d7a809f8225e97b1e50e4361df6acefd176)
pub fn remove_log_sink(sink: LogSink) -> bool {
    let idx = {
        let __flight_value = (sink).clone();
        (_SINKS.lock().unwrap())
            .iter()
            .position(|item| std::sync::Arc::ptr_eq(item, &__flight_value))
            .map_or(-1.0_f64, |index| index as f64)
    };
    if (idx < 0.0_f64) {
        return false;
    }
    {
        let __flight_start = (idx);
        let __flight_count = (1.0_f64);
        _SINKS
            .lock()
            .unwrap()
            .splice(
                (__flight_start) as usize..(__flight_start + __flight_count) as usize,
                vec![],
            )
            .collect::<Vec<_>>()
    };
    return true;
}

// Source: upstream/packages/log/src/log.ts:566 (sha256:be8f624fb520e41162a5a13bdfbc1d87828d18c18386764460c73a575dbb3cd5)
pub fn serialize_log_error(value: crate::FlightValue) -> Vec<(String, crate::FlightValue)> {
    if (!matches!(&((value).clone()), crate::FlightValue::Error { .. })) {
        return {
            let mut __flight_record = Vec::new();
            __flight_record.push(("value".to_owned(), {
                let __flight_portable_source = {
                    let __flight_value = (value).clone();
                    crate::flight_value_to_string(&__flight_value)
                };
                crate::FlightValue::String((&__flight_portable_source).clone())
            }));
            __flight_record
        };
    }
    let mut result: Vec<(String, crate::FlightValue)> = {
        let mut __flight_record = Vec::new();
        __flight_record.push(("name".to_owned(), {
            let __flight_portable_source = match &(value) {
                crate::FlightValue::Error { name, .. } => name.clone(),
                _ => unreachable!("instanceof Error narrowing must contain an Error value"),
            };
            crate::FlightValue::String((&__flight_portable_source).clone())
        }));
        __flight_record.push(("message".to_owned(), {
            let __flight_portable_source = match &(value) {
                crate::FlightValue::Error { message, .. } => message.clone(),
                _ => unreachable!("instanceof Error narrowing must contain an Error value"),
            };
            crate::FlightValue::String((&__flight_portable_source).clone())
        }));
        __flight_record
    };
    if (match &(value) {
        crate::FlightValue::Error { stack, .. } => stack.clone(),
        _ => unreachable!("instanceof Error narrowing must contain an Error value"),
    })
    .is_some()
    {
        {
            let __flight_key = "stack".to_owned();
            let __flight_value = {
                let __flight_portable_source = match &(value) {
                    crate::FlightValue::Error { stack, .. } => stack.clone(),
                    _ => unreachable!("instanceof Error narrowing must contain an Error value"),
                };
                match (&__flight_portable_source).as_ref() {
                    Some(value) => crate::FlightValue::String((value).clone()),
                    None => crate::FlightValue::Null,
                }
            };
            if let Some((_, value)) = result.iter_mut().find(|(key, _)| key == &__flight_key) {
                *value = __flight_value;
            } else {
                result.push((__flight_key, __flight_value));
            }
        };
    }
    if (match &(value) {
        crate::FlightValue::Error { cause, .. } => cause.as_deref().cloned(),
        _ => unreachable!("instanceof Error narrowing must contain an Error value"),
    })
    .is_some()
    {
        {
            let __flight_key = "cause".to_owned();
            let __flight_value = {
                let __flight_portable_source = serialize_log_error(
                    (match &(value) {
                        crate::FlightValue::Error { cause, .. } => cause.as_deref().cloned(),
                        _ => unreachable!("instanceof Error narrowing must contain an Error value"),
                    })
                    .unwrap_or(crate::FlightValue::Undefined),
                );
                crate::FlightValue::Record(
                    (&__flight_portable_source)
                        .iter()
                        .map(|(key, value)| (key.clone(), (value).clone()))
                        .collect(),
                )
            };
            if let Some((_, value)) = result.iter_mut().find(|(key, _)| key == &__flight_key) {
                *value = __flight_value;
            } else {
                result.push((__flight_key, __flight_value));
            }
        };
    }
    return result;
}

// Source: upstream/packages/log/src/log.ts:579 (sha256:be4ceb385460fdfe92d225ccec5a14f2c7874239e9940973af530c599edbd366)
pub fn set_log_channel_level(channel: String, level: LogLevel) -> () {
    {
        let __flight_key = (channel).clone();
        let __flight_value = level;
        if let Some((_, value)) = (*_CHANNEL_LEVELS.lock().unwrap())
            .iter_mut()
            .find(|(key, _)| key == &__flight_key)
        {
            *value = __flight_value;
        } else {
            (*_CHANNEL_LEVELS.lock().unwrap()).push((__flight_key, __flight_value));
        }
    };
}

// Source: upstream/packages/log/src/log.ts:585 (sha256:f725cf7f794af8de8328b3d52d8d7d29ef586e010bfe6a6b98242252da724bc5)
pub fn set_log_console_level(level: LogLevel) -> () {
    (*_CONSOLE_LEVEL.lock().unwrap()) = level;
}

// Source: upstream/packages/log/src/log.ts:591 (sha256:512aac4c1304c724e74f6f70dcaae6c829d98e1658ff2d9bfc0074a7342fc948)
pub fn set_log_level(level: LogLevel) -> () {
    (*_LEVEL.lock().unwrap()) = level;
}

// Source: upstream/packages/log/src/log.ts:598 (sha256:209336d8d8c47ace23e20ed60a1b01fef8989cebedca4ceb055ccec8e6be8fdc)
pub fn set_log_redaction_paths(paths: &Vec<String>) -> () {
    _REDACTION_PATHS.lock().unwrap().clear();
    for p in (paths).iter().cloned() {
        _REDACTION_PATHS.lock().unwrap().push(((p).clone()).clone());
    }
}

// Source: upstream/packages/log/src/log.ts:605 (sha256:2bc9fb903427e207762979b599062c8161137b321c79693c8cf136509096cf00)
pub fn set_log_sink(sink: &Option<LogSink>) -> () {
    _SINKS.lock().unwrap().clear();
    if (sink).is_some() {
        _SINKS
            .lock()
            .unwrap()
            .push(((*sink.as_ref().unwrap()).clone()).clone());
    }
}

// Source: upstream/packages/log/src/log.ts:613 (sha256:7e93ecd492fbfe8ba73a3b6fc0a8219b00a38a77a7b5f9a6d4777aeb0cab21f4)
pub fn set_log_transport_backend(backend: &Option<LogTransportBackend>) -> () {
    (*_TRANSPORT_BACKEND.lock().unwrap()) = (*backend).clone();
}

// Source: upstream/packages/log/src/log.ts:619 (sha256:0448f083288815db1b333f074234350d4e975f7e3f6993e75d2f30c8df5ee060)
pub fn start_log_timer(label: String, channel: Option<String>) -> LogTimer {
    return LogTimer {
        __flight_identity: std::sync::Arc::new(()),
        label: (label).clone(),
        channel: (channel).clone(),
        started_at: _timestamp(),
    };
}

// Source: upstream/packages/log/src/log.ts:623 (sha256:2d30bcd0c02cb95e44da4c77e655cdbca357993f9cbd63938f7a0ef9bc025b01)
#[derive(Clone)]
pub(crate) struct BufferedLogSinkState {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub buf: Vec<LogEntry>,
    pub flush: std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>,
    pub timer: Option<crate::FlightTimeout>,
}
impl PartialEq for BufferedLogSinkState {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/log/src/log.ts:629 (sha256:abe9820eb043acba87b1347446bcf11fd02111471bce49830812564752792d85)
#[derive(Clone, Default)]
pub(crate) struct MemoryLogSinkState {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub buf: Vec<LogEntry>,
    pub head: f64,
}
impl PartialEq for MemoryLogSinkState {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/log/src/log.ts:634 (sha256:8dccecefce1b4e90b9f108eadfeae8867e900bc55f94eee2e67a08dc78dc2168)
static _BUFFERED_SINK_STATES: std::sync::LazyLock<
    std::sync::Mutex<Vec<(BufferedLogSink, BufferedLogSinkState)>>,
> = std::sync::LazyLock::new(|| std::sync::Mutex::new(Vec::new()));

// Source: upstream/packages/log/src/log.ts:635 (sha256:21b47dffce4dcb81cf8f783be22dd314bc0c89354581bfc937685d9292deef77)
static _MEMORY_SINK_STATES: std::sync::LazyLock<
    std::sync::Mutex<Vec<(MemoryLogSink, MemoryLogSinkState)>>,
> = std::sync::LazyLock::new(|| std::sync::Mutex::new(Vec::new()));

// Source: upstream/packages/log/src/log.ts:637 (sha256:c775abcf705d7d5e91e76bdd4c61aa39becec02de0d0d91c4f01ca9b3dc3cb56)
static _CONSOLE_METHODS: std::sync::LazyLock<Vec<(LogLevel, String)>> =
    std::sync::LazyLock::new(|| {
        let mut __flight_record = Vec::new();
        __flight_record.push((LogLevel::None, "log".to_owned()));
        __flight_record.push((LogLevel::Error, "error".to_owned()));
        __flight_record.push((LogLevel::Warn, "warn".to_owned()));
        __flight_record.push((LogLevel::Info, "info".to_owned()));
        __flight_record.push((LogLevel::Debug, "debug".to_owned()));
        __flight_record.push((LogLevel::Verbose, "log".to_owned()));
        __flight_record
    });

// Source: upstream/packages/log/src/log.ts:646 (sha256:1768b008161f47c489c3b6b68dfeea08cc58ed51cdaa9b160d17e03281a22dfc)
static _LEVEL_NAMES: std::sync::LazyLock<Vec<(LogLevel, String)>> =
    std::sync::LazyLock::new(|| {
        let mut __flight_record = Vec::new();
        __flight_record.push((LogLevel::None, "none".to_owned()));
        __flight_record.push((LogLevel::Error, "error".to_owned()));
        __flight_record.push((LogLevel::Warn, "warn".to_owned()));
        __flight_record.push((LogLevel::Info, "info".to_owned()));
        __flight_record.push((LogLevel::Debug, "debug".to_owned()));
        __flight_record.push((LogLevel::Verbose, "verbose".to_owned()));
        __flight_record
    });

// Source: upstream/packages/log/src/log.ts:655 (sha256:e562f7e98e8298fddc43dda0f1976be92914f3355267275a3f7197f193ab1a1b)
static _LEVEL_BY_NAME: std::sync::LazyLock<Vec<(String, LogLevel)>> =
    std::sync::LazyLock::new(|| {
        vec![
            ("none".to_owned(), LogLevel::None),
            ("error".to_owned(), LogLevel::Error),
            ("warn".to_owned(), LogLevel::Warn),
            ("info".to_owned(), LogLevel::Info),
            ("debug".to_owned(), LogLevel::Debug),
            ("verbose".to_owned(), LogLevel::Verbose),
        ]
    });

// Source: upstream/packages/log/src/log.ts:664 (sha256:4b4faac93218a491ebbfb8954f69bf28685a1498dac5e3e76a16c1d77a8434ef)
static _CHANNEL_LEVELS: std::sync::LazyLock<std::sync::Mutex<Vec<(String, LogLevel)>>> =
    std::sync::LazyLock::new(|| std::sync::Mutex::new(Vec::new()));

// Source: upstream/packages/log/src/log.ts:665 (sha256:48a8dcb3b0a5993958e6b77ff7d490cbb843a01519d5b6daf637ad0673968370)
static _ONCE_KEYS: std::sync::LazyLock<std::sync::Mutex<Vec<String>>> =
    std::sync::LazyLock::new(|| std::sync::Mutex::new(Vec::new()));

// Source: upstream/packages/log/src/log.ts:666 (sha256:504e89f543bb3705f0a7345a276b430eff457709860cf172f146ecb49e956507)
static _REDACTION_PATHS: std::sync::LazyLock<std::sync::Mutex<Vec<String>>> =
    std::sync::LazyLock::new(|| std::sync::Mutex::new(vec![]));

// Source: upstream/packages/log/src/log.ts:667 (sha256:e95497c8f31f0a76bc02083fad722a7a2edeb0d07453d72343195ef793af9360)
static _SERIALIZERS: std::sync::LazyLock<
    std::sync::Mutex<
        Vec<(
            String,
            std::sync::Arc<
                std::sync::Mutex<
                    Box<
                        dyn FnMut(crate::FlightValue) -> Vec<(String, crate::FlightValue)>
                            + Send
                            + 'static,
                    >,
                >,
            >,
        )>,
    >,
> = std::sync::LazyLock::new(|| std::sync::Mutex::new(Vec::new()));

// Source: upstream/packages/log/src/log.ts:668 (sha256:55040bb8cc45815fddccba18805d226f5bf00f0ab8ff360507efa86bf0d4f18a)
static _SINKS: std::sync::LazyLock<std::sync::Mutex<Vec<LogSink>>> =
    std::sync::LazyLock::new(|| std::sync::Mutex::new(vec![]));

// Source: upstream/packages/log/src/log.ts:669 (sha256:dbe0d0d8162b962d9edc34383782acc904ec9f038ebdb31775dc7220278bb5e0)
static _SPAN_STACK: std::sync::LazyLock<std::sync::Mutex<Vec<LogSpan>>> =
    std::sync::LazyLock::new(|| std::sync::Mutex::new(vec![]));

// Source: upstream/packages/log/src/log.ts:671 (sha256:4170134a1d1ac4359e567ee04d9a0f56a1b1e46c449e31e39ab0cce7393af202)
static _CONSOLE_LEVEL: std::sync::LazyLock<std::sync::Mutex<LogLevel>> =
    std::sync::LazyLock::new(|| std::sync::Mutex::new(LogLevel::Info));

// Source: upstream/packages/log/src/log.ts:672 (sha256:7ed9d659489176028390f6825f6bc69849a959a913849f1ff9faa25795150211)
static _GROUP_DEPTH: std::sync::LazyLock<std::sync::Mutex<f64>> =
    std::sync::LazyLock::new(|| std::sync::Mutex::new(0.0_f64));

// Source: upstream/packages/log/src/log.ts:673 (sha256:d60c87911e5d8deddd4d85f44c5881ae81dce35da609c04e0a40abdb6c3d610a)
static _LEVEL: std::sync::LazyLock<std::sync::Mutex<LogLevel>> =
    std::sync::LazyLock::new(|| std::sync::Mutex::new(LogLevel::Verbose));

// Source: upstream/packages/log/src/log.ts:674 (sha256:c6d7c18ae8b174ac5cc5345cf8a69e2dc40e9dc25add31f4158b5487084e6d6c)
static _LOG_SIGNALS: std::sync::LazyLock<std::sync::Mutex<Option<LogSignals>>> =
    std::sync::LazyLock::new(|| std::sync::Mutex::new(None));

// Source: upstream/packages/log/src/log.ts:675 (sha256:bb87135de7b6b7e11dae257be17f25120480dfe5c3d251a524ec67b93f4303e1)
static _TRANSPORT_BACKEND: std::sync::LazyLock<std::sync::Mutex<Option<LogTransportBackend>>> =
    std::sync::LazyLock::new(|| std::sync::Mutex::new(None));

// Source: upstream/packages/log/src/log.ts:678 (sha256:a73f8d4506e0e15cce747ab78ca239d58f12af7faae2280a1a9a0c7fcf93bd41)
#[derive(Clone, Default)]
struct ApplySerializersRecord5 {
    __flight_identity: std::sync::Arc<()>,
}
impl PartialEq for ApplySerializersRecord5 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

fn _apply_serializers(
    data: &Vec<(String, crate::FlightValue)>,
) -> Vec<(String, crate::FlightValue)> {
    if (((*_SERIALIZERS.lock().unwrap()).len() as f64) == 0.0_f64) {
        return data.clone();
    }
    let mut result: Vec<(String, crate::FlightValue)> = {
        let mut __flight_record = Vec::new();
        __flight_record
    };
    for __iteration10 in (((*data).clone()).clone()).iter().cloned() {
        let key = __iteration10.0.clone();
        let value = __iteration10.1.clone();
        if (((!(matches!(&((value).clone()), crate::FlightValue::Null)))
            && ((match &((value).clone()) {
                crate::FlightValue::Undefined => "undefined",
                crate::FlightValue::Null
                | crate::FlightValue::Array(_)
                | crate::FlightValue::Record(_)
                | crate::FlightValue::Error { .. }
                | crate::FlightValue::Object => "object",
                crate::FlightValue::Bool(_) => "boolean",
                crate::FlightValue::Number(_) => "number",
                crate::FlightValue::String(_) => "string",
                crate::FlightValue::Function => "function",
                crate::FlightValue::Symbol => "symbol",
            })
            .to_owned()
                == "object"))
            && ({
                let __flight_key = "__kind".to_owned();
                matches!(&((value).clone()), crate::FlightValue::Record(entries) if entries.iter().any(|(key, _)| key == &__flight_key))
            }))
            && ((match &(match (value).clone() {
                crate::FlightValue::Record(entries) => entries,
                _ => panic!("TypeScript Record cast received a non-record portable value"),
            }
            .iter()
            .find(|(entry_key, _)| entry_key == &"__kind".to_owned())
            .map(|(_, value)| value.clone())
            .expect("TypeScript Record key was absent"))
            {
                crate::FlightValue::Undefined => "undefined",
                crate::FlightValue::Null
                | crate::FlightValue::Array(_)
                | crate::FlightValue::Record(_)
                | crate::FlightValue::Error { .. }
                | crate::FlightValue::Object => "object",
                crate::FlightValue::Bool(_) => "boolean",
                crate::FlightValue::Number(_) => "number",
                crate::FlightValue::String(_) => "string",
                crate::FlightValue::Function => "function",
                crate::FlightValue::Symbol => "symbol",
            })
            .to_owned()
                == "string")
        {
            let kind = match match (value).clone() {
                crate::FlightValue::Record(entries) => entries,
                _ => panic!("TypeScript Record cast received a non-record portable value"),
            }
            .iter()
            .find(|(entry_key, _)| entry_key == &"__kind".to_owned())
            .map(|(_, value)| value.clone())
            .expect("TypeScript Record key was absent")
            {
                crate::FlightValue::String(value) => value,
                _ => panic!("TypeScript String cast received an incompatible portable value"),
            };
            let fn_ = (*_SERIALIZERS.lock().unwrap())
                .iter()
                .find(|(entry_key, _)| entry_key == &(kind).clone())
                .map(|(_, value)| value.clone());
            {
                let __flight_key = (key).clone();
                let __flight_value = if (fn_).is_some() {
                    {
                        let __flight_portable_source = {
                            let __flight_callback = (fn_.as_ref().unwrap()).clone();
                            let __flight_result =
                                __flight_callback.lock().unwrap()((value).clone());
                            __flight_result
                        };
                        crate::FlightValue::Record(
                            (&__flight_portable_source)
                                .iter()
                                .map(|(key, value)| (key.clone(), (value).clone()))
                                .collect(),
                        )
                    }
                } else {
                    (value).clone()
                };
                if let Some((_, value)) = result.iter_mut().find(|(key, _)| key == &__flight_key) {
                    *value = __flight_value;
                } else {
                    result.push((__flight_key, __flight_value));
                }
            };
        } else {
            {
                let __flight_key = (key).clone();
                let __flight_value = (value).clone();
                if let Some((_, value)) = result.iter_mut().find(|(key, _)| key == &__flight_key) {
                    *value = __flight_value;
                } else {
                    result.push((__flight_key, __flight_value));
                }
            };
        }
    }
    return result;
}

// Source: upstream/packages/log/src/log.ts:699 (sha256:81eb779db390901d7bead526a299029a68af484f606cbc13807defc4f4ceb131)
fn _apply_redaction(data: &Vec<(String, crate::FlightValue)>) -> Vec<(String, crate::FlightValue)> {
    let mut result = (data).clone();
    for path in (_REDACTION_PATHS.lock().unwrap()).iter().cloned() {
        let parts = (path)
            .split(".".to_owned().as_str())
            .map(|part| part.to_owned())
            .collect::<Vec<_>>();
        _redact_path(&mut result, &parts, 0.0_f64);
    }
    return result;
}

// Source: upstream/packages/log/src/log.ts:709 (sha256:7c36f4fb96c9a70f85845e21ef2f365f794eacfeeefddf68349be258c9a648f8)
fn _redact_path(obj: &mut Vec<(String, crate::FlightValue)>, parts: &Vec<String>, idx: f64) -> () {
    let key = parts[idx as usize].clone();
    if (!{
        let __flight_key = (key).clone();
        obj.iter().any(|(key, _)| key == &__flight_key)
    }) {
        return;
    }
    if (idx == ((parts.len() as f64) - 1.0_f64)) {
        {
            let __flight_key = (key).clone();
            let __flight_value = {
                let __flight_portable_source = "[REDACTED]".to_owned();
                crate::FlightValue::String((&__flight_portable_source).clone())
            };
            if let Some((_, value)) = obj.iter_mut().find(|(key, _)| key == &__flight_key) {
                *value = __flight_value;
            } else {
                obj.push((__flight_key, __flight_value));
            }
        };
        return;
    }
    let next = obj
        .iter()
        .find(|(entry_key, _)| entry_key == &(key).clone())
        .map(|(_, value)| value.clone())
        .clone();
    if (((next).is_some())
        && ((match (next).as_ref() {
            None => "undefined",
            Some(value) => match value {
                crate::FlightValue::Undefined => "undefined",
                crate::FlightValue::Null
                | crate::FlightValue::Array(_)
                | crate::FlightValue::Record(_)
                | crate::FlightValue::Error { .. }
                | crate::FlightValue::Object => "object",
                crate::FlightValue::Bool(_) => "boolean",
                crate::FlightValue::Number(_) => "number",
                crate::FlightValue::String(_) => "string",
                crate::FlightValue::Function => "function",
                crate::FlightValue::Symbol => "symbol",
            },
        })
        .to_owned()
            == "object"))
        && (!false)
    {
        {
            let __flight_key = (key).clone();
            let __flight_value = crate::FlightValue::Record({
                let mut __flight_record = Vec::new();
                let __flight_spread_0 = {
                    let __flight_portable_source = match (next.as_ref().unwrap()).clone() {
                        crate::FlightValue::Record(entries) => entries,
                        _ => panic!("TypeScript Record cast received a non-record portable value"),
                    };
                    crate::FlightValue::Record(
                        (&__flight_portable_source)
                            .iter()
                            .map(|(key, value)| (key.clone(), (value).clone()))
                            .collect(),
                    )
                };
                match __flight_spread_0 {
                    crate::FlightValue::Record(entries) => {
                        for (__flight_key, __flight_value) in entries {
                            if let Some((_, __flight_existing)) = __flight_record
                                .iter_mut()
                                .find(|(existing, _)| existing == &__flight_key)
                            {
                                *__flight_existing = __flight_value;
                            } else {
                                __flight_record.push((__flight_key, __flight_value));
                            }
                        }
                    }
                    crate::FlightValue::Array(values) => {
                        for (__flight_index, __flight_value) in values.into_iter().enumerate() {
                            let __flight_key = __flight_index.to_string();
                            if let Some((_, __flight_existing)) = __flight_record
                                .iter_mut()
                                .find(|(existing, _)| existing == &__flight_key)
                            {
                                *__flight_existing = __flight_value;
                            } else {
                                __flight_record.push((__flight_key, __flight_value));
                            }
                        }
                    }
                    crate::FlightValue::Undefined
                    | crate::FlightValue::Null
                    | crate::FlightValue::Bool(_)
                    | crate::FlightValue::Number(_)
                    | crate::FlightValue::Function
                    | crate::FlightValue::Symbol => {}
                    crate::FlightValue::String(_) => panic!(
                        "portable object spread of strings requires UTF-16 property lowering"
                    ),
                    crate::FlightValue::Error { .. } | crate::FlightValue::Object => {
                        panic!("portable object spread cannot inspect an opaque host object")
                    }
                }
                __flight_record
            });
            if let Some((_, value)) = obj.iter_mut().find(|(key, _)| key == &__flight_key) {
                *value = __flight_value;
            } else {
                obj.push((__flight_key, __flight_value));
            }
        };
        _redact_path(
            {
                let __flight_key = (key).clone();
                let __flight_value = obj
                    .iter_mut()
                    .find(|(key, _)| key == &__flight_key)
                    .map(|(_, value)| value)
                    .expect("TypeScript Record key was absent");
                match __flight_value {
                    crate::FlightValue::Record(entries) => entries,
                    _ => panic!("TypeScript Record cast received a non-record portable value"),
                }
            },
            parts,
            (idx + 1.0_f64),
        );
    }
}

// Source: upstream/packages/log/src/log.ts:724 (sha256:5687a087982a83a17d7f858b4d859883ad8675360b262fbcd80bed9a938e6c77)
fn _emit_to_sinks(entry: &LogEntry) -> () {
    for sink in (_SINKS.lock().unwrap()).iter().cloned() {
        {
            let __flight_callback = (sink).clone();
            let __flight_result = __flight_callback.lock().unwrap()((*entry).clone());
            __flight_result
        };
    }
    if ((*_LOG_SIGNALS.lock().unwrap()).clone()).is_some() {
        emit_signal(
            ((*_LOG_SIGNALS.lock().unwrap())
                .as_ref()
                .unwrap()
                .on_log_entry)
                .clone(),
            ((*entry).clone(),),
        );
        if (entry.level == LogLevel::Error) {
            emit_signal(
                ((*_LOG_SIGNALS.lock().unwrap())
                    .as_ref()
                    .unwrap()
                    .on_log_error)
                    .clone(),
                ((*entry).clone(),),
            );
        }
    }
}

// Source: upstream/packages/log/src/log.ts:732 (sha256:5495059cd917240faea94fd4eb557824a2ccc294cea5b22cadb5e15ed9240347)
fn _merge_context_fields(context: &LogContext, data: &LogData) -> LogData {
    if ((context
        .fields
        .iter()
        .map(|(entry_key, _)| entry_key.clone())
        .collect::<Vec<_>>()
        .len() as f64)
        == 0.0_f64)
    {
        return data.clone();
    }
    if ((match &(data) {
        crate::FlightUnion2::A(_) => "string",
        crate::FlightUnion2::B(value) => "object",
    })
    .to_owned()
        == "string")
    {
        return flighthq_types::LogData::B({
            let mut __flight_record = Vec::new();
            let __flight_key_0 = "msg".to_owned();
            let __flight_value_0 = {
                let __flight_portable_source = match (*data).clone() {
                    LogData::A(value) => value,
                    LogData::B(_) => panic!("TypeScript union narrowing failed"),
                };
                crate::FlightValue::String((&__flight_portable_source).clone())
            };
            if let Some((_, __flight_existing)) = __flight_record
                .iter_mut()
                .find(|(key, _)| key == &__flight_key_0)
            {
                *__flight_existing = __flight_value_0;
            } else {
                __flight_record.push((__flight_key_0, __flight_value_0));
            }
            let __flight_spread_1 = (context.fields).clone();
            for (__flight_key, __flight_value) in __flight_spread_1.iter().cloned() {
                if let Some((_, __flight_existing)) = __flight_record
                    .iter_mut()
                    .find(|(key, _)| key == &__flight_key)
                {
                    *__flight_existing = __flight_value;
                } else {
                    __flight_record.push((__flight_key, __flight_value));
                }
            }
            __flight_record
        });
    }
    return flighthq_types::LogData::B({
        let mut __flight_record = Vec::new();
        let __flight_spread_0 = (context.fields).clone();
        for (__flight_key, __flight_value) in __flight_spread_0.iter().cloned() {
            if let Some((_, __flight_existing)) = __flight_record
                .iter_mut()
                .find(|(key, _)| key == &__flight_key)
            {
                *__flight_existing = __flight_value;
            } else {
                __flight_record.push((__flight_key, __flight_value));
            }
        }
        let __flight_spread_1 = match (*data).clone() {
            LogData::A(_) => panic!("TypeScript union narrowing failed"),
            LogData::B(value) => value,
        };
        for (__flight_key, __flight_value) in __flight_spread_1.iter().cloned() {
            if let Some((_, __flight_existing)) = __flight_record
                .iter_mut()
                .find(|(key, _)| key == &__flight_key)
            {
                *__flight_existing = __flight_value;
            } else {
                __flight_record.push((__flight_key, __flight_value));
            }
        }
        __flight_record
    });
}

// Source: upstream/packages/log/src/log.ts:741 (sha256:d3bcaac1968dbb501f40fb4cbc6e540c346a5a1fbbf5264613098bcf1d2763a0)
#[derive(Clone, Default)]
struct MergeSpanFieldsRecord5 {
    __flight_identity: std::sync::Arc<()>,
}
impl PartialEq for MergeSpanFieldsRecord5 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

fn _merge_span_fields(data: &LogData, _channel: Option<String>) -> LogData {
    if ((_SPAN_STACK.lock().unwrap().len() as f64) == 0.0_f64) {
        return data.clone();
    }
    let span_fields: Vec<(String, crate::FlightValue)> = {
        let mut __flight_record = Vec::new();
        __flight_record
    };
    for span in (_SPAN_STACK.lock().unwrap()).iter().cloned() {
        crate::host_value::<()>("host.assign");
    }
    if ((span_fields
        .iter()
        .map(|(entry_key, _)| entry_key.clone())
        .collect::<Vec<_>>()
        .len() as f64)
        == 0.0_f64)
    {
        return data.clone();
    }
    if ((match &(data) {
        crate::FlightUnion2::A(_) => "string",
        crate::FlightUnion2::B(value) => "object",
    })
    .to_owned()
        == "string")
    {
        return flighthq_types::LogData::B({
            let mut __flight_record = Vec::new();
            let __flight_key_0 = "msg".to_owned();
            let __flight_value_0 = {
                let __flight_portable_source = match (*data).clone() {
                    LogData::A(value) => value,
                    LogData::B(_) => panic!("TypeScript union narrowing failed"),
                };
                crate::FlightValue::String((&__flight_portable_source).clone())
            };
            if let Some((_, __flight_existing)) = __flight_record
                .iter_mut()
                .find(|(key, _)| key == &__flight_key_0)
            {
                *__flight_existing = __flight_value_0;
            } else {
                __flight_record.push((__flight_key_0, __flight_value_0));
            }
            let __flight_spread_1 = (span_fields).clone();
            for (__flight_key, __flight_value) in __flight_spread_1.iter().cloned() {
                if let Some((_, __flight_existing)) = __flight_record
                    .iter_mut()
                    .find(|(key, _)| key == &__flight_key)
                {
                    *__flight_existing = __flight_value;
                } else {
                    __flight_record.push((__flight_key, __flight_value));
                }
            }
            __flight_record
        });
    }
    return flighthq_types::LogData::B({
        let mut __flight_record = Vec::new();
        let __flight_spread_0 = (span_fields).clone();
        for (__flight_key, __flight_value) in __flight_spread_0.iter().cloned() {
            if let Some((_, __flight_existing)) = __flight_record
                .iter_mut()
                .find(|(key, _)| key == &__flight_key)
            {
                *__flight_existing = __flight_value;
            } else {
                __flight_record.push((__flight_key, __flight_value));
            }
        }
        let __flight_spread_1 = match (*data).clone() {
            LogData::A(_) => panic!("TypeScript union narrowing failed"),
            LogData::B(value) => value,
        };
        for (__flight_key, __flight_value) in __flight_spread_1.iter().cloned() {
            if let Some((_, __flight_existing)) = __flight_record
                .iter_mut()
                .find(|(key, _)| key == &__flight_key)
            {
                *__flight_existing = __flight_value;
            } else {
                __flight_record.push((__flight_key, __flight_value));
            }
        }
        __flight_record
    });
}

// Source: upstream/packages/log/src/log.ts:754 (sha256:2090ce9e096083230d4196d8df9e672c8a7e96dbde202c8f7390500b4dd05baf)
fn _passes_level_gate(level: LogLevel, channel: Option<String>) -> bool {
    if ((_SINKS.lock().unwrap().len() as f64) == 0.0_f64)
        && (((*_LOG_SIGNALS.lock().unwrap()).clone()).is_none())
    {
        return false;
    }
    let gate = if ((channel).is_some())
        && ((*_CHANNEL_LEVELS.lock().unwrap())
            .iter()
            .any(|(entry_key, _)| entry_key == &(channel).clone().unwrap()))
    {
        (*_CHANNEL_LEVELS.lock().unwrap())
            .iter()
            .find(|(entry_key, _)| entry_key == &(channel.as_ref().unwrap()).clone())
            .map(|(_, value)| value.clone())
    } else {
        Some((*_LEVEL.lock().unwrap()).clone())
    };
    return ((gate).as_ref().is_some_and(|value| level <= *value)) && (level != LogLevel::None);
}

// Source: upstream/packages/log/src/log.ts:760 (sha256:d585ebf20bfc2a78ee2f52cac90944d899e116c49d935d28c050cf6b0591a643)
fn _timestamp() -> f64 {
    return crate::host_value::<f64>("host.call");
}

// Source: upstream/packages/log/src/log.ts:764 (sha256:45f2fe54e019dace1c01f279269d85ace2486e38de7971fa46a2733350b7416e)
fn _default_json_formatter(entry: &LogEntry) -> String {
    let level = entry.level;
    let channel = (entry.channel).clone();
    return crate::flight_json_stringify(
        &(crate::FlightValue::Record({
            let mut __flight_record = Vec::new();
            let __flight_key_0 = "__flight".to_owned();
            let __flight_value_0 = {
                let __flight_portable_source = true;
                crate::FlightValue::Bool(*(&__flight_portable_source))
            };
            if let Some((_, __flight_existing)) = __flight_record
                .iter_mut()
                .find(|(existing, _)| existing == &__flight_key_0)
            {
                *__flight_existing = __flight_value_0;
            } else {
                __flight_record.push((__flight_key_0, __flight_value_0));
            }
            let __flight_key_1 = "t".to_owned();
            let __flight_value_1 = {
                let __flight_portable_source = _timestamp();
                crate::FlightValue::Number(*(&__flight_portable_source) as f64)
            };
            if let Some((_, __flight_existing)) = __flight_record
                .iter_mut()
                .find(|(existing, _)| existing == &__flight_key_1)
            {
                *__flight_existing = __flight_value_1;
            } else {
                __flight_record.push((__flight_key_1, __flight_value_1));
            }
            let __flight_key_2 = "level".to_owned();
            let __flight_value_2 = {
                let __flight_portable_source = _LEVEL_NAMES
                    .iter()
                    .find(|(entry_key, _)| entry_key == &level)
                    .map(|(_, value)| value.clone())
                    .clone();
                match (&__flight_portable_source).as_ref() {
                    Some(value) => crate::FlightValue::String((value).clone()),
                    None => crate::FlightValue::Null,
                }
            };
            if let Some((_, __flight_existing)) = __flight_record
                .iter_mut()
                .find(|(existing, _)| existing == &__flight_key_2)
            {
                *__flight_existing = __flight_value_2;
            } else {
                __flight_record.push((__flight_key_2, __flight_value_2));
            }
            let __flight_key_3 = "channel".to_owned();
            let __flight_value_3 = {
                let __flight_portable_source = (channel).clone();
                match (&__flight_portable_source).as_ref() {
                    Some(value) => crate::FlightValue::String((value).clone()),
                    None => crate::FlightValue::Null,
                }
            };
            if let Some((_, __flight_existing)) = __flight_record
                .iter_mut()
                .find(|(existing, _)| existing == &__flight_key_3)
            {
                *__flight_existing = __flight_value_3;
            } else {
                __flight_record.push((__flight_key_3, __flight_value_3));
            }
            let __flight_key_4 = "data".to_owned();
            let __flight_value_4 = if ((match &(entry.data) {
                crate::FlightUnion2::A(_) => "string",
                crate::FlightUnion2::B(value) => "object",
            })
            .to_owned()
                == "string")
            {
                crate::FlightValue::Record({
                    let mut __flight_record = Vec::new();
                    let __flight_key_0 = "msg".to_owned();
                    let __flight_value_0 = {
                        let __flight_portable_source = match (entry.data).clone() {
                            LogData::A(value) => value,
                            LogData::B(_) => panic!("TypeScript union narrowing failed"),
                        };
                        crate::FlightValue::String((&__flight_portable_source).clone())
                    };
                    if let Some((_, __flight_existing)) = __flight_record
                        .iter_mut()
                        .find(|(existing, _)| existing == &__flight_key_0)
                    {
                        *__flight_existing = __flight_value_0;
                    } else {
                        __flight_record.push((__flight_key_0, __flight_value_0));
                    }
                    __flight_record
                })
            } else {
                {
                    let __flight_portable_source = match (entry.data).clone() {
                        LogData::A(_) => panic!("TypeScript union narrowing failed"),
                        LogData::B(value) => value,
                    };
                    crate::FlightValue::Record(
                        (&__flight_portable_source)
                            .iter()
                            .map(|(key, value)| (key.clone(), (value).clone()))
                            .collect(),
                    )
                }
            };
            if let Some((_, __flight_existing)) = __flight_record
                .iter_mut()
                .find(|(existing, _)| existing == &__flight_key_4)
            {
                *__flight_existing = __flight_value_4;
            } else {
                __flight_record.push((__flight_key_4, __flight_value_4));
            }
            __flight_record
        })),
    )
    .expect("JSON.stringify encountered an opaque host object")
    .expect("JSON.stringify returned undefined where Rust requires String");
}

// Source: upstream/packages/log/src/log.ts:776 (sha256:3f220612b482f6b947e7c44fec4f0073691ff4d9bbc1092e4e523dd6f500ad04)
fn _write_console_capture_entry(entry: &LogEntry, envelope_formatter: LogFormatter) -> () {
    return;
}

// Source: upstream/packages/log/src/log.ts:791 (sha256:6444c1d2ac8bee5c434409d7798b3e2238035d9841798aeadec7ac0d6d3879ad)
fn _write_console_log_entry(entry: &LogEntry, formatter: LogFormatter) -> () {
    return;
}
