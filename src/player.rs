use bevy::prelude::{
    error, Commands, Component, Entity, Local, Query, Res, SpriteSheetBundle, TextureAtlasSprite,
    Time, Transform, Vec2, Vec3, With, Without,
};
use leafwing_input_manager::{prelude::ActionState, InputManagerBundle};

use crate::{
    animation::{Animation, Animations, FrameTime, PhoxAnimationBundle},
    hit_box::{check_hit, HitBox},
    map::Trigger,
    user_input::PlayerInput,
};

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Grounded(pub bool);

pub fn ground_detection(
    mut player: Query<(&Transform, &mut Grounded), With<Player>>,
    mut last: Local<Transform>,
) {
    let (pos, mut is_grounded) = player.single_mut();

    let current = if pos.translation.y == last.translation.y {
        true
    } else {
        false
    };

    if current != is_grounded.0 {
        is_grounded.0 = current;
    }

    *last = *pos;
}

pub fn spawn_player(mut commands: Commands, animations: Res<Animations>) {
    let Some((texture_atlas, animation)) = animations.get(Animation::PlayerIdle) else {error!("Failed to find animation: Idle"); return;};
    commands.spawn((
        SpriteSheetBundle {
            texture_atlas,
            sprite: TextureAtlasSprite {
                index: 0,
                ..Default::default()
            },
            ..Default::default()
        },
        Player,
        PhoxAnimationBundle {
            animation,
            frame_time: FrameTime(0.),
        },
        Grounded(true),
        HitBox(Vec2::new(18., 32.)),
        InputManagerBundle {
            input_map: PlayerInput::player_one(),
            ..Default::default()
        },
    ));
}

#[derive(Component)]
pub struct Jump(f32);

const MOVE_SPEED: f32 = 100.;

pub fn move_player(
    mut commands: Commands,
    mut player: Query<
        (
            Entity,
            &mut Transform,
            &Grounded,
            &HitBox,
            &ActionState<PlayerInput>,
        ),
        With<Player>,
    >,
    hitboxs: Query<(&HitBox, &Transform), (Without<Player>, Without<Trigger>)>,
    time: Res<Time>,
) {
    let (entity, mut p_offset, grounded, &p_hitbox, input) = player.single_mut();
    let delat = if input.just_pressed(PlayerInput::Jump) && grounded.0 {
        commands.entity(entity).insert(Jump(100.));
        return;
    } else if input.pressed(PlayerInput::Left) {
        -MOVE_SPEED * time.delta_seconds() * (0.5 + (grounded.0 as u16) as f32)
    } else if input.pressed(PlayerInput::Right) {
        MOVE_SPEED * time.delta_seconds() * (0.5 + (grounded.0 as u16) as f32)
    } else {
        return;
    };
    let new_pos = p_offset.translation + Vec3::X * delat;
    for (&hitbox, offset) in &hitboxs {
        if check_hit(p_hitbox, new_pos, hitbox, offset.translation) {
            return;
        }
    }
    p_offset.translation = new_pos;
}

const FALL_SPEED: f32 = 98.0;

pub fn player_fall(
    mut player: Query<(&mut Transform, &HitBox), (With<Player>, Without<Jump>)>,
    hitboxs: Query<(&HitBox, &Transform), (Without<Player>, Without<Trigger>)>,
    time: Res<Time>,
) {
    let Ok((mut p_offset, &p_hitbox)) = player.get_single_mut() else {return;};
    let new_pos = p_offset.translation - Vec3::Y * FALL_SPEED * time.delta_seconds();
    for (&hitbox, offset) in &hitboxs {
        if check_hit(p_hitbox, new_pos, hitbox, offset.translation) {
            return;
        }
    }
    p_offset.translation = new_pos;
}

pub fn player_jump(
    mut commands: Commands,
    mut player: Query<(Entity, &mut Transform, &mut Jump, &ActionState<PlayerInput>), With<Player>>,
    time: Res<Time>,
) {
    let Ok((player, mut transform,mut jump, input)) = player.get_single_mut() else {return;};
    let jump_power = (time.delta_seconds() * FALL_SPEED * 2.).min(jump.0);
    transform.translation.y += jump_power;
    jump.0 -= if input.pressed(PlayerInput::Jump) {
        jump_power
    } else {
        jump_power * 2.
    };
    if jump.0 <= 0. {
        commands.entity(player).remove::<Jump>();
    }
}
