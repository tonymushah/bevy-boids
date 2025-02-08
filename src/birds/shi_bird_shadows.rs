use bevy::prelude::*;

use super::Bird;

#[derive(Debug, Default, Deref, DerefMut, Resource)]
pub struct ShinyBirdShadow(pub bool);

fn toggle_shiny_bird_shadow(mut shadows: ResMut<ShinyBirdShadow>) {
    **shadows = !**shadows;
}

fn toggle_shiny_bird_shadow_condition(key: Res<ButtonInput<KeyCode>>) -> bool {
    key.just_pressed(KeyCode::F3)
}

fn toggle_shadows(mut birds: Query<&mut PointLight, With<Bird>>, shadows: Res<ShinyBirdShadow>) {
    for mut bird in &mut birds {
        bird.shadows_enabled = **shadows;
    }
}

pub struct ShinyBirdShadowPlugin;

impl Plugin for ShinyBirdShadowPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ShinyBirdShadow>()
            .add_systems(Startup, toggle_shadows)
            .add_systems(
                Update,
                (toggle_shiny_bird_shadow, toggle_shadows)
                    .chain()
                    .run_if(toggle_shiny_bird_shadow_condition),
            );
    }
}
