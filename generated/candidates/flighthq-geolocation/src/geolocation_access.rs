// @generated from upstream/packages/geolocation/src/geolocationAccess.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::{GeolocationAccessOutcome, HostGeolocationCapability};

// Source: upstream/packages/geolocation/src/geolocationAccess.ts:14 (sha256:78ed2c86243217f80d10e0018b1648a78b0ef3b2a7d6d80b86dea50cd4f6d27e)
#[derive(Clone, Default)]
struct PromptForGeolocationAccessRecord1 {
    __flight_identity: std::sync::Arc<()>,
    reason: String,
}
impl PartialEq for PromptForGeolocationAccessRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn prompt_for_geolocation_access(
    host_geolocation: Option<HostGeolocationCapability>,
) -> crate::FlightTask<GeolocationAccessOutcome> {
    crate::FlightTask::start(
        async move {
            if ((host_geolocation).is_none()) || ("function".to_owned() != "function") {
                return Ok(GeolocationAccessOutcome {
                    __flight_identity: std::sync::Arc::new(()),
                    reason: "runtime-unavailable".to_owned(),
                });
            }
            return match (async {
                {
                    return Ok(({
                        let __flight_callback =
                            (host_geolocation.as_ref().unwrap().prompt_for_access).clone();
                        let __flight_result = __flight_callback.lock().unwrap()();
                        __flight_result
                    })
                    .await?);
                }
            })
            .await
            {
                Ok(__flight_value) => Ok(__flight_value),
                Err(crate::FlightTaskError::Rejection(_)) => {
                    (async {
                        {
                            return Ok(GeolocationAccessOutcome {
                                __flight_identity: std::sync::Arc::new(()),
                                reason: "operation-failed".to_owned(),
                            });
                        }
                    })
                    .await
                }
                Err(__flight_error) => Err(__flight_error),
            };
        },
        crate::FlightTaskOrigin {
            package: "@flighthq/geolocation",
            source: "upstream/packages/geolocation/src/geolocationAccess.ts",
            line: 14_u32,
            column: 1_u32,
            lexical_path: "promptForGeolocationAccess",
            fingerprint: "sha256:78ed2c86243217f80d10e0018b1648a78b0ef3b2a7d6d80b86dea50cd4f6d27e",
        },
    )
}
