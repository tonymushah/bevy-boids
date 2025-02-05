use bevy::prelude::*;
use camera::MainCameraPluginGroup;

pub mod camera;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(MainCameraPluginGroup)
        .run();
}
