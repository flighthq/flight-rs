// @generated from upstream/packages/mesh/src/meshGeometryAttributes.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::{MeshGeometry, VertexAttribute, VertexAttributeLayout, VertexSemantic};

#[derive(Clone, Default)]
pub struct SharedStructuralRecord1 {
    pub __flight_identity: std::sync::Arc<()>,
    pub x: f64,
    pub y: f64,
    pub z: f64,
}
impl PartialEq for SharedStructuralRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct SharedStructuralRecord2 {
    pub __flight_identity: std::sync::Arc<()>,
    pub w: f64,
    pub x: f64,
    pub y: f64,
    pub z: f64,
}
impl PartialEq for SharedStructuralRecord2 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct SharedStructuralRecord3 {
    pub __flight_identity: std::sync::Arc<()>,
    pub x: f64,
    pub y: f64,
}
impl PartialEq for SharedStructuralRecord3 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/mesh/src/meshGeometryAttributes.ts:10 (sha256:585479d56561d263a6ab15effdb23136fcd5805f3be360ab1bfa3bdb76133626)
pub fn get_mesh_geometry_vertex_normal(
    out: &mut SharedStructuralRecord1,
    geometry: &MeshGeometry,
    vertex_index: f64,
) -> bool {
    return get_float3_attribute(out, geometry, vertex_index, "normal".to_owned());
}

// Source: upstream/packages/mesh/src/meshGeometryAttributes.ts:20 (sha256:0133be6c1d65c52e13178c7c2be8009820ca8ffe63e59244f61999b615e369d6)
pub fn get_mesh_geometry_vertex_position(
    out: &mut SharedStructuralRecord1,
    geometry: &MeshGeometry,
    vertex_index: f64,
) -> bool {
    return get_float3_attribute(out, geometry, vertex_index, "position".to_owned());
}

// Source: upstream/packages/mesh/src/meshGeometryAttributes.ts:30 (sha256:81c92306bfe892dd7973fef73b30be4ab4917dcb776c487b1fc78e8ca116af4d)
pub fn get_mesh_geometry_vertex_tangent(
    out: &mut SharedStructuralRecord2,
    geometry: &MeshGeometry,
    vertex_index: f64,
) -> bool {
    let float_offset = get_vertex_attribute_float_offset(&geometry.layout, "tangent".to_owned());
    if (float_offset < 0.0_f64) {
        return false;
    }
    let floats_per_vertex = (geometry.layout.stride / 4.0_f64);
    let vertex_count = if (floats_per_vertex > 0.0_f64) {
        ((geometry.vertices.len() as f64) / floats_per_vertex).floor()
    } else {
        0.0_f64
    };
    if (vertex_index < 0.0_f64) || (vertex_index >= vertex_count) {
        return false;
    }
    let base = ((vertex_index * floats_per_vertex) + float_offset);
    out.x = (geometry.vertices[base as usize] as f64);
    out.y = (geometry.vertices[(base + 1.0_f64) as usize] as f64);
    out.z = (geometry.vertices[(base + 2.0_f64) as usize] as f64);
    out.w = (geometry.vertices[(base + 3.0_f64) as usize] as f64);
    return true;
}

// Source: upstream/packages/mesh/src/meshGeometryAttributes.ts:50 (sha256:122f94199abdd869df0c89b68208e92cf131e4dac630ef5be406dd589374a987)
pub fn get_mesh_geometry_vertex_uv0(
    out: &mut SharedStructuralRecord3,
    geometry: &MeshGeometry,
    vertex_index: f64,
) -> bool {
    let float_offset = get_vertex_attribute_float_offset(&geometry.layout, "uv0".to_owned());
    if (float_offset < 0.0_f64) {
        return false;
    }
    let floats_per_vertex = (geometry.layout.stride / 4.0_f64);
    let vertex_count = if (floats_per_vertex > 0.0_f64) {
        ((geometry.vertices.len() as f64) / floats_per_vertex).floor()
    } else {
        0.0_f64
    };
    if (vertex_index < 0.0_f64) || (vertex_index >= vertex_count) {
        return false;
    }
    let base = ((vertex_index * floats_per_vertex) + float_offset);
    out.x = (geometry.vertices[base as usize] as f64);
    out.y = (geometry.vertices[(base + 1.0_f64) as usize] as f64);
    return true;
}

