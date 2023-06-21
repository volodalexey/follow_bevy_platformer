use bevy::prelude::{
    Color, Commands, Component, Res, Sprite, SpriteBundle, SpriteSheetBundle, Transform, Vec2, Vec3,
};

use crate::{
    animation::{Animation, Animations, FrameTime},
    hit_box::HitBox,
};

#[derive(Component)]
pub struct Trigger;

pub fn spawn_map(mut commands: Commands, animations: Res<Animations>) {
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_translation(Vec3::NEG_Y * 16.),
            sprite: Sprite {
                custom_size: Some(Vec2::new(200., 5.)),
                color: Color::WHITE,
                ..Default::default()
            },
            ..Default::default()
        },
        HitBox(Vec2::new(200., 5.)),
    ));
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_translation(Vec3::new(100., 25., 0.)),
            sprite: Sprite {
                custom_size: Some(Vec2::new(32., 32.)),
                color: Color::WHITE,
                ..Default::default()
            },
            ..Default::default()
        },
        HitBox(Vec2::new(32., 32.)),
    ));
    if let Some((texture_atlas, animation)) = animations.get(Animation::Strawberry) {
        commands.spawn((
            SpriteSheetBundle {
                transform: Transform::from_translation(Vec3::new(32., 16., 0.)),
                texture_atlas,
                ..Default::default()
            },
            HitBox(Vec2::new(32., 32.)),
            animation,
            FrameTime(0.0),
            Trigger,
        ));
    }
}
