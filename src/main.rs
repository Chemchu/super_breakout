use bevy::{
    DefaultPlugins,
    app::{App, FixedUpdate, Startup},
};
use bevy_enhanced_input::context::InputContextAppExt;

use crate::paddle::{Paddle, on_ball_and_paddle_collision, on_paddle_move, on_shoot_ball};

mod ball;
mod constants;
mod paddle;
mod systems;
mod wall;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            avian2d::PhysicsPlugins::default(),
            bevy_enhanced_input::EnhancedInputPlugin,
        ))
        .init_resource::<ball::BallPool>()
        .add_input_context::<Paddle>()
        .add_observer(on_shoot_ball)
        .add_observer(on_paddle_move)
        .add_observer(on_ball_and_paddle_collision)
        .add_systems(Startup, systems::setup_game)
        .add_systems(FixedUpdate, systems::apply_linear_impulse)
        .run();
}
