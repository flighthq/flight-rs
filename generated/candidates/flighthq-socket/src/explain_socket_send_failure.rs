// @generated from upstream/packages/socket/src/explainSocketSendFailure.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::{Socket, SocketReadyState, SocketSendFailureExplanation};

// Source: upstream/packages/socket/src/explainSocketSendFailure.ts:5 (sha256:fc1d976cc1aa347ddf4642a40a179a967c408157d18b0440462e0d663e7b5864)
pub fn explain_socket_send_failure(socket: &Socket) -> Option<SocketSendFailureExplanation> {
    if socket.runtime.disposed {
        return Some(SocketSendFailureExplanation {
            __flight_identity: std::sync::Arc::new(()),
            reason: "disposed".to_owned(),
            ready_state: crate::FlightUnion2::<String, SocketReadyState>::A("closed".to_owned()),
            url: (socket.url).clone(),
        });
    }
    if ((socket.runtime.connection).clone()).is_none() {
        return Some(SocketSendFailureExplanation {
            __flight_identity: std::sync::Arc::new(()),
            reason: "no-connection".to_owned(),
            ready_state: crate::FlightUnion2::<String, SocketReadyState>::A(
                (socket.runtime.ready_state).clone(),
            ),
            url: (socket.url).clone(),
        });
    }
    if ((socket.runtime.ready_state).clone() != "open") {
        return Some(SocketSendFailureExplanation {
            __flight_identity: std::sync::Arc::new(()),
            reason: "not-open".to_owned(),
            ready_state: crate::FlightUnion2::<String, SocketReadyState>::A(
                (socket.runtime.ready_state).clone(),
            ),
            url: (socket.url).clone(),
        });
    }
    return None;
}
