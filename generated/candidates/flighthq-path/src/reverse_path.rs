// @generated from upstream/packages/path/src/reversePath.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::for_each_path_segment;
use flighthq_types::{Path, PathCommand, PathSegment};

// Source: upstream/packages/path/src/reversePath.ts:13 (sha256:c96831c847af173cd8f42edd6babd6801f3fa7c020bca46e4dcaaf4ad0204d68)
pub fn reverse_path(source: &Path, out: &mut Path) -> () {
    let subpaths = decode_subpaths(source);
    out.commands.clear();
    out.data.clear();
    out.winding = (source.winding).clone();
    for subpath in (subpaths).iter().cloned() {
        encode_reversed_subpath(&subpath, out);
    }
}

// Source: upstream/packages/path/src/reversePath.ts:23 (sha256:fa601de91b2925148316f3dac6183d041ec502f2e101ca35686f91bf2ec024b6)
#[derive(Clone, Default)]
struct DecodeSubpathsRecord1 {
    __flight_identity: std::sync::Arc<()>,
    points: Vec<DecodeSubpathsRecord2>,
    closed: bool,
}
impl PartialEq for DecodeSubpathsRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
struct DecodeSubpathsRecord2 {
    __flight_identity: std::sync::Arc<()>,
    x: f64,
    y: f64,
    kind: String,
}
impl PartialEq for DecodeSubpathsRecord2 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

