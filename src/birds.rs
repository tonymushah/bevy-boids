use alignment::BirdAlignmentPlugin;
use bevy::prelude::*;
use rand::{rng, Rng};
use separation::BirdSeparationPlugin;

use crate::{
    utils::random_translation_uniform,
    velocity::{is_paused, MaxSpeed, ShowVelocityVector, Velocity},
    vision_radius::VisionRadius,
};

pub mod alignment;
pub mod cube_bound;
pub mod look_to;
pub mod random_vel;
pub mod separation;
pub mod shape;

fn default_vision_radius() -> VisionRadius {
    VisionRadius {
        min_distance: 1.5,
        neighboor_radius: 4.0,
    }
}

#[derive(Component)]
#[require(Mesh3d, Velocity, VisionRadius(default_vision_radius))]
pub struct Bird;

fn spawns(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mut _rng = rng();
    for _ in 1.._rng.random_range(2..20) {
        commands.spawn((
            Bird,
            Transform::from_translation(random_translation_uniform(&mut _rng, -15.0..15.0)),
            Mesh3d(meshes.add(shape::bird_meshes())),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: Color::Srgba(Srgba::hex("#ffa0d1").unwrap()),
                ..Default::default()
            })),
            Velocity(random_translation_uniform(&mut _rng, -4.0..5.0)),
            ShowVelocityVector,
            MaxSpeed(_rng.random_range(6.0..=12.0)),
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
        app.add_systems(Startup, spawns)
            .add_systems(Update, look_to::look_to.run_if(not(is_paused)))
            .add_plugins(random_vel::BirdsRandomVelPlugin)
            .add_plugins(cube_bound::BirdCubeBoundPlugin(Cuboid::new(
                49.0, 49.0, 49.0,
            )))
            .add_systems(
                Update,
                spawns.run_if(spawn_by_key_condition.and(not(is_paused))),
            )
            .add_systems(Update, despawn_all.run_if(despawn_all_by_key_condition))
            .add_plugins(BirdSeparationPlugin)
            .add_plugins(BirdAlignmentPlugin);
    }
}
