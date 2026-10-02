// @generated from upstream/packages/types/src/Shortcut.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{Accelerator, AcceleratorParseError, EntityRuntime, Signal};

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

#[derive(Clone, Default)]
pub struct CreateGlobalShortcutOutcomeRecord1 {
    pub __flight_identity: std::sync::Arc<()>,
    pub parse_error: AcceleratorParseError,
    pub reason: String,
}
impl PartialEq for CreateGlobalShortcutOutcomeRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone)]
pub struct CreateGlobalShortcutOutcomeRecord2 {
    pub __flight_identity: std::sync::Arc<()>,
    pub reason: String,
    pub shortcut: GlobalShortcut,
}
impl PartialEq for CreateGlobalShortcutOutcomeRecord2 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct ShortcutTriggerSubscribeOutcomeRecord1 {
    pub __flight_identity: std::sync::Arc<()>,
    pub reason: String,
    pub subscription: ShortcutTriggerSubscription,
}
impl PartialEq for ShortcutTriggerSubscribeOutcomeRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct ShortcutTriggerSubscribeOutcomeRecord2 {
    pub __flight_identity: std::sync::Arc<()>,
    pub reason: String,
}
impl PartialEq for ShortcutTriggerSubscribeOutcomeRecord2 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Shortcut.ts:8 (sha256:cbdb70020b68d4cbb4550897d718099b7f78d33707671e1d13b034182168ec5b)
#[derive(Clone)]
pub struct GlobalShortcut {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    #[doc(hidden)]
    pub __flight_entity_snapshot: Option<std::sync::Arc<dyn std::any::Any + Send + Sync>>,
    pub accelerator: Accelerator,
    pub on_trigger:
        Signal<std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>>,
}
impl PartialEq for GlobalShortcut {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for GlobalShortcut {
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

// Source: upstream/packages/types/src/Shortcut.ts:15 (sha256:17cf5c00d02783496416cdb41f483cc7c531dd74cf37d4328a2ab4185debc20f)
#[derive(Clone, Default)]
pub struct ShortcutTriggerSubscription {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    #[doc(hidden)]
    pub __flight_entity_snapshot: Option<std::sync::Arc<dyn std::any::Any + Send + Sync>>,
}
impl PartialEq for ShortcutTriggerSubscription {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for ShortcutTriggerSubscription {
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

// Source: upstream/packages/types/src/Shortcut.ts:17 (sha256:73394efc85071971339aa117fd160a9156de47e98c56fa3d4be08758f38fcb79)
pub type CreateGlobalShortcutOutcome =
    crate::FlightUnion2<CreateGlobalShortcutOutcomeRecord2, CreateGlobalShortcutOutcomeRecord1>;

// Source: upstream/packages/types/src/Shortcut.ts:21 (sha256:ad03fe4adc61d67fab48e134f5c215c061a721fb4208afed936337b52f8c8f6f)
#[derive(Clone, Default)]
pub struct GlobalShortcutAttachOutcome {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub reason: String,
}
impl PartialEq for GlobalShortcutAttachOutcome {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Shortcut.ts:31 (sha256:12f870f53644510cb94227bed537b5cbcd18529095a5cb7ccd7db4f6d4fbffef)
#[derive(Clone, Default)]
pub struct GlobalShortcutDetachOutcome {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub reason: String,
}
impl PartialEq for GlobalShortcutDetachOutcome {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Shortcut.ts:35 (sha256:91e18dc6cd4ca46ba051e1c4925d646c23a35d67a692c2ddc9931070dff86c22)
#[derive(Clone, Default)]
pub struct GlobalShortcutQueryOutcome {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub reason: String,
    pub parse_error: Option<AcceleratorParseError>,
}
impl PartialEq for GlobalShortcutQueryOutcome {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Shortcut.ts:39 (sha256:056736d548503490dd8f5f90a1f5764cca71963486d7e3d20100fbc8c2532ac6)
pub type ShortcutTriggerSubscribeOutcome = crate::FlightUnion2<
    ShortcutTriggerSubscribeOutcomeRecord2,
    ShortcutTriggerSubscribeOutcomeRecord1,
>;

// Source: upstream/packages/types/src/Shortcut.ts:43 (sha256:852cbb45c65d7b4d6a859e76198afc32ca4700ef71588c828723a41b71422b67)
#[derive(Clone, Default)]
pub struct ShortcutTriggerUnsubscribeOutcome {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub reason: String,
}
impl PartialEq for ShortcutTriggerUnsubscribeOutcome {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Shortcut.ts:50 (sha256:a44fa33e8d14576cd868539018ca3e553c7c4a849abd4e183381d705cca8f228)
#[derive(Clone)]
pub struct HostShortcutTriggerCapability {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub destroy: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut() -> crate::FlightTask<()> + Send + 'static>>,
    >,
    pub subscribe: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(
                        Accelerator,
                        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>,
                    ) -> crate::FlightTask<ShortcutTriggerSubscribeOutcome>
                    + Send
                    + 'static,
            >,
        >,
    >,
    pub unsubscribe: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(
                        ShortcutTriggerSubscription,
                    )
                        -> crate::FlightTask<ShortcutTriggerUnsubscribeOutcome>
                    + Send
                    + 'static,
            >,
        >,
    >,
}
impl PartialEq for HostShortcutTriggerCapability {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Shortcut.ts:58 (sha256:404a40af6714d42a48c1a20306bbe2e62ab7d722bdb33a668f302879ebb836ac)
#[derive(Clone)]
pub struct HostShortcutQueryCapability {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub is_registered: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut(Accelerator) -> crate::FlightTask<bool> + Send + 'static>>,
    >,
}
impl PartialEq for HostShortcutQueryCapability {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
