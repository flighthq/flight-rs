// @generated from upstream/packages/mesh/src/meshGeometryOperations.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{
    CANONICAL_MESH_GEOMETRY_LAYOUT as canonical_mesh_geometry_layout_constant, MeshGeometryOptions,
    compute_mesh_geometry_bounds, compute_mesh_geometry_normals, compute_mesh_geometry_tangents,
    create_mesh_geometry, get_mesh_geometry_vertex_count,
};
use flighthq_geometry::create_aabb;
use flighthq_types::{MeshGeometry, MeshSubset, VertexAttributeLayout};

// Source: upstream/packages/mesh/src/meshGeometryOperations.ts:16 (sha256:594c1b6b37974803ded7399e56221006adb4f5cfd748c999ef2740be53a08cce)
#[derive(Clone, Default)]
pub struct MeshGeometryFromAttributesOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub indices: Option<crate::FlightUnion2<Vec<f64>, crate::FlightUnion2<Vec<u16>, Vec<u32>>>>,
    pub normals: Option<Vec<f64>>,
    pub positions: Vec<f64>,
    pub uvs: Option<Vec<f64>>,
}
impl PartialEq for MeshGeometryFromAttributesOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/mesh/src/meshGeometryOperations.ts:28 (sha256:73f7ff0922b9bcebdec9e26ed58c053ee47723bedd5474a6659e95ea8a5c8421)
pub fn create_mesh_geometry_from_attributes(
    options: &MeshGeometryFromAttributesOptions,
) -> MeshGeometry {
    let vertex_count = ((options.positions.len() as f64) / 3.0_f64);
    let normals = (options.normals).clone();
    let uvs = (options.uvs).clone();
    let mut vertices: Vec<f32> =
        vec![0.0_f32; (vertex_count * CANONICAL_FLOATS_PER_VERTEX) as usize];
    {
        let mut i = 0.0_f64;
        while (i < vertex_count) {
            let base = (i * CANONICAL_FLOATS_PER_VERTEX);
            vertices[base as usize] = (options.positions[(i * 3.0_f64) as usize].clone()) as f32;
            vertices[(base + 1.0_f64) as usize] =
                (options.positions[((i * 3.0_f64) + 1.0_f64) as usize].clone()) as f32;
            vertices[(base + 2.0_f64) as usize] =
                (options.positions[((i * 3.0_f64) + 2.0_f64) as usize].clone()) as f32;
            if (normals).is_some() {
                vertices[(base + 3.0_f64) as usize] =
                    (normals.as_ref().unwrap()[(i * 3.0_f64) as usize].clone()) as f32;
                vertices[(base + 4.0_f64) as usize] =
                    (normals.as_ref().unwrap()[((i * 3.0_f64) + 1.0_f64) as usize].clone()) as f32;
                vertices[(base + 5.0_f64) as usize] =
                    (normals.as_ref().unwrap()[((i * 3.0_f64) + 2.0_f64) as usize].clone()) as f32;
            }
            if (uvs).is_some() {
                vertices[(base + 10.0_f64) as usize] =
                    (uvs.as_ref().unwrap()[(i * 2.0_f64) as usize].clone()) as f32;
                vertices[(base + 11.0_f64) as usize] =
                    (uvs.as_ref().unwrap()[((i * 2.0_f64) + 1.0_f64) as usize].clone()) as f32;
            }
            {
                i += 1.0;
                i
            };
        }
    }
    let mut index_array: Option<Vec<u32>>;
    if ((options.indices).clone()).is_some() {
        let src = (options.indices).clone();
        let needs_uint32 = (vertex_count > UINT16_INDEX_CEILING);
        if needs_uint32 {
            let mut a: Vec<u32> = vec![0_u32; (src.as_ref().unwrap().length) as usize];
            {
                let mut i = 0.0_f64;
                while (i < src.as_ref().unwrap().length) {
                    a[i as usize] = (src[i as usize].clone()) as u32;
                    {
                        i += 1.0;
                        i
                    };
                }
            }
            index_array = Some((a).clone());
        } else {
            let mut a: Vec<u16> = vec![0_u16; (src.as_ref().unwrap().length) as usize];
            {
                let mut i = 0.0_f64;
                while (i < src.as_ref().unwrap().length) {
                    a[i as usize] = (src[i as usize].clone()) as u16;
                    {
                        i += 1.0;
                        i
                    };
                }
            }
            index_array = Some(a);
        }
    }
    let mut geometry = create_mesh_geometry(&mut MeshGeometryOptions {
        __flight_identity: std::sync::Arc::new(()),
        indices: (index_array).clone(),
        layout: canonical_mesh_geometry_layout_constant,
        vertices: (vertices).clone(),
        subsets: None,
        topology: None,
    });
    if (normals).is_none() {
        {
            let __flight_argument_1 = (geometry).clone();
            compute_mesh_geometry_normals(&mut geometry, &__flight_argument_1)
        };
    }
    {
        let __flight_argument_1 = (geometry).clone();
        compute_mesh_geometry_tangents(&mut geometry, &__flight_argument_1)
    };
    let mut bounds = create_aabb(None, None, None, None, None, None);
    compute_mesh_geometry_bounds(&mut bounds, &geometry);
    geometry.bounds = Some((bounds).clone());
    return geometry;
}

