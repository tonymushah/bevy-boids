use bevy::prelude::*;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(20.0, 20.0, 20.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::Srgba(Srgba::hex("#d8fead10").unwrap()),
            ..Default::default()
        })),
    ));
    commands.spawn((
        PointLight {
            color: Color::Srgba(Srgba::hex("#f0fdf5").unwrap()),
            range: 30.0,
            ..Default::default()
        },
        Transform::from_translation(Vec3::new(7.0, 7.0, -5.0)),
    ));
}

pub struct EnvironmentPlugin;

impl Plugin for EnvironmentPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}
