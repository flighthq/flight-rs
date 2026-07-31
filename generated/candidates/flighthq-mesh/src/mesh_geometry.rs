// @generated from upstream/packages/mesh/src/meshGeometry.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_entity::create_entity;
use flighthq_geometry::create_aabb;
use flighthq_types::{
    EntityRuntime, MeshGeometry, MeshGeometryRuntime, MeshMorphBindPose, MeshSkinBindPose,
    MeshSubset, PrimitiveTopology, VertexAttributeLayout,
};

// Source: upstream/packages/mesh/src/meshGeometry.ts:19 (sha256:a33a9efe64b61d07fa86e41111d86d5ec5ba0c4a2c4d9d17ab09563800dc99cb)
#[derive(Clone, Default)]
pub struct MeshGeometryOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub indices: Option<Vec<u32>>,
    pub layout: VertexAttributeLayout,
    pub subsets: Option<Vec<MeshSubset>>,
    pub topology: Option<PrimitiveTopology>,
    pub vertices: Vec<f32>,
}
impl PartialEq for MeshGeometryOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/mesh/src/meshGeometry.ts:30 (sha256:6b7b647175f7a7dd07ef05d3607b4c22fd18fc1a74d35c72c62fb6700f4b6242)
pub fn clone_mesh_geometry(source: &MeshGeometry) -> MeshGeometry {
    let mut vertices: Vec<f32> = vec![0.0_f32; (source.vertices.len() as f64) as usize];
    {
        let __flight_offset = (0.0_f64) as usize;
        let __flight_values: Vec<f32> = ((source.vertices).clone())
            .iter()
            .map(|value| (*value) as f32)
            .collect();
        vertices[__flight_offset..__flight_offset + __flight_values.len()]
            .copy_from_slice(&__flight_values);
    };
    let mut indices: Option<Vec<u32>> = None;
    if ((source.indices).clone()).is_some() {
        if ((source.indices).clone()).is_some() {
            indices = Some(vec![
                0_u32;
                (source.indices.as_ref().unwrap().len() as f64) as usize
            ]);
        } else {
            indices = Some(vec![
                0_u32;
                (source.indices.as_ref().unwrap().len() as f64) as usize
            ]);
        }
        {
            let __flight_offset = (0.0_f64) as usize;
            let __flight_values: Vec<u32> = ((source.indices).clone())
                .iter()
                .map(|value| (*value) as u32)
                .collect();
            indices.as_mut().unwrap()[__flight_offset..__flight_offset + __flight_values.len()]
                .copy_from_slice(&__flight_values);
        };
    }
    let mut subsets: Vec<MeshSubset> = vec![];
    {
        let mut i = 0.0_f64;
        while (i < (source.subsets.len() as f64)) {
            subsets.push(MeshSubset {
                __flight_identity: std::sync::Arc::new(()),
                index_count: source.subsets[i as usize].index_count,
                index_offset: source.subsets[i as usize].index_offset,
            });
            {
                i += 1.0;
                i
            };
        }
    }
    let mut bounds: Option<crate::OpaqueHostValue> = None;
    if ((source.bounds).clone()).is_some() {
        let b = (source.bounds).clone();
        bounds = Some(create_aabb(
            Some(b.as_ref().unwrap().min.x),
            Some(b.as_ref().unwrap().min.y),
            Some(b.as_ref().unwrap().min.z),
            Some(b.as_ref().unwrap().max.x),
            Some(b.as_ref().unwrap().max.y),
            Some(b.as_ref().unwrap().max.z),
        ));
    }
    return create_mesh_geometry_runtime(&MeshGeometry {
        __flight_identity: std::sync::Arc::new(()),
        __flight_entity_runtime: Default::default(),
        bounds: bounds,
        indices: (indices).clone(),
        layout: (source.layout).clone(),
        subsets: (subsets).clone(),
        topology: (source.topology).clone(),
        version: 0.0_f64,
        vertices: (vertices).clone(),
    });
}

