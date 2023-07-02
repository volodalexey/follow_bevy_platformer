use bevy::prelude::{Camera2dBundle, Commands, Component};

#[derive(Component)]
pub struct MainCam;

pub fn spawn_cam(mut commands: Commands) {
    commands.spawn((Camera2dBundle::default(), MainCam));
}
