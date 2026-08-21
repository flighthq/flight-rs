// @generated from upstream/packages/path/src/forEachPathSegment.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::{Path, PathCommand, PathSegment};

// Source: upstream/packages/path/src/forEachPathSegment.ts:11 (sha256:b0ce2a07893d7e18c0ab3659b0128300f8eb880b269af127886cb7936881ae4f)
#[derive(Clone, Default)]
struct ForEachPathSegmentRecord9 {
    __flight_identity: std::sync::Arc<()>,
    kind: String,
}
impl PartialEq for ForEachPathSegmentRecord9 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn for_each_path_segment(path: &Path, visitor: &mut impl FnMut(PathSegment) -> ()) -> () {
    let mut di = 0.0_f64;
    {
        let mut ci = 0.0_f64;
        while (ci < (path.commands.len() as f64)) {
            let command = path.commands[ci as usize].clone();
            if (command == PathCommand::MOVE_TO) {
                let x = path.data[di as usize].clone();
                let y = path.data[(di + 1.0_f64) as usize].clone();
                di += 2.0_f64;
                visitor(flighthq_types::PathSegment::A(
                    flighthq_types::PathSegmentRecord5 {
                        __flight_identity: std::sync::Arc::new(()),
                        kind: "moveTo".to_owned(),
                        x: x,
                        y: y,
                    },
                ));
            } else {
                if (command == PathCommand::WIDE_MOVE_TO) {
                    let x = path.data[(di + 2.0_f64) as usize].clone();
                    let y = path.data[(di + 3.0_f64) as usize].clone();
                    di += 4.0_f64;
                    visitor(flighthq_types::PathSegment::A(
                        flighthq_types::PathSegmentRecord5 {
                            __flight_identity: std::sync::Arc::new(()),
                            kind: "moveTo".to_owned(),
                            x: x,
                            y: y,
                        },
                    ));
                } else {
                    if (command == PathCommand::LINE_TO) {
                        let x = path.data[di as usize].clone();
                        let y = path.data[(di + 1.0_f64) as usize].clone();
                        di += 2.0_f64;
                        visitor(flighthq_types::PathSegment::B(crate::FlightUnion2::<
                            flighthq_types::PathSegmentRecord4,
                            crate::FlightUnion2<
                                flighthq_types::PathSegmentRecord3,
                                crate::FlightUnion2<
                                    flighthq_types::PathSegmentRecord2,
                                    flighthq_types::PathSegmentRecord1,
                                >,
                            >,
                        >::A(
                            flighthq_types::PathSegmentRecord4 {
                                __flight_identity: std::sync::Arc::new(()),
                                kind: "lineTo".to_owned(),
                                x: x,
                                y: y,
                            },
                        )));
                    } else {
                        if (command == PathCommand::WIDE_LINE_TO) {
                            let x = path.data[(di + 2.0_f64) as usize].clone();
                            let y = path.data[(di + 3.0_f64) as usize].clone();
                            di += 4.0_f64;
                            visitor(flighthq_types::PathSegment::B(crate::FlightUnion2::<
                                flighthq_types::PathSegmentRecord4,
                                crate::FlightUnion2<
                                    flighthq_types::PathSegmentRecord3,
                                    crate::FlightUnion2<
                                        flighthq_types::PathSegmentRecord2,
                                        flighthq_types::PathSegmentRecord1,
                                    >,
                                >,
                            >::A(
                                flighthq_types::PathSegmentRecord4 {
                                    __flight_identity: std::sync::Arc::new(()),
                                    kind: "lineTo".to_owned(),
                                    x: x,
                                    y: y,
                                },
                            )));
                        } else {
                            if (command == PathCommand::CURVE_TO) {
                                let control_x = path.data[di as usize].clone();
                                let control_y = path.data[(di + 1.0_f64) as usize].clone();
                                let x = path.data[(di + 2.0_f64) as usize].clone();
                                let y = path.data[(di + 3.0_f64) as usize].clone();
                                di += 4.0_f64;
                                visitor(flighthq_types::PathSegment::B(crate::FlightUnion2::<
                                    flighthq_types::PathSegmentRecord4,
                                    crate::FlightUnion2<
                                        flighthq_types::PathSegmentRecord3,
                                        crate::FlightUnion2<
                                            flighthq_types::PathSegmentRecord2,
                                            flighthq_types::PathSegmentRecord1,
                                        >,
                                    >,
                                >::B(
                                    crate::FlightUnion2::<
                                        flighthq_types::PathSegmentRecord3,
                                        crate::FlightUnion2<
                                            flighthq_types::PathSegmentRecord2,
                                            flighthq_types::PathSegmentRecord1,
                                        >,
                                    >::A(
                                        flighthq_types::PathSegmentRecord3 {
                                            __flight_identity: std::sync::Arc::new(()),
                                            kind: "curveTo".to_owned(),
                                            control_x: control_x,
                                            control_y: control_y,
                                            x: x,
                                            y: y,
                                        },
                                    ),
                                )));
                            } else {
                                if (command == PathCommand::CUBIC_CURVE_TO) {
                                    let control1_x = path.data[di as usize].clone();
                                    let control1_y = path.data[(di + 1.0_f64) as usize].clone();
                                    let control2_x = path.data[(di + 2.0_f64) as usize].clone();
                                    let control2_y = path.data[(di + 3.0_f64) as usize].clone();
                                    let x = path.data[(di + 4.0_f64) as usize].clone();
                                    let y = path.data[(di + 5.0_f64) as usize].clone();
                                    di += 6.0_f64;
                                    visitor(flighthq_types::PathSegment::B(crate::FlightUnion2::<
                                        flighthq_types::PathSegmentRecord4,
                                        crate::FlightUnion2<
                                            flighthq_types::PathSegmentRecord3,
                                            crate::FlightUnion2<
                                                flighthq_types::PathSegmentRecord2,
                                                flighthq_types::PathSegmentRecord1,
                                            >,
                                        >,
                                    >::B(
                                        crate::FlightUnion2::<
                                            flighthq_types::PathSegmentRecord3,
                                            crate::FlightUnion2<
                                                flighthq_types::PathSegmentRecord2,
                                                flighthq_types::PathSegmentRecord1,
                                            >,
                                        >::B(
                                            crate::FlightUnion2::<
                                                flighthq_types::PathSegmentRecord2,
                                                flighthq_types::PathSegmentRecord1,
                                            >::A(
                                                flighthq_types::PathSegmentRecord2 {
                                                    __flight_identity: std::sync::Arc::new(()),
                                                    kind: "cubicCurveTo".to_owned(),
                                                    control1_x: control1_x,
                                                    control1_y: control1_y,
                                                    control2_x: control2_x,
                                                    control2_y: control2_y,
                                                    x: x,
                                                    y: y,
                                                },
                                            ),
                                        ),
                                    )));
                                } else {
                                    if (command == PathCommand::CLOSE) {
                                        visitor(flighthq_types::PathSegment::B(
                                            crate::FlightUnion2::<
                                                flighthq_types::PathSegmentRecord4,
                                                crate::FlightUnion2<
                                                    flighthq_types::PathSegmentRecord3,
                                                    crate::FlightUnion2<
                                                        flighthq_types::PathSegmentRecord2,
                                                        flighthq_types::PathSegmentRecord1,
                                                    >,
                                                >,
                                            >::B(
                                                crate::FlightUnion2::<
                                                    flighthq_types::PathSegmentRecord3,
                                                    crate::FlightUnion2<
                                                        flighthq_types::PathSegmentRecord2,
                                                        flighthq_types::PathSegmentRecord1,
                                                    >,
                                                >::B(
                                                    crate::FlightUnion2::<
                                                        flighthq_types::PathSegmentRecord2,
                                                        flighthq_types::PathSegmentRecord1,
                                                    >::B(
                                                        flighthq_types::PathSegmentRecord1 {
                                                            __flight_identity: std::sync::Arc::new(
                                                                (),
                                                            ),
                                                            kind: "close".to_owned(),
                                                        },
                                                    ),
                                                ),
                                            ),
                                        ));
                                    }
                                }
                            }
                        }
                    }
                }
            }
            {
                ci += 1.0;
                ci
            };
        }
    }
}
