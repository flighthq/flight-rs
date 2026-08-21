// @generated from upstream/packages/socket/src/enableSocketGuards.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::set_socket_guard;
use flighthq_log::log_once;
use flighthq_types::{LogData, LogDataProvider, LogLevel, SocketGuardNotice};

// Source: upstream/packages/socket/src/enableSocketGuards.ts:7 (sha256:30c6926d440c6aec0289e645850a58c35e8f2549f0afcb6c5c976767c0a13635)
pub fn are_socket_guards_enabled() -> bool {
    return _ENABLED.load(std::sync::atomic::Ordering::Relaxed);
}

// Source: upstream/packages/socket/src/enableSocketGuards.ts:11 (sha256:e4387cad258f84e476d72a85ba2bc92ca961aeaba1876773d4dfd8a9d1a59c2b)
pub fn disable_socket_guards() -> () {
    set_socket_guard(&(None));
    _ENABLED.store(false, std::sync::atomic::Ordering::Relaxed);
}

// Source: upstream/packages/socket/src/enableSocketGuards.ts:18 (sha256:5c749e4b47204918174a445a91c316505ad9750227eb4539ee4ec7d05edd62e7)
pub fn enable_socket_guards() -> () {
    set_socket_guard(
        &(Some(std::sync::Arc::new(std::sync::Mutex::new(Box::new(
            move |__flight_argument_0: SocketGuardNotice| -> () {
                warn_on_socket_misuse(&__flight_argument_0)
            },
        )
            as Box<dyn FnMut(SocketGuardNotice) -> () + Send + 'static>)))),
    );
    _ENABLED.store(true, std::sync::atomic::Ordering::Relaxed);
}

// Source: upstream/packages/socket/src/enableSocketGuards.ts:23 (sha256:3ee732e925aa905b688a9cade13522c71ef303203a234169fc9bc877cdf11330)
fn warn_on_socket_misuse(notice: &SocketGuardNotice) -> () {
    let url = ((match &((*notice).clone()) {
        crate::FlightUnion2::A(value) => (value).socket.clone(),
        crate::FlightUnion2::B(value) => (value).socket.clone(),
    })
    .url)
        .clone();
    let message = if matches!(&(notice), flighthq_types::SocketGuardNotice::A(_)) {
        "createSocket: active backend returned no connection — call setSocketBackend(...) with a backend that supports this transport".to_owned()
    } else {
        format!(
            "{}: socket is already disposed — call createSocket(...) to create a new socket",
            ((match (*notice).clone() {
                flighthq_types::SocketGuardNotice::A(_) =>
                    panic!("TypeScript union narrowing failed"),
                flighthq_types::SocketGuardNotice::B(value) => value,
            })
            .operation)
                .clone()
        )
    };
    log_once(
        format!(
            "socket:{}:{}",
            match &((*notice).clone()) {
                crate::FlightUnion2::A(value) => (value).operation.clone(),
                crate::FlightUnion2::B(value) => (value).operation.clone(),
            },
            match &((*notice).clone()) {
                crate::FlightUnion2::A(value) => (value).reason.clone(),
                crate::FlightUnion2::B(value) => (value).reason.clone(),
            }
        ),
        LogLevel::Warn,
        &(crate::FlightUnion2::<LogData, LogDataProvider>::A(crate::FlightUnion2::<
            String,
            Vec<(String, crate::FlightValue)>,
        >::B({
            let mut __flight_record = Vec::new();
            __flight_record.push(("message".to_owned(), {
                let __flight_portable_source = (message).clone();
                crate::FlightValue::String((&__flight_portable_source).clone())
            }));
            __flight_record.push(("operation".to_owned(), {
                let __flight_portable_source = match &((*notice).clone()) {
                    crate::FlightUnion2::A(value) => (value).operation.clone(),
                    crate::FlightUnion2::B(value) => (value).operation.clone(),
                };
                crate::FlightValue::String((&__flight_portable_source).clone())
            }));
            __flight_record.push(("reason".to_owned(), {
                let __flight_portable_source = match &((*notice).clone()) {
                    crate::FlightUnion2::A(value) => (value).reason.clone(),
                    crate::FlightUnion2::B(value) => (value).reason.clone(),
                };
                crate::FlightValue::String((&__flight_portable_source).clone())
            }));
            __flight_record.push(("url".to_owned(), {
                let __flight_portable_source = (url).clone();
                crate::FlightValue::String((&__flight_portable_source).clone())
            }));
            __flight_record
        }))),
        Some(("socket".to_owned()).clone()),
    );
}

// Source: upstream/packages/socket/src/enableSocketGuards.ts:37 (sha256:7229172ec48331971d8459b2c68e79d664373be87a769eb22f5997a746fc4f85)
static _ENABLED: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