// Source: upstream/packages/mesh/src/meshGeometryOperations.ts:83 (sha256:4a431b20dcf44afcc7c697c0ac465dcc084172057425a6351a204746e10b167a)
pub fn get_mesh_geometry_triangle_count(geometry: &MeshGeometry) -> f64 {
    if ((geometry.topology).clone() == "triangle-list") {
        let index_count = if ((geometry.indices).clone()).is_some() {
            (geometry.indices.as_ref().unwrap().len() as f64)
        } else {
            get_mesh_geometry_vertex_count(geometry)
        };
        return (index_count / 3.0_f64).floor();
    }
    if ((geometry.topology).clone() == "triangle-strip") {
        let index_count = if ((geometry.indices).clone()).is_some() {
            (geometry.indices.as_ref().unwrap().len() as f64)
        } else {
            get_mesh_geometry_vertex_count(geometry)
        };
        return if (index_count >= 3.0_f64) {
            (index_count - 2.0_f64)
        } else {
            0.0_f64
        };
    }
    return 0.0_f64;
}

// Source: upstream/packages/mesh/src/meshGeometryOperations.ts:100 (sha256:96ba3a5476fc56e6316f8263cd962fbd3b66ef6e949e9b18194e4a1db46ade0a)
pub fn merge_mesh_geometries(geometries: &Vec<MeshGeometry>) -> Option<MeshGeometry> {
    if ((geometries.len() as f64) == 0.0_f64) {
        return None;
    }
    let reference = geometries[0.0_f64 as usize].clone();
    {
        let mut i = 1.0_f64;
        while (i < (geometries.len() as f64)) {
            if (!layouts_match(&reference.layout, &geometries[i as usize].layout)) {
                return None;
            }
            {
                i += 1.0;
                i
            };
        }
    }
    let floats_per_vertex = (reference.layout.stride / 4.0_f64);
    let mut total_vertex_floats = 0.0_f64;
    let mut total_index_count = 0.0_f64;
    let mut all_indexed = true;
    for geo in (geometries).iter().cloned() {
        let vc = if (floats_per_vertex > 0.0_f64) {
            ((geo.vertices.len() as f64) / floats_per_vertex).floor()
        } else {
            0.0_f64
        };
        total_vertex_floats += (vc * floats_per_vertex);
        if ((geo.indices).clone()).is_some() {
            total_index_count += (geo.indices.as_ref().unwrap().len() as f64);
        } else {
            all_indexed = false;
            total_index_count += vc;
        }
    }
    let mut merged_vertices: Vec<f32> = vec![0.0_f32; (total_vertex_floats) as usize];
    let needs_uint32 = ((total_vertex_floats / floats_per_vertex) > UINT16_INDEX_CEILING);
    let mut merged_indices = if (all_indexed) || (total_index_count > 0.0_f64) {
        Some(if needs_uint32 {
            vec![0_u32; (total_index_count) as usize]
        } else {
            vec![0_u32; (total_index_count) as usize]
        })
    } else {
        None
    };
    let mut merged_subsets: Vec<MeshSubset> = vec![];
    let mut vertex_offset = 0.0_f64;
    let mut index_offset = 0.0_f64;
    let mut vertex_float_offset = 0.0_f64;
    for geo in (geometries).iter().cloned() {
        let vc = if (floats_per_vertex > 0.0_f64) {
            ((geo.vertices.len() as f64) / floats_per_vertex).floor()
        } else {
            0.0_f64
        };
        {
            let __flight_offset = (vertex_float_offset) as usize;
            let __flight_values: Vec<f32> = ((geo.vertices).clone()
                [(0.0_f64) as usize..(vc * floats_per_vertex) as usize]
                .to_vec())
            .iter()
            .map(|value| (*value) as f32)
            .collect();
            merged_vertices[__flight_offset..__flight_offset + __flight_values.len()]
                .copy_from_slice(&__flight_values);
        };
        if (merged_indices).is_some() {
            let src_count = if ((geo.indices).clone()).is_some() {
                (geo.indices.as_ref().unwrap().len() as f64)
            } else {
                vc
            };
            {
                let mut j = 0.0_f64;
                while (j < src_count) {
                    let src_idx = if ((geo.indices).clone()).is_some() {
                        (geo.indices.as_ref().unwrap()[j as usize].clone()) as u32
                    } else {
                        (j) as u32
                    };
                    merged_indices.as_mut().unwrap()[(index_offset + j) as usize] =
                        (src_idx + vertex_offset) as u32;
                    {
                        j += 1.0;
                        j
                    };
                }
            }
            for subset in ((geo.subsets).clone()).iter().cloned() {
                merged_subsets.push(MeshSubset {
                    __flight_identity: std::sync::Arc::new(()),
                    index_count: subset.index_count,
                    index_offset: (subset.index_offset + index_offset),
                });
            }
            index_offset += src_count;
        }
        vertex_offset += vc;
        vertex_float_offset += (vc * floats_per_vertex);
    }
    if ((merged_subsets.len() as f64) == 0.0_f64) {
        merged_subsets.push(MeshSubset {
            __flight_identity: std::sync::Arc::new(()),
            index_count: if (merged_indices).is_some() {
                (merged_indices.as_mut().unwrap().len() as f64)
            } else {
                (total_vertex_floats / floats_per_vertex)
            },
            index_offset: 0.0_f64,
        });
    }
    let mut merged = create_mesh_geometry(&mut MeshGeometryOptions {
        __flight_identity: std::sync::Arc::new(()),
        indices: merged_indices,
        layout: (reference.layout).clone(),
        subsets: Some((merged_subsets).clone()),
        topology: Some((reference.topology).clone()),
        vertices: (merged_vertices).clone(),
    });
    let mut bounds = create_aabb(None, None, None, None, None, None);
    compute_mesh_geometry_bounds(&mut bounds, &merged);
    merged.bounds = Some((bounds).clone());
    return Some((merged).clone());
}

