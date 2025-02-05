use bevy::prelude::*;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::new(Vec3::Y, Vec2::new(10.0, 10.0)))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::Srgba(Srgba::hex("#d8fead90").unwrap()),
            ..Default::default()
        })),
    ));
    commands.spawn((
        PointLight {
            intensity: 10_000.0,
            color: Color::Srgba(Srgba::hex("#f0fdf5").unwrap()),
            ..Default::default()
        },
        Transform::from_translation(Vec3::new(10.0, 5.0, -5.0)),
    ));
}

pub struct EnvironmentPlugin;

impl Plugin for EnvironmentPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}
