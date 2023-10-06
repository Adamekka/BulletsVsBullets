use bevy::prelude::*;
use camera::CameraPlugin;
use input::InputPlugin;
use world::WorldPlugin;

mod camera;
mod input;
mod world;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((CameraPlugin, InputPlugin, WorldPlugin));
    }
}
