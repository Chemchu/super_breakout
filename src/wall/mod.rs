use bevy::app::{App, Plugin};
pub mod bundle;
pub mod components;
pub mod constants;

pub struct WallPlugin;

impl Plugin for WallPlugin {
    fn build(&self, _app: &mut App) {}
}
