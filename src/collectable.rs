use bevy::prelude::{Component, Query, Transform, With, Without};
use rand::Rng;

use crate::hit_box::{check_hit, HitBox};
use crate::player::Player;

#[derive(Component)]
pub struct Collectable;

pub fn get_collectable(
    player: Query<(&Transform, &HitBox), With<Player>>,
    mut triggers: Query<(&mut Transform, &HitBox), (With<Collectable>, Without<Player>)>,
) {
    let (p_transform, &p_hitbox) = player.single();
    for (mut t_transform, &t_hitbox) in &mut triggers {
        if check_hit(
            p_hitbox,
            p_transform.translation,
            t_hitbox,
            t_transform.translation,
        ) {
            t_transform.translation.x = rand::thread_rng().gen_range(-100.0..100.);
            t_transform.translation.y = rand::thread_rng().gen_range(-10.0..150.);
        }
    }
}
