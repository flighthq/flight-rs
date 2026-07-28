// @generated from upstream/packages/mesh/src/meshGeometryLayout.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{MeshGeometryOptions, create_mesh_geometry};
use flighthq_types::{MeshGeometry, VertexAttribute, VertexAttributeLayout};

// Source: upstream/packages/mesh/src/meshGeometryLayout.ts:15 (sha256:0c1f90826c2b15369835edd583dc7bda176cc18f692ce16696c410422faa1a33)
#[derive(Clone)]
struct ConvertMeshGeometryLayoutRecord1 {
    __flight_identity: std::sync::Arc<()>,
    component_count: f64,
    dst_float_offset: f64,
    src_float_offset: f64,
}
impl PartialEq for ConvertMeshGeometryLayoutRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn convert_mesh_geometry_layout(
    source: &MeshGeometry,
    target_layout: &VertexAttributeLayout,
) -> MeshGeometry {
    let src_stride = source.layout.stride;
    let dst_stride = target_layout.stride;
    let src_floats_per_vertex = (src_stride / 4.0_f64);
    let dst_floats_per_vertex = (dst_stride / 4.0_f64);
    let vertex_count = if (src_floats_per_vertex > 0.0_f64) {
        ((source.vertices.len() as f64) / src_floats_per_vertex).floor()
    } else {
        0.0_f64
    };
    let mut dst_vertices: Vec<f32> = vec![0.0_f32; (vertex_count * dst_floats_per_vertex) as usize];
    let mut mapping: Vec<ConvertMeshGeometryLayoutRecord1> = vec![];
    for dst_attr in ((target_layout.attributes).clone()).iter().cloned() {
        if (!(dst_attr.format.starts_with)("float32")) {
            continue;
        }
        let dst_float_offset = (dst_attr.byte_offset / 4.0_f64);
        let component_count = get_float32_component_count((dst_attr.format).clone());
        if (component_count == 0.0_f64) {
            continue;
        }
        let src_attr = ((source.layout.attributes).clone())
            .iter()
            .find(|value| {
                (|a: VertexAttribute| -> bool {
                    ((a.semantic).clone() == (dst_attr.semantic).clone())
                        && ((a.format.starts_with)("float32"))
                })((*value).clone())
            })
            .cloned();
        if (src_attr).is_none() {
            continue;
        }
        mapping.push(ConvertMeshGeometryLayoutRecord1 {
            __flight_identity: std::sync::Arc::new(()),
            component_count: component_count,
            dst_float_offset: dst_float_offset,
            src_float_offset: (src_attr.as_ref().unwrap().byte_offset / 4.0_f64),
        });
    }
    {
        let mut i = 0.0_f64;
        while (i < vertex_count) {
            let src_base = (i * src_floats_per_vertex);
            let dst_base = (i * dst_floats_per_vertex);
            for __iteration0 in (mapping).iter().cloned() {
                let dst_float_offset = __iteration0.dst_float_offset;
                let src_float_offset = __iteration0.src_float_offset;
                let component_count = __iteration0.component_count;
                {
                    let mut c = 0.0_f64;
                    while (c < component_count) {
                        dst_vertices[((dst_base + dst_float_offset) + c) as usize] =
                            (source.vertices[((src_base + src_float_offset) + c) as usize] as f64)
                                as f32;
                        {
                            c += 1.0;
                            c
                        };
                    }
                }
            }
            {
                i += 1.0;
                i
            };
        }
    }
    return create_mesh_geometry(&mut MeshGeometryOptions {
        __flight_identity: std::sync::Arc::new(()),
        indices: (source.indices).clone(),
        layout: (*target_layout).clone(),
        subsets: Some((source.subsets).clone()),
        topology: Some((source.topology).clone()),
        vertices: (dst_vertices).clone(),
    });
}

// Source: upstream/packages/mesh/src/meshGeometryLayout.ts:68 (sha256:1c9b552a8ffb0034024674c0c5a77647d61d0ec428cdcfc020a087841a2e6f72)
fn get_float32_component_count(format: String) -> f64 {
    if (format == "float32") {
        return 1.0_f64;
    }
    if (format == "float32x2") {
        return 2.0_f64;
    }
    if (format == "float32x3") {
        return 3.0_f64;
    }
    if (format == "float32x4") {
        return 4.0_f64;
    }
    return 0.0_f64;
}

// Source: upstream/packages/mesh/src/meshGeometryLayout.ts:80 (sha256:666ee2b8ef5637e929f5845f7bc70e62e0f18958903b8e4b8e8094b9424be43b)
pub static CANONICAL_MESH_GEOMETRY_LAYOUT: std::sync::LazyLock<VertexAttributeLayout> =
    std::sync::LazyLock::new(|| VertexAttributeLayout {
        __flight_identity: std::sync::Arc::new(()),
        attributes: vec![
            VertexAttribute {
                __flight_identity: std::sync::Arc::new(()),
                byte_offset: 0.0_f64,
                format: "float32x3".to_owned(),
                semantic: "position".to_owned(),
            },
            VertexAttribute {
                __flight_identity: std::sync::Arc::new(()),
                byte_offset: 12.0_f64,
                format: "float32x3".to_owned(),
                semantic: "normal".to_owned(),
            },
            VertexAttribute {
                __flight_identity: std::sync::Arc::new(()),
                byte_offset: 24.0_f64,
                format: "float32x4".to_owned(),
                semantic: "tangent".to_owned(),
            },
            VertexAttribute {
                __flight_identity: std::sync::Arc::new(()),
                byte_offset: 40.0_f64,
                format: "float32x2".to_owned(),
                semantic: "uv0".to_owned(),
            },
        ],
        stride: 48.0_f64,
    });

// Source: upstream/packages/mesh/src/meshGeometryLayout.ts:98 (sha256:69c0de5b28bbbbd41ddba04a55c0bfc89132108dbe19de1a00c32e5999a3cc19)
pub static CANONICAL_SKINNED_MESH_GEOMETRY_LAYOUT: std::sync::LazyLock<VertexAttributeLayout> =
    std::sync::LazyLock::new(|| VertexAttributeLayout {
        __flight_identity: std::sync::Arc::new(()),
        attributes: vec![
            VertexAttribute {
                __flight_identity: std::sync::Arc::new(()),
                byte_offset: 0.0_f64,
                format: "float32x3".to_owned(),
                semantic: "position".to_owned(),
            },
            VertexAttribute {
                __flight_identity: std::sync::Arc::new(()),
                byte_offset: 12.0_f64,
                format: "float32x3".to_owned(),
                semantic: "normal".to_owned(),
            },
            VertexAttribute {
                __flight_identity: std::sync::Arc::new(()),
                byte_offset: 24.0_f64,
                format: "float32x4".to_owned(),
                semantic: "tangent".to_owned(),
            },
            VertexAttribute {
                __flight_identity: std::sync::Arc::new(()),
                byte_offset: 40.0_f64,
                format: "float32x2".to_owned(),
                semantic: "uv0".to_owned(),
            },
            VertexAttribute {
                __flight_identity: std::sync::Arc::new(()),
                byte_offset: 48.0_f64,
                format: "float32x4".to_owned(),
                semantic: "joints0".to_owned(),
            },
            VertexAttribute {
                __flight_identity: std::sync::Arc::new(()),
                byte_offset: 64.0_f64,
                format: "float32x4".to_owned(),
                semantic: "weights0".to_owned(),
            },
        ],
        stride: 80.0_f64,
    });
