use bevy::{
    asset::{Assets, Handle},
    ecs::resource::Resource,
    mesh::Mesh,
};

use crate::birds::shape::bird_meshes;

#[derive(Debug, Resource, Default)]
pub struct BirdMeshes(Option<Handle<Mesh>>);

impl BirdMeshes {
    pub fn get_bird_base_mesh(&mut self, meshes: &mut Assets<Mesh>) -> Handle<Mesh> {
        self.0.get_or_insert(meshes.add(bird_meshes())).clone()
    }
}
