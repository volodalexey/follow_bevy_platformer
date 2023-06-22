mod animation;
mod camera;
mod collectable;
mod map;
mod player;
mod tile_map;
mod user_input;

use bevy::prelude::{App, DefaultPlugins, ImagePlugin, PluginGroup, TextureAtlasSprite, Vec2};
use bevy_rapier2d::prelude::{NoUserData, RapierConfiguration, RapierPhysicsPlugin, TimestepMode};
use leafwing_input_manager::prelude::InputManagerPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugin(animation::PhoxAnimationPlugin)
        .add_plugin(player::PlayerPlugin)
        .add_startup_system(camera::spawn_cam)
        .add_startup_system(map::spawn_map)
        .add_system(collectable::get_collectable)
        .register_type::<TextureAtlasSprite>()
        .insert_resource(RapierConfiguration {
            gravity: Vec2::Y * -294.,
            timestep_mode: TimestepMode::Variable {
                max_dt: 1.0 / 60.0,
                time_scale: 1.0,
                substeps: 10,
            },
            ..Default::default()
        })
        .add_plugin(InputManagerPlugin::<user_input::PlayerInput>::default())
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(16.))
        .add_plugin(tile_map::MapPlugin)
        .run()
}
