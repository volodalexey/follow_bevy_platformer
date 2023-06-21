mod animation;
mod camera;
mod collectable;
mod hit_box;
mod map;
mod player;
mod user_input;

use bevy::prelude::{App, DefaultPlugins, ImagePlugin, PluginGroup, TextureAtlasSprite};
use leafwing_input_manager::prelude::InputManagerPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugin(animation::PhoxAnimationPlugin)
        .add_startup_system(camera::spawn_cam)
        .add_startup_system(player::spawn_player)
        .add_system(player::move_player)
        .add_system(player::player_fall)
        .add_system(player::player_jump)
        .add_system(player::ground_detection)
        .add_startup_system(map::spawn_map)
        .add_system(collectable::get_collectable)
        .init_resource::<map::TerrainSprites>()
        .register_type::<TextureAtlasSprite>()
        .add_plugin(InputManagerPlugin::<user_input::PlayerInput>::default())
        .run()
}
