use bevy::prelude::*;
use bevy_transform_interpolation::prelude::*;
use gizmos::ShowBirdsGizmoPlugin;
use kd_tree::BirdsKdTreePlugin;
use rand::{RngExt, rng};

// use alignment::BirdAlignmentPlugin;
use birds_number::BirdNumberTextPlugin;
use cohesion::BirdCohesionPlugin;
use separation::BirdSeparationPlugin;
use shi_bird_shadows::ShinyBirdShadowPlugin;
use teams::{Team, ToggleTeamingPlugin};

use crate::{
    birds::{
        birds_cache::BirdsCachePlugin,
        interactions::BirdInteractionsPlugin,
        mesh_material_registry::{BirdMaterialEntry, BirdMaterials, BirdMeshes},
    },
    utils::random_translation_uniform,
    velocity::{MaxSpeed, ShowVelocityVector, Velocity, is_paused},
    vision_radius::{CohesionRadius, VisionRadius},
};

pub mod alignment;
pub mod birds_cache;
pub mod birds_number;
pub mod cohesion;
pub mod cube_bound;
mod gizmos;
pub mod interactions;
pub mod kd_tree;
pub mod look_to;
pub mod mesh_material_registry;
pub mod random_vel;
pub mod separation;
pub mod shape;
pub mod shi_bird_shadows;
pub mod teams;

pub use gizmos::ShowBirdsGizmo;

fn default_vision_radius() -> VisionRadius {
    VisionRadius {
        min_distance: 2.0,
        neighboor_radius: 25.0,
        cohesion_radius: CohesionRadius(3.0, 30.0),
    }
}

#[derive(Component)]
#[require(Mesh3d, Velocity, VisionRadius{..default_vision_radius()})]
pub struct Bird;

fn spawns(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut bird_base_mesh: ResMut<BirdMeshes>,
    mut bird_materials: ResMut<BirdMaterials>,
) {
    let mut rng = rng();
    for _ in 1..rng.random_range(2..20) {
        let has_light = rng.random_bool(1.0 / 5.0);
        let metallic = rng.random_bool(1.0 / 4.0);
        let _bird = commands.spawn((
            Bird,
            Transform::from_translation(random_translation_uniform(&mut rng, -15.0..15.0)),
            Mesh3d(bird_base_mesh.get_bird_base_mesh(&mut meshes)),
            MeshMaterial3d(bird_materials.get_material(
                &mut materials,
                BirdMaterialEntry {
                    base_color: rng.random(),
                    light_color: rng.random(),
                    has_light,
                    metallic,
                },
            )),
            Velocity(random_translation_uniform(&mut rng, -4.0..5.0)),
            ShowVelocityVector,
            MaxSpeed(rng.random_range(6.0..=12.0)),
            Team(format!("{}", rng.random_range::<u8, _>(0..15))),
            TransformInterpolation,
        ));
    }
}

fn spawn_by_key_condition(keys: Res<ButtonInput<KeyCode>>) -> bool {
    keys.just_pressed(KeyCode::Space)
}

fn despawn_all(mut commands: Commands, birds: Query<Entity, With<Bird>>) {
    for bird in &birds {
        commands.entity(bird).despawn();
    }
}

fn despawn_all_by_key_condition(keys: Res<ButtonInput<KeyCode>>) -> bool {
    keys.just_pressed(KeyCode::F5)
}

pub struct BirdsPlugin;

impl Plugin for BirdsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<BirdMeshes>()
            .init_resource::<BirdMaterials>()
            .add_systems(Startup, spawns)
            .add_systems(FixedPostUpdate, look_to::look_to.run_if(not(is_paused)))
            .add_plugins(random_vel::BirdsRandomVelPlugin)
            .add_plugins(cube_bound::BirdCubeBoundPlugin(Cuboid::new(
                49.0, 49.0, 49.0,
            )))
            .add_systems(
                Update,
                spawns.run_if(spawn_by_key_condition.and_then(not(is_paused))),
            )
            .add_systems(Update, despawn_all.run_if(despawn_all_by_key_condition))
            .add_plugins(BirdSeparationPlugin)
            // .add_plugins(BirdAlignmentPlugin)
            .add_plugins(BirdCohesionPlugin)
            .add_plugins(BirdNumberTextPlugin)
            .add_plugins(BirdsKdTreePlugin)
            .add_plugins(ShowBirdsGizmoPlugin)
            .add_plugins(ShinyBirdShadowPlugin)
            .add_plugins(ToggleTeamingPlugin)
            .add_plugins(BirdsCachePlugin)
            .add_plugins(BirdInteractionsPlugin);
    }
}
