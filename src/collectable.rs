use bevy::prelude::{Commands, Component, Entity, Query, Transform, With, Without};

use crate::hit_box::{check_hit, HitBox};
use crate::player::Player;

#[derive(Component)]
pub struct Collectable;

pub fn get_collectable(
    player: Query<(&Transform, &HitBox), With<Player>>,
    triggers: Query<(Entity, &Transform, &HitBox), (With<Collectable>, Without<Player>)>,
    mut commands: Commands,
) {
    let (p_transform, &p_hitbox) = player.single();
    for (entity, t_transform, &t_hitbox) in &triggers {
        if check_hit(
            p_hitbox,
            p_transform.translation,
            t_hitbox,
            t_transform.translation,
        ) {
            commands.entity(entity).despawn();
        }
    }
}
