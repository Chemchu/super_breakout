use bevy::{
    DefaultPlugins,
    app::{App, FixedUpdate, Startup},
    ecs::{observer::ObserverSystemExt, schedule::SystemCondition},
    state::{app::AppExtStates, condition::in_state},
};
use bevy_enhanced_input::context::InputContextAppExt;

use crate::{
    paddle::{
        Paddle, on_action1, on_action2, on_action3, on_action4, on_paddle_move, on_pause_toggle,
        on_shoot_ball,
    },
    states::AppState,
    systems::on_died_event,
};

mod ball;
mod constants;
mod paddle;
mod states;
mod systems;
mod wall;

fn main() {
    let mut app = App::new();

    app.add_plugins((
        DefaultPlugins,
        avian2d::PhysicsPlugins::default(),
        bevy_enhanced_input::EnhancedInputPlugin,
    ))
    .init_state::<AppState>()
    .init_resource::<ball::BallPool>()
    .add_input_context::<Paddle>()
    .add_observer(on_shoot_ball)
    .add_observer(on_paddle_move)
    .add_observer(
        on_pause_toggle.run_if(in_state(AppState::InGame).or_else(in_state(AppState::Paused))),
    )
    .add_observer(on_action1.run_if(in_state(AppState::InGame)))
    .add_observer(on_action2.run_if(in_state(AppState::InGame)))
    .add_observer(on_action3.run_if(in_state(AppState::InGame)))
    .add_observer(on_action4.run_if(in_state(AppState::InGame)))
    .add_systems(Startup, systems::setup_game)
    .add_systems(FixedUpdate, systems::apply_linear_impulse)
    .add_observer(on_died_event)
    .run();
}
