// @generated from upstream/packages/shell/src/shell.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::{
    HostShellBeepCapability, HostShellExternalCapability, HostShellPathOpenCapability,
    HostShellPathRevealCapability, HostShellProcessCapability, HostShellShortcutLinkCapability,
    HostShellTrashCapability, ShellExternalOutcome, ShellExternalUrlPolicy, ShellPathOpenOutcome,
    ShellPathRevealOutcome, ShellProcess, ShellProcessOptions, ShellShortcutLink,
    ShellShortcutLinkReadOutcome, ShellShortcutLinkWriteOutcome, ShellShortcutWriteOperation,
    ShellTrashOutcome,
};

// Source: upstream/packages/shell/src/shell.ts:24 (sha256:330d859119e47f97aa0a3e8aab256efb6d86126b933587e608255735d01d688c)
pub fn is_shell_url_allowed(url: String, policy: &ShellExternalUrlPolicy) -> bool {
    let __flight_try_return: Option<bool> =
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| -> Option<bool> {
            {
                let scheme = (crate::host_value::<()>("host.replace").to_lower_case)();
                return Some(
                    ((policy.allowed_schemes).clone())
                        .iter()
                        .cloned()
                        .any(|allowed: String| -> bool { ((allowed).to_lowercase() == scheme) }),
                );
            }
            None
        })) {
            Ok(value) => value,
            Err(_) => (|| -> Option<bool> {
                {
                    return Some(false);
                }
                None
            })(),
        };
    return __flight_try_return.expect("TypeScript try/catch completed without returning");
}

// Source: upstream/packages/shell/src/shell.ts:35 (sha256:28a4c01a5f6a77d4ecc0950055e6c52f2c4fb78a7726b85fa6fa8936462fe134)
pub fn move_shell_items_to_trash(
    host_shell_trash: HostShellTrashCapability,
    paths: &Vec<String>,
) -> crate::FlightTask<Vec<ShellTrashOutcome>> {
    return crate::FlightTask::all(
        (paths)
            .iter()
            .cloned()
            .map(|path: String| -> crate::FlightTask<ShellTrashOutcome> {
                {
                    let __flight_callback = (host_shell_trash.move_to_trash).clone();
                    let __flight_result = __flight_callback.lock().unwrap()((path).clone());
                    __flight_result
                }
            })
            .collect::<Vec<_>>(),
        crate::FlightTaskOrigin {
            package: "@flighthq/shell",
            source: "upstream/packages/shell/src/shell.ts",
            line: 39_u32,
            column: 10_u32,
            lexical_path: "moveShellItemsToTrash.join-all:39:10:b5f44c658fea",
            fingerprint: "sha256:b5f44c658fea43205ab1e70bb8ae23ebe7ff64b86e6c8e46ac25e2d94d72115d",
        },
    );
}

// Source: upstream/packages/shell/src/shell.ts:42 (sha256:1de873385fc34bebc81da3e886aadee0969848915ab83a7edc44cbb43ad881ab)
pub fn move_shell_item_to_trash(
    host_shell_trash: &HostShellTrashCapability,
    path: String,
) -> crate::FlightTask<ShellTrashOutcome> {
    return {
        let __flight_callback = (host_shell_trash.move_to_trash).clone();
        let __flight_result = __flight_callback.lock().unwrap()((path).clone());
        __flight_result
    };
}

// Source: upstream/packages/shell/src/shell.ts:52 (sha256:d0995fc4cbfddbe01543716ea7a269cc598ea230c98d1b79a7126be779242a7c)
#[derive(Clone, Default)]
struct OpenShellExternalUrlRecord1 {
    __flight_identity: std::sync::Arc<()>,
    reason: String,
}
impl PartialEq for OpenShellExternalUrlRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn open_shell_external_url(
    host_shell_external: &HostShellExternalCapability,
    url: String,
    policy: &ShellExternalUrlPolicy,
) -> crate::FlightTask<ShellExternalOutcome> {
    if (!is_shell_url_allowed((url).clone(), policy)) {
        return crate::FlightTask::ready(
            ShellExternalOutcome {
                __flight_identity: std::sync::Arc::new(()),
                reason: "blocked-scheme".to_owned(),
            },
            crate::FlightTaskOrigin {
                package: "@flighthq/shell",
                source: "upstream/packages/shell/src/shell.ts",
                line: 57_u32,
                column: 47_u32,
                lexical_path: "openShellExternalUrl.ready:57:47:74a14dda2a1b",
                fingerprint: "sha256:74a14dda2a1bd621f7809eb07ce693f39f4bd468c9313c4604489e24fcddfc4c",
            },
        );
    }
    return {
        let __flight_callback = (host_shell_external.open).clone();
        let __flight_result = __flight_callback.lock().unwrap()((url).clone());
        __flight_result
    };
}

