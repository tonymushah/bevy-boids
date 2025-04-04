use bevy::prelude::*;

pub fn bird_meshes() -> Mesh {
    let mut tetra = Tetrahedron::default();
    tetra
        .vertices
        .iter_mut()
        .for_each(|vertice| *vertice *= 0.75_f32);
    tetra.into()
}
