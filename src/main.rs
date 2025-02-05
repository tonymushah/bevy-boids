use bevy::prelude::*;
use birds::BirdsPlugin;
use camera::MainCameraPluginGroup;
use env::EnvironmentPlugin;

pub mod birds;
pub mod camera;
pub mod env;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(MainCameraPluginGroup)
        .add_plugins(EnvironmentPlugin)
        .add_plugins(BirdsPlugin)
        .run();
}
