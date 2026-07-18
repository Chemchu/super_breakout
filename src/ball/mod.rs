use bevy::app::{App, Plugin};

use crate::ball::{components::BallPool, systems::on_launch_ball_requested};

pub mod bundle;
pub mod components;
pub mod constants;
pub mod events;
pub mod systems;

pub struct BallPlugin;

impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<BallPool>()
            .add_observer(on_launch_ball_requested);
    }
}