fn decode_subpaths(path: &Path) -> Vec<Subpath> {
    let subpaths: std::sync::Arc<std::sync::Mutex<Vec<Subpath>>> =
        std::sync::Arc::new(std::sync::Mutex::new(vec![]));
    let current: std::sync::Arc<std::sync::Mutex<Option<Subpath>>> =
        std::sync::Arc::new(std::sync::Mutex::new(None));
    let ensure_current: std::sync::Arc<
        std::sync::Mutex<
            std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> Subpath + Send + 'static>>>,
        >,
    > = std::sync::Arc::new(std::sync::Mutex::new(std::sync::Arc::new(
        std::sync::Mutex::new(Box::new({
            let mut current = current.clone();
            let mut subpaths = subpaths.clone();
            move || -> Subpath {
                if ((*current.lock().unwrap()).clone()).is_none() {
                    (*current.lock().unwrap()) = Some(Subpath {
                        __flight_identity: std::sync::Arc::new(()),
                        points: vec![SubpathPoint {
                            __flight_identity: std::sync::Arc::new(()),
                            x: 0.0_f64,
                            y: 0.0_f64,
                            kind: "move".to_owned(),
                            cx: None,
                            cy: None,
                            c1x: None,
                            c1y: None,
                            c2x: None,
                            c2y: None,
                        }],
                        closed: false,
                    });
                    (*subpaths.lock().unwrap())
                        .push((((*current.lock().unwrap()).clone()).clone().unwrap()).clone());
                }
                return (((*current.lock().unwrap()).clone()).clone().unwrap()).clone();
            }
        }) as Box<dyn FnMut() -> Subpath + Send + 'static>),
    )));
    for_each_path_segment(path, &mut |segment: PathSegment| -> () {
        if ((segment.kind).clone() == "moveTo") {
            (*current.lock().unwrap()) = Some(Subpath {
                __flight_identity: std::sync::Arc::new(()),
                points: vec![SubpathPoint {
                    __flight_identity: std::sync::Arc::new(()),
                    x: (segment.x).unwrap(),
                    y: (segment.y).unwrap(),
                    kind: "move".to_owned(),
                    cx: None,
                    cy: None,
                    c1x: None,
                    c1y: None,
                    c2x: None,
                    c2y: None,
                }],
                closed: false,
            });
            (*subpaths.lock().unwrap())
                .push((((*current.lock().unwrap()).clone()).clone().unwrap()).clone());
        } else {
            if ((segment.kind).clone() == "lineTo") {
                {
                    let __flight_callback = (*ensure_current.lock().unwrap()).clone();
                    let __flight_result = __flight_callback.lock().unwrap()();
                    __flight_result
                }
                .points
                .push(SubpathPoint {
                    __flight_identity: std::sync::Arc::new(()),
                    x: (segment.x).unwrap(),
                    y: (segment.y).unwrap(),
                    kind: "line".to_owned(),
                    cx: None,
                    cy: None,
                    c1x: None,
                    c1y: None,
                    c2x: None,
                    c2y: None,
                });
            } else {
                if ((segment.kind).clone() == "curveTo") {
                    {
                        let __flight_callback = (*ensure_current.lock().unwrap()).clone();
                        let __flight_result = __flight_callback.lock().unwrap()();
                        __flight_result
                    }
                    .points
                    .push(SubpathPoint {
                        __flight_identity: std::sync::Arc::new(()),
                        x: (segment.x).unwrap(),
                        y: (segment.y).unwrap(),
                        kind: "quad".to_owned(),
                        cx: Some((segment.control_x).unwrap()),
                        cy: Some((segment.control_y).unwrap()),
                        c1x: None,
                        c1y: None,
                        c2x: None,
                        c2y: None,
                    });
                } else {
                    if ((segment.kind).clone() == "cubicCurveTo") {
                        {
                            let __flight_callback = (*ensure_current.lock().unwrap()).clone();
                            let __flight_result = __flight_callback.lock().unwrap()();
                            __flight_result
                        }
                        .points
                        .push(SubpathPoint {
                            __flight_identity: std::sync::Arc::new(()),
                            x: (segment.x).unwrap(),
                            y: (segment.y).unwrap(),
                            kind: "cubic".to_owned(),
                            c1x: Some((segment.control1_x).unwrap()),
                            c1y: Some((segment.control1_y).unwrap()),
                            c2x: Some((segment.control2_x).unwrap()),
                            c2y: Some((segment.control2_y).unwrap()),
                            cx: None,
                            cy: None,
                        });
                    } else {
                        if ((segment.kind).clone() == "close") {
                            if ((*current.lock().unwrap()).clone()).is_some() {
                                (*current.lock().unwrap()).as_mut().unwrap().closed = true;
                            }
                        }
                    }
                }
            }
        }
    });
    return (*subpaths.lock().unwrap()).clone();
}

// Source: upstream/packages/path/src/reversePath.ts:64 (sha256:a7282389073d11dba3b2017b91aeddedaa4fc631b240ee48c5ce5a983358ae7a)
fn encode_reversed_subpath(subpath: &Subpath, out: &mut Path) -> () {
    if ((subpath.points.len() as f64) == 0.0_f64) {
        return;
    }
    let last = subpath.points[((subpath.points.len() as f64) - 1.0_f64) as usize].clone();
    out.commands.push(PathCommand::MOVE_TO);
    out.data.extend(vec![last.x, last.y]);
    {
        let mut i = ((subpath.points.len() as f64) - 1.0_f64);
        while (i >= 1.0_f64) {
            let from = subpath.points[i as usize].clone();
            let to = subpath.points[(i - 1.0_f64) as usize].clone();
            if ((from.kind).clone() == "line") || ((from.kind).clone() == "move") {
                out.commands.push(PathCommand::LINE_TO);
                out.data.extend(vec![to.x, to.y]);
            } else {
                if ((from.kind).clone() == "quad") {
                    out.commands.push(PathCommand::CURVE_TO);
                    out.data
                        .extend(vec![(from.cx).unwrap(), (from.cy).unwrap(), to.x, to.y]);
                } else {
                    if ((from.kind).clone() == "cubic") {
                        out.commands.push(PathCommand::CUBIC_CURVE_TO);
                        out.data.extend(vec![
                            (from.c2x).unwrap(),
                            (from.c2y).unwrap(),
                            (from.c1x).unwrap(),
                            (from.c1y).unwrap(),
                            to.x,
                            to.y,
                        ]);
                    }
                }
            }
            {
                i -= 1.0;
                i
            };
        }
    }
    if subpath.closed {
        out.commands.push(PathCommand::CLOSE);
    }
}

// Source: upstream/packages/path/src/reversePath.ts:93 (sha256:fe5d45a0ab024b667e20ec67427ec99ed04779d3a6abaa0d84db92db0553e28a)
#[derive(Clone, Default)]
struct Subpath {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub closed: bool,
    pub points: Vec<SubpathPoint>,
}
impl PartialEq for Subpath {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/path/src/reversePath.ts:98 (sha256:87f9d701440ffe67edb4a00fd36f932e5c0a4241738e1f26384e0b1c335ff7c4)
#[derive(Clone, Default)]
struct SubpathPoint {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub kind: String,
    pub x: f64,
    pub y: f64,
    pub cx: Option<f64>,
    pub cy: Option<f64>,
    pub c1x: Option<f64>,
    pub c1y: Option<f64>,
    pub c2x: Option<f64>,
    pub c2y: Option<f64>,
}
impl PartialEq for SubpathPoint {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
