use bevy::prelude::*;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(30.0, 30.0, 30.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::Srgba(Srgba::new(0.8, 0.34, 0.61, 0.2)),
            alpha_mode: AlphaMode::AlphaToCoverage,
            ..Default::default()
        })),
    ));

    commands.spawn((
        Mesh3d(meshes.add(Plane3d::new(Vec3::Y, Vec2::new(10.0, 10.0)))),
        MeshMaterial3d(materials.add(StandardMaterial::default())),
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
