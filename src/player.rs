use bevy::prelude::{
    error, App, Commands, Component, Entity, Local, Plugin, Query, Res, SpriteSheetBundle,
    TextureAtlasSprite, Time, Transform, Vec2, Vec3, With, Without,
};
use leafwing_input_manager::{prelude::ActionState, InputManagerBundle};

use crate::{
    animation::{Animation, Animations, FrameTime, PhoxAnimationBundle},
    hit_box::{check_hit, HitBox},
    map::Trigger,
    user_input::PlayerInput,
};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_player)
            .add_system(move_player)
            .add_system(player_fall)
            .add_system(player_jump)
            .add_system(doubble_jump)
            .add_system(ground_detection);
    }
}

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
        Jump(0., false),
    ));
}

#[derive(Component)]
pub struct Jump(pub f32, pub bool);

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
        commands.entity(entity).insert(Jump(100., true));
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
    mut player: Query<(&mut Transform, &HitBox, &mut Jump), With<Player>>,
    hitboxs: Query<(&HitBox, &Transform), (Without<Player>, Without<Trigger>)>,
    time: Res<Time>,
) {
    let (mut p_offset, &p_hitbox, mut jump) = player.single_mut();
    if jump.0 > 0. {
        return;
    }
    let new_pos = p_offset.translation - Vec3::Y * FALL_SPEED * time.delta_seconds();
    for (&hitbox, offset) in &hitboxs {
        if check_hit(p_hitbox, new_pos, hitbox, offset.translation) {
            jump.1 = true;
            return;
        }
    }
    p_offset.translation = new_pos;
}

pub fn player_jump(
    mut player: Query<(&mut Transform, &mut Jump, &ActionState<PlayerInput>), With<Player>>,
    time: Res<Time>,
) {
    let (mut transform, mut jump, input) = player.single_mut();
    if jump.0 <= 0. {
        return;
    }
    let jump_power = (time.delta_seconds() * FALL_SPEED * 2.).min(jump.0);
    transform.translation.y += jump_power;
    jump.0 -= if input.pressed(PlayerInput::Fall) {
        jump.0
    } else {
        jump_power
    };
}

pub fn doubble_jump(mut player: Query<(&mut Jump, &ActionState<PlayerInput>), With<Player>>) {
    let (mut jump, input) = player.single_mut();
    if input.just_pressed(PlayerInput::Jump) && jump.1 {
        jump.0 = 100.;
        jump.1 = false;
    }
}
