use std::time::Duration;

use bevy::{color::palettes::css::PURPLE, prelude::*};
use rand::{rng, Rng};

use crate::{velocity::Velocity, vision_radius::VisionRadius};

use super::Bird;

const SIZE: usize = 2;

#[derive(Debug, Resource, Clone, Copy, DerefMut, Deref)]
pub struct ShowCohesionForceGizmo(pub bool);

#[derive(Debug, Resource, Clone, Copy, DerefMut, Deref)]
pub struct IsCohersionActive(pub bool);

#[derive(Debug, Resource, Deref, DerefMut)]
pub struct CohesionTimer(pub Timer);

#[allow(clippy::type_complexity)]
pub fn cohesion(
    mut birds: Query<(&mut Velocity, &Transform, &VisionRadius, Entity), With<Bird>>,
    time: Res<Time>,
    mut gizmos: Gizmos,
    mut timer: ResMut<CohesionTimer>,
    show_gizmo: Res<ShowCohesionForceGizmo>,
) {
    if timer.tick(time.delta()).just_finished() {
        let mut combinaisons = birds.iter_combinations_mut::<SIZE>();
        while let Some(mut to_cohese) = combinaisons.fetch_next() {
            let to_use = rng().random_range(0..(SIZE as u32)) as usize;
            let center = {
                let points = to_cohese
                    .iter()
                    .map(|(_, t, _, _)| t.translation)
                    .collect::<Vec<_>>();
                points.iter().sum::<Vec3>() / points.len() as f32
            };
            let bird = &mut to_cohese[to_use];
            let center_distance = bird.1.translation.distance(center);

            let min = bird.2.cohesion_radius.0;
            let max = bird.2.cohesion_radius.1;

            if !(min < center_distance && center_distance < max) {
                continue;
            }
            let pos_vec = bird.1.translation - center;

            let center_force =
                (1.0 / rng().random_range::<i32, _>(10..100) as f32) * pos_vec.normalize();

            if **show_gizmo {
                gizmos.arrow(
                    bird.1.translation,
                    bird.1.translation - center_force.normalize(),
                    PURPLE,
                );
            }

            **bird.0 -= center_force;
        }
    }
}

fn toogle_cohesion(mut res: ResMut<IsCohersionActive>) {
    **res = !**res;
}

fn toggle_timer(is_active: Res<IsCohersionActive>, mut timer: ResMut<CohesionTimer>) {
    if **is_active {
        timer.unpause();
    } else {
        timer.pause();
    }
}

fn toogle_cohesion_condition(key: Res<ButtonInput<KeyCode>>) -> bool {
    key.just_pressed(KeyCode::KeyC)
}

pub fn is_cohesion_active(res: Res<IsCohersionActive>) -> bool {
    **res
}

fn toggle_gizmos(mut show_gizmo: ResMut<ShowCohesionForceGizmo>) {
    **show_gizmo = !**show_gizmo;
}

fn toogle_gismo_condition(key: Res<ButtonInput<KeyCode>>) -> bool {
    key.all_just_pressed([KeyCode::F1])
}

pub struct BirdCohesionPlugin;

impl Plugin for BirdCohesionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, cohesion.run_if(is_cohesion_active))
            .add_systems(
                Update,
                (toogle_cohesion, toggle_timer)
                    .chain()
                    .run_if(toogle_cohesion_condition),
            )
            .insert_resource(IsCohersionActive(true))
            .insert_resource(CohesionTimer(Timer::new(
                Duration::from_millis(100 * SIZE as u64),
                TimerMode::Repeating,
            )))
            .insert_resource(ShowCohesionForceGizmo(true))
            .add_systems(Update, toggle_gizmos.run_if(toogle_gismo_condition));
    }
}
