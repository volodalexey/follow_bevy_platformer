use bevy::{
    prelude::{
        error, App, Changed, Commands, Component, Entity, IntoSystemConfig, Local, Plugin, Query,
        Res, SpriteSheetBundle, TextureAtlasSprite, Transform, With,
    },
    reflect::Reflect,
};
use bevy_rapier2d::prelude::{
    Collider, LockedAxes, NoUserData, RapierPhysicsPlugin, RigidBody, Velocity,
};
use leafwing_input_manager::{prelude::ActionState, InputManagerBundle};

use crate::{
    animation::{Animation, Animations, FrameTime, PhoxAnimationBundle},
    user_input::PlayerInput,
};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_player)
            .add_system(move_player)
            .add_system(double_jump.before(move_player))
            .add_system(ground_detection)
            .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(16.))
            .register_type::<Grounded>()
            .register_type::<Jump>();
    }
}

#[derive(Component)]
pub struct Player;

#[derive(Component, Reflect)]
pub struct Grounded(pub bool);

pub fn ground_detection(
    mut player: Query<(&Transform, &mut Grounded), With<Player>>,
    mut last: Local<(f32, isize)>,
) {
    let (pos, mut on_ground) = player.single_mut();

    if (pos.translation.y * 100.).round() == last.0 {
        last.1 += 1;
    } else {
        last.1 -= 1;
    };
    last.1 = last.1.clamp(0, 3);

    if last.1 == 3 && !on_ground.0 {
        on_ground.0 = true;
    } else if last.1 == 0 && on_ground.0 {
        on_ground.0 = false;
    }

    last.0 = (pos.translation.y * 100.).round();
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
        InputManagerBundle {
            input_map: PlayerInput::player_one(),
            ..Default::default()
        },
        Jump(false),
        RigidBody::Dynamic,
        Velocity::default(),
        Collider::cuboid(9., 16.),
        LockedAxes::ROTATION_LOCKED_Z,
    ));
}

#[derive(Component, Reflect)]
pub struct Jump(pub bool);

const MOVE_SPEED: f32 = 100.;

pub fn move_player(
    mut player: Query<(&mut Velocity, &ActionState<PlayerInput>, &Grounded), With<Player>>,
) {
    let (mut velocity, input, grounded) = player.single_mut();
    if input.just_pressed(PlayerInput::Jump) && grounded.0 {
        velocity.linvel.y = 100.;
    } else if input.just_pressed(PlayerInput::Fall) {
        velocity.linvel.y = velocity.linvel.y.min(0.0);
    } else if input.pressed(PlayerInput::Left) {
        velocity.linvel.x = -MOVE_SPEED;
    } else if input.pressed(PlayerInput::Right) {
        velocity.linvel.x = MOVE_SPEED;
    } else if input.just_released(PlayerInput::Left) {
        velocity.linvel.x = 0.0;
    } else if input.just_released(PlayerInput::Right) {
        velocity.linvel.x = 0.0;
    };
}

pub fn double_jump(
    mut player: Query<(&mut Jump, &mut Velocity, &ActionState<PlayerInput>), With<Player>>,
    can_jump: Query<(Entity, &Grounded), Changed<Grounded>>,
) {
    for (entity, grounded) in &can_jump {
        if let Ok((mut jump, _, _)) = player.get_mut(entity) {
            if grounded.0 {
                jump.0 = true;
            }
        }
    }
    for (mut jump, mut velocity, input) in player.iter_mut() {
        if velocity.linvel.y.abs() < 0.01 {
            return;
        }
        if input.just_pressed(PlayerInput::Jump) && jump.0 {
            jump.0 = false;
            velocity.linvel.y = 100.;
        }
    }
}
