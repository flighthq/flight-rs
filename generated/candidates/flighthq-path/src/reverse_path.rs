// @generated from upstream/packages/path/src/reversePath.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::for_each_path_segment;
use flighthq_types::{Path, PathCommand, PathSegment};

#[derive(Clone, Default)]
pub struct SubpathPointRecord1 {
    pub __flight_identity: std::sync::Arc<()>,
    pub kind: String,
    pub x: f64,
    pub y: f64,
    pub c1x: f64,
    pub c1y: f64,
    pub c2x: f64,
    pub c2y: f64,
}
impl PartialEq for SubpathPointRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct SubpathPointRecord2 {
    pub __flight_identity: std::sync::Arc<()>,
    pub kind: String,
    pub x: f64,
    pub y: f64,
    pub cx: f64,
    pub cy: f64,
}
impl PartialEq for SubpathPointRecord2 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct SubpathPointRecord3 {
    pub __flight_identity: std::sync::Arc<()>,
    pub kind: String,
    pub x: f64,
    pub y: f64,
}
impl PartialEq for SubpathPointRecord3 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct SubpathPointRecord4 {
    pub __flight_identity: std::sync::Arc<()>,
    pub kind: String,
    pub x: f64,
    pub y: f64,
}
impl PartialEq for SubpathPointRecord4 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

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
struct DecodeSubpathsRecord13 {
    __flight_identity: std::sync::Arc<()>,
    points: Vec<DecodeSubpathsRecord14>,
    closed: bool,
}
impl PartialEq for DecodeSubpathsRecord13 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
struct DecodeSubpathsRecord14 {
    __flight_identity: std::sync::Arc<()>,
    x: f64,
    y: f64,
    kind: String,
}
impl PartialEq for DecodeSubpathsRecord14 {
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
                        points: vec![SubpathPoint::A(SubpathPointRecord4 {
                            __flight_identity: std::sync::Arc::new(()),
                            x: 0.0_f64,
                            y: 0.0_f64,
                            kind: "move".to_owned(),
                        })],
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
        if matches!(&(segment), flighthq_types::PathSegment::A(_)) {
            (*current.lock().unwrap()) = Some(Subpath {
                __flight_identity: std::sync::Arc::new(()),
                points: vec![SubpathPoint::A(SubpathPointRecord4 {
                    __flight_identity: std::sync::Arc::new(()),
                    x: (match (segment).clone() {
                        flighthq_types::PathSegment::A(value) => value,
                        flighthq_types::PathSegment::B(_) => {
                            panic!("TypeScript union narrowing failed")
                        }
                    })
                    .x,
                    y: (match (segment).clone() {
                        flighthq_types::PathSegment::A(value) => value,
                        flighthq_types::PathSegment::B(_) => {
                            panic!("TypeScript union narrowing failed")
                        }
                    })
                    .y,
                    kind: "move".to_owned(),
                })],
                closed: false,
            });
            (*subpaths.lock().unwrap())
                .push((((*current.lock().unwrap()).clone()).clone().unwrap()).clone());
        } else {
            if matches!(
                &(segment),
                crate::FlightUnion2::B(crate::FlightUnion2::A(_))
            ) {
                {
                    let __flight_callback = (*ensure_current.lock().unwrap()).clone();
                    let __flight_result = __flight_callback.lock().unwrap()();
                    __flight_result
                }
                .points
                .push(SubpathPoint::B(crate::FlightUnion2::<
                    SubpathPointRecord3,
                    crate::FlightUnion2<SubpathPointRecord2, SubpathPointRecord1>,
                >::A(SubpathPointRecord3 {
                    __flight_identity: std::sync::Arc::new(()),
                    x: (match (segment).clone() {
                        flighthq_types::PathSegment::A(_) => {
                            panic!("TypeScript union narrowing failed")
                        }
                        flighthq_types::PathSegment::B(value) => match value {
                            crate::FlightUnion2::A(value) => value,
                            crate::FlightUnion2::B(_) => {
                                panic!("TypeScript union narrowing failed")
                            }
                        },
                    })
                    .x,
                    y: (match (segment).clone() {
                        flighthq_types::PathSegment::A(_) => {
                            panic!("TypeScript union narrowing failed")
                        }
                        flighthq_types::PathSegment::B(value) => match value {
                            crate::FlightUnion2::A(value) => value,
                            crate::FlightUnion2::B(_) => {
                                panic!("TypeScript union narrowing failed")
                            }
                        },
                    })
                    .y,
                    kind: "line".to_owned(),
                })));
            } else {
                if matches!(
                    &(segment),
                    crate::FlightUnion2::B(crate::FlightUnion2::B(crate::FlightUnion2::A(_)))
                ) {
                    {
                        let __flight_callback = (*ensure_current.lock().unwrap()).clone();
                        let __flight_result = __flight_callback.lock().unwrap()();
                        __flight_result
                    }
                    .points
                    .push(SubpathPoint::B(crate::FlightUnion2::<
                        SubpathPointRecord3,
                        crate::FlightUnion2<SubpathPointRecord2, SubpathPointRecord1>,
                    >::B(
                        crate::FlightUnion2::<SubpathPointRecord2, SubpathPointRecord1>::A(
                            SubpathPointRecord2 {
                                __flight_identity: std::sync::Arc::new(()),
                                x: (match (segment).clone() {
                                    flighthq_types::PathSegment::A(_) => {
                                        panic!("TypeScript union narrowing failed")
                                    }
                                    flighthq_types::PathSegment::B(value) => match value {
                                        crate::FlightUnion2::A(_) => {
                                            panic!("TypeScript union narrowing failed")
                                        }
                                        crate::FlightUnion2::B(value) => match value {
                                            crate::FlightUnion2::A(value) => value,
                                            crate::FlightUnion2::B(_) => {
                                                panic!("TypeScript union narrowing failed")
                                            }
                                        },
                                    },
                                })
                                .x,
                                y: (match (segment).clone() {
                                    flighthq_types::PathSegment::A(_) => {
                                        panic!("TypeScript union narrowing failed")
                                    }
                                    flighthq_types::PathSegment::B(value) => match value {
                                        crate::FlightUnion2::A(_) => {
                                            panic!("TypeScript union narrowing failed")
                                        }
                                        crate::FlightUnion2::B(value) => match value {
                                            crate::FlightUnion2::A(value) => value,
                                            crate::FlightUnion2::B(_) => {
                                                panic!("TypeScript union narrowing failed")
                                            }
                                        },
                                    },
                                })
                                .y,
                                kind: "quad".to_owned(),
                                cx: (match (segment).clone() {
                                    flighthq_types::PathSegment::A(_) => {
                                        panic!("TypeScript union narrowing failed")
                                    }
                                    flighthq_types::PathSegment::B(value) => match value {
                                        crate::FlightUnion2::A(_) => {
                                            panic!("TypeScript union narrowing failed")
                                        }
                                        crate::FlightUnion2::B(value) => match value {
                                            crate::FlightUnion2::A(value) => value,
                                            crate::FlightUnion2::B(_) => {
                                                panic!("TypeScript union narrowing failed")
                                            }
                                        },
                                    },
                                })
                                .control_x,
                                cy: (match (segment).clone() {
                                    flighthq_types::PathSegment::A(_) => {
                                        panic!("TypeScript union narrowing failed")
                                    }
                                    flighthq_types::PathSegment::B(value) => match value {
                                        crate::FlightUnion2::A(_) => {
                                            panic!("TypeScript union narrowing failed")
                                        }
                                        crate::FlightUnion2::B(value) => match value {
                                            crate::FlightUnion2::A(value) => value,
                                            crate::FlightUnion2::B(_) => {
                                                panic!("TypeScript union narrowing failed")
                                            }
                                        },
                                    },
                                })
                                .control_y,
                            },
                        ),
                    )));
                } else {
                    if matches!(
                        &(segment),
                        crate::FlightUnion2::B(crate::FlightUnion2::B(crate::FlightUnion2::B(
                            crate::FlightUnion2::A(_)
                        )))
                    ) {
                        {
                            let __flight_callback = (*ensure_current.lock().unwrap()).clone();
                            let __flight_result = __flight_callback.lock().unwrap()();
                            __flight_result
                        }
                        .points
                        .push(SubpathPoint::B(crate::FlightUnion2::<
                            SubpathPointRecord3,
                            crate::FlightUnion2<SubpathPointRecord2, SubpathPointRecord1>,
                        >::B(
                            crate::FlightUnion2::<SubpathPointRecord2, SubpathPointRecord1>::B(
                                SubpathPointRecord1 {
                                    __flight_identity: std::sync::Arc::new(()),
                                    x: (match (segment).clone() {
                                        flighthq_types::PathSegment::A(_) => {
                                            panic!("TypeScript union narrowing failed")
                                        }
                                        flighthq_types::PathSegment::B(value) => match value {
                                            crate::FlightUnion2::A(_) => {
                                                panic!("TypeScript union narrowing failed")
                                            }
                                            crate::FlightUnion2::B(value) => match value {
                                                crate::FlightUnion2::A(_) => {
                                                    panic!("TypeScript union narrowing failed")
                                                }
                                                crate::FlightUnion2::B(value) => match value {
                                                    crate::FlightUnion2::A(value) => value,
                                                    crate::FlightUnion2::B(_) => {
                                                        panic!("TypeScript union narrowing failed")
                                                    }
                                                },
                                            },
                                        },
                                    })
                                    .x,
                                    y: (match (segment).clone() {
                                        flighthq_types::PathSegment::A(_) => {
                                            panic!("TypeScript union narrowing failed")
                                        }
                                        flighthq_types::PathSegment::B(value) => match value {
                                            crate::FlightUnion2::A(_) => {
                                                panic!("TypeScript union narrowing failed")
                                            }
                                            crate::FlightUnion2::B(value) => match value {
                                                crate::FlightUnion2::A(_) => {
                                                    panic!("TypeScript union narrowing failed")
                                                }
                                                crate::FlightUnion2::B(value) => match value {
                                                    crate::FlightUnion2::A(value) => value,
                                                    crate::FlightUnion2::B(_) => {
                                                        panic!("TypeScript union narrowing failed")
                                                    }
                                                },
                                            },
                                        },
                                    })
                                    .y,
                                    kind: "cubic".to_owned(),
                                    c1x: (match (segment).clone() {
                                        flighthq_types::PathSegment::A(_) => {
                                            panic!("TypeScript union narrowing failed")
                                        }
                                        flighthq_types::PathSegment::B(value) => match value {
                                            crate::FlightUnion2::A(_) => {
                                                panic!("TypeScript union narrowing failed")
                                            }
                                            crate::FlightUnion2::B(value) => match value {
                                                crate::FlightUnion2::A(_) => {
                                                    panic!("TypeScript union narrowing failed")
                                                }
                                                crate::FlightUnion2::B(value) => match value {
                                                    crate::FlightUnion2::A(value) => value,
                                                    crate::FlightUnion2::B(_) => {
                                                        panic!("TypeScript union narrowing failed")
                                                    }
                                                },
                                            },
                                        },
                                    })
                                    .control1_x,
                                    c1y: (match (segment).clone() {
                                        flighthq_types::PathSegment::A(_) => {
                                            panic!("TypeScript union narrowing failed")
                                        }
                                        flighthq_types::PathSegment::B(value) => match value {
                                            crate::FlightUnion2::A(_) => {
                                                panic!("TypeScript union narrowing failed")
                                            }
                                            crate::FlightUnion2::B(value) => match value {
                                                crate::FlightUnion2::A(_) => {
                                                    panic!("TypeScript union narrowing failed")
                                                }
                                                crate::FlightUnion2::B(value) => match value {
                                                    crate::FlightUnion2::A(value) => value,
                                                    crate::FlightUnion2::B(_) => {
                                                        panic!("TypeScript union narrowing failed")
                                                    }
                                                },
                                            },
                                        },
                                    })
                                    .control1_y,
                                    c2x: (match (segment).clone() {
                                        flighthq_types::PathSegment::A(_) => {
                                            panic!("TypeScript union narrowing failed")
                                        }
                                        flighthq_types::PathSegment::B(value) => match value {
                                            crate::FlightUnion2::A(_) => {
                                                panic!("TypeScript union narrowing failed")
                                            }
                                            crate::FlightUnion2::B(value) => match value {
                                                crate::FlightUnion2::A(_) => {
                                                    panic!("TypeScript union narrowing failed")
                                                }
                                                crate::FlightUnion2::B(value) => match value {
                                                    crate::FlightUnion2::A(value) => value,
                                                    crate::FlightUnion2::B(_) => {
                                                        panic!("TypeScript union narrowing failed")
                                                    }
                                                },
                                            },
                                        },
                                    })
                                    .control2_x,
                                    c2y: (match (segment).clone() {
                                        flighthq_types::PathSegment::A(_) => {
                                            panic!("TypeScript union narrowing failed")
                                        }
                                        flighthq_types::PathSegment::B(value) => match value {
                                            crate::FlightUnion2::A(_) => {
                                                panic!("TypeScript union narrowing failed")
                                            }
                                            crate::FlightUnion2::B(value) => match value {
                                                crate::FlightUnion2::A(_) => {
                                                    panic!("TypeScript union narrowing failed")
                                                }
                                                crate::FlightUnion2::B(value) => match value {
                                                    crate::FlightUnion2::A(value) => value,
                                                    crate::FlightUnion2::B(_) => {
                                                        panic!("TypeScript union narrowing failed")
                                                    }
                                                },
                                            },
                                        },
                                    })
                                    .control2_y,
                                },
                            ),
                        )));
                    } else {
                        if (((match (segment).clone() {
                            flighthq_types::PathSegment::A(_) => {
                                panic!("TypeScript union narrowing failed")
                            }
                            flighthq_types::PathSegment::B(value) => match value {
                                crate::FlightUnion2::A(_) => {
                                    panic!("TypeScript union narrowing failed")
                                }
                                crate::FlightUnion2::B(value) => match value {
                                    crate::FlightUnion2::A(_) => {
                                        panic!("TypeScript union narrowing failed")
                                    }
                                    crate::FlightUnion2::B(value) => match value {
                                        crate::FlightUnion2::A(_) => {
                                            panic!("TypeScript union narrowing failed")
                                        }
                                        crate::FlightUnion2::B(value) => value,
                                    },
                                },
                            },
                        })
                        .kind)
                            .clone()
                            == "close")
                        {
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
    out.data.extend(vec![
        match &((last).clone()) {
            crate::FlightUnion2::A(value) => (value).x.clone(),
            crate::FlightUnion2::B(value) => match value {
                crate::FlightUnion2::A(value) => (value).x.clone(),
                crate::FlightUnion2::B(value) => match value {
                    crate::FlightUnion2::A(value) => (value).x.clone(),
                    crate::FlightUnion2::B(value) => (value).x.clone(),
                },
            },
        },
        match &((last).clone()) {
            crate::FlightUnion2::A(value) => (value).y.clone(),
            crate::FlightUnion2::B(value) => match value {
                crate::FlightUnion2::A(value) => (value).y.clone(),
                crate::FlightUnion2::B(value) => match value {
                    crate::FlightUnion2::A(value) => (value).y.clone(),
                    crate::FlightUnion2::B(value) => (value).y.clone(),
                },
            },
        },
    ]);
    {
        let mut i = ((subpath.points.len() as f64) - 1.0_f64);
        while (i >= 1.0_f64) {
            let from = subpath.points[i as usize].clone();
            let to = subpath.points[(i - 1.0_f64) as usize].clone();
            if (matches!(&(from), crate::FlightUnion2::B(crate::FlightUnion2::A(_))))
                || (matches!(&(from), SubpathPoint::A(_)))
            {
                out.commands.push(PathCommand::LINE_TO);
                out.data.extend(vec![
                    match &((to).clone()) {
                        crate::FlightUnion2::A(value) => (value).x.clone(),
                        crate::FlightUnion2::B(value) => match value {
                            crate::FlightUnion2::A(value) => (value).x.clone(),
                            crate::FlightUnion2::B(value) => match value {
                                crate::FlightUnion2::A(value) => (value).x.clone(),
                                crate::FlightUnion2::B(value) => (value).x.clone(),
                            },
                        },
                    },
                    match &((to).clone()) {
                        crate::FlightUnion2::A(value) => (value).y.clone(),
                        crate::FlightUnion2::B(value) => match value {
                            crate::FlightUnion2::A(value) => (value).y.clone(),
                            crate::FlightUnion2::B(value) => match value {
                                crate::FlightUnion2::A(value) => (value).y.clone(),
                                crate::FlightUnion2::B(value) => (value).y.clone(),
                            },
                        },
                    },
                ]);
            } else {
                if matches!(
                    &(from),
                    crate::FlightUnion2::B(crate::FlightUnion2::B(crate::FlightUnion2::A(_)))
                ) {
                    out.commands.push(PathCommand::CURVE_TO);
                    out.data.extend(vec![
                        (match (from).clone() {
                            SubpathPoint::A(_) => panic!("TypeScript union narrowing failed"),
                            SubpathPoint::B(value) => match value {
                                crate::FlightUnion2::A(_) => {
                                    panic!("TypeScript union narrowing failed")
                                }
                                crate::FlightUnion2::B(value) => match value {
                                    crate::FlightUnion2::A(value) => value,
                                    crate::FlightUnion2::B(_) => {
                                        panic!("TypeScript union narrowing failed")
                                    }
                                },
                            },
                        })
                        .cx,
                        (match (from).clone() {
                            SubpathPoint::A(_) => panic!("TypeScript union narrowing failed"),
                            SubpathPoint::B(value) => match value {
                                crate::FlightUnion2::A(_) => {
                                    panic!("TypeScript union narrowing failed")
                                }
                                crate::FlightUnion2::B(value) => match value {
                                    crate::FlightUnion2::A(value) => value,
                                    crate::FlightUnion2::B(_) => {
                                        panic!("TypeScript union narrowing failed")
                                    }
                                },
                            },
                        })
                        .cy,
                        match &((to).clone()) {
                            crate::FlightUnion2::A(value) => (value).x.clone(),
                            crate::FlightUnion2::B(value) => match value {
                                crate::FlightUnion2::A(value) => (value).x.clone(),
                                crate::FlightUnion2::B(value) => match value {
                                    crate::FlightUnion2::A(value) => (value).x.clone(),
                                    crate::FlightUnion2::B(value) => (value).x.clone(),
                                },
                            },
                        },
                        match &((to).clone()) {
                            crate::FlightUnion2::A(value) => (value).y.clone(),
                            crate::FlightUnion2::B(value) => match value {
                                crate::FlightUnion2::A(value) => (value).y.clone(),
                                crate::FlightUnion2::B(value) => match value {
                                    crate::FlightUnion2::A(value) => (value).y.clone(),
                                    crate::FlightUnion2::B(value) => (value).y.clone(),
                                },
                            },
                        },
                    ]);
                } else {
                    if (((match (from).clone() {
                        SubpathPoint::A(_) => panic!("TypeScript union narrowing failed"),
                        SubpathPoint::B(value) => match value {
                            crate::FlightUnion2::A(_) => {
                                panic!("TypeScript union narrowing failed")
                            }
                            crate::FlightUnion2::B(value) => match value {
                                crate::FlightUnion2::A(_) => {
                                    panic!("TypeScript union narrowing failed")
                                }
                                crate::FlightUnion2::B(value) => value,
                            },
                        },
                    })
                    .kind)
                        .clone()
                        == "cubic")
                    {
                        out.commands.push(PathCommand::CUBIC_CURVE_TO);
                        out.data.extend(vec![
                            (match (from).clone() {
                                SubpathPoint::A(_) => panic!("TypeScript union narrowing failed"),
                                SubpathPoint::B(value) => match value {
                                    crate::FlightUnion2::A(_) => {
                                        panic!("TypeScript union narrowing failed")
                                    }
                                    crate::FlightUnion2::B(value) => match value {
                                        crate::FlightUnion2::A(_) => {
                                            panic!("TypeScript union narrowing failed")
                                        }
                                        crate::FlightUnion2::B(value) => value,
                                    },
                                },
                            })
                            .c2x,
                            (match (from).clone() {
                                SubpathPoint::A(_) => panic!("TypeScript union narrowing failed"),
                                SubpathPoint::B(value) => match value {
                                    crate::FlightUnion2::A(_) => {
                                        panic!("TypeScript union narrowing failed")
                                    }
                                    crate::FlightUnion2::B(value) => match value {
                                        crate::FlightUnion2::A(_) => {
                                            panic!("TypeScript union narrowing failed")
                                        }
                                        crate::FlightUnion2::B(value) => value,
                                    },
                                },
                            })
                            .c2y,
                            (match (from).clone() {
                                SubpathPoint::A(_) => panic!("TypeScript union narrowing failed"),
                                SubpathPoint::B(value) => match value {
                                    crate::FlightUnion2::A(_) => {
                                        panic!("TypeScript union narrowing failed")
                                    }
                                    crate::FlightUnion2::B(value) => match value {
                                        crate::FlightUnion2::A(_) => {
                                            panic!("TypeScript union narrowing failed")
                                        }
                                        crate::FlightUnion2::B(value) => value,
                                    },
                                },
                            })
                            .c1x,
                            (match (from).clone() {
                                SubpathPoint::A(_) => panic!("TypeScript union narrowing failed"),
                                SubpathPoint::B(value) => match value {
                                    crate::FlightUnion2::A(_) => {
                                        panic!("TypeScript union narrowing failed")
                                    }
                                    crate::FlightUnion2::B(value) => match value {
                                        crate::FlightUnion2::A(_) => {
                                            panic!("TypeScript union narrowing failed")
                                        }
                                        crate::FlightUnion2::B(value) => value,
                                    },
                                },
                            })
                            .c1y,
                            match &((to).clone()) {
                                crate::FlightUnion2::A(value) => (value).x.clone(),
                                crate::FlightUnion2::B(value) => match value {
                                    crate::FlightUnion2::A(value) => (value).x.clone(),
                                    crate::FlightUnion2::B(value) => match value {
                                        crate::FlightUnion2::A(value) => (value).x.clone(),
                                        crate::FlightUnion2::B(value) => (value).x.clone(),
                                    },
                                },
                            },
                            match &((to).clone()) {
                                crate::FlightUnion2::A(value) => (value).y.clone(),
                                crate::FlightUnion2::B(value) => match value {
                                    crate::FlightUnion2::A(value) => (value).y.clone(),
                                    crate::FlightUnion2::B(value) => match value {
                                        crate::FlightUnion2::A(value) => (value).y.clone(),
                                        crate::FlightUnion2::B(value) => (value).y.clone(),
                                    },
                                },
                            },
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
pub(crate) struct Subpath {
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
pub(crate) type SubpathPoint = crate::FlightUnion2<
    SubpathPointRecord4,
    crate::FlightUnion2<
        SubpathPointRecord3,
        crate::FlightUnion2<SubpathPointRecord2, SubpathPointRecord1>,
    >,
>;
