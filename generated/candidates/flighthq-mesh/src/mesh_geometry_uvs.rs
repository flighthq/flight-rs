// @generated from upstream/packages/mesh/src/meshGeometryUvs.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::get_vertex_attribute_float_offset;
use flighthq_types::MeshGeometry;

// Source: upstream/packages/mesh/src/meshGeometryUvs.ts:12 (sha256:baee1d88472961e5de743b6557f84131a407047baf2d57ded7521f6614995dd8)
pub fn offset_mesh_geometry_uvs(geometry: &mut MeshGeometry, du: f64, dv: f64) -> () {
    let float_offset = get_vertex_attribute_float_offset(&geometry.layout, "uv0".to_owned());
    if (float_offset < 0.0_f64) {
        return;
    }
    let floats_per_vertex = (geometry.layout.stride / 4.0_f64);
    let vertex_count = if (floats_per_vertex > 0.0_f64) {
        ((geometry.vertices.len() as f64) / floats_per_vertex).floor()
    } else {
        0.0_f64
    };
    {
        let mut i = 0.0_f64;
        while (i < vertex_count) {
            let base = ((i * floats_per_vertex) + float_offset);
            geometry.vertices[base as usize] += (du) as f32;
            geometry.vertices[(base + 1.0_f64) as usize] += (dv) as f32;
            {
                i += 1.0;
                i
            };
        }
    }
    if (vertex_count > 0.0_f64) {
        {
            geometry.version += 1.0;
            geometry.version
        };
    }
}

// Source: upstream/packages/mesh/src/meshGeometryUvs.ts:28 (sha256:7294abc7a535fdcd40cadb19926ebc138c38c8a6ae38607516be33b10c678858)
pub fn scale_mesh_geometry_uvs(geometry: &mut MeshGeometry, su: f64, sv: f64) -> () {
    let float_offset = get_vertex_attribute_float_offset(&geometry.layout, "uv0".to_owned());
    if (float_offset < 0.0_f64) {
        return;
    }
    let floats_per_vertex = (geometry.layout.stride / 4.0_f64);
    let vertex_count = if (floats_per_vertex > 0.0_f64) {
        ((geometry.vertices.len() as f64) / floats_per_vertex).floor()
    } else {
        0.0_f64
    };
    {
        let mut i = 0.0_f64;
        while (i < vertex_count) {
            let base = ((i * floats_per_vertex) + float_offset);
            geometry.vertices[base as usize] *= (su) as f32;
            geometry.vertices[(base + 1.0_f64) as usize] *= (sv) as f32;
            {
                i += 1.0;
                i
            };
        }
    }
    if (vertex_count > 0.0_f64) {
        {
            geometry.version += 1.0;
            geometry.version
        };
    }
}

// Source: upstream/packages/mesh/src/meshGeometryUvs.ts:45 (sha256:ed61056b5f25d891820c41bdaf257842328da10cbbf686895e41e12117263658)
pub fn wrap_mesh_geometry_uvs(geometry: &mut MeshGeometry) -> () {
    let float_offset = get_vertex_attribute_float_offset(&geometry.layout, "uv0".to_owned());
    if (float_offset < 0.0_f64) {
        return;
    }
    let floats_per_vertex = (geometry.layout.stride / 4.0_f64);
    let vertex_count = if (floats_per_vertex > 0.0_f64) {
        ((geometry.vertices.len() as f64) / floats_per_vertex).floor()
    } else {
        0.0_f64
    };
    {
        let mut i = 0.0_f64;
        while (i < vertex_count) {
            let base = ((i * floats_per_vertex) + float_offset);
            geometry.vertices[base as usize] = ((geometry.vertices[base as usize] as f64)
                - (geometry.vertices[base as usize] as f64).floor())
                as f32;
            geometry.vertices[(base + 1.0_f64) as usize] =
                ((geometry.vertices[(base + 1.0_f64) as usize] as f64)
                    - (geometry.vertices[(base + 1.0_f64) as usize] as f64).floor())
                    as f32;
            {
                i += 1.0;
                i
            };
        }
    }
    if (vertex_count > 0.0_f64) {
        {
            geometry.version += 1.0;
            geometry.version
        };
    }
}
