use std::collections::HashMap;

use bevy::prelude::{
    AssetServer, Assets, Component, FromWorld, Handle, Query, Res, Resource, TextureAtlas,
    TextureAtlasSprite, Time, Vec2, World,
};

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

#[derive(Component)]
pub struct FrameTime(pub f32);

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
        let mut map = Animations {
            map: HashMap::new(),
        };
        let asset_server = world.resource::<AssetServer>();
        let idel_atlas = TextureAtlas::from_grid(
            asset_server.load("Main Characters/Mask Dude/Idle (32x32).png"),
            Vec2::splat(32.),
            11,
            1,
            None,
            None,
        );
        let run_atlas = TextureAtlas::from_grid(
            asset_server.load("Main Characters/Mask Dude/Run (32x32).png"),
            Vec2::splat(32.),
            12,
            1,
            None,
            None,
        );
        let jump_atlas = TextureAtlas::from_grid(
            asset_server.load("Main Characters/Mask Dude/Jump (32x32).png"),
            Vec2::splat(32.),
            1,
            1,
            None,
            None,
        );
        let fall_atlas = TextureAtlas::from_grid(
            asset_server.load("Main Characters/Mask Dude/Fall (32x32).png"),
            Vec2::splat(32.),
            1,
            1,
            None,
            None,
        );

        let mut texture_atles = world.resource_mut::<Assets<TextureAtlas>>();

        map.add(
            Animation::PlayerIdle,
            texture_atles.add(idel_atlas),
            SpriteAnimation {
                len: 11,
                frame_time: 1. / 10.,
            },
        );
        map.add(
            Animation::PlayerRun,
            texture_atles.add(run_atlas),
            SpriteAnimation {
                len: 12,
                frame_time: 1. / 10.,
            },
        );
        map.add(
            Animation::PlayerJump,
            texture_atles.add(jump_atlas),
            SpriteAnimation {
                len: 1,
                frame_time: 1.,
            },
        );
        map.add(
            Animation::PlayerFall,
            texture_atles.add(fall_atlas),
            SpriteAnimation {
                len: 1,
                frame_time: 1.,
            },
        );

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
