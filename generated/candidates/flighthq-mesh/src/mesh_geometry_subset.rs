// @generated from upstream/packages/mesh/src/meshGeometrySubset.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::{MeshGeometry, MeshSubset};

// Source: upstream/packages/mesh/src/meshGeometrySubset.ts:12 (sha256:2e7019c91ea38a42e0a899a3e25026be4b9edd456e7343cce828a2e887c94b6f)
pub fn add_mesh_geometry_subset(geometry: &mut MeshGeometry, subset: &MeshSubset) -> () {
    let mut next: Vec<MeshSubset> = vec![];
    {
        let mut i = 0.0_f64;
        while (i < (geometry.subsets.len() as f64)) {
            next.push(MeshSubset {
                __flight_identity: std::sync::Arc::new(()),
                index_count: geometry.subsets[i as usize].index_count,
                index_offset: geometry.subsets[i as usize].index_offset,
            });
            {
                i += 1.0;
                i
            };
        }
    }
    next.push(MeshSubset {
        __flight_identity: std::sync::Arc::new(()),
        index_count: subset.index_count,
        index_offset: subset.index_offset,
    });
    geometry.subsets = (next).clone();
}

// Source: upstream/packages/mesh/src/meshGeometrySubset.ts:24 (sha256:dfb5f55502c9216b5ed03809547d2bd2edf116e9cecdde9f39600b9fd8b1a188)
pub fn get_mesh_geometry_subset_triangle_count(geometry: &MeshGeometry, subset_index: f64) -> f64 {
    if (subset_index < 0.0_f64) || (subset_index >= (geometry.subsets.len() as f64)) {
        return 0.0_f64;
    }
    let index_count = geometry.subsets[subset_index as usize].index_count;
    if ((geometry.topology).clone() == "triangle-list") {
        return (index_count / 3.0_f64).floor();
    }
    if ((geometry.topology).clone() == "triangle-strip") {
        return if (index_count >= 2.0_f64) {
            (index_count - 2.0_f64)
        } else {
            0.0_f64
        };
    }
    return 0.0_f64;
}

// Source: upstream/packages/mesh/src/meshGeometrySubset.ts:35 (sha256:a1407c45db7136a001f35bf0db83d8ca70f9e56db481bb64b4d6476439c80334)
pub fn set_mesh_geometry_subsets(geometry: &mut MeshGeometry, subsets: &Vec<MeshSubset>) -> () {
    let mut next: Vec<MeshSubset> = vec![];
    {
        let mut i = 0.0_f64;
        while (i < (subsets.len() as f64)) {
            next.push(MeshSubset {
                __flight_identity: std::sync::Arc::new(()),
                index_count: subsets[i as usize].index_count,
                index_offset: subsets[i as usize].index_offset,
            });
            {
                i += 1.0;
                i
            };
        }
    }
    geometry.subsets = (next).clone();
}