// Source: upstream/packages/mesh/src/meshGeometryOperations.ts:181 (sha256:051872805493989f4b3db529bf32d3c6979b60c9de2f3c38bca6b93ca231238e)
pub fn validate_mesh_geometry(geometry: &MeshGeometry) -> bool {
    let floats_per_vertex = (geometry.layout.stride / 4.0_f64);
    if (floats_per_vertex <= 0.0_f64) {
        return false;
    }
    if (((geometry.vertices.len() as f64) % floats_per_vertex) != 0.0_f64) {
        return false;
    }
    let vertex_count = ((geometry.vertices.len() as f64) / floats_per_vertex).floor();
    if ((geometry.indices).clone()).is_some() {
        {
            let mut i = 0.0_f64;
            while (i < (geometry.indices.as_ref().unwrap().len() as f64)) {
                if (geometry.indices.as_ref().unwrap()[i as usize].clone() >= vertex_count) {
                    return false;
                }
                {
                    i += 1.0;
                    i
                };
            }
        }
    }
    let mut pos_offset = (-1.0_f64);
    {
        let mut i = 0.0_f64;
        while (i < (geometry.layout.attributes.len() as f64)) {
            let attr = geometry.layout.attributes[i as usize].clone();
            if ((attr.semantic).clone() == "position") && ((attr.format.starts_with)("float32")) {
                pos_offset = (attr.byte_offset / 4.0_f64);
                break;
            }
            {
                i += 1.0;
                i
            };
        }
    }
    if (pos_offset >= 0.0_f64) {
        {
            let mut i = 0.0_f64;
            while (i < vertex_count) {
                let base = ((i * floats_per_vertex) + pos_offset);
                let x = (geometry.vertices[base as usize] as f64);
                let y = (geometry.vertices[(base + 1.0_f64) as usize] as f64);
                let z = (geometry.vertices[(base + 2.0_f64) as usize] as f64);
                if ((!is_finite(x)) || (!is_finite(y))) || (!is_finite(z)) {
                    return false;
                }
                {
                    i += 1.0;
                    i
                };
            }
        }
    }
    return true;
}

// Source: upstream/packages/mesh/src/meshGeometryOperations.ts:217 (sha256:92214532d3b480dd53dcd9d5543ed834f598080ad85e31b30c4bf8cca06742f0)
fn layouts_match(a: &VertexAttributeLayout, b: &VertexAttributeLayout) -> bool {
    if (a.stride != b.stride) {
        return false;
    }
    if ((a.attributes.len() as f64) != (b.attributes.len() as f64)) {
        return false;
    }
    {
        let mut i = 0.0_f64;
        while (i < (a.attributes.len() as f64)) {
            let aa = a.attributes[i as usize].clone();
            let ba = b.attributes[i as usize].clone();
            if (((aa.semantic).clone() != (ba.semantic).clone())
                || ((aa.format).clone() != (ba.format).clone()))
                || (aa.byte_offset != ba.byte_offset)
            {
                return false;
            }
            {
                i += 1.0;
                i
            };
        }
    }
    return true;
}

// Source: upstream/packages/mesh/src/meshGeometryOperations.ts:228 (sha256:a7b0354de76d287339f53dd191039a613081dc1dd7ce7309de50dd4faab3d7e6)
const CANONICAL_FLOATS_PER_VERTEX: f64 = 12.0_f64;

// Source: upstream/packages/mesh/src/meshGeometryOperations.ts:229 (sha256:9b733a20b1617d87fbd91ece36aa50d6080f1bc799022c1f7a79d98c1f71628c)
const UINT16_INDEX_CEILING: f64 = 65535.0_f64;
