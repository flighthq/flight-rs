// @generated from upstream/packages/types/src/HostFileDialog.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{
    DirectoryOpenDialogResult, FileOpenDialogResult, FileSaveDialogResult,
    OpenDirectoryDialogOptions, OpenFileDialogOptions, SaveFileDialogOptions,
};

// Source: upstream/packages/types/src/HostFileDialog.ts:10 (sha256:63b2a409bf6cda73bd9dc36fe69c48e7b13f4afd6d068127a7dcf25f8e7e5a7e)
#[derive(Clone)]
pub struct HostDirectoryOpenDialogCapability {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub open: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(
                        Option<OpenDirectoryDialogOptions>,
                    ) -> crate::FlightTask<DirectoryOpenDialogResult>
                    + Send
                    + 'static,
            >,
        >,
    >,
}
impl PartialEq for HostDirectoryOpenDialogCapability {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/HostFileDialog.ts:14 (sha256:d395401315c4362a0d6e8ddc57dcf1d527a68fe7ad669535b63dae69c252a729)
#[derive(Clone)]
pub struct HostFileOpenDialogCapability {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub open: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(OpenFileDialogOptions) -> crate::FlightTask<FileOpenDialogResult>
                    + Send
                    + 'static,
            >,
        >,
    >,
}
impl PartialEq for HostFileOpenDialogCapability {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/HostFileDialog.ts:18 (sha256:54dbae9a61d51e6b02a25d9199d41513c573face50102e2af2960b2b1850f47b)
#[derive(Clone)]
pub struct HostFileSaveDialogCapability {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub save: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(SaveFileDialogOptions) -> crate::FlightTask<FileSaveDialogResult>
                    + Send
                    + 'static,
            >,
        >,
    >,
}
impl PartialEq for HostFileSaveDialogCapability {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
