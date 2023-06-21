mod animation;
mod camera;
mod collectable;
mod hit_box;
mod map;
mod player;

use bevy::prelude::{App, DefaultPlugins, ImagePlugin, PluginGroup, TextureAtlasSprite};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_startup_system(camera::spawn_cam)
        .add_startup_system(player::spawn_player)
        .add_system(animation::animate_sprite)
        .add_system(player::move_player)
        .add_system(player::change_player_animation)
        .init_resource::<animation::Animations>()
        .add_system(player::player_fall)
        .add_system(player::player_jump)
        .add_system(player::ground_detection)
        .add_startup_system(map::spawn_map)
        .add_system(collectable::get_collectable)
        .init_resource::<map::TerrainSprites>()
        .register_type::<TextureAtlasSprite>()
        .run()
}
