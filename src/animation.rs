use std::collections::HashMap;

use bevy::prelude::{
    App, AssetServer, Assets, Component, FromWorld, Handle, Mut, Plugin, Query, Res, Resource,
    TextureAtlas, TextureAtlasSprite, Time, Vec2, World, With, Input, KeyCode, error, Bundle
};

use crate::player::{Jump, Grounded, Player};

pub struct PhoxAnimationPlugin;

impl Plugin for PhoxAnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(animate_sprite)
            .add_system(change_player_animation)
            .init_resource::<Animations>();
    }
}

#[derive(Debug, Hash, PartialEq, Eq)]
pub enum Animation {
    PlayerRun,
    PlayerIdle,
    PlayerJump,
    PlayerFall,
    Strawberry,
}

#[derive(Component, Clone, Copy)]
pub struct SpriteAnimation {
    pub len: usize,
    pub frame_time: f32,
}

impl SpriteAnimation {
    fn new(len: usize, fps: usize) -> SpriteAnimation {
        SpriteAnimation { len, frame_time: 1./fps as f32 }
    }
}

#[derive(Component)]
pub struct FrameTime(pub f32);

#[derive(Bundle)]
pub struct PhoxAnimationBundle {
    pub animation: SpriteAnimation,
    pub frame_time: FrameTime,
}

pub fn animate_sprite(
    mut animations: Query<(&mut TextureAtlasSprite, &SpriteAnimation, &mut FrameTime)>,
    time: Res<Time>,
) {
    for (mut sprite, animation, mut frame_time) in animations.iter_mut() {
        frame_time.0 += time.delta_seconds();
        if frame_time.0 > animation.frame_time {
            let frames = (frame_time.0 / animation.frame_time) as usize;
            sprite.index += frames;
            if sprite.index >= animation.len {
                sprite.index %= animation.len;
            }
            frame_time.0 -= animation.frame_time;
        }
    }
}

#[derive(Resource)]
pub struct Animations {
    pub map: HashMap<Animation, (Handle<TextureAtlas>, SpriteAnimation)>,
}

impl FromWorld for Animations {
    fn from_world(world: &mut World) -> Self {
        let mut map = Animations {map: HashMap::new()};
        world.resource_scope(|world, mut texture_atles: Mut<Assets<TextureAtlas>>| {
            let asset_server = world.resource::<AssetServer>();
            let idel_atlas = TextureAtlas::from_grid(
                asset_server.load("Main Characters/Mask Dude/Idle (32x32).png"),
                Vec2::splat(32.),
                11, 1, None, None);
            map.add(Animation::PlayerIdle, texture_atles.add(idel_atlas), SpriteAnimation::new(11, 20));

            let run_atlas = TextureAtlas::from_grid(
                asset_server.load("Main Characters/Mask Dude/Run (32x32).png"),
                Vec2::splat(32.),
                12, 1, None, None);
            map.add(Animation::PlayerRun, texture_atles.add(run_atlas), SpriteAnimation::new(12, 20));

            let jump_atlas = TextureAtlas::from_grid(
                asset_server.load("Main Characters/Mask Dude/Jump (32x32).png"),
                Vec2::splat(32.),
                1, 1, None, None);
            map.add(Animation::PlayerJump, texture_atles.add(jump_atlas), SpriteAnimation::new(1, 1));

            let fall_atlas = TextureAtlas::from_grid(
                asset_server.load("Main Characters/Mask Dude/Fall (32x32).png"),
                Vec2::splat(32.),
                1, 1, None, None);
            map.add(Animation::PlayerFall, texture_atles.add(fall_atlas), SpriteAnimation::new(1,1));

            let strawberry_atlas = TextureAtlas::from_grid(
                asset_server.load("Items/Fruits/Strawberry.png"),
                Vec2::splat(32.),
                17, 1, None, None);
            map.add(Animation::Strawberry, texture_atles.add(strawberry_atlas), SpriteAnimation::new(17, 20));
        });

        map
    }
}

impl Animations {
    pub fn add(&mut self, id: Animation, handle: Handle<TextureAtlas>, animation: SpriteAnimation) {
        self.map.insert(id, (handle, animation));
    }
    pub fn get(&self, id: Animation) -> Option<(Handle<TextureAtlas>, SpriteAnimation)> {
        self.map.get(&id).cloned()
    }
}

pub fn change_player_animation(
    mut player: Query<(&mut Handle<TextureAtlas>, &mut SpriteAnimation, &mut TextureAtlasSprite), With<Player>>,
    player_jump: Query<(Option<&Jump>, &Grounded), With<Player>>,
    input: Res<Input<KeyCode>>,
    animations: Res<Animations>,
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
        Animation::PlayerJump
    //Falling if no on ground
    } else if !grounded.0 {
        Animation::PlayerFall
    // if any move keys pressed set run sprite
    } else if input.any_pressed([KeyCode::A, KeyCode::Left, KeyCode::D, KeyCode::Right]) {
        Animation::PlayerRun
    } else {
        Animation::PlayerIdle
    };

    let Some((new_atlas, new_animaiton)) = animations.get(set) else {error!("No Animation Jump Loaded"); return;};
    *atlas = new_atlas;
    sprite.index %= new_animaiton.len;
    *animation = new_animaiton;
}