use bevy::prelude::*;
use bevy_fps_counter::FpsCounterPlugin;
use camera::CameraPlugin;
use input::InputPlugin;
use player::PlayerPlugin;
use world::WorldPlugin;

mod camera;
mod input;
mod player;
mod world;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            // External
            .add_plugins(FpsCounterPlugin)
            // Mine
            .add_plugins((CameraPlugin, InputPlugin, PlayerPlugin, WorldPlugin));
    }
}
