use bevy::{
    color::palettes::css::{GREEN, PURPLE},
    ecs::system::SystemParam,
    prelude::*,
};
use kiddo::dist::SquaredEuclidean;

use crate::{
    birds::{
        Bird, ShowBirdsGizmo,
        birds_cache::BirdsCache,
        cohesion::IsCohersionActive,
        kd_tree::BirdsKdTree,
        teams::{Team, Teaming},
    },
    velocity::{Velocity, is_paused},
    vision_radius::VisionRadius,
};

#[derive(SystemParam)]
pub struct InteractionParam<'w, 's> {
    pub birds: Query<
        'w,
        's,
        (
            &'static mut Velocity,
            &'static Transform,
            &'static VisionRadius,
            Entity,
            &'static Team,
        ),
        With<Bird>,
    >,
    // pub time: Res<'w, Time>,
    pub gizmos: Gizmos<'w, 's>,
    // pub timer: ResMut<'w, CohesionTimer>,
    pub kd_tree: Res<'w, BirdsKdTree>,
    // bird_number: Res<BirdNumber>,
    pub show_gizmos: Res<'w, ShowBirdsGizmo>,
    pub teaming: Res<'w, Teaming>,
    pub bird_cache_data: Res<'w, BirdsCache>,
    pub is_cohesion_active: Res<'w, IsCohersionActive>,
}

struct NeighborEntry<'a> {
    velocity: &'a Velocity,
    transform: &'a Transform,
    team: &'a Team,
    distance: f32,
    entity: Entity,
}

pub fn interactions_system(param: InteractionParam) {
    let InteractionParam {
        mut birds,
        mut gizmos,
        kd_tree,
        show_gizmos,
        teaming,
        bird_cache_data,
        is_cohesion_active,
    } = param;
    for mut bird in &mut birds {
        let neigbors = kd_tree
            .query(&bird.1.translation.to_array())
            // .nearest_n(
            //     NonZeroUsize::new(rng().random_range::<u64, _>(3..20) as usize).ok_or_else(
            //         || {
            //             BevyError::warning(anyhow::format_err!(
            //                 "Cannot transform usize to NonZero variant"
            //             ))
            //         },
            //     )?,
            // )
            .within::<SquaredEuclidean<_>>(bird.2.cohesion_radius.1)
            .with_points()
            .execute()
            .into_iter()
            .flat_map(|a| {
                let entity = Entity::from_raw_u32(a.item)?;
                let cache_data = bird_cache_data.get(&entity)?;
                Some(NeighborEntry {
                    distance: a.distance,
                    velocity: &cache_data.velocity,
                    team: &cache_data.team,
                    transform: &cache_data.transform,
                    entity,
                })
            })
            .collect::<Box<[_]>>();
        // alignment calculations
        {
            let neighbors = neigbors
                .iter()
                .filter(|near_bird| {
                    if near_bird.entity == bird.3 {
                        false
                    } else if bird.2.min_distance < near_bird.distance
                        && near_bird.distance < bird.2.neighboor_radius
                    {
                        !(teaming.0 && near_bird.team != bird.4)
                    } else {
                        false
                    }
                })
                .collect::<Box<[_]>>();
            if !neighbors.is_empty() {
                let average_vel =
                    neighbors.iter().map(|d| d.velocity.0).sum::<Vec3>() / neighbors.len() as f32;

                if **show_gizmos {
                    gizmos.arrow(
                        bird.1.translation,
                        bird.1.translation + average_vel.normalize(),
                        GREEN,
                    );
                }

                // drop(bird);

                **bird.0 += average_vel.normalize();
            }
        }
        // Cohesion calculations
        if **is_cohesion_active {
            let neigbors = neigbors
                .iter()
                .filter(|near_bird| {
                    if near_bird.entity == bird.3 {
                        false
                    } else if bird.2.cohesion_radius.0 < near_bird.distance
                        && near_bird.distance < bird.2.cohesion_radius.1
                    {
                        !(teaming.0 && near_bird.team != bird.4)
                    } else {
                        false
                    }
                })
                .collect::<Box<[_]>>();

            if !neigbors.is_empty() {
                let center = neigbors
                    .iter()
                    .map(|bird| bird.transform.translation)
                    .sum::<Vec3>()
                    / neigbors.len() as f32;

                let pos_vec = bird.1.translation - center;

                let center_force =
                                /*(rng().random_range::<f32, _>(0.25..=3.0) as f32) */ pos_vec.normalize();

                if **show_gizmos {
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
}

pub struct BirdInteractionsPlugin;

impl Plugin for BirdInteractionsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, interactions_system.run_if(not(is_paused)));
    }
}
