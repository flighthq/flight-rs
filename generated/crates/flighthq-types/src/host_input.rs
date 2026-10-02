// @generated from upstream/packages/types/src/HostInput.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::InputTargetHandle;

#[derive(Clone, Default)]
pub struct SharedStructuralRecord1 {
    pub __flight_identity: std::sync::Arc<()>,
    pub reason: String,
}
impl PartialEq for SharedStructuralRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/HostInput.ts:6 (sha256:bd14c10dd06368e81dff3f686227f8c6f6daaade7e9df6337c8794062de36a16)
#[derive(Clone)]
pub struct HostInputDropFileCapability {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub subscribe: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(
                        InputTargetHandle,
                        std::sync::Arc<
                            std::sync::Mutex<Box<dyn FnMut(String) -> () + Send + 'static>>,
                        >,
                    ) -> std::sync::Arc<
                        std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>,
                    > + Send
                    + 'static,
            >,
        >,
    >,
}
impl PartialEq for HostInputDropFileCapability {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/HostInput.ts:12 (sha256:57f3cdc02145fb7283bb15486b9935ccd599c49f29491c0835b83c495bf5e058)
#[derive(Clone)]
pub struct HostInputFocusCapability {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub subscribe: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(
                        InputTargetHandle,
                        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>,
                        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>,
                    ) -> std::sync::Arc<
                        std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>,
                    > + Send
                    + 'static,
            >,
        >,
    >,
}
impl PartialEq for HostInputFocusCapability {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/HostInput.ts:21 (sha256:a2e6953dfc2b9d2a12c4277ce959bb76debe490a2336111e7704d50d0e195da6)
#[derive(Clone, Default)]
pub struct InputPointerLockExitOutcome {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub reason: String,
}
impl PartialEq for InputPointerLockExitOutcome {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/HostInput.ts:25 (sha256:06ffa663b55ee969b3185a9d737b19e545bae8cb3d0a9bd2ee78bd76ca933b25)
#[derive(Clone, Default)]
pub struct InputPointerLockRequestOutcome {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub reason: String,
}
impl PartialEq for InputPointerLockRequestOutcome {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/HostInput.ts:29 (sha256:34b72dec1237e2195961c680c1936aaf35be9ea71671969dbaf0b734e2eae156)
#[derive(Clone)]
pub struct HostInputPointerLockCapability {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub exit: std::sync::Arc<
        std::sync::Mutex<
            Box<dyn FnMut() -> crate::FlightTask<InputPointerLockExitOutcome> + Send + 'static>,
        >,
    >,
    pub request: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(InputTargetHandle) -> crate::FlightTask<InputPointerLockRequestOutcome>
                    + Send
                    + 'static,
            >,
        >,
    >,
}
impl PartialEq for HostInputPointerLockCapability {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
