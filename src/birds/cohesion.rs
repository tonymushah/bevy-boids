// use std::{
//     // collections::HashSet,
//     // num::NonZeroUsize,
//     // time::Duration,
// };

use bevy::{
    // color::palettes::css::PURPLE, ecs::system::SystemParam,
    prelude::*,
};
// use kiddo::dist::SquaredEuclidean;
// use rand::{RngExt, rng};

// use crate::{
//     birds::{birds_cache::BirdsCache, teams::Teaming},
//     velocity::{Velocity, is_paused},
//     vision_radius::VisionRadius,
// };

// use super::{
//     Bird,
//     ShowBirdsGizmo,
//     // birds_number::BirdNumber,
//     kd_tree::BirdsKdTree,
//     teams::Team,
// };

// const COHESION_TIMER: u64 = 10;

/*
#[derive(Debug, Resource, Clone, Copy, DerefMut, Deref)]
pub struct ShowCohesionForceGizmo(pub bool);
*/

#[derive(Debug, Resource, Clone, Copy, DerefMut, Deref)]
pub struct IsCohersionActive(pub bool);

#[derive(Debug, Resource, Deref, DerefMut)]
pub struct CohesionTimer(pub Timer);

// struct BirdCohesionNeibhorHood<'k> {
//     limit: Option<usize>,
//     target: &'k InnerCoordVec3,
//     team: &'k Team,
//     entries: Vec<BirdsKdTreeEntry>,
//     vision: VisionRadius,
// }

// impl<'k, 'v> Neighborhood<&'k InnerCoordVec3, &'v BirdsKdTreeEntry>
//     for BirdCohesionNeibhorHood<'k>
// {
//     fn target(&self) -> &'k InnerCoordVec3 {
//         self.target
//     }

//     fn contains<D>(&self, distance: D) -> bool
//     where
//         D: PartialOrd<<&'k InnerCoordVec3 as acap::Proximity>::Distance>,
//     {
//         let min = self.vision.cohesion_radius.0;
//         let max = self.vision.cohesion_radius.1;
//         distance > min && distance < max
//     }

//     fn consider(
//         &mut self,
//         item: &'v BirdsKdTreeEntry,
//     ) -> <&'k InnerCoordVec3 as acap::Proximity>::Distance {
//         let distance = self.target.distance(item.coord);

//         if self.contains(distance)
//             && item
//                 .team
//                 .as_ref()
//                 .map(|team| self.team == team)
//                 .unwrap_or(true)
//         {
//             if let Some(limit) = self.limit {
//                 if self.entries.len() <= limit {
//                     self.entries.push(item.clone());
//                 }
//             } else {
//                 self.entries.push(item.clone());
//             }
//         }
//         distance
//     }
// }

// #[derive(SystemParam)]
// pub struct CohesionParam<'w, 's> {
//     pub birds: Query<
//         'w,
//         's,
//         (
//             &'static mut Velocity,
//             &'static Transform,
//             &'static VisionRadius,
//             Entity,
//             &'static Team,
//         ),
//         With<Bird>,
//     >,
//     // pub time: Res<'w, Time>,
//     pub gizmos: Gizmos<'w, 's>,
//     // pub timer: ResMut<'w, CohesionTimer>,
//     pub kd_tree: Res<'w, BirdsKdTree>,
//     // bird_number: Res<BirdNumber>,
//     pub show_gizmo: Res<'w, ShowBirdsGizmo>,
//     pub teaming: Res<'w, Teaming>,
//     pub bird_cache_data: Res<'w, BirdsCache>,
// }

// #[allow(clippy::type_complexity)]
// pub fn cohesion(param: CohesionParam) -> Result<(), BevyError> {
//     let CohesionParam {
//         mut birds,
//         // time,
//         mut gizmos,
//         // mut timer,
//         kd_tree,
//         show_gizmo,
//         teaming,
//         bird_cache_data,
//     } = param;
//     // if timer.tick(time.delta()).just_finished() {
//     // let mut computed: HashSet<Entity> = Default::default();

//     for mut bird in &mut birds {
//         // if !computed.contains(&bird.3) {
//         //     computed.insert(bird.3);
//         // } else {
//         //     continue;
//         // }

//         let center = {
//             let points = { get_neighbor(&kd_tree, &teaming, &bird_cache_data, &bird)? };
//             // kd_tree
//             //     .search(BirdCohesionNeibhorHood {
//             //         limit: Some(),
//             //         target: &bird_trans,
//             //         entries: Default::default(),
//             //         vision: *bird.2,
//             //         team: bird.4,
//             //     })
//             //     .entries
//             //     .iter()
//             //     .map(|entry| entry.coord)
//             //     .collect::<Vec<_>>();
//             if !points.is_empty() {
//                 points.iter().sum::<Vec3>() / points.len() as f32
//             } else {
//                 Vec3::ZERO
//             }
//         };

//         if center == Vec3::ZERO {
//             continue;
//         }

//         let pos_vec = bird.1.translation - center;

//         let center_force =
//                 /*(rng().random_range::<f32, _>(0.25..=3.0) as f32) */ pos_vec.normalize();

//         if **show_gizmo {
//             gizmos.arrow(
//                 bird.1.translation,
//                 bird.1.translation - center_force.normalize(),
//                 PURPLE,
//             );
//         }

//         **bird.0 -= center_force;
//     }
//     // This is old code for reference
//     /*
//     while let Some(mut to_cohese) = combinaisons.fetch_next() {
//         let to_use = rng().random_range(0..(SIZE as u64)) as usize;

