// @generated from upstream/packages/types/src/ShareFile.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/ShareFile.ts:4 (sha256:a711f287d7c262f8edf953607a9ea92f281847d56d31dea115091e548556bb89)
#[derive(Clone)]
pub struct ShareFile {
    pub name: String,
    pub mime_type: String,
    pub data_url: String,
}