// Source: upstream/packages/mesh/src/meshGeometry.ts:71 (sha256:49af2d17be599e23dd66013d3affb8841e113bcbf180de7898bf9c8e24cb9031)
pub fn create_mesh_geometry(options: &mut MeshGeometryOptions) -> MeshGeometry {
    let vertex_count = get_vertex_count_from_layout(&options.vertices, &options.layout);
    let mut indices: Option<Vec<u32>> = None;
    if ((options.indices).clone()).is_some() {
        indices = Some(promote_indices(
            options.indices.as_ref().unwrap(),
            vertex_count,
        ));
    }
    let mut subsets = (options.subsets).clone();
    if (subsets).is_none() {
        let count = if (indices).is_some() {
            (indices.as_mut().unwrap().len() as f64)
        } else {
            vertex_count
        };
        subsets = Some(vec![MeshSubset {
            __flight_identity: std::sync::Arc::new(()),
            index_count: count,
            index_offset: 0.0_f64,
        }]);
    }
    return create_mesh_geometry_runtime(&MeshGeometry {
        __flight_identity: std::sync::Arc::new(()),
        __flight_entity_runtime: Default::default(),
        bounds: None,
        indices: (indices).clone(),
        layout: (options.layout).clone(),
        subsets: (subsets).clone().unwrap(),
        topology: ((options.topology).clone()).unwrap_or("triangle-list".to_owned()),
        version: 0.0_f64,
        vertices: (options.vertices).clone(),
    });
}

// Source: upstream/packages/mesh/src/meshGeometry.ts:101 (sha256:df52879100a3365cd866c3ae4f8eb5a58427625c2cfaa55962d74556faedfcbf)
pub fn destroy_mesh_geometry_gl_data(geometry: &mut MeshGeometry) -> () {
    let mut runtime = Some(
        ({
            let __flight_runtime = flighthq_types::FlightEntity::__flight_entity_runtime(geometry)
                .lock()
                .unwrap()
                .clone()
                .expect("entity runtime was read before initialization");
            __flight_runtime
        })
        .clone(),
    );
    if (runtime).is_some() {
        {
            let __flight_runtime = runtime.as_mut().unwrap();
            let __flight_value = None;
            let mut __flight_storage = __flight_runtime.inner.lock().unwrap();
            __flight_storage.webgl_data = __flight_value;
        };
    }
}

// Source: upstream/packages/mesh/src/meshGeometry.ts:110 (sha256:5839cafe51a3d9980ae3086d406171b7906afe0b5bcb4c24cf82b5858bde56af)
pub fn destroy_mesh_geometry_wgpu_data(geometry: &mut MeshGeometry) -> () {
    let mut runtime = Some(
        ({
            let __flight_runtime = flighthq_types::FlightEntity::__flight_entity_runtime(geometry)
                .lock()
                .unwrap()
                .clone()
                .expect("entity runtime was read before initialization");
            __flight_runtime
        })
        .clone(),
    );
    if (runtime).is_some() {
        {
            let __flight_runtime = runtime.as_mut().unwrap();
            let __flight_value = None;
            let mut __flight_storage = __flight_runtime.inner.lock().unwrap();
            __flight_storage.webgpu_data = __flight_value;
        };
    }
}

// Source: upstream/packages/mesh/src/meshGeometry.ts:116 (sha256:a3a5eec956d2c63f4cfde0ce7c819c173c4c9bab902738bf341882bf91e22c5a)
pub fn get_mesh_geometry_index_count(geometry: &MeshGeometry) -> f64 {
    return if ((geometry.indices).clone()).is_some() {
        (geometry.indices.as_ref().unwrap().len() as f64)
    } else {
        0.0_f64
    };
}

