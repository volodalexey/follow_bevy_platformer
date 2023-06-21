use bevy::prelude::{
    AssetServer, Assets, BuildChildren, Color, Commands, Component, FromWorld, Handle, Res,
    Resource, SpriteSheetBundle, TextureAtlas, TextureAtlasSprite, Transform, Vec2, Vec3, World,
};

use crate::{
    animation::{Animation, Animations, FrameTime},
    collectable::Collectable,
    hit_box::HitBox,
};

#[derive(Component)]
pub struct Trigger;

pub fn spawn_map(
    mut commands: Commands,
    animations: Res<Animations>,
    terrain: Res<TerrainSprites>,
) {
    commands
        .spawn((
            SpriteSheetBundle {
                transform: Transform::from_translation(Vec3::NEG_Y * 32.),
                sprite: TextureAtlasSprite {
                    custom_size: Some(Vec2::new(168., 16.)),
                    color: Color::WHITE,
                    index: TerrainType::GoldStright as usize,
                    ..Default::default()
                },
                texture_atlas: terrain.get_atlas(),
                ..Default::default()
            },
            HitBox(Vec2::new(200., 16.)),
        ))
        .with_children(|p| {
            p.spawn(SpriteSheetBundle {
                transform: Transform::from_translation(Vec3::X * 92.),
                sprite: TextureAtlasSprite {
                    custom_size: Some(Vec2::new(16., 16.)),
                    color: Color::WHITE,
                    index: TerrainType::GoldRightEnd as usize,
                    ..Default::default()
                },
                texture_atlas: terrain.get_atlas(),
                ..Default::default()
            });
            p.spawn(SpriteSheetBundle {
                transform: Transform::from_translation(Vec3::NEG_X * 92.),
                sprite: TextureAtlasSprite {
                    custom_size: Some(Vec2::new(16., 16.)),
                    color: Color::WHITE,
                    index: TerrainType::GoldLeftEnd as usize,
                    ..Default::default()
                },
                texture_atlas: terrain.get_atlas(),
                ..Default::default()
            });
        });
    commands.spawn((
        SpriteSheetBundle {
            transform: Transform::from_translation(Vec3::new(110., 20., 0.)),
            sprite: TextureAtlasSprite {
                custom_size: Some(Vec2::new(32., 32.)),
                color: Color::WHITE,
                index: TerrainType::GoldLeftEnd as usize,
                ..Default::default()
            },
            texture_atlas: terrain.get_atlas(),
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
            HitBox(Vec2::new(16., 16.)),
            animation,
            FrameTime(0.0),
            Trigger,
            Collectable,
        ));
    }
}

#[derive(Resource)]
pub struct TerrainSprites(Handle<TextureAtlas>);

impl TerrainSprites {
    fn new(handle: Handle<TextureAtlas>) -> TerrainSprites {
        TerrainSprites(handle)
    }
    fn get_atlas(&self) -> Handle<TextureAtlas> {
        self.0.clone()
    }
}

impl FromWorld for TerrainSprites {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();
        let texture_atles = TextureAtlas::from_grid(
            asset_server.load("Terrain/Terrain (16x16).png"),
            Vec2::splat(16.),
            22,
            11,
            None,
            None,
        );
        let mut assets = world.resource_mut::<Assets<TextureAtlas>>();
        TerrainSprites::new(assets.add(texture_atles))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum TerrainType {
    GoldLeftEnd = 193,
    GoldStright = 194,
    GoldRightEnd = 195,
}
