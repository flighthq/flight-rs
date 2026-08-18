// @generated from upstream/packages/types/src/Permission.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/Permission.ts:8 (sha256:8e9328571f6365aae27a1489a5fb3401db56cbc25d4c3858824e7e6d95d9354b)
pub type PermissionName = String;

// Source: upstream/packages/types/src/Permission.ts:23 (sha256:30b2b229abb900b58e118a111f38eaa67c2d375e4a109a85f5f520345328c9ec)
pub type PermissionState = String;

// Source: upstream/packages/types/src/Permission.ts:25 (sha256:aa10c0c5b6faf235212cbb21a7e1ff46688f0ffc39fc6592f7349ab40980b0c3)
#[derive(Clone)]
pub struct PermissionBackend {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub get_state: std::sync::Arc<
        std::sync::Mutex<
            Box<dyn FnMut(PermissionName) -> crate::FlightTask<PermissionState> + Send + 'static>,
        >,
    >,
    pub request: std::sync::Arc<
        std::sync::Mutex<
            Box<dyn FnMut(PermissionName) -> crate::FlightTask<PermissionState> + Send + 'static>,
        >,
    >,
}
impl PartialEq for PermissionBackend {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Permission.ts:44 (sha256:7b0c7da21bcd342f14b2a0b827a5d2bfef3e6f22a25fad7b74af4cf5727e8632)
pub type PermissionStateSource = String;

// Source: upstream/packages/types/src/Permission.ts:46 (sha256:c2f95926f1ef42fb294a8577fe81ddf113198ab431294bdcd801d01b6caeb36b)
#[derive(Clone, Default)]
pub struct PermissionStateExplanation {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub name: PermissionName,
    pub source: PermissionStateSource,
    pub state: PermissionState,
}
impl PartialEq for PermissionStateExplanation {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Permission.ts:54 (sha256:f13b4db38a8191513b1d06c3710936f07864b7830f28cd96b3623f9c6d355332)
pub type PermissionRequestFallbackGuard = std::sync::Arc<
    std::sync::Mutex<Box<dyn FnMut(PermissionName, PermissionState) -> () + Send + 'static>>,
>;
