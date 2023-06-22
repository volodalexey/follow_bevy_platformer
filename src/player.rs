use bevy::{
    prelude::{
        error, info, App, Changed, Commands, Component, Entity, IntoSystemConfig, Local, Name,
        Plugin, Query, Res, SpriteSheetBundle, TextureAtlasSprite, Transform, Vec2, With,
    },
    reflect::Reflect,
};
use bevy_rapier2d::prelude::{
    CoefficientCombineRule, Collider, Friction, LockedAxes, NoUserData, QueryFilter, RapierContext,
    RapierPhysicsPlugin, RigidBody, Velocity,
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
            .add_system(change_player)
            .add_system(double_jump.before(move_player))
            .add_system(ground_detection)
            .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(16.))
            .register_type::<Grounded>()
            .register_type::<Jump>()
            .register_type::<Player>();
    }
}

#[derive(Component, Reflect, PartialEq)]
pub enum Player {
    Mask,
    Ninja,
    Pink,
    Guy,
}

#[derive(Component, Reflect)]
pub struct Grounded(pub bool);

impl std::ops::BitAnd<bool> for Grounded {
    type Output = bool;
    fn bitand(self, rhs: bool) -> Self::Output {
        self.0 & rhs
    }
}

impl std::ops::BitAnd<&Grounded> for bool {
    type Output = bool;
    fn bitand(self, rhs: &Grounded) -> Self::Output {
        self & rhs.0
    }
}

pub fn ground_detection(
    mut player: Query<(&Transform, &mut Grounded), With<Player>>,
    mut last: Local<(f32, isize)>,
) {
    let (pos, mut on_ground) = player.single_mut();

    if (pos.translation.y * 1000.).round() == last.0 {
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

    last.0 = (pos.translation.y * 1000.).round();
}

pub fn spawn_player(mut commands: Commands, animations: Res<Animations>) {
    let Some((texture_atlas, animation)) = animations.get(Animation::MaskIdle) else {error!("Failed to find animation: Idle"); return;};
    commands.spawn((
        SpriteSheetBundle {
            texture_atlas,
            sprite: TextureAtlasSprite {
                index: 0,
                ..Default::default()
            },
            ..Default::default()
        },
        Player::Mask,
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
        Friction {
            coefficient: 5.,
            combine_rule: CoefficientCombineRule::Multiply,
        },
        Name::new("Player"),
    ));
}

#[derive(Component, Reflect)]
pub struct Jump(pub bool);

const MOVE_SPEED: f32 = 100.;

pub fn move_player(
    mut player: Query<
        (
            &mut Velocity,
            &ActionState<PlayerInput>,
            &Grounded,
            &Transform,
        ),
        With<Player>,
    >,
    rapier_context: Res<RapierContext>,
) {
    let (mut velocity, input, grounded, pos) = player.single_mut();
    if input.just_pressed(PlayerInput::Jump) & grounded {
        velocity.linvel.y = 100.;
    } else if input.just_pressed(PlayerInput::Fall) {
        velocity.linvel.y = velocity.linvel.y.min(0.0);
    } else if input.pressed(PlayerInput::Left) {
        let hit = rapier_context.cast_ray(
            pos.translation.truncate() + Vec2::new(-10., 16.),
            Vec2::NEG_Y,
            31.9,
            false,
            QueryFilter::exclude_dynamic().exclude_sensors(),
        );
        if hit.is_none() {
            velocity.linvel.x = -MOVE_SPEED;
        }
    } else if input.pressed(PlayerInput::Right) {
        let hit = rapier_context.cast_ray(
            pos.translation.truncate() + Vec2::new(10., 16.),
            Vec2::NEG_Y,
            31.9,
            false,
            QueryFilter::exclude_dynamic().exclude_sensors(),
        );
        if let Some(hit) = hit {
            info!("Player hit {:?}", hit.0);
            //velocity.linvel.x = 0.0
        }
        if hit.is_none() {
            velocity.linvel.x = MOVE_SPEED;
        }
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

fn change_player(mut query: Query<(&mut Player, &ActionState<PlayerInput>)>) {
    for (mut player, state) in &mut query {
        if state.just_pressed(PlayerInput::NextPlayer) {
            *player = match *player {
                Player::Mask => Player::Ninja,
                Player::Ninja => Player::Pink,
                Player::Pink => Player::Guy,
                Player::Guy => Player::Mask,
            };
        } else if state.just_pressed(PlayerInput::PevPlayer) {
            *player = match *player {
                Player::Mask => Player::Ninja,
                Player::Ninja => Player::Pink,
                Player::Pink => Player::Guy,
                Player::Guy => Player::Mask,
            };
        }
    }
}
