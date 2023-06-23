use bevy::{
    prelude::{
        Bundle, ComputedVisibility, EventWriter, GlobalTransform, Handle, IVec3, Transform,
        Visibility,
    },
    sprite::{TextureAtlas, TextureAtlasSprite},
};
use bevy_rapier2d::prelude::{Collider, RigidBody};

use self::{
    square::MapBox,
    tile_map::{spawn_map_objects, MapData, MapEvent, TerrainMaterial},
};

mod collectable;
mod square;
mod tile_map;
pub struct MapPlugin;

impl bevy::prelude::Plugin for MapPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_startup_system(spawn_map)
            .add_event::<MapEvent>()
            .add_system(collectable::get_collectable)
            .add_system(spawn_map_objects)
            .init_resource::<MapData>();
    }
}

fn spawn_map(mut map_event: EventWriter<MapEvent>) {
    map_event.send(MapEvent::Spawn(Box::new(MapBox {
        offset: IVec3 { x: -6, y: -1, z: 1 },
        width: 13,
        hight: 1,
        material: TerrainMaterial::Gold,
    })));
    map_event.send(MapEvent::Spawn(Box::new(MapBox {
        offset: IVec3 { x: 7, y: 1, z: 1 },
        width: 2,
        hight: 2,
        material: TerrainMaterial::Gold,
    })));
    map_event.send(MapEvent::Spawn(Box::new(MapBox {
        offset: IVec3 { x: 7, y: 1, z: 1 },
        width: 1,
        hight: 1,
        material: TerrainMaterial::Clay,
    })));
    for i in 0..5 {
        map_event.send(MapEvent::Spawn(Box::new(MapBox {
            offset: IVec3 {
                x: -11,
                y: 4 - i,
                z: 1,
            },
            width: 1 + i,
            hight: 1,
            material: TerrainMaterial::Gold,
        })));
    }

    for i in 0..5 {
        map_event.send(MapEvent::Spawn(Box::new(MapBox {
            offset: IVec3 {
                x: i * 2,
                y: 15,
                z: 1,
            },
            width: 1,
            hight: 1,
            material: TerrainMaterial::Brick,
        })));
    }

    map_event.send(MapEvent::Spawn(Box::new(MapBox {
        offset: IVec3 { x: -5, y: 10, z: 1 },
        width: 1,
        hight: 4,
        material: TerrainMaterial::Gold,
    })));

    map_event.send(MapEvent::Spawn(Box::new(MapBox {
        offset: IVec3 { x: -6, y: 9, z: 1 },
        width: 1,
        hight: 5,
        material: TerrainMaterial::Gold,
    })));
    map_event.send(MapEvent::Spawn(Box::new(MapBox {
        offset: IVec3 { x: -6, y: 9, z: 1 },
        width: 1,
        hight: 1,
        material: TerrainMaterial::Clay,
    })));

    map_event.send(MapEvent::Spawn(Box::new(MapBox {
        offset: IVec3 { x: -10, y: 6, z: 1 },
        width: 2,
        hight: 2,
        material: TerrainMaterial::Gold,
    })));

    map_event.send(MapEvent::Spawn(Box::new(MapBox {
        offset: IVec3 { x: -2, y: 7, z: 1 },
        width: 5,
        hight: 1,
        material: TerrainMaterial::Copper,
    })));

    map_event.send(MapEvent::Spawn(Box::new(MapBox {
        offset: IVec3 { x: -2, y: 8, z: 1 },
        width: 4,
        hight: 1,
        material: TerrainMaterial::Iron,
    })));
    /*
    map_event.send(MapEvent::Spawn(Collectable {
        collectable_type: CollectableType::Strawberry,
        spawn_type: SpawnType::Fixed(IVec2::new(2, 1)),
    }));

    map_event.send(MapEvent::Spawn(Collectable {
        collectable_type: CollectableType::Bananan,
        spawn_type: SpawnType::RandomRange(IVec2::new(-10, 0), IVec2::new(10, 20)),
    }));
    */
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