// Source: upstream/packages/shell/src/shell.ts:61 (sha256:5caea5d006269177d187552d7259eef1155aed19ba78356fb75ddb83b985f83e)
pub fn open_shell_path(
    host_shell_path_open: &HostShellPathOpenCapability,
    path: String,
) -> crate::FlightTask<ShellPathOpenOutcome> {
    return {
        let __flight_callback = (host_shell_path_open.open).clone();
        let __flight_result = __flight_callback.lock().unwrap()((path).clone());
        __flight_result
    };
}

// Source: upstream/packages/shell/src/shell.ts:68 (sha256:ebf5254a723c2210e02b5de7f82c98d45e3855710599fd7c4b6c22bd30600607)
pub fn read_shell_shortcut_link(
    host_shell_shortcut_link: &HostShellShortcutLinkCapability,
    shortcut_path: String,
) -> crate::FlightTask<ShellShortcutLinkReadOutcome> {
    return {
        let __flight_callback = (host_shell_shortcut_link.read).clone();
        let __flight_result = __flight_callback.lock().unwrap()((shortcut_path).clone());
        __flight_result
    };
}

// Source: upstream/packages/shell/src/shell.ts:75 (sha256:b94a676d98abeb3eb57a97d8930e6237daada3e20e04434410d7b6dc3174f77c)
pub fn reveal_shell_path(
    host_shell_path_reveal: &HostShellPathRevealCapability,
    path: String,
) -> crate::FlightTask<ShellPathRevealOutcome> {
    return {
        let __flight_callback = (host_shell_path_reveal.reveal).clone();
        let __flight_result = __flight_callback.lock().unwrap()((path).clone());
        __flight_result
    };
}

// Source: upstream/packages/shell/src/shell.ts:82 (sha256:a5d879c6cc074bf402934576b2b1764f0a40da9213803872035b1bad7fe8aa3a)
pub fn shell_beep(host_shell_beep: &HostShellBeepCapability) -> () {
    {
        let __flight_callback = (host_shell_beep.beep).clone();
        let __flight_result = __flight_callback.lock().unwrap()();
        __flight_result
    };
}

// Source: upstream/packages/shell/src/shell.ts:86 (sha256:85a571bd6a32b93e165503c4121da0fb20fa2e2e18e253d345e86126b5275764)
pub fn spawn_shell_process(
    host_shell_process: &HostShellProcessCapability,
    command: String,
    args: &Vec<String>,
    options: Option<ShellProcessOptions>,
) -> ShellProcess {
    return {
        let __flight_callback = (host_shell_process.spawn).clone();
        let __flight_result = __flight_callback.lock().unwrap()(
            (command).clone(),
            (*args).clone(),
            (options).clone(),
        );
        __flight_result
    };
}

// Source: upstream/packages/shell/src/shell.ts:95 (sha256:c89720609dfc8c3bb953197757626012a6d7c0763558f6cd1cb8d9be4c238adc)
pub fn write_shell_shortcut_link(
    host_shell_shortcut_link: &HostShellShortcutLinkCapability,
    shortcut_path: String,
    link: &ShellShortcutLink,
    operation: ShellShortcutWriteOperation,
) -> crate::FlightTask<ShellShortcutLinkWriteOutcome> {
    return {
        let __flight_callback = (host_shell_shortcut_link.write).clone();
        let __flight_result = __flight_callback.lock().unwrap()(
            (shortcut_path).clone(),
            (*link).clone(),
            (operation).clone(),
        );
        __flight_result
    };
}
