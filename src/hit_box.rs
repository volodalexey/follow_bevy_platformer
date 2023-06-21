use bevy::prelude::{Component, Vec2, Vec3};

#[derive(Debug, Component, Clone, Copy)]
pub struct HitBox(pub Vec2);

pub fn check_hit(hitbox: HitBox, offset: Vec3, other_hitbox: HitBox, other_offset: Vec3) -> bool {
    let h_size = hitbox.0.y / 2.;
    let oh_size = other_hitbox.0.y / 2.;
    let w_size = hitbox.0.x / 2.;
    let ow_size = other_hitbox.0.x / 2.;

    offset.x + w_size > other_offset.x - ow_size
        && offset.x - w_size < other_offset.x + ow_size
        && offset.y + h_size > other_offset.y - oh_size
        && offset.y - h_size < other_offset.y + oh_size
}
