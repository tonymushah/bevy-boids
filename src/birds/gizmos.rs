use bevy::prelude::*;

#[derive(Debug, Clone, Copy, Resource, DerefMut, Deref)]
pub struct ShowBirdsGizmo(pub bool);

fn toggle_bird_gizmo(mut should_show: ResMut<ShowBirdsGizmo>) {
    **should_show = !**should_show;
}

fn toggle_bird_gizmo_run_key(keys: Res<ButtonInput<KeyCode>>) -> bool {
    keys.just_pressed(KeyCode::F1)
}

pub struct ShowBirdsGizmoPlugin;

impl Plugin for ShowBirdsGizmoPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ShowBirdsGizmo(true))
            .add_systems(Update, toggle_bird_gizmo.run_if(toggle_bird_gizmo_run_key));
    }
}
