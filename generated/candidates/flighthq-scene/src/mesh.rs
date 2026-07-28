// @generated from upstream/packages/scene/src/mesh.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{create_scene_node, get_scene_node_runtime};
use flighthq_node::{
    enable_node_signals, get_node_local_matrix4, get_node_signals, is_node_local_matrix4_detached,
    set_node_local_matrix4, set_node_transform3_d,
};
use flighthq_types::{
    Kind, MESH_DEFORMER_MORPH as mesh_deformer_morph_constant,
    MESH_DEFORMER_NONE as mesh_deformer_none_constant,
    MESH_DEFORMER_SKELETAL as mesh_deformer_skeletal_constant, Material, MeshDeformer,
    MeshGeometry, NodeSignals,
};
pub use flighthq_types::{MESH_KIND, Mesh, MeshRuntime};

// Source: upstream/packages/scene/src/mesh.ts:27 (sha256:277fcef755c1a9cd8ec6c38994524888dc06d449d304767d88375391d4aa9076)
pub fn clone_mesh(source: &Mesh) -> Mesh {
    let mut clone = create_mesh(
        &source.geometry,
        &((source.materials).clone()).clone(),
        Some(((source.kind).clone()).clone()),
        Some(Mesh {
            __flight_identity: std::sync::Arc::new(()),
            enabled: source.enabled,
            name: (source.name).clone(),
            morph: None,
            skin: None,
        }),
    );
    clone.alpha = source.alpha;
    set_node_transform3_d(&mut clone, source);
    if is_node_local_matrix4_detached(source) {
        set_node_local_matrix4(&clone, &get_node_local_matrix4(source));
    }
    if ((source.skin).clone()).is_some() {
        clone.skin = (source.skin).clone();
    }
    if ((source.morph).clone()).is_some() {
        clone.morph = (source.morph).clone();
    }
    return (clone).clone();
}

// Source: upstream/packages/scene/src/mesh.ts:46 (sha256:4fbd8df439f616ec0665150fd447de8d79e1ff6b28bcfd083df5d9954a20f3bc)
pub fn create_mesh(
    geometry: &MeshGeometry,
    materials: &Vec<Option<Material>>,
    kind: Option<Kind>,
    obj: Option<Mesh>,
) -> Mesh {
    let kind = kind.unwrap_or(MESH_KIND);
    let mut mesh = create_scene_node(
        Some(((kind).clone()).clone()),
        Some(((obj).clone().unwrap()).clone()),
    );
    mesh.geometry = (*geometry).clone();
    mesh.materials = (*materials).clone();
    return (mesh).clone();
}

// Source: upstream/packages/scene/src/mesh.ts:58 (sha256:3075d4aa4f8277ac28e3a3c15c21c3b9e4f7403ec139088ca5b0afc9a5e0e2b5)
pub fn enable_mesh_signals(source: &Mesh) -> NodeSignals {
    return enable_node_signals(source);
}

// Source: upstream/packages/scene/src/mesh.ts:68 (sha256:1c56f3460bc3d04c757d8c00e42698868b33fdeda60b5ecb831ce35c206de8e1)
pub fn get_mesh_deformer(source: &Mesh) -> MeshDeformer {
    if ((source.skin).clone()).is_some() {
        return mesh_deformer_skeletal_constant;
    }
    if ((source.morph).clone()).is_some() {
        return mesh_deformer_morph_constant;
    }
    return mesh_deformer_none_constant;
}

// Source: upstream/packages/scene/src/mesh.ts:74 (sha256:c7679192eae905074e4e69117bcd5251d816c92261d0ce8146a129a121728f1c)
pub fn get_mesh_runtime(source: &Mesh) -> MeshRuntime {
    return get_scene_node_runtime(source);
}

// Source: upstream/packages/scene/src/mesh.ts:78 (sha256:466ba13ecedfaf8075dc2fe6b2531259b77deb132787565db2b95555df9410c9)
pub fn get_mesh_signals(source: &Mesh) -> Option<NodeSignals> {
    return get_node_signals(source);
}

// Source: upstream/packages/scene/src/mesh.ts:85 (sha256:408835db9e498dd60caad0fe098ac5f58f6578b0f0e1801afb47505911eb1317)
pub fn is_mesh(source: crate::OpaqueHostValue) -> bool {
    return ((source.geometry).clone()).is_some();
}
