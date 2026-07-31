pub mod components;
pub mod constants;

use bevy::app::{App, Plugin};

pub struct WallPlugin;

impl Plugin for WallPlugin {
    fn build(&self, _app: &mut App) {}
}
