use bevy::prelude::*;

#[derive(Debug, Component, Deref, DerefMut, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Team(pub String);

#[derive(Debug, Resource, Deref, DerefMut, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Teaming(pub bool);

fn toggle_teaming(mut teaming: ResMut<Teaming>) {
    **teaming = !**teaming;
}

fn toggle_teaming_condition(keys: Res<ButtonInput<KeyCode>>) -> bool {
    keys.just_pressed(KeyCode::KeyT)
}

pub struct ToggleTeamingPlugin;

impl Plugin for ToggleTeamingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, toggle_teaming.run_if(toggle_teaming_condition));
    }
}
