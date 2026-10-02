// @generated from upstream/packages/share/src/share.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_entity::{allocate_entity, finish_entity};
use flighthq_signals::{clear_signal, create_signal, emit_signal};
use flighthq_types::{
    EntityConstruction, HostShareContentCapability, HostShareFilesCapability, ShareContent,
    ShareFile, ShareFilesContent, ShareResult, ShareSignals,
};

// Source: upstream/packages/share/src/share.ts:14 (sha256:9e4e07e0391e6a393e52771d34a1d2f69980c4d5a6380f37f6bef5c8e3372e88)
pub fn attach_share_signals(signals: &ShareSignals) -> () {
    {
        let __flight_value = (*signals).clone();
        if !(*_ATTACHED_SIGNALS.lock().unwrap()).contains(&__flight_value) {
            (*_ATTACHED_SIGNALS.lock().unwrap()).push(__flight_value);
        }
    };
}

// Source: upstream/packages/share/src/share.ts:21 (sha256:a228983948571668a135db0148faa78cffc34e4bd9d24d343987d282fe3cf021)
pub fn can_share_content(
    host_share_content: &HostShareContentCapability,
    content: &ShareContent,
) -> bool {
    return (has_share_content_fields(content))
        && ({
            let __flight_callback = (host_share_content.can_share_content).clone();
            let __flight_result = __flight_callback.lock().unwrap()((*content).clone());
            __flight_result
        });
}

// Source: upstream/packages/share/src/share.ts:28 (sha256:c8eb8603f43faace504ba2b0887e4f0852a8004a6699a17536a246e80a029c47)
pub fn can_share_files(
    host_share_files: &HostShareFilesCapability,
    files: &Vec<ShareFile>,
) -> bool {
    let content = files_content(files);
    return ((content).is_some())
        && ({
            let __flight_callback = (host_share_files.can_share_content).clone();
            let __flight_result =
                __flight_callback.lock().unwrap()((content.as_ref().unwrap()).clone());
            __flight_result
        });
}

// Source: upstream/packages/share/src/share.ts:36 (sha256:8ada4c658c6274d239a5d3955b5696b80e04a5aaa33568a636d0645eed894048)
pub fn detach_share_signals(signals: &ShareSignals) -> () {
    {
        let __flight_value = (*signals).clone();
        if let Some(__flight_index) = (*_ATTACHED_SIGNALS.lock().unwrap())
            .iter()
            .position(|item| item == &__flight_value)
        {
            (*_ATTACHED_SIGNALS.lock().unwrap()).remove(__flight_index);
            true
        } else {
            false
        }
    };
}

// Source: upstream/packages/share/src/share.ts:40 (sha256:b91510a85374edd3107971dbc03f2d697265133590b4b21246cc9787842d85e5)
pub fn dispose_share_signals(signals: &mut ShareSignals) -> () {
    detach_share_signals(signals);
    clear_signal(&mut signals.on_share_result);
}

// Source: upstream/packages/share/src/share.ts:45 (sha256:a25e0b94322f0a24141a142f353e457c0b1b8caf312110dd325b73b3f95acd8e)
pub fn enable_share_signals() -> ShareSignals {
    let mut out = allocate_entity();
    initialize_share_signals((out).clone());
    return finish_entity((out).clone());
}

// Source: upstream/packages/share/src/share.ts:51 (sha256:c7359d8fe5515cf276222dad92c9570116ecbc65df03d9537742c43d856aae8d)
pub fn has_share_content_fields(content: &ShareContent) -> bool {
    if (((content.title).clone()).is_some())
        && (!(((content.title).clone()) == Some("".to_owned())))
    {
        return true;
    }
    if (((content.text).clone()).is_some()) && (!(((content.text).clone()) == Some("".to_owned())))
    {
        return true;
    }
    if (((content.url).clone()).is_some()) && (!(((content.url).clone()) == Some("".to_owned()))) {
        return true;
    }
    return false;
}

// Source: upstream/packages/share/src/share.ts:58 (sha256:5f2219a9647c3e9a382202c9137dd6169e200e04cd7e329f97e025d1f808c196)
pub fn initialize_share_signals(out: EntityConstruction<ShareSignals>) -> () {
    crate::host_set("host.onShareResult", create_signal());
}

// Source: upstream/packages/share/src/share.ts:65 (sha256:e7dce1d69594ea9c18ed5e700c4686d5c96bb02b1700bf97449cdc4b8134a62f)
pub fn is_share_file_valid(file: &ShareFile) -> bool {
    return ((((file.name).clone() != "") && ((file.mime_type).clone() != ""))
        && (((file.data_url).clone()).starts_with(("data:".to_owned()).as_str())))
        && (((file.data_url).clone()).contains((",".to_owned()).as_str()));
}

// Source: upstream/packages/share/src/share.ts:69 (sha256:45931fd20de574694ea8f4645e0c860f300b8127e84e61e565554d3f33c7466a)
pub fn share_content(
    host_share_content: &HostShareContentCapability,
    content: &ShareContent,
) -> crate::FlightTask<bool> {
    if (!has_share_content_fields(content)) {
        return crate::FlightTask::ready(
            false,
            crate::FlightTaskOrigin {
                package: "@flighthq/share",
                source: "upstream/packages/share/src/share.ts",
                line: 73_u32,
                column: 47_u32,
                lexical_path: "shareContent.ready:73:47:a913f3cb1f97",
                fingerprint: "sha256:a913f3cb1f9734d159588904231d21b38dd02a4b990de3f76aaea042e535d166",
            },
        );
    }
    return {
        let __flight_callback = (host_share_content.share_content).clone();
        let __flight_result = __flight_callback.lock().unwrap()((*content).clone());
        __flight_result
    };
}

