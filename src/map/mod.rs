use bevy::{
    prelude::{
        AddAsset, AssetServer, Assets, Bundle, Commands, Component, ComputedVisibility,
        DespawnRecursiveExt, DetectChanges, Entity, EventWriter, GlobalTransform, Handle, Query,
        Res, Resource, Transform, Visibility, With,
    },
    sprite::{TextureAtlas, TextureAtlasSprite},
};
use bevy_rapier2d::prelude::{Collider, RigidBody};

pub use self::levels::Level;
use self::tile_map::{spawn_map_objects, MapData, MapEvent, MapObject};

mod collectable;
mod levels;
mod square;
mod tile_map;
pub struct MapPlugin;

impl bevy::prelude::Plugin for MapPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_startup_system(spawn_map)
            .add_event::<MapEvent>()
            .add_system(collectable::get_collectable)
            .add_system(spawn_map_objects)
            .init_resource::<MapData>()
            .add_asset::<Level>()
            .add_asset_loader(levels::LevelLoader)
            .init_resource::<LoadedLevel>()
            .add_system(load_map);
    }
}

#[derive(Debug, Resource)]
struct CurrentLevel(Handle<levels::Level>, bool);

fn spawn_map(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(LoadedLevel(asset_server.load("Levels/test.lvl.ron")));
}

#[derive(Bundle, Default)]
struct CellBundle {
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub visibility: Visibility,
    pub computed_visibility: ComputedVisibility,
    pub sprite: TextureAtlasSprite,
    pub texture_atlas: Handle<TextureAtlas>,
    pub collider: Collider,
    pub rigid_body: RigidBody,
    pub map_item: MapItem,
}

use crate::{ghost::GhostEvents, player::RealPlayer};

#[derive(Resource, Default)]
pub struct LoadedLevel(pub Handle<Level>);

#[derive(Component, Default)]
pub struct MapItem;

fn load_map(
    mut map_event: EventWriter<MapEvent>,
    levels: Res<Assets<Level>>,
    current_level: Res<LoadedLevel>,
    map_item: Query<Entity, With<MapItem>>,
    mut commands: Commands,
    mut events: EventWriter<GhostEvents>,
    mut player: Query<&mut Transform, With<RealPlayer>>,
) {
    if !current_level.is_changed() {
        return;
    }
    let Some(level) = levels.get(&current_level.0) else {return;};
    events.send(GhostEvents::ClearGhosts);
    events.send(GhostEvents::ClearTrail);
    let mut player = player.single_mut();
    player.translation = level.player_start.as_vec2().extend(0.0);
    for item in &map_item {
        commands.entity(item).despawn_recursive();
    }
    for obj in level.objects.iter() {
        map_event.send(MapEvent::Spawn(MapObject::clone(obj.as_ref())))
    }
}