// Source: upstream/packages/mesh/src/meshGeometryAttributes.ts:67 (sha256:ad1ba81f22dd52d4481be91f0a4c426f570f76784c4af111fa5a4578de338a18)
pub fn get_vertex_attribute(
    layout: &VertexAttributeLayout,
    semantic: VertexSemantic,
) -> Option<VertexAttribute> {
    {
        let mut i = 0.0_f64;
        while (i < (layout.attributes.len() as f64)) {
            if ((layout.attributes[i as usize].semantic).clone() == semantic) {
                return Some(layout.attributes[i as usize].clone());
            }
            {
                i += 1.0;
                i
            };
        }
    }
    return None;
}

// Source: upstream/packages/mesh/src/meshGeometryAttributes.ts:81 (sha256:9db648c799b40a320a44fc19a9355231b883731852d2c3a05c592a224e0b6e61)
pub fn get_vertex_attribute_float_offset(
    layout: &VertexAttributeLayout,
    semantic: VertexSemantic,
) -> f64 {
    {
        let mut i = 0.0_f64;
        while (i < (layout.attributes.len() as f64)) {
            let attr = layout.attributes[i as usize].clone();
            if ((attr.semantic).clone() == semantic) {
                if (!(attr.format.starts_with)("float32")) {
                    return (-1.0_f64);
                }
                return (attr.byte_offset / 4.0_f64);
            }
            {
                i += 1.0;
                i
            };
        }
    }
    return (-1.0_f64);
}

// Source: upstream/packages/mesh/src/meshGeometryAttributes.ts:98 (sha256:dfee0b2ff9b766eb9615439fa83bcf8e019252fcbc98c8522bc45f16219c45c2)
pub fn set_mesh_geometry_vertex_normal(
    geometry: &mut MeshGeometry,
    vertex_index: f64,
    x: f64,
    y: f64,
    z: f64,
) -> bool {
    return set_float3_attribute(geometry, vertex_index, "normal".to_owned(), x, y, z);
}

// Source: upstream/packages/mesh/src/meshGeometryAttributes.ts:110 (sha256:d9b28d40f3a29ad7100efedfea1fc2e0a835b66ffce8ad44049d30f413a8e5bf)
pub fn set_mesh_geometry_vertex_position(
    geometry: &mut MeshGeometry,
    vertex_index: f64,
    x: f64,
    y: f64,
    z: f64,
) -> bool {
    return set_float3_attribute(geometry, vertex_index, "position".to_owned(), x, y, z);
}

// Source: upstream/packages/mesh/src/meshGeometryAttributes.ts:122 (sha256:ec4abf85ebef45e2b7f7d6f0062aa4abb1dc38304f321a112374851235849a4d)
pub fn set_mesh_geometry_vertex_tangent(
    geometry: &mut MeshGeometry,
    vertex_index: f64,
    x: f64,
    y: f64,
    z: f64,
    w: f64,
) -> bool {
    let float_offset = get_vertex_attribute_float_offset(&geometry.layout, "tangent".to_owned());
    if (float_offset < 0.0_f64) {
        return false;
    }
    let floats_per_vertex = (geometry.layout.stride / 4.0_f64);
    let vertex_count = if (floats_per_vertex > 0.0_f64) {
        ((geometry.vertices.len() as f64) / floats_per_vertex).floor()
    } else {
        0.0_f64
    };
    if (vertex_index < 0.0_f64) || (vertex_index >= vertex_count) {
        return false;
    }
    let base = ((vertex_index * floats_per_vertex) + float_offset);
    geometry.vertices[base as usize] = (x) as f32;
    geometry.vertices[(base + 1.0_f64) as usize] = (y) as f32;
    geometry.vertices[(base + 2.0_f64) as usize] = (z) as f32;
    geometry.vertices[(base + 3.0_f64) as usize] = (w) as f32;
    {
        geometry.version += 1.0;
        geometry.version
    };
    return true;
}

