pub mod bundle;
pub mod components;
pub mod constants;
pub mod systems;

use bevy::{
    app::{App, Plugin},
    ecs::observer::ObserverSystemExt,
    state::condition::in_state,
};
use bevy_enhanced_input::context::InputContextAppExt;

use crate::{
    common::game_states::AppState,
    paddle::{components::Paddle, systems::on_paddle_move},
};

pub struct PaddlePlugin;

impl Plugin for PaddlePlugin {
    fn build(&self, app: &mut App) {
        app.add_input_context::<Paddle>()
            .add_observer(on_paddle_move.run_if(in_state(AppState::InGame)));
        /* .add_observer(
            on_pause_toggle
                .run_if(in_state(AppState::InGame).or_else(in_state(AppState::Paused))),
        ); */
    }
}
