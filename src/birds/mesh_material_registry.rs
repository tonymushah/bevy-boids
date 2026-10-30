use std::collections::HashMap;

use bevy::{
    asset::{Assets, Handle},
    color::{ColorToPacked, LinearRgba, Srgba},
    ecs::resource::Resource,
    mesh::Mesh,
    pbr::StandardMaterial,
};

use crate::birds::{shape::bird_meshes, teams::Team};

#[derive(Debug, Resource, Default)]
pub struct BirdMeshes(Option<Handle<Mesh>>);

impl BirdMeshes {
    pub fn get_bird_base_mesh(&mut self, meshes: &mut Assets<Mesh>) -> Handle<Mesh> {
        self.0.get_or_insert(meshes.add(bird_meshes())).clone()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BirdMaterialEntry {
    pub base_color: [u8; 4],
    pub light_color: [u8; 4],
    pub has_light: bool,
    pub metallic: bool,
}

#[derive(Debug, Resource, Default)]
pub struct BirdMaterials {
    base_materials: HashMap<BirdMaterialEntry, Handle<StandardMaterial>>,
    team_materials: HashMap<Team, BirdMaterialEntry>,
}

impl BirdMaterials {
    pub fn get_material(
        &mut self,
        materials: &mut Assets<StandardMaterial>,
        entry: BirdMaterialEntry,
    ) -> Handle<StandardMaterial> {
        self.base_materials
            .entry(entry)
            .or_insert_with_key(|entry| {
                let bird_color = Srgba::from_u8_array(entry.base_color);
                materials.add(StandardMaterial {
                    base_color: bird_color.into(),
                    emissive_exposure_weight: if entry.has_light { 1.0 } else { 0.0 },
                    emissive: if entry.has_light {
                        LinearRgba::from_u8_array(entry.light_color) * 10000.0
                    } else {
                        LinearRgba::BLACK
                    },
                    reflectance: 0.9,
                    metallic: if entry.metallic { 1.0 } else { 0.0 },
                    ..Default::default()
                })
            })
            .clone()
    }

    pub fn get_team_material(
        &mut self,
        materials: &mut Assets<StandardMaterial>,
        team: Team,
    ) -> Option<Handle<StandardMaterial>> {
        Some(
            self.get_material(materials, self.team_materials.get(&team)?.clone())
                .clone(),
        )
    }

    pub fn set_team_material(
        &mut self,
        materials: &mut Assets<StandardMaterial>,
        team: Team,
        entry: BirdMaterialEntry,
    ) -> Handle<StandardMaterial> {
        self.team_materials.insert(team, entry.clone());
        self.get_material(materials, entry)
    }
}
