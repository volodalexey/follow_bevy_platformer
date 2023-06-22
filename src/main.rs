mod animation;
mod camera;
mod collectable;
mod map;
mod player;
mod user_input;

use bevy::prelude::{App, DefaultPlugins, ImagePlugin, PluginGroup, TextureAtlasSprite};
use leafwing_input_manager::prelude::InputManagerPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugin(animation::PhoxAnimationPlugin)
        .add_plugin(player::PlayerPlugin)
        .add_startup_system(camera::spawn_cam)
        .add_startup_system(map::spawn_map)
        .add_system(collectable::get_collectable)
        .init_resource::<map::TerrainSprites>()
        .register_type::<TextureAtlasSprite>()
        .add_plugin(InputManagerPlugin::<user_input::PlayerInput>::default())
        .run()
}
