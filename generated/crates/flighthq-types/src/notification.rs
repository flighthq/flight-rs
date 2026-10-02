// @generated from upstream/packages/types/src/Notification.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{EntityRuntime, Signal};

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
    pub reason: String,
}
impl PartialEq for SharedStructuralRecord2 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone)]
pub struct SharedStructuralRecord3 {
    pub __flight_identity: std::sync::Arc<()>,
    pub attach: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(
                        std::sync::Arc<
                            std::sync::Mutex<Box<dyn FnMut(Notification) -> () + Send + 'static>>,
                        >,
                    )
                        -> crate::FlightTask<NotificationEventBackendAttachOutcome>
                    + Send
                    + 'static,
            >,
        >,
    >,
}
impl PartialEq for SharedStructuralRecord3 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct NotificationPermissionQueryOutcomeRecord2 {
    pub __flight_identity: std::sync::Arc<()>,
    pub permission: NotificationPermission,
    pub reason: String,
}
impl PartialEq for NotificationPermissionQueryOutcomeRecord2 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct NotificationActiveListOutcomeRecord2 {
    pub __flight_identity: std::sync::Arc<()>,
    pub notifications: Vec<Notification>,
    pub reason: String,
}
impl PartialEq for NotificationActiveListOutcomeRecord2 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct NotificationPendingListOutcomeRecord2 {
    pub __flight_identity: std::sync::Arc<()>,
    pub notifications: Vec<ScheduledNotification>,
    pub reason: String,
}
impl PartialEq for NotificationPendingListOutcomeRecord2 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct NotificationEventBackendAttachOutcomeRecord1 {
    pub __flight_identity: std::sync::Arc<()>,
    pub reason: String,
    pub release_failed: bool,
}
impl PartialEq for NotificationEventBackendAttachOutcomeRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone)]
pub struct NotificationEventBackendAttachOutcomeRecord2 {
    pub __flight_identity: std::sync::Arc<()>,
    pub attachment: NotificationEventAttachment,
    pub reason: String,
}
impl PartialEq for NotificationEventBackendAttachOutcomeRecord2 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct NotificationSubscriptionAttachOutcomeRecord1 {
    pub __flight_identity: std::sync::Arc<()>,
    pub attach_failed: bool,
    pub reason: String,
    pub release_failed: bool,
}
impl PartialEq for NotificationSubscriptionAttachOutcomeRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct NotificationSubscriptionAttachOutcomeRecord2 {
    pub __flight_identity: std::sync::Arc<()>,
    pub reason: String,
}
impl PartialEq for NotificationSubscriptionAttachOutcomeRecord2 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Notification.ts:4 (sha256:7fd0bbb82b314b177b287a556e835762396dc52116979186506eaf66bd2d0ced)
#[derive(Clone, Default)]
pub struct NotificationAction {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub id: String,
    pub title: String,
    pub icon: Option<String>,
}
impl PartialEq for NotificationAction {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Notification.ts:10 (sha256:10e3f2c0b3b3de4ab198ab3ff6e7ff8116cc5ef89b7cd2ee336f9f66a67deab7)
#[derive(Clone, Default)]
pub struct NotificationRequest {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub title: String,
    pub id: Option<String>,
    pub body: Option<String>,
    pub icon: Option<String>,
    pub badge: Option<String>,
    pub tag: Option<String>,
    pub silent: Option<bool>,
    pub actions: Option<Vec<NotificationAction>>,
    pub dir: Option<String>,
    pub image: Option<String>,
    pub lang: Option<String>,
    pub renotify: Option<bool>,
    pub require_interaction: Option<bool>,
    pub timestamp: Option<f64>,
    pub vibrate: Option<Vec<f64>>,
    pub data: Option<crate::FlightValue>,
}
impl PartialEq for NotificationRequest {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Notification.ts:29 (sha256:58d8bf308312a2a4721a45805528a8755ae0615d3e5d32fc45909f7c6304ec32)
pub type NotificationRequestField = NotificationRequest;

// Source: upstream/packages/types/src/Notification.ts:30 (sha256:b29f15d1256c2335ebcae2aac1e3dd1a67786c35805b49c30d0c82184c3ea00a)
pub type NotificationPermission = String;

// Source: upstream/packages/types/src/Notification.ts:32 (sha256:0209b04bdf408588f14a4ae6964f9c75687d6db63592d5bb2088ba98c8ee3b93)
#[derive(Clone, Default)]
pub struct NotificationSchedule {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub at: f64,
    pub repeat: Option<String>,
}
impl PartialEq for NotificationSchedule {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Notification.ts:39 (sha256:231197375a81a8139c44008a733ab786bf8c020ef92498c898c71df9689a870e)
#[derive(Clone, Default)]
pub struct Notification {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    #[doc(hidden)]
    pub __flight_entity_snapshot: Option<std::sync::Arc<dyn std::any::Any + Send + Sync>>,
    pub id: String,
    pub tag: String,
    pub title: String,
}
impl PartialEq for Notification {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for Notification {
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

// Source: upstream/packages/types/src/Notification.ts:45 (sha256:af37dc742976841075f38f569efa20d27ddd5b8907fa71c229f7ca6fff5b5153)
#[derive(Clone, Default)]
pub struct ScheduledNotification {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    #[doc(hidden)]
    pub __flight_entity_snapshot: Option<std::sync::Arc<dyn std::any::Any + Send + Sync>>,
    pub id: String,
    pub request: NotificationRequest,
    pub schedule: NotificationSchedule,
}
impl PartialEq for ScheduledNotification {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for ScheduledNotification {
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

// Source: upstream/packages/types/src/Notification.ts:51 (sha256:968842c0b2e96aab3d3e50a2fe3eef7064267b55bced819321f0c1a48aa6aeab)
pub type NotificationPermissionQueryOutcome =
    crate::FlightUnion2<NotificationPermissionQueryOutcomeRecord2, SharedStructuralRecord1>;

// Source: upstream/packages/types/src/Notification.ts:55 (sha256:8d1a52c189a161ebb6097cc2395bcec29bfdce926cb761b7e4cfb77ca4e85cb1)
#[derive(Clone, Default)]
pub struct NotificationPermissionRequestOutcome {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub reason: String,
}
impl PartialEq for NotificationPermissionRequestOutcome {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Notification.ts:59 (sha256:c1ed6b9574a3777804325a2b7405c87a2930a5b5044caa4b11b9bdbd4263a49d)
#[derive(Clone, Default)]
pub struct NotificationDeliveryOutcome {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub notification: Option<Notification>,
    pub reason: String,
    pub fields: Option<Vec<NotificationRequestField>>,
}
impl PartialEq for NotificationDeliveryOutcome {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Notification.ts:67 (sha256:bcdde4895a9924380a9224a4ffd533f391a1c734498469ecd03d8432f832457b)
#[derive(Clone, Default)]
pub struct NotificationScheduleOutcome {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub precision: Option<String>,
    pub reason: String,
    pub scheduled: Option<ScheduledNotification>,
    pub fields: Option<Vec<crate::FlightUnion2<NotificationRequestField, String>>>,
}
impl PartialEq for NotificationScheduleOutcome {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Notification.ts:79 (sha256:1f91525ecf1e3fdadb0b1717a70bd9a2970eb76c6cfd2f4bcc9e1ab3f6650374)
#[derive(Clone, Default)]
pub struct NotificationCloseOutcome {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub reason: String,
}
impl PartialEq for NotificationCloseOutcome {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Notification.ts:82 (sha256:a063a207ade8aece1368bc9bdcfda3bc2244c376f4fe2d98aef06e9037e31a33)
#[derive(Clone, Default)]
pub struct NotificationCancelOutcome {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub reason: String,
}
impl PartialEq for NotificationCancelOutcome {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Notification.ts:86 (sha256:268bfa028354945d887487ba18464d68a0f5896bf0032408ea6fe72a95a0f9fa)
pub type NotificationActiveListOutcome =
    crate::FlightUnion2<NotificationActiveListOutcomeRecord2, SharedStructuralRecord1>;

// Source: upstream/packages/types/src/Notification.ts:93 (sha256:30dbef21881e9ba9e62bd764dc7285ea3df012f5648bfeb59e10e77bf5f413b1)
pub type NotificationPendingListOutcome =
    crate::FlightUnion2<NotificationPendingListOutcomeRecord2, SharedStructuralRecord1>;

// Source: upstream/packages/types/src/Notification.ts:100 (sha256:a74785857d3baf72a942725e10f1534d665ea080229e616ff6ce58548d1ee9ba)
#[derive(Clone, Default)]
pub struct NotificationLifecycleFailure {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub id: String,
    pub operation: String,
}
impl PartialEq for NotificationLifecycleFailure {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Notification.ts:105 (sha256:4f4ecfad19f9172bdd22ca1a2d60835ea4c9588352bf8642de1d17c019f4e204)
#[derive(Clone, Default)]
pub struct NotificationLifecycleOutcome {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub reason: String,
    pub failures: Option<Vec<NotificationLifecycleFailure>>,
}
impl PartialEq for NotificationLifecycleOutcome {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Notification.ts:112 (sha256:6f05889a17330d1190f4db7b3d2fdc5c3017f3e0b54f66c98327b4bc629f5f98)
#[derive(Clone, Default)]
pub struct NotificationEventReleaseOutcome {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub reason: String,
}
impl PartialEq for NotificationEventReleaseOutcome {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Notification.ts:116 (sha256:5373aa8c6af77f99aecc8744e4970f8281e1d44de7b3df86ee04e7a79c3072d2)
#[derive(Clone)]
pub struct NotificationEventAttachment {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub release: std::sync::Arc<
        std::sync::Mutex<
            Box<dyn FnMut() -> crate::FlightTask<NotificationEventReleaseOutcome> + Send + 'static>,
        >,
    >,
}
impl PartialEq for NotificationEventAttachment {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Notification.ts:120 (sha256:66aebee9977322c40ac7e68f2b3d8e64432d8fc9b18b5d388ce8d69885d5f318)
pub type NotificationEventBackendAttachOutcome = crate::FlightUnion2<
    NotificationEventBackendAttachOutcomeRecord2,
    NotificationEventBackendAttachOutcomeRecord1,
>;

// Source: upstream/packages/types/src/Notification.ts:124 (sha256:cada77bfb7a7fde93de345d73fb6359cd8ca621f6e6d265e22c88cbf1f49bb71)
pub type NotificationSubscriptionAttachOutcome = crate::FlightUnion2<
    NotificationSubscriptionAttachOutcomeRecord2,
    NotificationSubscriptionAttachOutcomeRecord1,
>;

// Source: upstream/packages/types/src/Notification.ts:132 (sha256:50b9c76be639e2e7b0afdb96599eee1172c871506071c16119f79cbc0610a7ef)
#[derive(Clone, Default)]
pub struct NotificationSubscriptionDetachOutcome {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub reason: String,
    pub release_failed: Option<bool>,
}
impl PartialEq for NotificationSubscriptionDetachOutcome {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Notification.ts:136 (sha256:8385bdd9bb047f59b93a54e4ad68709c3c3df9a62ae25897f9b0610f5d1508c3)
#[derive(Clone, Default)]
pub struct NotificationSubscriptionDisposeOutcome {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub reason: String,
    pub attach_failed: Option<bool>,
    pub release_failed: Option<bool>,
}
impl PartialEq for NotificationSubscriptionDisposeOutcome {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Notification.ts:144 (sha256:4eef00c5ecfd19c52e811056ee4115c924f1da77bcedf5e8bfd8b1299ca747d4)
#[derive(Clone)]
pub struct NotificationActionSubscription {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    #[doc(hidden)]
    pub __flight_entity_snapshot: Option<std::sync::Arc<dyn std::any::Any + Send + Sync>>,
    pub on_notification_action: Signal<
        std::sync::Arc<
            std::sync::Mutex<Box<dyn FnMut(Notification, String) -> () + Send + 'static>>,
        >,
    >,
}
impl PartialEq for NotificationActionSubscription {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for NotificationActionSubscription {
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

// Source: upstream/packages/types/src/Notification.ts:148 (sha256:c3c1de3b109210f5a632f49a1d9852f22183dd9d5de285cee4cceb3b98c3e4cd)
#[derive(Clone)]
pub struct NotificationClickSubscription {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    #[doc(hidden)]
    pub __flight_entity_snapshot: Option<std::sync::Arc<dyn std::any::Any + Send + Sync>>,
    pub on_notification_click: Signal<
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(Notification) -> () + Send + 'static>>>,
    >,
}
impl PartialEq for NotificationClickSubscription {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for NotificationClickSubscription {
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

// Source: upstream/packages/types/src/Notification.ts:152 (sha256:3cf7d0d700fbf24a7c170c9fd140c2963265bb6ef00537b92ec4ff52d2cde583)
#[derive(Clone)]
pub struct NotificationDismissSubscription {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    #[doc(hidden)]
    pub __flight_entity_snapshot: Option<std::sync::Arc<dyn std::any::Any + Send + Sync>>,
    pub on_notification_dismiss: Signal<
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(Notification) -> () + Send + 'static>>>,
    >,
}
impl PartialEq for NotificationDismissSubscription {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for NotificationDismissSubscription {
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

// Source: upstream/packages/types/src/Notification.ts:156 (sha256:d67b68b47337d1d0d8f2c833f71dc13a6d2846142b21cb0b11ae919aba22f990)
#[derive(Clone)]
pub struct NotificationReplySubscription {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    #[doc(hidden)]
    pub __flight_entity_snapshot: Option<std::sync::Arc<dyn std::any::Any + Send + Sync>>,
    pub on_notification_reply: Signal<
        std::sync::Arc<
            std::sync::Mutex<Box<dyn FnMut(Notification, String, String) -> () + Send + 'static>>,
        >,
    >,
}
impl PartialEq for NotificationReplySubscription {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for NotificationReplySubscription {
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

// Source: upstream/packages/types/src/Notification.ts:160 (sha256:347810963230aca91ee269a7381c05a3e56b32ab060eb574b496b562328bc5af)
#[derive(Clone)]
pub struct NotificationReceivedSubscription {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    #[doc(hidden)]
    pub __flight_entity_snapshot: Option<std::sync::Arc<dyn std::any::Any + Send + Sync>>,
    pub on_notification_received: Signal<
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(Notification) -> () + Send + 'static>>>,
    >,
}
impl PartialEq for NotificationReceivedSubscription {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for NotificationReceivedSubscription {
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

// Source: upstream/packages/types/src/Notification.ts:164 (sha256:b5cccc084bcd86d67bb6d8ae357c8b78a7189fa25e6e7a848b8a0bab0c356fe6)
#[derive(Clone)]
pub struct HostNotificationPermissionCapability {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub get_permission: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut() -> crate::FlightTask<NotificationPermissionQueryOutcome>
                    + Send
                    + 'static,
            >,
        >,
    >,
    pub request_permission: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut() -> crate::FlightTask<NotificationPermissionRequestOutcome>
                    + Send
                    + 'static,
            >,
        >,
    >,
}
impl PartialEq for HostNotificationPermissionCapability {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Notification.ts:169 (sha256:e181a9fd3c7943fdf001476e37bf9dc1fd6f4e79fad1a37d33729a492c680176)
#[derive(Clone)]
pub struct HostNotificationDeliveryCapability {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub notify: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(NotificationRequest) -> crate::FlightTask<NotificationDeliveryOutcome>
                    + Send
                    + 'static,
            >,
        >,
    >,
}
impl PartialEq for HostNotificationDeliveryCapability {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Notification.ts:173 (sha256:1b51fb014852896138a12cea1fc4e56da924eeb849cff09227da9c406bbf8b44)
#[derive(Clone)]
pub struct HostNotificationSchedulingCapability {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub cancel_all_scheduled_notifications: std::sync::Arc<
        std::sync::Mutex<
            Box<dyn FnMut() -> crate::FlightTask<NotificationLifecycleOutcome> + Send + 'static>,
        >,
    >,
    pub get_pending_notifications: std::sync::Arc<
        std::sync::Mutex<
            Box<dyn FnMut() -> crate::FlightTask<NotificationPendingListOutcome> + Send + 'static>,
        >,
    >,
    pub schedule_notification: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(
                        NotificationRequest,
                        NotificationSchedule,
                    ) -> crate::FlightTask<NotificationScheduleOutcome>
                    + Send
                    + 'static,
            >,
        >,
    >,
}
impl PartialEq for HostNotificationSchedulingCapability {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Notification.ts:182 (sha256:bf87adcfc70991c8b2694c7f6be557dc0330d4e0d578fc785d6abdbf897e1a75)
#[derive(Clone)]
pub struct HostNotificationCloseCapability {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub close_all_notifications: std::sync::Arc<
        std::sync::Mutex<
            Box<dyn FnMut() -> crate::FlightTask<NotificationLifecycleOutcome> + Send + 'static>,
        >,
    >,
}
impl PartialEq for HostNotificationCloseCapability {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Notification.ts:186 (sha256:2d3205f63a53f1c4010caf4e55c820cf26afffca618fea7561a317ee431f864e)
#[derive(Clone)]
pub struct HostNotificationActiveListCapability {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub get_active_notifications: std::sync::Arc<
        std::sync::Mutex<
            Box<dyn FnMut() -> crate::FlightTask<NotificationActiveListOutcome> + Send + 'static>,
        >,
    >,
}
impl PartialEq for HostNotificationActiveListCapability {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Notification.ts:190 (sha256:4a013d3385d216f2a0cddf8d3f290c9db2fab83cbec046980978d67db18d119f)
#[derive(Clone)]
pub struct HostNotificationActionCapability {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub attach: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(
                        std::sync::Arc<
                            std::sync::Mutex<
                                Box<dyn FnMut(Notification, String) -> () + Send + 'static>,
                            >,
                        >,
                    )
                        -> crate::FlightTask<NotificationEventBackendAttachOutcome>
                    + Send
                    + 'static,
            >,
        >,
    >,
}
impl PartialEq for HostNotificationActionCapability {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Notification.ts:196 (sha256:bfad4c4b525ee497d2febd8eebd8dbcee08f6748733968dabd2ef09b9549ccd1)
#[derive(Clone)]
pub struct HostNotificationClickCapability {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub attach: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(
                        std::sync::Arc<
                            std::sync::Mutex<Box<dyn FnMut(Notification) -> () + Send + 'static>>,
                        >,
                    )
                        -> crate::FlightTask<NotificationEventBackendAttachOutcome>
                    + Send
                    + 'static,
            >,
        >,
    >,
}
impl PartialEq for HostNotificationClickCapability {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Notification.ts:200 (sha256:4f73321fe45c3e34f4cc9d03b37c0d1f9a0f74a3ff0551065701fb3df4f0d425)
#[derive(Clone)]
pub struct HostNotificationDismissCapability {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub attach: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(
                        std::sync::Arc<
                            std::sync::Mutex<Box<dyn FnMut(Notification) -> () + Send + 'static>>,
                        >,
                    )
                        -> crate::FlightTask<NotificationEventBackendAttachOutcome>
                    + Send
                    + 'static,
            >,
        >,
    >,
}
impl PartialEq for HostNotificationDismissCapability {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Notification.ts:204 (sha256:b5c8cb9a9871bccecdf52a717655926efc6868d3fe9998dfe22cac4caf1b0c92)
#[derive(Clone)]
pub struct HostNotificationReplyCapability {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub attach: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(
                        std::sync::Arc<
                            std::sync::Mutex<
                                Box<dyn FnMut(Notification, String, String) -> () + Send + 'static>,
                            >,
                        >,
                    )
                        -> crate::FlightTask<NotificationEventBackendAttachOutcome>
                    + Send
                    + 'static,
            >,
        >,
    >,
}
impl PartialEq for HostNotificationReplyCapability {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Notification.ts:210 (sha256:44863947f2f28277255888d8420ced8a7ee9839424c4b6731a9a9c3fc421a088)
#[derive(Clone)]
pub struct HostNotificationReceivedCapability {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub attach: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(
                        std::sync::Arc<
                            std::sync::Mutex<Box<dyn FnMut(Notification) -> () + Send + 'static>>,
                        >,
                    )
                        -> crate::FlightTask<NotificationEventBackendAttachOutcome>
                    + Send
                    + 'static,
            >,
        >,
    >,
}
impl PartialEq for HostNotificationReceivedCapability {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Notification.ts:214 (sha256:b6fa88a212ae5961c9efe7448dd354f6dac78644c06dc60b804903f5c3fc5b5f)
#[derive(Clone)]
pub struct HostNotificationLifecycleCapability {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub destroy: std::sync::Arc<
        std::sync::Mutex<
            Box<dyn FnMut() -> crate::FlightTask<NotificationLifecycleOutcome> + Send + 'static>,
        >,
    >,
}
impl PartialEq for HostNotificationLifecycleCapability {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Notification.ts:218 (sha256:fde963202a7fa0679b3fa5a4882d22eeb4c9ba8ce888866830a442ad616e239b)
#[derive(Clone)]
pub struct WebPageNotificationCapabilities {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub click: HostNotificationClickCapability,
    pub close: HostNotificationCloseCapability,
    pub delivery: HostNotificationDeliveryCapability,
    pub dismiss: HostNotificationDismissCapability,
    pub lifecycle: HostNotificationLifecycleCapability,
    pub permission: HostNotificationPermissionCapability,
    pub received: HostNotificationReceivedCapability,
}
impl PartialEq for WebPageNotificationCapabilities {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Notification.ts:228 (sha256:9cc67bacf9c38c2b8d40e223967544739c031c12d578e77b5a70f6899b7fb24a)
#[derive(Clone)]
pub struct WebServiceWorkerNotificationCapabilities {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub action: HostNotificationActionCapability,
    pub active_list: HostNotificationActiveListCapability,
    pub click: HostNotificationClickCapability,
    pub close: HostNotificationCloseCapability,
    pub delivery: HostNotificationDeliveryCapability,
    pub dismiss: HostNotificationDismissCapability,
    pub lifecycle: HostNotificationLifecycleCapability,
    pub permission: HostNotificationPermissionCapability,
}
impl PartialEq for WebServiceWorkerNotificationCapabilities {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Notification.ts:239 (sha256:c1acf3fa194d24fbf675acf009a0f755c0c0fae5f3149bb4b99598c3b1bac96e)
#[derive(Clone)]
pub struct ElectronNotificationCapabilities {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub click: HostNotificationClickCapability,
    pub close: HostNotificationCloseCapability,
    pub delivery: HostNotificationDeliveryCapability,
    pub dismiss: HostNotificationDismissCapability,
    pub lifecycle: HostNotificationLifecycleCapability,
    pub received: HostNotificationReceivedCapability,
}
impl PartialEq for ElectronNotificationCapabilities {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Notification.ts:248 (sha256:e2af7501413d77fb2c6cf0823a857129dded0dd4b9eb92d224717a22efbaf00d)
#[derive(Clone)]
pub struct ElectronMacosNotificationCapabilities {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub click: HostNotificationClickCapability,
    pub close: HostNotificationCloseCapability,
    pub delivery: HostNotificationDeliveryCapability,
    pub dismiss: HostNotificationDismissCapability,
    pub lifecycle: HostNotificationLifecycleCapability,
    pub received: HostNotificationReceivedCapability,
    pub action: HostNotificationActionCapability,
    pub reply: HostNotificationReplyCapability,
}
impl PartialEq for ElectronMacosNotificationCapabilities {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Notification.ts:254 (sha256:6caa5e9c7a39fe0ee2e85066e540992a43ee227180a33b739eac8a221b3403d3)
#[derive(Clone)]
pub struct TauriNotificationCapabilities {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub delivery: HostNotificationDeliveryCapability,
    pub lifecycle: HostNotificationLifecycleCapability,
    pub permission: HostNotificationPermissionCapability,
}
impl PartialEq for TauriNotificationCapabilities {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Notification.ts:260 (sha256:cdbcbc86d298f9f7ca664a17ecbbf79b42d66eebf9bf1ffa4ce3a8eb2600e249)
#[derive(Clone)]
pub struct CapacitorNotificationCapabilities {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub action: HostNotificationActionCapability,
    pub click: HostNotificationClickCapability,
    pub delivery: HostNotificationDeliveryCapability,
    pub lifecycle: HostNotificationLifecycleCapability,
    pub permission: HostNotificationPermissionCapability,
    pub scheduling: HostNotificationSchedulingCapability,
}
impl PartialEq for CapacitorNotificationCapabilities {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Notification.ts:269 (sha256:13d6cd24c2b0e67521bed107a16eb7c04c5f74c8e8b06f60fde72b0475ea5ade)
#[derive(Clone, Default)]
pub struct WebNotificationOptionsRecord1 {
    pub __flight_identity: std::sync::Arc<()>,
    pub action: String,
    pub icon: Option<String>,
    pub title: String,
}
impl PartialEq for WebNotificationOptionsRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct WebNotificationOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub actions: Option<Vec<WebNotificationOptionsRecord1>>,
    pub badge: Option<String>,
    pub body: Option<String>,
    pub data: Option<crate::FlightValue>,
    pub dir: Option<String>,
    pub icon: Option<String>,
    pub image: Option<String>,
    pub lang: Option<String>,
    pub renotify: Option<bool>,
    pub require_interaction: Option<bool>,
    pub silent: Option<bool>,
    pub tag: Option<String>,
    pub timestamp: Option<f64>,
    pub vibrate: Option<Vec<f64>>,
}
impl PartialEq for WebNotificationOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Notification.ts:286 (sha256:529e25f6eba45b867f721e9f28e10ea9638603e7000c59d9f8b7a5c3d4b1edd2)
#[derive(Clone)]
pub struct WebPageNotificationInstance {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub close: std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>,
    pub onclick: Option<std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>>,
    pub onclose: Option<std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>>,
    pub onerror: Option<std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>>,
    pub onshow: Option<std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>>,
}
impl PartialEq for WebPageNotificationInstance {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Notification.ts:294 (sha256:8474ba3ad5630737d518b843d04521e8e01341bf904136f3382d0a5ef6363982)
#[derive(Clone, Default)]
pub struct WebPageNotificationApiRecord1 {
    pub __flight_identity: std::sync::Arc<()>,
    pub __construct: Option<crate::OpaqueHostValue>,
}
impl PartialEq for WebPageNotificationApiRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct WebPageNotificationApi {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub notification: WebPageNotificationApiRecord1,
}
impl PartialEq for WebPageNotificationApi {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Notification.ts:300 (sha256:bb5184da6924a17e1bd02ca8a964c555ed5aacb657b5ea703e27cca455f3433b)
#[derive(Clone)]
pub struct WebServiceWorkerNotificationInstance {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub data: Option<crate::FlightValue>,
    pub tag: String,
    pub title: String,
    pub close: std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>,
}
impl PartialEq for WebServiceWorkerNotificationInstance {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Notification.ts:307 (sha256:4492dd0d4caae01f061f662b09500a0dd976178494ff03282f77cdbe52f3d451)
#[derive(Clone, Default)]
pub struct WebServiceWorkerNotificationRegistrationRecord1 {
    pub __flight_identity: std::sync::Arc<()>,
    pub tag: Option<String>,
}
impl PartialEq for WebServiceWorkerNotificationRegistrationRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone)]
pub struct WebServiceWorkerNotificationRegistration {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub get_notifications: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(
                        Option<WebServiceWorkerNotificationRegistrationRecord1>,
                    )
                        -> crate::FlightTask<Vec<WebServiceWorkerNotificationInstance>>
                    + Send
                    + 'static,
            >,
        >,
    >,
    pub show_notification: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(String, Option<WebNotificationOptions>) -> crate::FlightTask<()>
                    + Send
                    + 'static,
            >,
        >,
    >,
}
impl PartialEq for WebServiceWorkerNotificationRegistration {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Notification.ts:312 (sha256:34334b661053b8f2d863bd7238f18cacf030c0efda19471af2db806cf5f9d1d0)
#[derive(Clone)]
pub struct WebServiceWorkerNotificationApiRecord1 {
    pub __flight_identity: std::sync::Arc<()>,
    pub get_permission: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut() -> NotificationPermission + Send + 'static>>,
    >,
    pub request_permission: std::sync::Arc<
        std::sync::Mutex<
            Box<dyn FnMut() -> crate::FlightTask<NotificationPermission> + Send + 'static>,
        >,
    >,
}
impl PartialEq for WebServiceWorkerNotificationApiRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone)]
pub struct WebServiceWorkerNotificationApi {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub permission: WebServiceWorkerNotificationApiRecord1,
    pub registration: WebServiceWorkerNotificationRegistration,
}
impl PartialEq for WebServiceWorkerNotificationApi {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Notification.ts:320 (sha256:c390dc415faf0fe4c5802b061ee43837192d55fc203115323f058f0c0f58f777)
#[derive(Clone, Default)]
pub struct WebServiceWorkerNotificationEvent {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub action_id: Option<String>,
    pub notification_tag: String,
    pub type_: String,
}
impl PartialEq for WebServiceWorkerNotificationEvent {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
