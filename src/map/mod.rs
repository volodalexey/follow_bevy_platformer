use bevy::{
    prelude::{
        AddAsset, AssetServer, Assets, Bundle, Commands, ComputedVisibility, EventWriter,
        GlobalTransform, Handle, Res, ResMut, Resource, Transform, Visibility,
    },
    sprite::{TextureAtlas, TextureAtlasSprite},
};
use bevy_rapier2d::prelude::{Collider, RigidBody};

use self::levels::Level;
use self::tile_map::{spawn_map_objects, MapData, MapEvent, MapObject};

mod collectable;
mod levels;
mod square;
mod tile_map;
pub struct MapPlugin;

impl bevy::prelude::Plugin for MapPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_startup_system(spawn_map)
            .add_system(update_map)
            .add_event::<MapEvent>()
            .add_system(collectable::get_collectable)
            .add_system(spawn_map_objects)
            .init_resource::<MapData>()
            .add_asset::<Level>()
            .add_asset_loader(levels::LevelLoader);
    }
}

#[derive(Debug, Resource)]
struct CurrentLevel(Handle<levels::Level>, bool);

fn spawn_map(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(CurrentLevel(
        asset_server.load("Levels/test.lvl.ron"),
        false,
    ));
}

fn update_map(
    mut map_event: EventWriter<MapEvent>,
    levels: Res<Assets<Level>>,
    mut current_level: ResMut<CurrentLevel>,
) {
    if current_level.1 {
        return;
    }
    let Some(level) = levels.get(&current_level.0) else {return;};
    for obj in level.objects.iter() {
        map_event.send(MapEvent::Spawn(MapObject::clone(obj.as_ref())))
    }
    current_level.1 = true;
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
}
