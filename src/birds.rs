use bevy::prelude::*;

use crate::velocity::{ShowVelocityVector, Velocity};

pub mod look_to;
pub mod random_vel;
pub mod shape;

#[derive(Component)]
#[require(Mesh3d, Velocity)]
pub struct Bird;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Bird,
        Transform::from_translation(Vec3::new(0.0, 1.0, 0.0)),
        Mesh3d(meshes.add(shape::bird_meshes())),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::Srgba(Srgba::hex("#ffa0d1").unwrap()),
            ..Default::default()
        })),
        Velocity(Vec3::Z * 1.25),
        ShowVelocityVector,
    ));
}

pub struct BirdsPlugin;

impl Plugin for BirdsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, look_to::look_to)
            .add_plugins(random_vel::BirdsRandomVelPlugin);
    }
}
