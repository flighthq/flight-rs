// @generated from upstream/packages/types/src/Midi.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{EntityRuntime, PermissionQueryOutcome, Signal};

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
pub struct MidiEventBackendAttachOutcomeRecord1 {
    pub __flight_identity: std::sync::Arc<()>,
    pub reason: String,
    pub release_failed: bool,
}
impl PartialEq for MidiEventBackendAttachOutcomeRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone)]
pub struct MidiEventBackendAttachOutcomeRecord2 {
    pub __flight_identity: std::sync::Arc<()>,
    pub attachment: MidiEventAttachment,
    pub reason: String,
}
impl PartialEq for MidiEventBackendAttachOutcomeRecord2 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct MidiSubscriptionAttachOutcomeRecord1 {
    pub __flight_identity: std::sync::Arc<()>,
    pub attach_failed: bool,
    pub reason: String,
    pub release_failed: bool,
}
impl PartialEq for MidiSubscriptionAttachOutcomeRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct MidiSubscriptionAttachOutcomeRecord2 {
    pub __flight_identity: std::sync::Arc<()>,
    pub reason: String,
}
impl PartialEq for MidiSubscriptionAttachOutcomeRecord2 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Midi.ts:7 (sha256:cc9f2a866bd6e1f9db62b0b1dc32a474f493f94d9af8b6b36a6f09cb6c41d56e)
#[derive(Clone, Default)]
pub struct MidiAccess {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    #[doc(hidden)]
    pub __flight_entity_snapshot: Option<std::sync::Arc<dyn std::any::Any + Send + Sync>>,
}
impl PartialEq for MidiAccess {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for MidiAccess {
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

// Source: upstream/packages/types/src/Midi.ts:9 (sha256:c836cf24aee60240c56482cdb309246ce456a68e295e67619586278cd1849bd4)
pub type MidiPortConnection = String;

// Source: upstream/packages/types/src/Midi.ts:10 (sha256:527167a5c0a606d3195d9c82a756d5fa4bd4c4daa4212c4b5a98a9c1d94e1aef)
pub type MidiPortState = String;

// Source: upstream/packages/types/src/Midi.ts:12 (sha256:607082e0c330281216b8cf2229d7137346132a196ffa568dcaceae97e6639cad)
#[derive(Clone, Default)]
pub struct MidiInputPort {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    #[doc(hidden)]
    pub __flight_entity_snapshot: Option<std::sync::Arc<dyn std::any::Any + Send + Sync>>,
    pub id: String,
    pub manufacturer: Option<String>,
    pub name: Option<String>,
    pub type_: String,
    pub version: Option<String>,
}
impl PartialEq for MidiInputPort {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for MidiInputPort {
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

// Source: upstream/packages/types/src/Midi.ts:20 (sha256:a6b390dc8ce7a92a3460f01338280b6e1d5150e8f928282d10e9b928da390107)
#[derive(Clone, Default)]
pub struct MidiOutputPort {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    #[doc(hidden)]
    pub __flight_entity_snapshot: Option<std::sync::Arc<dyn std::any::Any + Send + Sync>>,
    pub id: String,
    pub manufacturer: Option<String>,
    pub name: Option<String>,
    pub type_: String,
    pub version: Option<String>,
}
impl PartialEq for MidiOutputPort {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for MidiOutputPort {
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

// Source: upstream/packages/types/src/Midi.ts:28 (sha256:073f9c06f1f703f30be2f3991e183f19d9ad453a05fc95013f14b8878f6579d5)
pub type MidiPort = crate::FlightUnion2<MidiInputPort, MidiOutputPort>;

// Source: upstream/packages/types/src/Midi.ts:30 (sha256:235f481d510a7be0c94f50af8e51b4077d5ef340ff69bfcc0daa453309a55871)
#[derive(Clone, Default)]
pub struct MidiInputMessage {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub data: Vec<u8>,
    pub timestamp: f64,
}
impl PartialEq for MidiInputMessage {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Midi.ts:35 (sha256:70f38a483d10ac85287d3ab7813cbd18d092c1f66829e0dcd2c00237a7149647)
#[derive(Clone, Default)]
pub struct MidiAccessRequestOutcome {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub access: Option<MidiAccess>,
    pub reason: String,
}
impl PartialEq for MidiAccessRequestOutcome {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Midi.ts:39 (sha256:5c2b4033caf014fc9088ea754741a2423bf9d63cf3e9e0fd4917933e6e8c4ed2)
#[derive(Clone)]
pub struct MidiAccessPortsOutcome<Port> {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub ports: Option<Vec<Port>>,
    pub reason: String,
}
impl<Port> Default for MidiAccessPortsOutcome<Port> {
    fn default() -> Self {
        Self {
            __flight_identity: Default::default(),
            ports: Default::default(),
            reason: Default::default(),
        }
    }
}
impl<Port> PartialEq for MidiAccessPortsOutcome<Port> {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Midi.ts:43 (sha256:fd9a81532dd0422ca83864de1f859a0d7c30cb1a1cd3b16541d62fc1d9707691)
#[derive(Clone, Default)]
pub struct MidiAccessLifecycleFailure {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub operation: String,
    pub id: Option<String>,
    pub type_: Option<crate::OpaqueHostValue>,
}
impl PartialEq for MidiAccessLifecycleFailure {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Midi.ts:51 (sha256:8aa29845b2969d41014ce331f9d37fcaa0b31f2231b3020dae492e1a52f7aa5d)
#[derive(Clone, Default)]
pub struct MidiAccessDisposeOutcome {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub reason: String,
    pub failures: Option<Vec<MidiAccessLifecycleFailure>>,
}
impl PartialEq for MidiAccessDisposeOutcome {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Midi.ts:58 (sha256:4fb0b4361712744810bd9392ff28185dbd21a1d687181d7f245500dcfcd0af5a)
#[derive(Clone, Default)]
pub struct MidiPortCloseOutcome {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub reason: String,
}
impl PartialEq for MidiPortCloseOutcome {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Midi.ts:62 (sha256:8fec6f728537317c47a7191d86275fb770029cc51a160a73954fe63ebc4a47b7)
#[derive(Clone, Default)]
pub struct MidiPortOpenOutcome {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub reason: String,
}
impl PartialEq for MidiPortOpenOutcome {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Midi.ts:66 (sha256:84c24aebb4d3bd3863ecf8c6e0f1d881903644d4b5f55c46b4b033ca56dc6dad)
#[derive(Clone, Default)]
pub struct MidiPortConnectionOutcome {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub connection: Option<MidiPortConnection>,
    pub reason: String,
}
impl PartialEq for MidiPortConnectionOutcome {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Midi.ts:70 (sha256:104544d327186e2408aa379b890da69668ae5d2a6f468a9288ec6ed0e80589d6)
#[derive(Clone, Default)]
pub struct MidiPortStateOutcome {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub reason: String,
    pub state: Option<MidiPortState>,
}
impl PartialEq for MidiPortStateOutcome {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Midi.ts:74 (sha256:73bf0631908b0e54ac1f57c2c42b8b135113ea67d853d1dacee1d86c091cb95f)
#[derive(Clone, Default)]
pub struct MidiPortLifecycleFailure {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub operation: String,
}
impl PartialEq for MidiPortLifecycleFailure {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Midi.ts:78 (sha256:cd8c4aeaa7cf8b92a89b218c5d8fecadf30f22a819d737f07639ac751d3072af)
#[derive(Clone, Default)]
pub struct MidiPortDisposeOutcome {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub reason: String,
    pub failures: Option<Vec<MidiPortLifecycleFailure>>,
}
impl PartialEq for MidiPortDisposeOutcome {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Midi.ts:85 (sha256:afd6c90cbe7719a8bfb27398f9024f47d72b29c590f20cc8251e3fc219691199)
#[derive(Clone, Default)]
pub struct MidiMessageSendOutcome {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub reason: String,
}
impl PartialEq for MidiMessageSendOutcome {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Midi.ts:96 (sha256:63f357fb9d3509d502d59b3f156a09479b9b02d7f51a401d7895636cf631a557)
#[derive(Clone, Default)]
pub struct MidiEventReleaseOutcome {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub reason: String,
}
impl PartialEq for MidiEventReleaseOutcome {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Midi.ts:100 (sha256:82cd57662c9fd331cf7025eb779d793c81a224135eb6be3eed7b96f876a16f7d)
#[derive(Clone)]
pub struct MidiEventAttachment {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    #[doc(hidden)]
    pub __flight_entity_snapshot: Option<std::sync::Arc<dyn std::any::Any + Send + Sync>>,
    pub release: std::sync::Arc<
        std::sync::Mutex<
            Box<dyn FnMut() -> crate::FlightTask<MidiEventReleaseOutcome> + Send + 'static>,
        >,
    >,
}
impl PartialEq for MidiEventAttachment {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for MidiEventAttachment {
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

// Source: upstream/packages/types/src/Midi.ts:104 (sha256:de00256836e420c67437cfa3bb63e0488e1f3f308e0969fe21056357b3cfc122)
pub type MidiEventBackendAttachOutcome =
    crate::FlightUnion2<MidiEventBackendAttachOutcomeRecord2, MidiEventBackendAttachOutcomeRecord1>;

// Source: upstream/packages/types/src/Midi.ts:108 (sha256:f333dd894fb4c9271a5b1f322c702d8d3cf0d59ebd0eae88d0ead9a44a0c13ba)
pub type MidiSubscriptionAttachOutcome =
    crate::FlightUnion2<MidiSubscriptionAttachOutcomeRecord2, MidiSubscriptionAttachOutcomeRecord1>;

// Source: upstream/packages/types/src/Midi.ts:116 (sha256:e791834543964e9a540e201da2bbc79706862607e94956739f32f0fe1ed3ffae)
#[derive(Clone, Default)]
pub struct MidiSubscriptionDetachOutcome {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub reason: String,
    pub release_failed: Option<bool>,
}
impl PartialEq for MidiSubscriptionDetachOutcome {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Midi.ts:120 (sha256:69994328853cf18cc2d339727f41c98e99f0cc2d1b8f16799e3d02c9a97b0061)
#[derive(Clone, Default)]
pub struct MidiSubscriptionDisposeOutcome {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub reason: String,
    pub attach_failed: Option<bool>,
    pub release_failed: Option<bool>,
}
impl PartialEq for MidiSubscriptionDisposeOutcome {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Midi.ts:128 (sha256:d1e7b522af9fbaf4b36a27ae88d6298f1795d32cac3cde5057d2ec62fc2f6dc5)
#[derive(Clone)]
pub struct MidiAccessStateSubscription {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    #[doc(hidden)]
    pub __flight_entity_snapshot: Option<std::sync::Arc<dyn std::any::Any + Send + Sync>>,
    pub on_midi_access_state_change:
        Signal<std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(MidiPort) -> () + Send + 'static>>>>,
}
impl PartialEq for MidiAccessStateSubscription {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for MidiAccessStateSubscription {
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

// Source: upstream/packages/types/src/Midi.ts:132 (sha256:9ed71dc3c34bd12333d960075b7906825cfee404a615d4a27891632ffa7f9a9c)
#[derive(Clone)]
pub struct MidiInputMessageSubscription {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    #[doc(hidden)]
    pub __flight_entity_snapshot: Option<std::sync::Arc<dyn std::any::Any + Send + Sync>>,
    pub on_midi_input_message: Signal<
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(MidiInputMessage) -> () + Send + 'static>>>,
    >,
}
impl PartialEq for MidiInputMessageSubscription {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for MidiInputMessageSubscription {
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

// Source: upstream/packages/types/src/Midi.ts:136 (sha256:c1d8c569b0a3c16d3e0be009e9e163709cc82d64fdeefbfc8901cf72d116772b)
#[derive(Clone)]
pub struct MidiPortStateSubscription {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    #[doc(hidden)]
    pub __flight_entity_snapshot: Option<std::sync::Arc<dyn std::any::Any + Send + Sync>>,
    pub on_midi_port_state_change:
        Signal<std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(MidiPort) -> () + Send + 'static>>>>,
}
impl PartialEq for MidiPortStateSubscription {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for MidiPortStateSubscription {
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

// Source: upstream/packages/types/src/Midi.ts:140 (sha256:45fdd11b64d65a0facc102d512b4e0b4d80878f31aa9943dc00c84ef4bfe5c71)
#[derive(Clone)]
pub struct HostMidiAccessCapability {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub request_access: std::sync::Arc<
        std::sync::Mutex<
            Box<dyn FnMut() -> crate::FlightTask<MidiAccessRequestOutcome> + Send + 'static>,
        >,
    >,
}
impl PartialEq for HostMidiAccessCapability {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Midi.ts:144 (sha256:b1ca3c95afd419e66c6f2e05845a02e2de50d48b9387afe75b9df54eb497272b)
#[derive(Clone)]
pub struct HostMidiPermissionCapability {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub get_permission: std::sync::Arc<
        std::sync::Mutex<
            Box<dyn FnMut() -> crate::FlightTask<PermissionQueryOutcome> + Send + 'static>,
        >,
    >,
}
impl PartialEq for HostMidiPermissionCapability {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Midi.ts:148 (sha256:588b3dc6f463ef251968c5b3c4e331ddcdae04b2f312ce9b51b440bfdfbd681d)
#[derive(Clone)]
pub struct MidiAccessResourceOperations {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub attach_state_change: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(
                        std::sync::Arc<
                            std::sync::Mutex<Box<dyn FnMut(MidiPort) -> () + Send + 'static>>,
                        >,
                    ) -> crate::FlightTask<MidiEventBackendAttachOutcome>
                    + Send
                    + 'static,
            >,
        >,
    >,
    pub get_input_ports:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> Vec<MidiInputPort> + Send + 'static>>>,
    pub get_output_ports:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> Vec<MidiOutputPort> + Send + 'static>>>,
}
impl PartialEq for MidiAccessResourceOperations {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Midi.ts:154 (sha256:41d712bd19b17aca1973c8149c93c01785e11874b1dc22884c5ca3021e1a3f19)
#[derive(Clone)]
pub struct MidiPortResourceOperations {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub attach_state_change: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(
                        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>,
                    ) -> crate::FlightTask<MidiEventBackendAttachOutcome>
                    + Send
                    + 'static,
            >,
        >,
    >,
    pub close: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut() -> crate::FlightTask<()> + Send + 'static>>,
    >,
    pub get_connection:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> MidiPortConnection + Send + 'static>>>,
    pub get_state:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> MidiPortState + Send + 'static>>>,
    pub open: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut() -> crate::FlightTask<()> + Send + 'static>>,
    >,
}
impl PartialEq for MidiPortResourceOperations {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Midi.ts:162 (sha256:f550abe72cdd1c0850a0c94aa097c2f72c1e00c781b19bc4274c1a8a2c1aba3f)
#[derive(Clone)]
pub struct MidiInputPortResourceOperations {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub attach_state_change: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(
                        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>,
                    ) -> crate::FlightTask<MidiEventBackendAttachOutcome>
                    + Send
                    + 'static,
            >,
        >,
    >,
    pub close: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut() -> crate::FlightTask<()> + Send + 'static>>,
    >,
    pub get_connection:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> MidiPortConnection + Send + 'static>>>,
    pub get_state:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> MidiPortState + Send + 'static>>>,
    pub open: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut() -> crate::FlightTask<()> + Send + 'static>>,
    >,
    pub attach_message: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(
                        std::sync::Arc<
                            std::sync::Mutex<Box<dyn FnMut(Vec<u8>, f64) -> () + Send + 'static>>,
                        >,
                    ) -> crate::FlightTask<MidiEventBackendAttachOutcome>
                    + Send
                    + 'static,
            >,
        >,
    >,
}
impl PartialEq for MidiInputPortResourceOperations {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Midi.ts:166 (sha256:21dab8f479a097ee84632a9f193ddfdf5cc68f7ff37bbb3d0cfeda69adef4a47)
#[derive(Clone)]
pub struct MidiOutputPortResourceOperations {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub attach_state_change: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(
                        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>,
                    ) -> crate::FlightTask<MidiEventBackendAttachOutcome>
                    + Send
                    + 'static,
            >,
        >,
    >,
    pub close: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut() -> crate::FlightTask<()> + Send + 'static>>,
    >,
    pub get_connection:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> MidiPortConnection + Send + 'static>>>,
    pub get_state:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> MidiPortState + Send + 'static>>>,
    pub open: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut() -> crate::FlightTask<()> + Send + 'static>>,
    >,
    pub send: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut(Vec<f64>, Option<f64>) -> () + Send + 'static>>,
    >,
}
impl PartialEq for MidiOutputPortResourceOperations {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
