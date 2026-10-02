// @generated from upstream/packages/types/src/StatusBar.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{EntityRuntime, Signal};

// Source: upstream/packages/types/src/StatusBar.ts:7 (sha256:1ada9f3d5cd5e30174d9676172afc07caf571b4891f7439a987722af3adaf398)
pub type StatusBarStyle = String;

// Source: upstream/packages/types/src/StatusBar.ts:11 (sha256:06190a39d8776df1d3d892dd1d1b6440fc7dbb71bc4889a3bb37a65932138cab)
pub type StatusBarAnimation = String;

// Source: upstream/packages/types/src/StatusBar.ts:15 (sha256:39a60095237c9cfdaeb731229049618a8b0c81e14de71e4a10e547ee1622284e)
pub type StatusBarStyleEntryHandle = f64;

// Source: upstream/packages/types/src/StatusBar.ts:19 (sha256:2f29b043a31a414a2a4115df74a947128693513d810b877ed4ea92484bf52224)
#[derive(Clone, Default)]
pub struct StatusBarInfo {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    #[doc(hidden)]
    pub __flight_entity_snapshot: Option<std::sync::Arc<dyn std::any::Any + Send + Sync>>,
    pub color: f64,
    pub height: f64,
    pub overlays_content: bool,
    pub style: StatusBarStyle,
    pub visible: bool,
}
impl PartialEq for StatusBarInfo {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for StatusBarInfo {
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

// Source: upstream/packages/types/src/StatusBar.ts:32 (sha256:cb0442851b0b549082926c288dde8cd8d17ba423dbd107e50192db91817e5273)
#[derive(Clone, Default)]
pub struct StatusBarStyleEntry {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub animation: Option<StatusBarAnimation>,
    pub color: Option<f64>,
    pub overlays_content: Option<bool>,
    pub style: Option<StatusBarStyle>,
    pub visible: Option<bool>,
}
impl PartialEq for StatusBarStyleEntry {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/StatusBar.ts:41 (sha256:82e20a365ec081672a0ccf0fb5f5b10320d8dfb3663835b90c343a1a704193c1)
#[derive(Clone)]
pub struct HostStatusBarChangeCapability {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub subscribe: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(
                        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>,
                    ) -> std::sync::Arc<
                        std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>,
                    > + Send
                    + 'static,
            >,
        >,
    >,
}
impl PartialEq for HostStatusBarChangeCapability {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/StatusBar.ts:46 (sha256:0615b071c7f2c1dc8ce0722cb704d5d4d6223d45dd2909a155efddf4db1989c2)
#[derive(Clone)]
pub struct HostStatusBarInfoCapability {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub get_info: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut(StatusBarInfo) -> StatusBarInfo + Send + 'static>>,
    >,
}
impl PartialEq for HostStatusBarInfoCapability {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/StatusBar.ts:51 (sha256:0b0c7866f71fee6f928c20f5049d16478105e54e16e71fdeb9a53538d62a0503)
#[derive(Clone)]
pub struct HostStatusBarOverlaysCapability {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub set_overlays_content:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(bool) -> () + Send + 'static>>>,
}
impl PartialEq for HostStatusBarOverlaysCapability {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/StatusBar.ts:55 (sha256:8d8a2ae85943d09fa5467daff2d5720ffafcb87f4fabb77a133741c61a46a4c9)
#[derive(Clone)]
pub struct HostStatusBarStyleCapability {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub set_style:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(StatusBarStyle) -> () + Send + 'static>>>,
}
impl PartialEq for HostStatusBarStyleCapability {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/StatusBar.ts:59 (sha256:6f508a57b4dd0c9c26e959cc2879e9800ff930a48ec906d8e5605476e78fc862)
#[derive(Clone)]
pub struct HostStatusBarColorCapability {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub set_background_color:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(f64, Option<bool>) -> () + Send + 'static>>>,
}
impl PartialEq for HostStatusBarColorCapability {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/StatusBar.ts:65 (sha256:16ed78a4d71464e425931995968b0beade78bd3bb2f056baf9bd698230acb875)
#[derive(Clone)]
pub struct HostStatusBarVisibilityCapability {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub set_visible: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut(bool, Option<StatusBarAnimation>) -> () + Send + 'static>>,
    >,
}
impl PartialEq for HostStatusBarVisibilityCapability {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/StatusBar.ts:70 (sha256:5e86d900894b87ac979d117752be1513db34308a81773c93ce66f55f7c54395a)
#[derive(Clone)]
pub struct StatusBar {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    #[doc(hidden)]
    pub __flight_entity_snapshot: Option<std::sync::Arc<dyn std::any::Any + Send + Sync>>,
    pub on_change: Signal<
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(StatusBarInfo) -> () + Send + 'static>>>,
    >,
}
impl PartialEq for StatusBar {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for StatusBar {
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
