use bevy::prelude::*;
use camera::MainCameraPluginGroup;
use env::EnvironmentPlugin;

pub mod camera;
pub mod env;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(MainCameraPluginGroup)
        .add_plugins(EnvironmentPlugin)
        .run();
}
