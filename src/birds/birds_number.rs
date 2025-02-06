use bevy::prelude::*;

use super::Bird;

#[derive(Component)]
pub struct BirdNumberText;

#[derive(Resource, DerefMut, Deref)]
pub struct BirdNumber(pub usize);

fn update_bird_number(mut number: ResMut<BirdNumber>, query: Query<(), With<Bird>>) {
    let num_ = query.iter().count();
    if number.0 != num_ {
        **number = num_;
    }
}

fn setup(mut commands: Commands) {
    commands.spawn((Text::new("Number of birds: 0"), BirdNumberText));
}

fn update_text(number: Res<BirdNumber>, mut texts: Query<&mut Text, With<BirdNumberText>>) {
    for mut text in &mut texts {
        **text = format!("Number of birds : {}", **number);
    }
}

fn update_text_condition(number: Res<BirdNumber>) -> bool {
    number.is_changed() && !number.is_added()
}

pub struct BirdNumberTextPlugin;

impl Plugin for BirdNumberTextPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(BirdNumber(0))
            .add_systems(Update, update_text.run_if(update_text_condition))
            .add_systems(Update, update_bird_number)
            .add_systems(Startup, setup);
    }
}
