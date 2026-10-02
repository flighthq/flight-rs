// @generated from upstream/packages/types/src/Ipc.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::EntityRuntime;

// Source: upstream/packages/types/src/Ipc.ts:7 (sha256:a83bb039a71af505946af6757fa4c57f03be55593d4834a09c5253b904c95796)
#[derive(Clone)]
pub struct HostIpcHandleCapability {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub handle: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(
                        String,
                        std::sync::Arc<
                            std::sync::Mutex<
                                Box<
                                    dyn FnMut(
                                            Vec<crate::FlightValue>,
                                        )
                                            -> crate::FlightUnion2<
                                            crate::FlightValue,
                                            crate::FlightTask<crate::FlightValue>,
                                        > + Send
                                        + 'static,
                                >,
                            >,
                        >,
                    ) -> std::sync::Arc<
                        std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>,
                    > + Send
                    + 'static,
            >,
        >,
    >,
}
impl PartialEq for HostIpcHandleCapability {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Ipc.ts:11 (sha256:7a666452c9995ba8150c2aaaa4eacc4a8aff81320d7f2e44df1589596af640b6)
#[derive(Clone)]
pub struct HostIpcInvokeCapability {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub invoke: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(String, Vec<crate::FlightValue>) -> crate::FlightTask<crate::FlightValue>
                    + Send
                    + 'static,
            >,
        >,
    >,
}
impl PartialEq for HostIpcInvokeCapability {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Ipc.ts:15 (sha256:aed7e6de0793be5e86827701c46d1401421cbfbcde0dccf098efb166c5948a6c)
#[derive(Clone)]
pub struct HostIpcMessageCapability {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub subscribe: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(
                        String,
                        std::sync::Arc<
                            std::sync::Mutex<
                                Box<dyn FnMut(Vec<crate::FlightValue>) -> () + Send + 'static>,
                            >,
                        >,
                    ) -> std::sync::Arc<
                        std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>,
                    > + Send
                    + 'static,
            >,
        >,
    >,
}
impl PartialEq for HostIpcMessageCapability {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Ipc.ts:20 (sha256:d6bbcc40a4ad5311578dcf54025fcfe061543852580ba0dfe422c10c33997189)
#[derive(Clone)]
pub struct HostIpcSendCapability {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub send: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut(String, Vec<crate::FlightValue>) -> () + Send + 'static>>,
    >,
}
impl PartialEq for HostIpcSendCapability {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Ipc.ts:26 (sha256:98ceffcd5ff4d102835d86e97312fdf0eff51b9e9a0132cb5e6109d1b26a769f)
#[derive(Clone)]
pub struct HostIpcTargetedSendCapability<Target = std::convert::Infallible> {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    #[doc(hidden)]
    pub __flight_entity_snapshot: Option<std::sync::Arc<dyn std::any::Any + Send + Sync>>,
    pub send: std::sync::Arc<
        std::sync::Mutex<
            Box<dyn FnMut(Target, String, Vec<crate::FlightValue>) -> () + Send + 'static>,
        >,
    >,
}
impl<Target> PartialEq for HostIpcTargetedSendCapability<Target> {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl<Target: Clone + Send + Sync + 'static> crate::FlightEntity
    for HostIpcTargetedSendCapability<Target>
{
    fn __flight_entity_runtime(
        &self,
    ) -> &std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>> {
        &self.__flight_entity_runtime
    }
    fn __flight_entity_snapshot(&self) -> &Option<std::sync::Arc<dyn std::any::Any + Send + Sync>> {
        &self.__flight_entity_snapshot
    }
    fn __flight_fresh_clone(&self) -> Self {
        let mut cloned = self.clone();
        cloned.__flight_identity = std::sync::Arc::new(());
        cloned.__flight_entity_runtime = std::sync::Arc::new(std::sync::Mutex::new(
            self.__flight_entity_runtime.lock().unwrap().clone(),
        ));
        cloned
    }
}
