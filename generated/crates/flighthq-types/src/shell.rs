// @generated from upstream/packages/types/src/Shell.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::EntityRuntime;

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
pub struct SharedStructuralRecord2 {
    pub __flight_identity: std::sync::Arc<()>,
    pub message: String,
    pub reason: String,
}
impl PartialEq for SharedStructuralRecord2 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct ShellPathOpenOutcomeRecord2 {
    pub __flight_identity: std::sync::Arc<()>,
    pub reason: String,
}
impl PartialEq for ShellPathOpenOutcomeRecord2 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct ShellShortcutLinkReadOutcomeRecord2 {
    pub __flight_identity: std::sync::Arc<()>,
    pub link: ShellShortcutLink,
    pub reason: String,
}
impl PartialEq for ShellShortcutLinkReadOutcomeRecord2 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Shell.ts:6 (sha256:55c65ce0a218025579028a5f76bedaab6dafd54f969bfc9c8b8097c09503b2fc)
#[derive(Clone, Default)]
pub struct ShellExternalUrlPolicy {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub allowed_schemes: Vec<String>,
}
impl PartialEq for ShellExternalUrlPolicy {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Shell.ts:10 (sha256:7dd5e78208c381e58244c5262c06034b37dc97391e56687f1c4afc54b9bfb2e4)
#[derive(Clone, Default)]
pub struct ShellExternalOutcome {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub reason: String,
}
impl PartialEq for ShellExternalOutcome {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Shell.ts:14 (sha256:7b03d7dc7277d67c00116ef4950908549d0e79825dc4935bd74b5f0d302f6498)
pub type ShellPathOpenOutcome =
    crate::FlightUnion2<ShellPathOpenOutcomeRecord2, SharedStructuralRecord2>;

// Source: upstream/packages/types/src/Shell.ts:18 (sha256:f659ec7a492e8edd6786c6711c98cd10dde3c18eca480c15c73bba034cfed591)
#[derive(Clone, Default)]
pub struct ShellPathRevealOutcome {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub reason: String,
}
impl PartialEq for ShellPathRevealOutcome {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Shell.ts:20 (sha256:d4cf4cae88947ff4cbbe57e53af695e322115bfce429e56df53adb0963b51abc)
pub type ShellShortcutLinkReadOutcome =
    crate::FlightUnion2<ShellShortcutLinkReadOutcomeRecord2, SharedStructuralRecord2>;

// Source: upstream/packages/types/src/Shell.ts:24 (sha256:b0f8a2c6ff4d2ec1340d76d8dcdbdcde344ff861e1b2bdcb08cf53bc8735b38e)
#[derive(Clone, Default)]
pub struct ShellShortcutLinkWriteOutcome {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub reason: String,
}
impl PartialEq for ShellShortcutLinkWriteOutcome {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Shell.ts:26 (sha256:fdfaf7ac93be48d309fe6ecf64b49a0ed8c1fd19018f3546423144de5b4d2603)
#[derive(Clone, Default)]
pub struct ShellTrashOutcome {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub reason: String,
}
impl PartialEq for ShellTrashOutcome {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Shell.ts:30 (sha256:7bd5d8e7631a8e83102f7a414fb9d93606d350dba1403ec808dab8f39e96d603)
#[derive(Clone, Default)]
pub struct ShellProcessOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub cwd: Option<String>,
    pub environment: Option<Vec<(String, String)>>,
}
impl PartialEq for ShellProcessOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Shell.ts:38 (sha256:08253036765b45b3a94138f4fb4898fe27bde56a8a630dcd1017e4dd6fa60236)
#[derive(Clone, Default)]
pub struct ShellProcessExitStatus {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub code: Option<f64>,
    pub signal: Option<String>,
}
impl PartialEq for ShellProcessExitStatus {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Shell.ts:46 (sha256:c6c08309eed8c2496841c9fa78cf75e3ce8880bdc5a8557ab85faeca9f7af593)
#[derive(Clone)]
pub struct ShellProcess {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    #[doc(hidden)]
    pub __flight_entity_snapshot: Option<std::sync::Arc<dyn std::any::Any + Send + Sync>>,
    pub exit: crate::FlightTask<ShellProcessExitStatus>,
    pub stderr: crate::OpaqueHostValue,
    pub stdin: crate::OpaqueHostValue,
    pub stdout: crate::OpaqueHostValue,
    pub terminate: std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>,
}
impl PartialEq for ShellProcess {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for ShellProcess {
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

// Source: upstream/packages/types/src/Shell.ts:58 (sha256:163ffac6e4011c907f5dac066cda15d35287e506d6afba36eedf7045515efb74)
#[derive(Clone)]
pub struct HostShellBeepCapability {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub beep: std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>,
}
impl PartialEq for HostShellBeepCapability {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Shell.ts:62 (sha256:aa5f9112f00045446768a15dfdb9442fbdab3887bc7807acebd8bd31620d7cea)
#[derive(Clone)]
pub struct HostShellExternalCapability {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub open: std::sync::Arc<
        std::sync::Mutex<
            Box<dyn FnMut(String) -> crate::FlightTask<ShellExternalOutcome> + Send + 'static>,
        >,
    >,
}
impl PartialEq for HostShellExternalCapability {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Shell.ts:66 (sha256:f4aa2f7933a5dc0d62d4cad34f494f607c8d9039e11f71d8980d671bfc0e72b7)
#[derive(Clone)]
pub struct HostShellPathOpenCapability {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub open: std::sync::Arc<
        std::sync::Mutex<
            Box<dyn FnMut(String) -> crate::FlightTask<ShellPathOpenOutcome> + Send + 'static>,
        >,
    >,
}
impl PartialEq for HostShellPathOpenCapability {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Shell.ts:70 (sha256:69725f95fae5f127d56ca6bd882d36c87478e8fcd4188d682e28559d7d3c5623)
#[derive(Clone)]
pub struct HostShellPathRevealCapability {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub reveal: std::sync::Arc<
        std::sync::Mutex<
            Box<dyn FnMut(String) -> crate::FlightTask<ShellPathRevealOutcome> + Send + 'static>,
        >,
    >,
}
impl PartialEq for HostShellPathRevealCapability {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Shell.ts:77 (sha256:62ae714d804abef323718520beec2294775b53717e62dce90bfe12503b798718)
#[derive(Clone)]
pub struct HostShellProcessCapability {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub spawn: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(String, Vec<String>, Option<ShellProcessOptions>) -> ShellProcess
                    + Send
                    + 'static,
            >,
        >,
    >,
}
impl PartialEq for HostShellProcessCapability {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Shell.ts:81 (sha256:d8edd660aac86f107c7324b285d04fb36827f6fe9bdade793cc8d22d4e5176ad)
#[derive(Clone)]
pub struct HostShellShortcutLinkCapability {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub read: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(String) -> crate::FlightTask<ShellShortcutLinkReadOutcome>
                    + Send
                    + 'static,
            >,
        >,
    >,
    pub write: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(
                        String,
                        ShellShortcutLink,
                        ShellShortcutWriteOperation,
                    ) -> crate::FlightTask<ShellShortcutLinkWriteOutcome>
                    + Send
                    + 'static,
            >,
        >,
    >,
}
impl PartialEq for HostShellShortcutLinkCapability {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Shell.ts:90 (sha256:ead55ecd228eb268861a7616401bf06de828e65fcb6910dcd6c0d5d36a17cd18)
#[derive(Clone)]
pub struct HostShellTrashCapability {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub move_to_trash: std::sync::Arc<
        std::sync::Mutex<
            Box<dyn FnMut(String) -> crate::FlightTask<ShellTrashOutcome> + Send + 'static>,
        >,
    >,
}
impl PartialEq for HostShellTrashCapability {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Shell.ts:96 (sha256:4c98c49be5de57273aa9d3cc5ea9b68fd36302b9c7efca25e2d585194a7a49f8)
#[derive(Clone, Default)]
pub struct ShellShortcutLink {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub target: String,
    pub app_user_model_id: Option<String>,
    pub args: Option<String>,
    pub description: Option<String>,
    pub icon: Option<String>,
    pub icon_index: Option<f64>,
    pub working_directory: Option<String>,
}
impl PartialEq for ShellShortcutLink {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Shell.ts:107 (sha256:38507f893c5bbb8ac211e8e7345d46b15e1359bd9e58e014b53b898861b3c665)
pub type ShellShortcutWriteOperation = String;
