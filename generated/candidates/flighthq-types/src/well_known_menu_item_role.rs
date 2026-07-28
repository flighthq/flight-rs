// @generated from upstream/packages/types/src/WellKnownMenuItemRole.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/WellKnownMenuItemRole.ts:18 (sha256:81e36f65ff4a6a2f8c06a20ee3278e2aa35915e3ca5a4657532097c40d8c12a7)
#[derive(Clone, Default)]
pub struct WellKnownMenuItemRole {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub copy: String,
    pub cut: String,
    pub delete: String,
    pub paste: String,
    pub paste_and_match_style: String,
    pub redo: String,
    pub select_all: String,
    pub toggle_spell_checker: String,
    pub undo: String,
    pub about: String,
    pub close: String,
    pub front: String,
    pub hide: String,
    pub hide_others: String,
    pub merge_all_windows: String,
    pub minimize: String,
    pub move_tab_to_new_window: String,
    pub quit: String,
    pub select_next_tab: String,
    pub select_previous_tab: String,
    pub toggle_tab_bar: String,
    pub unhide: String,
    pub zoom: String,
    pub force_reload: String,
    pub reload: String,
    pub reset_zoom: String,
    pub toggle_dev_tools: String,
    pub toggle_fullscreen: String,
    pub zoom_in: String,
    pub zoom_out: String,
    pub help: String,
    pub services: String,
    pub start_speaking: String,
    pub stop_speaking: String,
    pub clear_recent_documents: String,
    pub recent_documents: String,
    pub app_menu: String,
    pub edit_menu: String,
    pub file_menu: String,
    pub help_menu: String,
    pub share_menu: String,
    pub view_menu: String,
    pub window_menu: String,
}
impl PartialEq for WellKnownMenuItemRole {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub static WELL_KNOWN_MENU_ITEM_ROLE: std::sync::LazyLock<WellKnownMenuItemRole> =
    std::sync::LazyLock::new(|| WellKnownMenuItemRole {
        __flight_identity: std::sync::Arc::new(()),
        copy: "copy".to_owned(),
        cut: "cut".to_owned(),
        delete: "delete".to_owned(),
        paste: "paste".to_owned(),
        paste_and_match_style: "pasteAndMatchStyle".to_owned(),
        redo: "redo".to_owned(),
        select_all: "selectAll".to_owned(),
        toggle_spell_checker: "toggleSpellChecker".to_owned(),
        undo: "undo".to_owned(),
        about: "about".to_owned(),
        close: "close".to_owned(),
        front: "front".to_owned(),
        hide: "hide".to_owned(),
        hide_others: "hideOthers".to_owned(),
        merge_all_windows: "mergeAllWindows".to_owned(),
        minimize: "minimize".to_owned(),
        move_tab_to_new_window: "moveTabToNewWindow".to_owned(),
        quit: "quit".to_owned(),
        select_next_tab: "selectNextTab".to_owned(),
        select_previous_tab: "selectPreviousTab".to_owned(),
        toggle_tab_bar: "toggleTabBar".to_owned(),
        unhide: "unhide".to_owned(),
        zoom: "zoom".to_owned(),
        force_reload: "forceReload".to_owned(),
        reload: "reload".to_owned(),
        reset_zoom: "resetZoom".to_owned(),
        toggle_dev_tools: "toggleDevTools".to_owned(),
        toggle_fullscreen: "toggleFullscreen".to_owned(),
        zoom_in: "zoomIn".to_owned(),
        zoom_out: "zoomOut".to_owned(),
        help: "help".to_owned(),
        services: "services".to_owned(),
        start_speaking: "startSpeaking".to_owned(),
        stop_speaking: "stopSpeaking".to_owned(),
        clear_recent_documents: "clearRecentDocuments".to_owned(),
        recent_documents: "recentDocuments".to_owned(),
        app_menu: "appMenu".to_owned(),
        edit_menu: "editMenu".to_owned(),
        file_menu: "fileMenu".to_owned(),
        help_menu: "helpMenu".to_owned(),
        share_menu: "shareMenu".to_owned(),
        view_menu: "viewMenu".to_owned(),
        window_menu: "windowMenu".to_owned(),
    });

// Source: upstream/packages/types/src/WellKnownMenuItemRole.ts:70 (sha256:f4f892ec5dc1d27505969d4667c4839c5b068f9f6aab8ea5304b585cfc886e34)
pub type WellKnownMenuItemRoleValue = String;