// Source: upstream/packages/mesh/src/meshGeometryAttributes.ts:146 (sha256:bc2e4c0efbc0c95ba645d4b1170c714d58cc31ad91b8e09303d97b96817466f4)
pub fn set_mesh_geometry_vertex_uv0(
    geometry: &mut MeshGeometry,
    vertex_index: f64,
    u: f64,
    v: f64,
) -> bool {
    let float_offset = get_vertex_attribute_float_offset(&geometry.layout, "uv0".to_owned());
    if (float_offset < 0.0_f64) {
        return false;
    }
    let floats_per_vertex = (geometry.layout.stride / 4.0_f64);
    let vertex_count = if (floats_per_vertex > 0.0_f64) {
        ((geometry.vertices.len() as f64) / floats_per_vertex).floor()
    } else {
        0.0_f64
    };
    if (vertex_index < 0.0_f64) || (vertex_index >= vertex_count) {
        return false;
    }
    let base = ((vertex_index * floats_per_vertex) + float_offset);
    geometry.vertices[base as usize] = (u) as f32;
    geometry.vertices[(base + 1.0_f64) as usize] = (v) as f32;
    {
        geometry.version += 1.0;
        geometry.version
    };
    return true;
}

// Source: upstream/packages/mesh/src/meshGeometryAttributes.ts:159 (sha256:96e9c693dd4eac183d97b7476ad5a690f4fd2583bb7249efd50bcd651ebb23e6)
fn get_float3_attribute(
    out: &mut SharedStructuralRecord1,
    geometry: &MeshGeometry,
    vertex_index: f64,
    semantic: VertexSemantic,
) -> bool {
    let float_offset = get_vertex_attribute_float_offset(&geometry.layout, (semantic).clone());
    if (float_offset < 0.0_f64) {
        return false;
    }
    let floats_per_vertex = (geometry.layout.stride / 4.0_f64);
    let vertex_count = if (floats_per_vertex > 0.0_f64) {
        ((geometry.vertices.len() as f64) / floats_per_vertex).floor()
    } else {
        0.0_f64
    };
    if (vertex_index < 0.0_f64) || (vertex_index >= vertex_count) {
        return false;
    }
    let base = ((vertex_index * floats_per_vertex) + float_offset);
    out.x = (geometry.vertices[base as usize] as f64);
    out.y = (geometry.vertices[(base + 1.0_f64) as usize] as f64);
    out.z = (geometry.vertices[(base + 2.0_f64) as usize] as f64);
    return true;
}

// Source: upstream/packages/mesh/src/meshGeometryAttributes.ts:177 (sha256:5f206702bdff9dd1b7b5dec5c7afecc080ef0c14bfb1a6266fafaf7de07b1dbc)
fn set_float3_attribute(
    geometry: &mut MeshGeometry,
    vertex_index: f64,
    semantic: VertexSemantic,
    x: f64,
    y: f64,
    z: f64,
) -> bool {
    let float_offset = get_vertex_attribute_float_offset(&geometry.layout, (semantic).clone());
    if (float_offset < 0.0_f64) {
        return false;
    }
    let floats_per_vertex = (geometry.layout.stride / 4.0_f64);
    let vertex_count = if (floats_per_vertex > 0.0_f64) {
        ((geometry.vertices.len() as f64) / floats_per_vertex).floor()
    } else {
        0.0_f64
    };
    if (vertex_index < 0.0_f64) || (vertex_index >= vertex_count) {
        return false;
    }
    let base = ((vertex_index * floats_per_vertex) + float_offset);
    geometry.vertices[base as usize] = (x) as f32;
    geometry.vertices[(base + 1.0_f64) as usize] = (y) as f32;
    geometry.vertices[(base + 2.0_f64) as usize] = (z) as f32;
    {
        geometry.version += 1.0;
        geometry.version
    };
    return true;
}
