use bevy::prelude::*;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(50.0, 50.0, 50.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::Srgba(Srgba::new(0.8, 0.34, 0.61, 0.2)),
            alpha_mode: AlphaMode::Add,
            ..Default::default()
        })),
    ));

    commands.spawn(PointLight {
        color: Color::Srgba(Srgba::hex("#f0fdf5").unwrap()),
        range: 30.0,
        radius: 10.0,
        intensity: 5_000_000f32,
        shadows_enabled: true,
        ..Default::default()
    });
}

pub struct EnvironmentPlugin;

impl Plugin for EnvironmentPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}
