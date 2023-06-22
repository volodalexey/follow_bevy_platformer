mod animation;
mod camera;
mod collectable;
mod ghost;
mod map;
mod player;
mod tile_map;
mod user_input;

use bevy::prelude::{
    App, DefaultPlugins, ImagePlugin, PluginGroup, Resource, TextureAtlasSprite, Vec2,
};
use bevy_rapier2d::prelude::{NoUserData, RapierConfiguration, RapierPhysicsPlugin, TimestepMode};
use leafwing_input_manager::prelude::InputManagerPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugin(animation::PhoxAnimationPlugin)
        .add_startup_system(camera::spawn_cam)
        .add_startup_system(map::spawn_map)
        .add_system(collectable::get_collectable)
        .register_type::<TextureAtlasSprite>()
        .insert_resource(RapierConfiguration {
            gravity: Vec2::Y * -294.,
            timestep_mode: TimestepMode::Fixed {
                dt: 1. / 60.,
                substeps: 1,
            },
            ..Default::default()
        })
        .add_plugin(InputManagerPlugin::<user_input::PlayerInput>::default())
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(16.))
        .add_plugin(player::PlayerPlugin)
        .add_plugin(tile_map::MapPlugin)
        .add_plugin(ghost::GhostPlugin)
        .insert_resource(Score(0))
        .run()
}

#[derive(Resource)]
pub struct Score(usize);
