// @generated from upstream/packages/path/src/copyPath.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::Path;

// Source: upstream/packages/path/src/copyPath.ts:5 (sha256:8ce8bbbb3d3e013cc970db851c6ab4063c7d18ccb7250aafb9c885b9be4c7d48)
pub fn clone_path(source: &Path) -> Path {
    return copy_path(source, None);
}

// Source: upstream/packages/path/src/copyPath.ts:12 (sha256:aa7a862288b323c73936844ffe1196ad18c7ce94f0475c5af7ba1a43289bea20)
pub fn copy_path(source: &Path, mut out: Option<Path>) -> Path {
    if (out).is_none() {
        return Path {
            __flight_identity: std::sync::Arc::new(()),
            commands: ((source.commands).clone()).clone(),
            data: ((source.data).clone()).clone(),
            winding: (source.winding).clone(),
        };
    }
    if (out.as_mut().unwrap() != source) {
        out.as_mut().unwrap().commands.clear();
        {
            let mut i = 0.0_f64;
            while (i < (source.commands.len() as f64)) {
                out.as_mut()
                    .unwrap()
                    .commands
                    .push(source.commands[i as usize].clone());
                {
                    i += 1.0;
                    i
                };
            }
        }
        out.as_mut().unwrap().data.clear();
        {
            let mut i = 0.0_f64;
            while (i < (source.data.len() as f64)) {
                out.as_mut()
                    .unwrap()
                    .data
                    .push(source.data[i as usize].clone());
                {
                    i += 1.0;
                    i
                };
            }
        }
        out.as_mut().unwrap().winding = (source.winding).clone();
    }
    return ((out.as_mut().unwrap()).clone()).clone();
}