// Source: upstream/packages/mesh/src/meshGeometry.ts:123 (sha256:7788618d4d7ce2cbcd71f0b067bfde1204973a5d39ed757445f73baa86e88a20)
pub fn get_mesh_geometry_morph_bind_pose(geometry: &MeshGeometry) -> Option<MeshMorphBindPose> {
    let runtime = Some(
        ({
            let __flight_runtime = flighthq_types::FlightEntity::__flight_entity_runtime(geometry)
                .lock()
                .unwrap()
                .clone()
                .expect("entity runtime was read before initialization");
            __flight_runtime
        })
        .clone(),
    );
    return if (runtime).is_some() {
        (runtime
            .as_ref()
            .unwrap()
            .inner
            .lock()
            .unwrap()
            .morph_bind_pose)
            .clone()
    } else {
        None
    };
}

// Source: upstream/packages/mesh/src/meshGeometry.ts:131 (sha256:27c9edc09b30f1e6ad990096a17f607d49cb71aa501adbe0c2b18cba8401978b)
pub fn get_mesh_geometry_skin_bind_pose(geometry: &MeshGeometry) -> Option<MeshSkinBindPose> {
    let runtime = Some(
        ({
            let __flight_runtime = flighthq_types::FlightEntity::__flight_entity_runtime(geometry)
                .lock()
                .unwrap()
                .clone()
                .expect("entity runtime was read before initialization");
            __flight_runtime
        })
        .clone(),
    );
    return if (runtime).is_some() {
        (runtime
            .as_ref()
            .unwrap()
            .inner
            .lock()
            .unwrap()
            .skin_bind_pose)
            .clone()
    } else {
        None
    };
}

// Source: upstream/packages/mesh/src/meshGeometry.ts:138 (sha256:ffe3fac4bc5a3603dd6357d7a30c86e26bd34457995f561bd599203723fef882)
pub fn get_mesh_geometry_vertex_count(geometry: &MeshGeometry) -> f64 {
    return get_vertex_count_from_layout(&geometry.vertices, &geometry.layout);
}

// Source: upstream/packages/mesh/src/meshGeometry.ts:145 (sha256:1626a6571d4b856c79e7f5eec0abe70c5af2c10a726fd19ff24cadd0d3a6cbaf)
pub fn has_mesh_geometry_skin(geometry: &MeshGeometry) -> bool {
    {
        let mut i = 0.0_f64;
        while (i < (geometry.layout.attributes.len() as f64)) {
            if ((geometry.layout.attributes[i as usize].semantic).clone() == "joints0") {
                return true;
            }
            {
                i += 1.0;
                i
            };
        }
    }
    return false;
}

// Source: upstream/packages/mesh/src/meshGeometry.ts:156 (sha256:6170796cb91e167eed96931c6a466ebbf696c77e4d397232050a67b67795ef7d)
pub fn set_mesh_geometry_morph_bind_pose(
    geometry: &mut MeshGeometry,
    bind_pose: Option<MeshMorphBindPose>,
) -> () {
    let mut runtime = Some(
        ({
            let __flight_runtime = flighthq_types::FlightEntity::__flight_entity_runtime(geometry)
                .lock()
                .unwrap()
                .clone()
                .expect("entity runtime was read before initialization");
            __flight_runtime
        })
        .clone(),
    );
    if (runtime).is_some() {
        {
            let __flight_runtime = runtime.as_mut().unwrap();
            let __flight_value = (bind_pose).clone();
            let mut __flight_storage = __flight_runtime.inner.lock().unwrap();
            __flight_storage.morph_bind_pose = __flight_value;
        };
    }
}

// Source: upstream/packages/mesh/src/meshGeometry.ts:167 (sha256:8240cec1378dd45d4cafd84f587598c667baf88c0414ec379018d52e59bf2a1f)
pub fn set_mesh_geometry_skin_bind_pose(
    geometry: &mut MeshGeometry,
    bind_pose: Option<MeshSkinBindPose>,
) -> () {
    let mut runtime = Some(
        ({
            let __flight_runtime = flighthq_types::FlightEntity::__flight_entity_runtime(geometry)
                .lock()
                .unwrap()
                .clone()
                .expect("entity runtime was read before initialization");
            __flight_runtime
        })
        .clone(),
    );
    if (runtime).is_some() {
        {
            let __flight_runtime = runtime.as_mut().unwrap();
            let __flight_value = (bind_pose).clone();
            let mut __flight_storage = __flight_runtime.inner.lock().unwrap();
            __flight_storage.skin_bind_pose = __flight_value;
        };
    }
}

