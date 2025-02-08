use bevy::prelude::*;
use birds_number::BirdNumberTextPlugin;
use gizmos::ShowBirdsGizmoPlugin;
use kd_tree::BirdsKdTreePlugin;
use rand::{rng, Rng};

use alignment::BirdAlignmentPlugin;
use cohesion::BirdCohesionPlugin;
use separation::BirdSeparationPlugin;
use shi_bird_shadows::ShinyBirdShadowPlugin;
use teams::Team;

use crate::{
    utils::random_translation_uniform,
    velocity::{is_paused, MaxSpeed, ShowVelocityVector, Velocity},
    vision_radius::VisionRadius,
};

pub mod alignment;
pub mod birds_number;
pub mod cohesion;
pub mod cube_bound;
mod gizmos;
pub mod kd_tree;
pub mod look_to;
pub mod random_vel;
pub mod separation;
pub mod shape;
pub mod shi_bird_shadows;
pub mod teams;

pub use gizmos::ShowBirdsGizmo;

fn default_vision_radius() -> VisionRadius {
    VisionRadius {
        min_distance: 2.0,
        neighboor_radius: 4.0,
        cohesion_radius: (4.0, 8.0),
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
    let bird_color: Color = LinearRgba::rgb(1.0, 0.5, 0.3).into();

    let mut _rng = rng();
    for _ in 1.._rng.random_range(2..20) {
        let has_light = rng().random_bool(1.0 / 5.0);
        let metalic = rng().random_bool(1.0 / 4.0);
        let light_color: LinearRgba = Srgba::hex("#f0d010").unwrap().into();
        let mut bird = commands.spawn((
            Bird,
            Transform::from_translation(random_translation_uniform(&mut _rng, -15.0..15.0)),
            Mesh3d(meshes.add(shape::bird_meshes())),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: bird_color,
                emissive_exposure_weight: if has_light { 1.0 } else { 0.0 },
                emissive: if has_light {
                    light_color * 10000.0
                } else {
                    LinearRgba::BLACK
                },
                reflectance: 0.9,
                metallic: if metalic { 1.0 } else { 0.0 },
                ..Default::default()
            })),
            Velocity(random_translation_uniform(&mut _rng, -4.0..5.0)),
            ShowVelocityVector,
            MaxSpeed(_rng.random_range(6.0..=12.0)),
            Team(format!("{}", rng().random_range::<u8, _>(0..15))),
        ));
        if has_light {
            bird.insert(PointLight {
                intensity: 10000.0,
                color: light_color.into(),
                shadows_enabled: false,
                ..Default::default()
            });
        }
    }
}

fn spawn_by_key_condition(keys: Res<ButtonInput<KeyCode>>) -> bool {
    keys.just_pressed(KeyCode::Space)
}

fn despawn_all(mut commands: Commands, birds: Query<Entity, With<Bird>>) {
    for bird in &birds {
        commands.entity(bird).despawn_recursive();
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
            .add_plugins(BirdAlignmentPlugin)
            .add_plugins(BirdCohesionPlugin)
            .add_plugins(BirdNumberTextPlugin)
            .add_plugins(BirdsKdTreePlugin)
            .add_plugins(ShowBirdsGizmoPlugin)
            .add_plugins(ShinyBirdShadowPlugin);
    }
}
