// @generated from upstream/packages/types/src/App.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{EntityRuntime, MenuItemTemplate, Signal};

#[derive(Clone)]
pub struct SharedStructuralRecord1 {
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
impl PartialEq for SharedStructuralRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/App.ts:8 (sha256:d3f9581fb8f160890538a1b921de3c394e43f6d8c0dffc2a85f2855c87719740)
pub type AppActivationPolicy = String;

// Source: upstream/packages/types/src/App.ts:11 (sha256:0281de8d7faa1c59332d24025a12c05617382864b9ea653e696fc73447d42e38)
#[derive(Clone, Default)]
pub struct AppLoginItem {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub open_at_login: bool,
    pub open_as_hidden: bool,
    pub path: String,
    pub args: Vec<String>,
}
impl PartialEq for AppLoginItem {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/App.ts:23 (sha256:b5fa820102a6212ceff5de2018db5d6130b3cac3032ba564f49a3b3cf978f7ee)
#[derive(Clone, Default)]
pub struct AppLoginItemLike {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub open_at_login: Option<bool>,
    pub open_as_hidden: Option<bool>,
    pub path: Option<String>,
    pub args: Option<Vec<String>>,
}
impl PartialEq for AppLoginItemLike {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/App.ts:32 (sha256:919ed3b8ac65b8961138f34a70d769b628ba8dc2cfe7c5f269cb7ff76c134806)
pub type AppPathKind = String;

// Source: upstream/packages/types/src/App.ts:35 (sha256:9acc1c11acf7bcf51900f1dc4dd7f32256a048f8880bf604ea77a9c17e6517bc)
#[derive(Clone)]
pub struct AppEvents {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    #[doc(hidden)]
    pub __flight_entity_snapshot: Option<std::sync::Arc<dyn std::any::Any + Send + Sync>>,
    pub on_activate:
        Signal<std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>>,
    pub on_all_windows_closed:
        Signal<std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>>,
    pub on_open_file:
        Signal<std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(String) -> () + Send + 'static>>>>,
    pub on_quit_request:
        Signal<std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>>,
    pub on_ready: Signal<std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>>,
    pub on_second_instance: Signal<
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(Vec<String>) -> () + Send + 'static>>>,
    >,
}
impl PartialEq for AppEvents {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for AppEvents {
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

// Source: upstream/packages/types/src/App.ts:45 (sha256:289838e73c38e7c83e99c71e9fcb45c5529877abb4b0d3ad4eb1aee83318d4a8)
pub type MobileOsProfile = String;

// Source: upstream/packages/types/src/App.ts:47 (sha256:9d1a30f214d759b6ada05bda254df6b163eafd5a08599f4b45af17f19eee988c)
#[derive(Clone)]
pub struct HostAppActivateCapability {
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
impl PartialEq for HostAppActivateCapability {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/App.ts:51 (sha256:f8085523b1c5780fba0aea6d87251bd22c48f056dd3167abde938b6eb69c7a18)
#[derive(Clone)]
pub struct HostAppActivationPolicyCapability {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub set_activation_policy: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut(AppActivationPolicy) -> () + Send + 'static>>,
    >,
}
impl PartialEq for HostAppActivationPolicyCapability {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/App.ts:55 (sha256:464ce8cde3003c062c5b834c5c57f26bfd4b4e135adc00252d0270d446699f1a)
#[derive(Clone)]
pub struct HostAppAllWindowsClosedCapability {
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
impl PartialEq for HostAppAllWindowsClosedCapability {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/App.ts:59 (sha256:37a3e56524fb38e9e5da301e165d2d0954656a567b61a0026d683fcfe16452e0)
#[derive(Clone)]
pub struct HostAppBadgeCapability {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub set_badge_count: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut(f64) -> crate::FlightTask<bool> + Send + 'static>>,
    >,
}
impl PartialEq for HostAppBadgeCapability {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/App.ts:63 (sha256:a94a0f7437b82b429509c3b34d640f29367afe111fc0485de78fbf85eb4eb8d2)
#[derive(Clone)]
pub struct HostAppDockCapability {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub bounce_dock: std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> f64 + Send + 'static>>>,
    pub cancel_attention:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(f64) -> () + Send + 'static>>>,
    pub cancel_dock_bounce:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(f64) -> () + Send + 'static>>>,
    pub request_attention:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(bool) -> f64 + Send + 'static>>>,
    pub set_dock_badge:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(String) -> () + Send + 'static>>>,
    pub set_dock_menu: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut(Vec<MenuItemTemplate>) -> () + Send + 'static>>,
    >,
}
impl PartialEq for HostAppDockCapability {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/App.ts:72 (sha256:bd4553048edba1de72f9987b93e1b1ee69f28d1cd3983aebb8389a7437fc7917)
#[derive(Clone)]
pub struct HostAppFocusCapability {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub focus: std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>,
}
impl PartialEq for HostAppFocusCapability {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/App.ts:76 (sha256:0b3746fca909c792384c8ec43133c35e7136953a7ac9822e68712f2d5a6e4274)
#[derive(Clone)]
pub struct HostAppLocaleCapability {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub get_locale: std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> String + Send + 'static>>>,
    pub get_preferred_system_languages:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> Vec<String> + Send + 'static>>>,
    pub get_system_locale:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> String + Send + 'static>>>,
}
impl PartialEq for HostAppLocaleCapability {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/App.ts:82 (sha256:b1c1688b2fe00f3f48fe9f2760eb10d31abc82a942e6d15d5bff1888cbf4b0b2)
#[derive(Clone)]
pub struct HostAppLoginItemCapability {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub get_login_item:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> AppLoginItem + Send + 'static>>>,
    pub set_login_item:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(AppLoginItemLike) -> () + Send + 'static>>>,
}
impl PartialEq for HostAppLoginItemCapability {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/App.ts:87 (sha256:cc163a978105e1ed9fa898d8e6c514262c3f3fa0a3d647181ff1e17d89932116)
#[derive(Clone)]
pub struct HostAppNameCapability {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub get_name: std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> String + Send + 'static>>>,
}
impl PartialEq for HostAppNameCapability {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/App.ts:91 (sha256:cc74468e58a59cf246ba1ed6e95d938ff5e187dc811a45b3bc42ba09d4e68bb1)
#[derive(Clone)]
pub struct HostAppNameWriteCapability {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub set_name: std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(String) -> () + Send + 'static>>>,
}
impl PartialEq for HostAppNameWriteCapability {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/App.ts:95 (sha256:0dbea04efe993cb30492661812d4b5918c303436e8b148253253e878b43b1762)
#[derive(Clone)]
pub struct HostAppOpenFileCapability {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub subscribe: std::sync::Arc<
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
impl PartialEq for HostAppOpenFileCapability {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/App.ts:99 (sha256:a51cb9c29e14472167e170d0a14c8b0cc75746d22fe5a5506090c9e6a1218973)
#[derive(Clone)]
pub struct HostAppPathCapability {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub get_app_directory_path:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(AppPathKind) -> String + Send + 'static>>>,
    pub get_app_path: std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> String + Send + 'static>>>,
    pub get_executable_path:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> String + Send + 'static>>>,
}
impl PartialEq for HostAppPathCapability {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/App.ts:105 (sha256:86fe4d077f2b6c57e1254e6cdcfd1b303ab866f10b6d79de2db3727960d20ce0)
#[derive(Clone)]
pub struct HostAppQuitCapability {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub quit: std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>,
}
impl PartialEq for HostAppQuitCapability {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/App.ts:109 (sha256:10d56782663a247082acb5fa870914261039c9d54bfbfead8b18c8240b1c8f73)
#[derive(Clone)]
pub struct HostAppQuitRequestCapability {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub subscribe: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(
                        std::sync::Arc<
                            std::sync::Mutex<
                                Box<
                                    dyn FnMut(
                                            std::sync::Arc<
                                                std::sync::Mutex<
                                                    Box<dyn FnMut() -> () + Send + 'static>,
                                                >,
                                            >,
                                        ) -> ()
                                        + Send
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
impl PartialEq for HostAppQuitRequestCapability {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/App.ts:113 (sha256:db53bc5397bab3b3419f7a3cc53dc30caa9ee30e0f0c405a30236590f3eab022)
#[derive(Clone)]
pub struct HostAppReadyCapability {
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
impl PartialEq for HostAppReadyCapability {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/App.ts:117 (sha256:73e415c89632dad51db9e26eef00b9c776f0f6bf3e50171f6ea1986dc97f8d52)
#[derive(Clone)]
pub struct HostAppRecentDocumentsCapability {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub add_recent_document:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(String) -> () + Send + 'static>>>,
    pub clear_recent_documents:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>,
}
impl PartialEq for HostAppRecentDocumentsCapability {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/App.ts:122 (sha256:4c7ea709ecd7f6447501515679bf187402604c6baf25022d1f7d07943b7d7f7d)
#[derive(Clone)]
pub struct HostAppRelaunchCapability {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub relaunch: std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>,
}
impl PartialEq for HostAppRelaunchCapability {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/App.ts:126 (sha256:9f92bfd10c88d936db495a0cf133c4d5f883b26967684632564859f8aa3d1d78)
#[derive(Clone)]
pub struct HostAppSecondInstanceCapability {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub subscribe: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(
                        std::sync::Arc<
                            std::sync::Mutex<Box<dyn FnMut(Vec<String>) -> () + Send + 'static>>,
                        >,
                    ) -> std::sync::Arc<
                        std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>,
                    > + Send
                    + 'static,
            >,
        >,
    >,
}
impl PartialEq for HostAppSecondInstanceCapability {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/App.ts:130 (sha256:6950e13aa51ee5498842ccb19f68501b26a2443a2bc4357cf6c70b5a728891fb)
#[derive(Clone)]
pub struct HostAppSingleInstanceCapability {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub has_single_instance_lock:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> bool + Send + 'static>>>,
    pub release_single_instance_lock:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>,
    pub request_single_instance_lock:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> bool + Send + 'static>>>,
}
impl PartialEq for HostAppSingleInstanceCapability {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/App.ts:136 (sha256:4f3a5ef0980a2c875dabc1f230ce3112e54ec38b630592d2a9fcbbe0448d701d)
#[derive(Clone)]
pub struct HostAppUserModelIdCapability {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub set_user_model_id:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(String) -> () + Send + 'static>>>,
}
impl PartialEq for HostAppUserModelIdCapability {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/App.ts:140 (sha256:1ccb0950c0814707099c4448f0535c4857fb15b69c4d7488b65005e4cb54b628)
#[derive(Clone)]
pub struct HostAppVersionCapability {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub get_version: std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> String + Send + 'static>>>,
}
impl PartialEq for HostAppVersionCapability {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/App.ts:144 (sha256:3f8afd9f18906ccfb7d95cad9b1b92d5cc729896072c9c44976e539ca449186d)
#[derive(Clone)]
pub struct HostAppHideCapability {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub hide_app: std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>,
}
impl PartialEq for HostAppHideCapability {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/App.ts:148 (sha256:bc7b021f934d2329dfd12d7f93a70006f160dc1263a1767054b63f3ed21f402d)
#[derive(Clone)]
pub struct HostAppShowCapability {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub show_app: std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>,
}
impl PartialEq for HostAppShowCapability {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
