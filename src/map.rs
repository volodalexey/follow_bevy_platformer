use bevy::prelude::{Commands, EventWriter, IVec3, Res, SpriteSheetBundle, Transform, Vec3};
use bevy_rapier2d::prelude::{Collider, RigidBody, Sensor};

use crate::{
    animation::{Animation, Animations, FrameTime},
    collectable::Collectable,
    tile_map::{MapBox, MapEvent, TerrainMaterial},
};

pub fn spawn_map(
    mut commands: Commands,
    animations: Res<Animations>,
    mut map_event: EventWriter<MapEvent>,
) {
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
    for i in 0..5 {
        map_event.send(MapEvent::Spawn(Box::new(MapBox {
            offset: IVec3 {
                x: -7 - i,
                y: i,
                z: 1,
            },
            width: 1,
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
        offset: IVec3 { x: -6, y: 11, z: 1 },
        width: 1,
        hight: 5,
        material: TerrainMaterial::Gold,
    })));

    map_event.send(MapEvent::Spawn(Box::new(MapBox {
        offset: IVec3 { x: -10, y: 6, z: 1 },
        width: 2,
        hight: 2,
        material: TerrainMaterial::Gold,
    })));

    if let Some((texture_atlas, animation)) = animations.get(Animation::Strawberry) {
        commands.spawn((
            SpriteSheetBundle {
                transform: Transform::from_translation(Vec3::new(32., 16., 0.)),
                texture_atlas,
                ..Default::default()
            },
            animation,
            FrameTime(0.0),
            RigidBody::Fixed,
            Collider::ball(8.),
            Sensor,
            Collectable,
        ));
    }
}
