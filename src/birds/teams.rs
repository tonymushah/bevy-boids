use bevy::prelude::*;

#[derive(Debug, Component, Deref, DerefMut, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Team(pub String);

#[derive(Debug, Resource, Deref, DerefMut, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Teaming(pub bool);

fn toggle_teaming(mut teaming: ResMut<Teaming>) {
    **teaming = !**teaming;
}

fn toggle_teaming_condition(keys: Res<ButtonInput<KeyCode>>) -> bool {
    keys.just_pressed(KeyCode::KeyT)
}

#[derive(Debug, Component, Clone, Default)]
struct TeamText;

fn add_ui(mut commands: Commands) {
    commands.spawn_scene(bsn! {
        #Theme_text
        Text("Teaming actifivated. Use `T` to deactifivate.")
        UiTransform {
            translation: Val2 { y: Val::Px(20.0) }
        }
        TeamText
    });
}

fn update_teaming_text(
    teaming: Res<Teaming>,
    mut text_query: Query<&mut Text, With<TeamText>>,
) -> Result<(), BevyError> {
    let mut text = text_query.single_mut()?;
    if teaming.0 {
        text.0 = String::from("Teaming actifivated. Use `T` to deactifivate.");
    } else {
        text.0 = String::from("Teaming deactifivated. Use `T` to actifivate.");
    }
    Ok(())
}

pub struct ToggleTeamingPlugin;

impl Plugin for ToggleTeamingPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Teaming(true))
            .add_systems(Startup, add_ui)
            .add_systems(
                Update,
                (toggle_teaming, update_teaming_text)
                    .chain()
                    .run_if(toggle_teaming_condition),
            );
    }
}
