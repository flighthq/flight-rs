// @generated from upstream/packages/types/src/Command.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{EntityRuntime, Kind, NodeAny, Signal};

#[derive(Clone, Default)]
pub struct SharedStructuralRecord1 {
    pub __flight_identity: std::sync::Arc<()>,
    pub kind: Kind,
    pub label: String,
    pub child: NodeAny,
    pub index: f64,
    pub parent: NodeAny,
}
impl PartialEq for SharedStructuralRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Command.ts:18 (sha256:ac55d32e8196d2f550269d3204ca13dfd5ee7c98f202f721b97b428ec203723d)
#[derive(Clone, Default)]
pub struct Command {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    #[doc(hidden)]
    pub __flight_entity_snapshot: Option<std::sync::Arc<dyn std::any::Any + Send + Sync>>,
    pub kind: Kind,
    pub label: String,
}
impl PartialEq for Command {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for Command {
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

// Source: upstream/packages/types/src/Command.ts:29 (sha256:26324c4549cb390ff2aa57d9af7c6eb7d25bf24f81773f094fad46eb722b7845)
#[derive(Clone)]
pub struct CommandBinding {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub execute: std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(Command) -> () + Send + 'static>>>,
    pub merge: Option<
        std::sync::Arc<
            std::sync::Mutex<Box<dyn FnMut(Command, Command) -> Option<Command> + Send + 'static>>,
        >,
    >,
    pub undo: std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(Command) -> () + Send + 'static>>>,
}
impl PartialEq for CommandBinding {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Command.ts:37 (sha256:a59e3d6a379e370bbf54615f18b577f7038fc449656690f5700ee50193cb7f7c)
pub type CommandBindingTable = Vec<(Kind, CommandBinding)>;

// Source: upstream/packages/types/src/Command.ts:44 (sha256:392f18ab32b04c2b3c4fc52a8a22c88a84eed89e4494843b181bcbcbc24a68da)
#[derive(Clone, Default)]
pub struct CommandHistory {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    #[doc(hidden)]
    pub __flight_entity_snapshot: Option<std::sync::Arc<dyn std::any::Any + Send + Sync>>,
    pub bindings: CommandBindingTable,
    pub entries: Vec<Command>,
    pub index: f64,
    pub max_size: f64,
    pub on_change:
        Option<Signal<std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>>>,
    pub transaction_depth: f64,
    pub transaction_index: f64,
    pub transaction_label: Option<String>,
}
impl PartialEq for CommandHistory {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for CommandHistory {
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

// Source: upstream/packages/types/src/Command.ts:67 (sha256:ac4232cbde64df3fb141885de8903b9a5a241cc198c39cd8f251ab3d6da04812)
#[derive(Clone, Default)]
pub struct CommandPropertyEntry {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub after: crate::FlightValue,
    pub before: crate::FlightValue,
    pub property: String,
    pub target: NodeAny,
}
impl PartialEq for CommandPropertyEntry {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Command.ts:76 (sha256:f70ae7bf50d473289485629c2dc9374917e29bdf5f58c3d2df645b3265f5d8a1)
#[derive(Clone, Default)]
pub struct CompositeCommand {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    #[doc(hidden)]
    pub __flight_entity_snapshot: Option<std::sync::Arc<dyn std::any::Any + Send + Sync>>,
    pub kind: Kind,
    pub label: String,
    pub children: Vec<Command>,
}
impl PartialEq for CompositeCommand {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for CompositeCommand {
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

// Source: upstream/packages/types/src/Command.ts:80 (sha256:2ee9b3de96352d0db344fb0d027d638c6020e5da1094998791dc906fa736f558)
#[derive(Clone, Default)]
pub struct AddNodeChildCommand {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    #[doc(hidden)]
    pub __flight_entity_snapshot: Option<std::sync::Arc<dyn std::any::Any + Send + Sync>>,
    pub kind: Kind,
    pub label: String,
    pub child: NodeAny,
    pub index: f64,
    pub parent: NodeAny,
}
impl PartialEq for AddNodeChildCommand {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for AddNodeChildCommand {
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

// Source: upstream/packages/types/src/Command.ts:87 (sha256:466bcf9a62f26e293c48ffb9ddda5ef5a81dbf45cd08d58ac0d277cc61b5309f)
#[derive(Clone, Default)]
pub struct RemoveNodeChildCommand {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    #[doc(hidden)]
    pub __flight_entity_snapshot: Option<std::sync::Arc<dyn std::any::Any + Send + Sync>>,
    pub kind: Kind,
    pub label: String,
    pub child: NodeAny,
    pub index: f64,
    pub parent: NodeAny,
}
impl PartialEq for RemoveNodeChildCommand {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for RemoveNodeChildCommand {
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

// Source: upstream/packages/types/src/Command.ts:95 (sha256:8a5f74086aa6f15d122cc950fe4e2baaf15c866c93293930e8e7fdb23f42801c)
#[derive(Clone, Default)]
pub struct ReorderNodeChildCommand {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    #[doc(hidden)]
    pub __flight_entity_snapshot: Option<std::sync::Arc<dyn std::any::Any + Send + Sync>>,
    pub kind: Kind,
    pub label: String,
    pub child: NodeAny,
    pub from_index: f64,
    pub parent: NodeAny,
    pub to_index: f64,
}
impl PartialEq for ReorderNodeChildCommand {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for ReorderNodeChildCommand {
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

// Source: upstream/packages/types/src/Command.ts:104 (sha256:2cafae6d3513c700c93e38c8dd5413389a08fe5ba05be0c8a2addf7dde376f42)
#[derive(Clone, Default)]
pub struct SetNodePropertyCommand {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    #[doc(hidden)]
    pub __flight_entity_snapshot: Option<std::sync::Arc<dyn std::any::Any + Send + Sync>>,
    pub kind: Kind,
    pub label: String,
    pub entries: Vec<CommandPropertyEntry>,
    pub merge_window: f64,
    pub time: f64,
}
impl PartialEq for SetNodePropertyCommand {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for SetNodePropertyCommand {
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

// Source: upstream/packages/types/src/Command.ts:117 (sha256:cacbcfbba0b76784c1587dba16a1ae51bdbd2c0cad03561aa2fbf566383f6deb)
#[derive(Clone, Default)]
pub struct CommandDispatchExplanation {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub missing_kind: Option<Kind>,
    pub resolved: bool,
}
impl PartialEq for CommandDispatchExplanation {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Command.ts:124 (sha256:81eb8a0fcd1983b02108c1af74b8db09e25f9aa2fdf56a44c2aa13d558ee1eed)
pub const ADD_NODE_CHILD_COMMAND_KIND: &'static str = "AddNodeChildCommand";

// Source: upstream/packages/types/src/Command.ts:125 (sha256:b75a655a794070e8a18e0ca683a782b37f41a435d40982a2c018c765faf6f170)
pub const COMPOSITE_COMMAND_KIND: &'static str = "CompositeCommand";

// Source: upstream/packages/types/src/Command.ts:126 (sha256:7b606322fe66a8f9db9e3904c54b9dbdd6c0ea08f5415425779f6660941d0aa6)
pub const REMOVE_NODE_CHILD_COMMAND_KIND: &'static str = "RemoveNodeChildCommand";

// Source: upstream/packages/types/src/Command.ts:127 (sha256:6cde6f96de6b01f9221ff0b5bd13af89ca6c60ecdd328b770989d96ffed6c2e2)
pub const REORDER_NODE_CHILD_COMMAND_KIND: &'static str = "ReorderNodeChildCommand";

// Source: upstream/packages/types/src/Command.ts:128 (sha256:d88dfdf5c1432f53806a43feee0ac3406ee6520f6b18b2eaff05b8df60c38761)
pub const SET_NODE_PROPERTY_COMMAND_KIND: &'static str = "SetNodePropertyCommand";
