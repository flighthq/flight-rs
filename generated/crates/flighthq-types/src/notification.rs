// @generated from upstream/packages/types/src/Notification.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

#[derive(Clone, Default)]
pub struct FlightPartialRecord1805580541 {
    pub __flight_identity: std::sync::Arc<()>,
    pub title: Option<String>,
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
impl PartialEq for FlightPartialRecord1805580541 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Notification.ts:3 (sha256:7fd0bbb82b314b177b287a556e835762396dc52116979186506eaf66bd2d0ced)
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

// Source: upstream/packages/types/src/Notification.ts:41 (sha256:0f489909137afb911d21dce4bdd290acbfa4e6dfce45ab5823bc4dc2b6105d53)
#[derive(Clone, Default)]
pub struct NotificationChannel {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub id: String,
    pub name: String,
}
impl PartialEq for NotificationChannel {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Notification.ts:48 (sha256:b29f15d1256c2335ebcae2aac1e3dd1a67786c35805b49c30d0c82184c3ea00a)
pub type NotificationPermission = String;

// Source: upstream/packages/types/src/Notification.ts:52 (sha256:320b9d34e90f00b4c40a8ab680705177fb8843f754fa87f576471dce44afbf68)
#[derive(Clone, Default)]
pub struct NotificationCapabilities {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub actions: bool,
    pub channels: bool,
    pub cold_start: bool,
    pub image: bool,
    pub list_active: bool,
    pub scheduling: bool,
    pub text_reply: bool,
}
impl PartialEq for NotificationCapabilities {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Notification.ts:70 (sha256:0209b04bdf408588f14a4ae6964f9c75687d6db63592d5bb2088ba98c8ee3b93)
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

// Source: upstream/packages/types/src/Notification.ts:79 (sha256:c7c5d774886f84fb54b5104b263c47ed6a533f7029791896fdf5ecd90d3a9c5c)
#[derive(Clone, Default)]
pub struct ScheduledNotification {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub id: String,
    pub request: NotificationRequest,
    pub schedule: NotificationSchedule,
}
impl PartialEq for ScheduledNotification {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Notification.ts:90 (sha256:522a0ab8c797f8ba7291fab6544c051946f8210c211b0b706d7d43736217a954)
#[derive(Clone)]
pub struct NotificationBackend {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub notify: std::sync::Arc<
        std::sync::Mutex<
            Box<dyn FnMut(NotificationRequest) -> crate::FlightTask<String> + Send + 'static>,
        >,
    >,
    pub request_permission: std::sync::Arc<
        std::sync::Mutex<
            Box<dyn FnMut() -> crate::FlightTask<NotificationPermission> + Send + 'static>,
        >,
    >,
    pub get_permission: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut() -> NotificationPermission + Send + 'static>>,
    >,
    pub is_supported: std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> bool + Send + 'static>>>,
    pub get_capabilities: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut() -> NotificationCapabilities + Send + 'static>>,
    >,
    pub get_launch_notification: std::sync::Arc<
        std::sync::Mutex<
            Box<dyn FnMut() -> crate::FlightTask<Option<NotificationRequest>> + Send + 'static>,
        >,
    >,
    pub get_active_notifications: std::sync::Arc<
        std::sync::Mutex<
            Box<dyn FnMut() -> crate::FlightTask<Vec<NotificationRequest>> + Send + 'static>,
        >,
    >,
    pub get_pending_notifications: std::sync::Arc<
        std::sync::Mutex<
            Box<dyn FnMut() -> crate::FlightTask<Vec<ScheduledNotification>> + Send + 'static>,
        >,
    >,
    pub schedule_notification: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(NotificationRequest, NotificationSchedule) -> crate::FlightTask<String>
                    + Send
                    + 'static,
            >,
        >,
    >,
    pub cancel_scheduled_notification:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(String) -> () + Send + 'static>>>,
    pub close_notification:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(String) -> () + Send + 'static>>>,
    pub close_all_notifications:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>,
    pub update_notification: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(String, FlightPartialRecord1805580541) -> crate::FlightTask<bool>
                    + Send
                    + 'static,
            >,
        >,
    >,
    pub subscribe_click: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(
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
    pub subscribe_action: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(
                        std::sync::Arc<
                            std::sync::Mutex<Box<dyn FnMut(String, String) -> () + Send + 'static>>,
                        >,
                    ) -> std::sync::Arc<
                        std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>,
                    > + Send
                    + 'static,
            >,
        >,
    >,
    pub subscribe_dismiss: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(
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
    pub subscribe_reply: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(
                        std::sync::Arc<
                            std::sync::Mutex<
                                Box<dyn FnMut(String, String, String) -> () + Send + 'static>,
                            >,
                        >,
                    ) -> std::sync::Arc<
                        std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>,
                    > + Send
                    + 'static,
            >,
        >,
    >,
    pub subscribe_show: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(
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
impl PartialEq for NotificationBackend {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
