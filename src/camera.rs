use bevy::prelude::{Camera2dBundle, Commands};

pub fn spawn_cam(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