// Source: upstream/packages/share/src/share.ts:77 (sha256:3e471eb40e9c9bccd59569969daa597afc9bf1b347229d0065d7ca8367712781)
pub fn share_content_with_result(
    host_share_content: HostShareContentCapability,
    content: ShareContent,
) -> crate::FlightTask<ShareResult> {
    crate::FlightTask::start(
        async move {
            if (!has_share_content_fields(&content)) {
                return Ok(ShareResult {
                    __flight_identity: std::sync::Arc::new(()),
                    completed: false,
                    activity_type: None,
                    dismissed: false,
                });
            }
            let result = ({
                let __flight_callback = (host_share_content.share_content_with_result).clone();
                let __flight_result = __flight_callback.lock().unwrap()((content).clone());
                __flight_result
            })
            .await?;
            for signals in ((*_ATTACHED_SIGNALS.lock().unwrap()).clone())
                .iter()
                .cloned()
            {
                emit_signal((signals.on_share_result).clone(), ((result).clone(),));
            }
            return Ok((result).clone());
        },
        crate::FlightTaskOrigin {
            package: "@flighthq/share",
            source: "upstream/packages/share/src/share.ts",
            line: 77_u32,
            column: 1_u32,
            lexical_path: "shareContentWithResult",
            fingerprint: "sha256:3e471eb40e9c9bccd59569969daa597afc9bf1b347229d0065d7ca8367712781",
        },
    )
}

// Source: upstream/packages/share/src/share.ts:91 (sha256:a2f09225f363a70e8eaca3350a52e10c2914970d24953d6c27e156c671d99521)
pub fn share_files(
    host_share_files: &HostShareFilesCapability,
    files: &Vec<ShareFile>,
) -> crate::FlightTask<bool> {
    let content = files_content(files);
    if (content).is_none() {
        return crate::FlightTask::ready(
            false,
            crate::FlightTaskOrigin {
                package: "@flighthq/share",
                source: "upstream/packages/share/src/share.ts",
                line: 96_u32,
                column: 32_u32,
                lexical_path: "shareFiles.ready:96:32:a913f3cb1f97",
                fingerprint: "sha256:a913f3cb1f9734d159588904231d21b38dd02a4b990de3f76aaea042e535d166",
            },
        );
    }
    return {
        let __flight_callback = (host_share_files.share_content).clone();
        let __flight_result =
            __flight_callback.lock().unwrap()((content.as_ref().unwrap()).clone());
        __flight_result
    };
}

// Source: upstream/packages/share/src/share.ts:100 (sha256:ec2026d3c2abae9fc7b457df3d46cac06e5d23d3b3c13fee84f0e94fc695fc32)
pub fn share_text(
    host_share_content: &HostShareContentCapability,
    text: String,
) -> crate::FlightTask<bool> {
    return share_content(
        host_share_content,
        &ShareContent {
            __flight_identity: std::sync::Arc::new(()),
            text: Some((text).clone()),
            title: None,
            url: None,
        },
    );
}

// Source: upstream/packages/share/src/share.ts:104 (sha256:f115ef14a47aa4275b4f9fe7f5b5c8a053cd48c9e543080eff45808a537ff223)
pub fn share_url(
    host_share_content: &HostShareContentCapability,
    url: String,
) -> crate::FlightTask<bool> {
    return share_content(
        host_share_content,
        &ShareContent {
            __flight_identity: std::sync::Arc::new(()),
            url: Some((url).clone()),
            text: None,
            title: None,
        },
    );
}

// Source: upstream/packages/share/src/share.ts:108 (sha256:af560526000036d61afa1845821b4d810a4f30dca032aa38f8e3cb6f5536c424)
static _ATTACHED_SIGNALS: std::sync::LazyLock<std::sync::Mutex<Vec<ShareSignals>>> =
    std::sync::LazyLock::new(|| std::sync::Mutex::new(Vec::new()));

// Source: upstream/packages/share/src/share.ts:110 (sha256:bf22308cb8b58b42deec69fc5222d4e7f37dedbabb827126495d1375ebb0b269)
fn files_content(files: &Vec<ShareFile>) -> Option<ShareFilesContent> {
    let first: Option<ShareFile> = files.get(0.0_f64 as usize).cloned();
    return if ((first).is_none())
        || (!(files)
            .iter()
            .cloned()
            .all(|__flight_item| is_share_file_valid(&__flight_item)))
    {
        None
    } else {
        Some(ShareFilesContent {
            __flight_identity: std::sync::Arc::new(()),
            files: {
                let mut __flight_array = Vec::new();
                __flight_array.push((first).clone().unwrap());
                __flight_array.extend(
                    ((files)[(1.0_f64) as usize..((files).len() as f64) as usize].to_vec())
                        .iter()
                        .cloned(),
                );
                __flight_array
            },
            text: None,
            title: None,
            url: None,
        })
    };
}
