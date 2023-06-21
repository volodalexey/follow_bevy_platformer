use std::collections::HashMap;

use bevy::prelude::{
    error, Commands, Component, Entity, Handle, Input, KeyCode, Query, Res, Resource,
    SpriteSheetBundle, TextureAtlas, TextureAtlasSprite, Time, Transform, Vec2, Vec3, With,
    Without, FromWorld, World, AssetServer, Assets, Local
};

use crate::{
    animation::{FrameTime, SpriteAnimation},
    hit_box::{check_hit, HitBox},
};

#[derive(Component)]
pub struct Player;

#[derive(Resource)]
pub struct PlayerAnimations {
    pub map: HashMap<Animation, (Handle<TextureAtlas>, SpriteAnimation)>,
}

impl FromWorld for PlayerAnimations {
    fn from_world(world: &mut World) -> Self {
        let mut map = PlayerAnimations {map: HashMap::new()};
        let asset_server = world.resource::<AssetServer>();
        let idel_atlas = TextureAtlas::from_grid(
            asset_server.load("Main Characters/Mask Dude/Idle (32x32).png"),
            Vec2::splat(32.),
            11, 1, None, None);
        let run_atlas = TextureAtlas::from_grid(
            asset_server.load("Main Characters/Mask Dude/Run (32x32).png"),
            Vec2::splat(32.),
            12, 1, None, None);
        let jump_atlas = TextureAtlas::from_grid(
            asset_server.load("Main Characters/Mask Dude/Jump (32x32).png"),
            Vec2::splat(32.),
            1, 1, None, None);
        let fall_atlas = TextureAtlas::from_grid(
            asset_server.load("Main Characters/Mask Dude/Fall (32x32).png"),
            Vec2::splat(32.),
            1, 1, None, None);
        
        let mut texture_atles = world.resource_mut::<Assets<TextureAtlas>>();
        
        map.add(Animation::Idle, texture_atles.add(idel_atlas), SpriteAnimation { len: 11, frame_time: 1./10. });
        map.add(Animation::Run, texture_atles.add(run_atlas), SpriteAnimation { len: 12, frame_time: 1./10. });
        map.add(Animation::Jump, texture_atles.add(jump_atlas), SpriteAnimation { len: 1, frame_time: 1. });
        map.add(Animation::Fall, texture_atles.add(fall_atlas), SpriteAnimation { len: 1, frame_time: 1. });

        map
    }
}

impl PlayerAnimations {
    fn add(&mut self, id: Animation, handle: Handle<TextureAtlas>, animation: SpriteAnimation) {
        self.map.insert(id, (handle, animation));
    }
    fn get(&self, id: Animation) -> Option<(Handle<TextureAtlas>, SpriteAnimation)> {
        self.map.get(&id).cloned()
    }
}

#[derive(Debug, Hash, PartialEq, Eq)]
pub enum Animation {
    Run,
    Idle,
    Jump,
    Fall,
}

#[derive(Component)]
pub struct Grounded(bool);

pub fn ground_detection(
    mut player: Query<(&Transform, &mut Grounded), With<Player>>,
    mut last: Local<Transform>,
) {
    let (pos,mut on_ground) = player.single_mut();

    let current = if pos.translation.y == last.translation.y {
        true
    } else {
        false
    };

    if current != on_ground.0 {
        on_ground.0 = current;
    }

    *last = *pos;
}

pub fn spawn_player(mut commands: Commands, animaitons: Res<PlayerAnimations>) {
    let Some((texture_atlas, animation)) = animaitons.get(Animation::Idle) else {error!("Failed to find animation: Idle"); return;};
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
        animation,
        FrameTime(0.0),
        Grounded(true),
        HitBox(Vec2::splat(32.)),
    ));
}

#[derive(Component)]
pub struct Jump(f32);

const MOVE_SPEED: f32 = 100.;

pub fn move_player(
    mut commands: Commands,
    mut player: Query<(Entity, &mut Transform, &Grounded, &HitBox), With<Player>>,
    hitboxs: Query<(&HitBox, &Transform), Without<Player>>,
    time: Res<Time>,
    input: Res<Input<KeyCode>>,
) {
    let (entity, mut p_offset, grounded, &p_hitbox) = player.single_mut();
    let delat = if input.any_just_pressed([KeyCode::W, KeyCode::Up, KeyCode::Space]) && grounded.0 {
        commands.entity(entity).insert(Jump(100.));
        return;
    } else if input.any_pressed([KeyCode::A, KeyCode::Left]) {
        -MOVE_SPEED * time.delta_seconds() * (0.5 + (grounded.0 as u16) as f32)
    } else if input.any_pressed([KeyCode::D, KeyCode::Right]) {
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

pub fn change_player_animation(
    mut player: Query<(&mut Handle<TextureAtlas>, &mut SpriteAnimation, &mut TextureAtlasSprite), With<Player>>,
    player_jump: Query<(Option<&Jump>, &Grounded), With<Player>>,
    input: Res<Input<KeyCode>>,
    animaitons: Res<PlayerAnimations>,
) {
    let (mut atlas, mut animation, mut sprite) = player.single_mut();
    let (jump, grounded) = player_jump.single();
    if input.any_just_pressed([KeyCode::A, KeyCode::Left]) {
        sprite.flip_x = true;
    } else if input.any_just_pressed([KeyCode::D, KeyCode::Right])
    && !input.any_pressed([KeyCode::A, KeyCode::Left]) {
        sprite.flip_x = false;
    } else if input.any_just_released([KeyCode::A, KeyCode::Left])
    && !input.any_pressed([KeyCode::A, KeyCode::Left])
    && input.any_pressed([KeyCode::D, KeyCode::Right]) {
        sprite.flip_x = false;
    }
    
    let set = 
    //Jumping if jump
    if jump.is_some() {
        Animation::Jump
    //Falling if no on ground
    } else if !grounded.0 {
        Animation::Fall
    // if any move keys pressed set run sprite
    } else if input.any_pressed([KeyCode::A, KeyCode::Left, KeyCode::D, KeyCode::Right]) {
        Animation::Run
    } else {
        Animation::Idle
    };

    let Some((new_atlas, new_animaiton)) = animaitons.get(set) else {error!("No Animation Jump Loaded"); return;};
    *atlas = new_atlas;
    sprite.index %= new_animaiton.len;
    *animation = new_animaiton;
}

const FALL_SPEED: f32 = 98.0;

pub fn player_fall(
    mut player: Query<(&mut Transform, &HitBox), (With<Player>, Without<Jump>)>,
    hitboxs: Query<(&HitBox, &Transform), Without<Player>>,
    time: Res<Time>,
) {
    let Ok((mut p_offset, &p_hitbox)) = player.get_single_mut() else {return;};
    let new_pos = p_offset.translation - Vec3::Y * FALL_SPEED * time.delta_seconds();
    for (&hitbox, offset) in &hitboxs {
        if check_hit(p_hitbox, new_pos, hitbox, offset.translation) {return;}
    }
    p_offset.translation = new_pos;
}

pub fn player_jump(
    mut commands: Commands,
    mut player: Query<(Entity, &mut Transform, &mut Jump), With<Player>>,
    time: Res<Time>,
) {
    let Ok((player, mut transform,mut jump)) = player.get_single_mut() else {return;};
    let jump_power = (time.delta_seconds() * FALL_SPEED * 2.).min(jump.0);
    jump.0 -= jump_power;
    transform.translation.y += jump_power;
    if jump.0 == 0. {
        commands.entity(player).remove::<Jump>();
    }
}