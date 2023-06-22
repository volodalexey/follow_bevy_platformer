use bevy::prelude::{Component, Entity, EventWriter, Query, Res, Transform, With};
use bevy_rapier2d::prelude::RapierContext;
use rand::Rng;

use crate::{ghost::GhostEvents, player::RealPlayer};

#[derive(Component)]
pub struct Collectable;

pub fn get_collectable(
    player: Query<Entity, With<RealPlayer>>,
    mut collectables: Query<&mut Transform, With<Collectable>>,
    rapier_context: Res<RapierContext>,
    mut events: EventWriter<GhostEvents>,
) {
    let entity = player.single();

    /* Iterate through all the intersection pairs involving a specific collider. */
    for (collider1, collider2, intersecting) in rapier_context.intersections_with(entity) {
        if intersecting {
            println!(
                "The entities {:?} and {:?} have intersecting colliders!",
                collider1, collider2
            );
            if let Ok(mut pos) = collectables.get_mut(collider2) {
                pos.translation.x = rand::thread_rng().gen_range(-100.0..100.);
                pos.translation.y = rand::thread_rng().gen_range(-10.0..150.);
                events.send(GhostEvents::SpawnGhost);
            }
            if let Ok(mut pos) = collectables.get_mut(collider1) {
                pos.translation.x = rand::thread_rng().gen_range(-100.0..100.);
                pos.translation.y = rand::thread_rng().gen_range(-10.0..150.);
                events.send(GhostEvents::SpawnGhost);
            }
        }
    }
}
