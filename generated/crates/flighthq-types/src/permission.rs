// @generated from upstream/packages/types/src/Permission.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::HostNotificationPermissionCapability;

// Source: upstream/packages/types/src/Permission.ts:7 (sha256:8e9328571f6365aae27a1489a5fb3401db56cbc25d4c3858824e7e6d95d9354b)
pub type PermissionName = String;

// Source: upstream/packages/types/src/Permission.ts:22 (sha256:30b2b229abb900b58e118a111f38eaa67c2d375e4a109a85f5f520345328c9ec)
pub type PermissionState = String;

// Source: upstream/packages/types/src/Permission.ts:24 (sha256:52160301e125399798e4f5e0b23e05e62679df1563ae118d82c14ee13ab2df1c)
pub type PermissionQueryFailureReason = String;

// Source: upstream/packages/types/src/Permission.ts:28 (sha256:e97cce17cadd3816ec6f7115475717cf68b3d4ac354fe736847574a0792ea754)
#[derive(Clone)]
pub struct PermissionQueryOutcome {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub reason: crate::FlightUnion2<String, PermissionQueryFailureReason>,
    pub state: Option<crate::FlightUnion2<PermissionState, Option<PermissionState>>>,
}
impl PartialEq for PermissionQueryOutcome {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Permission.ts:37 (sha256:12dbcdef839572682f6e37948e931c32448ad7412f9cb40e2ca5c1b45c2ba25c)
pub type PermissionRequestFailureReason = String;

// Source: upstream/packages/types/src/Permission.ts:46 (sha256:914a2faac984b627be0e74bb8d6eea62f802d09260ec17e9f877abb3d7e2560d)
#[derive(Clone)]
pub struct PermissionRequestOutcome {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub reason: crate::FlightUnion2<String, PermissionRequestFailureReason>,
    pub state: Option<crate::FlightUnion2<String, Option<PermissionState>>>,
}
impl PartialEq for PermissionRequestOutcome {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Permission.ts:58 (sha256:d96c08f9b0c90a54156109ed6a22f70e3dcee26f0175dc7844e060df9c48ebac)
#[derive(Clone)]
pub struct HostPermissionsCapability {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub notification: HostNotificationPermissionCapability,
    pub query_permission: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(PermissionName) -> crate::FlightTask<PermissionQueryOutcome>
                    + Send
                    + 'static,
            >,
        >,
    >,
    pub request_media_access: std::sync::Arc<
        std::sync::Mutex<
            Box<dyn FnMut(String) -> crate::FlightTask<PermissionRequestOutcome> + Send + 'static>,
        >,
    >,
    pub request_wake_lock: std::sync::Arc<
        std::sync::Mutex<
            Box<dyn FnMut() -> crate::FlightTask<PermissionRequestOutcome> + Send + 'static>,
        >,
    >,
}
impl PartialEq for HostPermissionsCapability {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
