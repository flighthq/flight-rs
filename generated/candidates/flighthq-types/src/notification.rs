// @generated from upstream/packages/types/src/Notification.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/Notification.ts:3 (sha256:7fd0bbb82b314b177b287a556e835762396dc52116979186506eaf66bd2d0ced)
#[derive(Clone)]
pub struct NotificationAction {
    pub id: String,
    pub title: String,
    pub icon: Option<String>,
}

// Source: upstream/packages/types/src/Notification.ts:10 (sha256:10e3f2c0b3b3de4ab198ab3ff6e7ff8116cc5ef89b7cd2ee336f9f66a67deab7)
#[derive(Clone)]
pub struct NotificationRequest {
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
    pub data: Option<crate::OpaqueHostValue>,
}

// Source: upstream/packages/types/src/Notification.ts:41 (sha256:0f489909137afb911d21dce4bdd290acbfa4e6dfce45ab5823bc4dc2b6105d53)
#[derive(Clone)]
pub struct NotificationChannel {
    pub id: String,
    pub name: String,
}

// Source: upstream/packages/types/src/Notification.ts:48 (sha256:b29f15d1256c2335ebcae2aac1e3dd1a67786c35805b49c30d0c82184c3ea00a)
pub type NotificationPermission = String;

// Source: upstream/packages/types/src/Notification.ts:52 (sha256:320b9d34e90f00b4c40a8ab680705177fb8843f754fa87f576471dce44afbf68)
#[derive(Clone)]
pub struct NotificationCapabilities {
    pub actions: bool,
    pub channels: bool,
    pub cold_start: bool,
    pub image: bool,
    pub list_active: bool,
    pub scheduling: bool,
    pub text_reply: bool,
}

// Source: upstream/packages/types/src/Notification.ts:70 (sha256:0209b04bdf408588f14a4ae6964f9c75687d6db63592d5bb2088ba98c8ee3b93)
#[derive(Clone)]
pub struct NotificationSchedule {
    pub at: f64,
    pub repeat: Option<String>,
}

// Source: upstream/packages/types/src/Notification.ts:79 (sha256:c7c5d774886f84fb54b5104b263c47ed6a533f7029791896fdf5ecd90d3a9c5c)
#[derive(Clone)]
pub struct ScheduledNotification {
    pub id: String,
    pub request: NotificationRequest,
    pub schedule: NotificationSchedule,
}

// Source: upstream/packages/types/src/Notification.ts:90 (sha256:522a0ab8c797f8ba7291fab6544c051946f8210c211b0b706d7d43736217a954)
#[derive(Clone)]
pub struct NotificationBackend {
    pub notify: crate::OpaqueHostValue,
    pub request_permission: crate::OpaqueHostValue,
    pub get_permission: crate::OpaqueHostValue,
    pub is_supported: crate::OpaqueHostValue,
    pub get_capabilities: crate::OpaqueHostValue,
    pub get_launch_notification: crate::OpaqueHostValue,
    pub get_active_notifications: crate::OpaqueHostValue,
    pub get_pending_notifications: crate::OpaqueHostValue,
    pub schedule_notification: crate::OpaqueHostValue,
    pub cancel_scheduled_notification: crate::OpaqueHostValue,
    pub close_notification: crate::OpaqueHostValue,
    pub close_all_notifications: crate::OpaqueHostValue,
    pub update_notification: crate::OpaqueHostValue,
    pub subscribe_click: crate::OpaqueHostValue,
    pub subscribe_action: crate::OpaqueHostValue,
    pub subscribe_dismiss: crate::OpaqueHostValue,
    pub subscribe_reply: crate::OpaqueHostValue,
    pub subscribe_show: crate::OpaqueHostValue,
}
