use bevy::app::{App, Plugin};

use crate::ball::{
    components::{BallAssets, BallPool},
    systems::{
        on_double_ball_requested, on_launch_ball_requested, on_reverse_ball_requested,
        on_triple_ball_requested,
    },
};

pub mod bundle;
pub mod components;
pub mod constants;
pub mod events;
pub mod systems;

pub struct BallPlugin;

impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<BallPool>()
            .init_resource::<BallAssets>()
            .add_observer(on_launch_ball_requested)
            .add_observer(on_double_ball_requested)
            .add_observer(on_triple_ball_requested)
            .add_observer(on_reverse_ball_requested);
    }
}