//         let center = {
//             let points = to_cohese
//                 .iter()
//                 .filter(|b| {
//                     let bird = &to_cohese[to_use];
//                     let next_one = bird.1.translation;
//                     let next_two = b.1.translation;

//                     let min = bird.2.cohesion_radius.0;
//                     let max = bird.2.cohesion_radius.1;

//                     let distance = next_one.distance(next_two);

//                     min < distance && distance < max
//                 })
//                 .map(|(_, t, _, _)| t.translation)
//                 .collect::<Vec<_>>();
//             points.iter().sum::<Vec3>() / points.len() as f32
//         };
//         let bird = &mut to_cohese[to_use];

//         /*
//         let center_distance = bird.1.translation.distance(center);

//         let min = bird.2.cohesion_radius.0;
//         let max = bird.2.cohesion_radius.1;

//         if !(min < center_distance && center_distance < max) {
//             continue;
//         } */
//         let pos_vec = bird.1.translation - center;

//         let center_force =
//             (1.0 / rng().random_range::<i32, _>(1..=10) as f32) * pos_vec.normalize();

//         if **show_gizmo {
//             gizmos.arrow(
//                 bird.1.translation,
//                 bird.1.translation - center_force.normalize(),
//                 PURPLE,
//             );
//         }

//         **bird.0 -= center_force;
//     }
//     */
//     // }
//     Ok(())
// }

// fn get_neighbor(
//     kd_tree: &Res<'_, BirdsKdTree>,
//     teaming: &Res<'_, Teaming>,
//     birds_map: &Res<BirdsCache>,
//     bird: &(Mut<'_, Velocity>, &Transform, &VisionRadius, Entity, &Team),
// ) -> Result<Box<[Vec3]>, BevyError> {
//     Ok(kd_tree
//         .query(&bird.1.translation.to_array())
//         // .nearest_n(
//         //     NonZeroUsize::new(rng().random_range::<u64, _>(3..20) as usize).ok_or_else(|| {
//         //         BevyError::warning(anyhow::format_err!(
//         //             "Cannot transform usize to NonZero variant"
//         //         ))
//         //     })?,
//         // )
//         .within::<SquaredEuclidean<_>>(bird.2.cohesion_radius.1)
//         .with_points()
//         .execute()
//         .into_iter()
//         .filter_map(|near_bird| {
//             if near_bird.item == bird.3.index_u32() {
//                 None
//             } else if bird.2.cohesion_radius.0 < near_bird.distance {
//                 let near_bird_entity = Entity::from_raw_u32(near_bird.item)?;
//                 let near_bird_info = birds_map.get(&near_bird_entity)?;
//                 if teaming.0 && &near_bird_info.team != bird.4 {
//                     None
//                 } else {
//                     Some(Vec3::from_array(near_bird.point))
//                 }
//             } else {
//                 None
//             }
//         })
//         .collect::<Box<[_]>>())
// }

fn toogle_cohesion(mut res: ResMut<IsCohersionActive>) {
    **res = !**res;
}

// fn toggle_timer(is_active: Res<IsCohersionActive>, mut timer: ResMut<CohesionTimer>) {
//     if **is_active {
//         timer.unpause();
//     } else {
//         timer.pause();
//     }
// }

fn toogle_cohesion_condition(key: Res<ButtonInput<KeyCode>>) -> bool {
    key.just_pressed(KeyCode::KeyC)
}

pub fn is_cohesion_active(res: Res<IsCohersionActive>) -> bool {
    **res
}

/*
fn toggle_gizmos(mut show_gizmo: ResMut<ShowCohesionForceGizmo>) {
    **show_gizmo = !**show_gizmo;
}

fn toogle_gismo_condition(key: Res<ButtonInput<KeyCode>>) -> bool {
    key.all_just_pressed([KeyCode::F1])
}
*/

#[derive(Debug, Component, Clone, Default)]
struct CohesionText;

fn add_ui(mut commands: Commands) {
    commands.spawn_scene(bsn! {
        #Theme_text
        Text("Cohersion actifivated. Use `C` to deactifivate.")
        UiTransform {
            translation: Val2 { y: Val::Px(40.0) }
        }
        CohesionText
    });
}

fn update_cohersion_text(
    is_cohersion_active: Res<IsCohersionActive>,
    mut text_query: Query<&mut Text, With<CohesionText>>,
) -> Result<(), BevyError> {
    let mut text = text_query.single_mut()?;
    if is_cohersion_active.0 {
        text.0 = String::from("Cohersion actifivated. Use `C` to deactifivate.");
    } else {
        text.0 = String::from("Cohersion deactifivated. Use `C` to actifivate.");
    }
    Ok(())
}

pub struct BirdCohesionPlugin;

impl Plugin for BirdCohesionPlugin {
    fn build(&self, app: &mut App) {
        app
        //     .add_systems(
        //     FixedUpdate,
        //     cohesion.run_if(is_cohesion_active.and_then(not(is_paused))),
        // )
        .add_systems(
            Update,
            (toogle_cohesion,
                // toggle_timer,
                update_cohersion_text)
                .chain()
                .run_if(toogle_cohesion_condition),
        )
        .insert_resource(IsCohersionActive(true))
        .add_systems(Startup, add_ui)
        // .insert_resource(CohesionTimer(Timer::new(
        //     Duration::from_millis(COHESION_TIMER),
        //     TimerMode::Repeating,
        // )))
        //.insert_resource(ShowCohesionForceGizmo(true))
        //.add_systems(Update, toggle_gizmos.run_if(toogle_gismo_condition))
        ;
    }
}