// Source: upstream/packages/mesh/src/meshGeometry.ts:174 (sha256:a879b4a36c59bde4ca19e59310681ed22ecf7e0876e3bdaf6da6af355aaeb14d)
fn create_mesh_geometry_runtime(fields: &MeshGeometry) -> MeshGeometry {
    let mut geometry = create_entity(Some(MeshGeometry {
        __flight_identity: std::sync::Arc::new(()),
        __flight_entity_runtime: Default::default(),
        bounds: (fields.bounds).clone(),
        indices: (fields.indices).clone(),
        layout: (fields.layout).clone(),
        subsets: (fields.subsets).clone(),
        topology: (fields.topology).clone(),
        version: fields.version,
        vertices: (fields.vertices).clone(),
    }));
    let runtime: MeshGeometryRuntime = {
        let __flight_runtime = flighthq_types::EntityRuntime::default();
        {
            __flight_runtime.inner.lock().unwrap().binding = None;
        }
        {
            __flight_runtime.inner.lock().unwrap().morph_bind_pose = None;
        }
        {
            __flight_runtime.inner.lock().unwrap().skin_bind_pose = None;
        }
        {
            __flight_runtime.inner.lock().unwrap().webgl_data = None;
        }
        {
            __flight_runtime.inner.lock().unwrap().webgpu_data = None;
        }
        __flight_runtime
    };
    *flighthq_types::FlightEntity::__flight_entity_runtime(&(geometry))
        .lock()
        .unwrap() = Some((runtime).clone());
    return geometry;
}

// Source: upstream/packages/mesh/src/meshGeometry.ts:195 (sha256:8c642d670d8f2a4d133ccfe66b6c05a42bf325663e1e11d567b8f6865327a697)
fn get_vertex_count_from_layout(vertices: &Vec<f32>, layout: &VertexAttributeLayout) -> f64 {
    let floats_per_vertex = (layout.stride / 4.0_f64);
    if (floats_per_vertex <= 0.0_f64) {
        return 0.0_f64;
    }
    return ((vertices.len() as f64) / floats_per_vertex).floor();
}

// Source: upstream/packages/mesh/src/meshGeometry.ts:206 (sha256:3a35231a9da89f55792d7b4cb82ff1db0f9883cca85391b75edf07f106a64c18)
fn promote_indices(source: &Vec<u32>, vertex_count: f64) -> Vec<u32> {
    if (vertex_count > UINT16_INDEX_CEILING) || (true) {
        let mut out: Vec<u32> = vec![0_u32; (source.len() as f64) as usize];
        {
            let __flight_offset = (0.0_f64) as usize;
            let __flight_values: Vec<u32> = (source).iter().map(|value| (*value) as u32).collect();
            out[__flight_offset..__flight_offset + __flight_values.len()]
                .copy_from_slice(&__flight_values);
        };
        return out;
    }
    let mut out: Vec<u16> = vec![0_u16; (source.len() as f64) as usize];
    {
        let __flight_offset = (0.0_f64) as usize;
        let __flight_values: Vec<u16> = (source).iter().map(|value| (*value) as u16).collect();
        out[__flight_offset..__flight_offset + __flight_values.len()]
            .copy_from_slice(&__flight_values);
    };
    return out;
}

// Source: upstream/packages/mesh/src/meshGeometry.ts:220 (sha256:9b733a20b1617d87fbd91ece36aa50d6080f1bc799022c1f7a79d98c1f71628c)
const UINT16_INDEX_CEILING: f64 = 65535.0_f64;
